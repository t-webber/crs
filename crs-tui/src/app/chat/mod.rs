//! Main page displayed with the chats

mod current_room;
mod menu;

extern crate alloc;
use alloc::sync::Arc;
use core::convert::Infallible;
use core::time::Duration;
use std::sync::Mutex;
use std::thread;

use crs_backend::room::DisplayRoom;
use crs_backend::user::User;
use ratatui::Frame;
use ratatui::crossterm::event::{Event, KeyModifiers};
use ratatui::layout::{Constraint, Direction, Layout, Rect};

use crate::app::chat::current_room::{
    CreateRoomAction, CurrentRoom, UpdateCurrentRoomPanel
};
use crate::app::chat::menu::{ROOM_LIST_WIDTH, RoomList};
use crate::ui::component::Component;
use crate::utils::safe_unlock;

/// This page renders and gives the user an interface to list the chat and
/// communicate in those chats.
pub struct ChatPage {
    /// Currently opened room
    current_room: CurrentRoom,
    /// Menu with the list of rooms
    menu:         RoomList,
    /// Rooms visible by the user
    rooms:        Arc<Mutex<Vec<Arc<Mutex<DisplayRoom>>>>>,
    /// User to interact with matrix server
    user:         Arc<User>,
}

impl ChatPage {
    /// Create a new chat page with the given logged in user
    ///
    /// The rooms and their content will load in the background.
    pub fn new(user: Arc<User>) -> Self {
        let rooms = Arc::new(Mutex::new(vec![]));
        let mut menu = RoomList::new(Arc::clone(&rooms));
        menu.end_loading(); // TODO
        let this =
            Self { rooms, user, menu, current_room: CurrentRoom::default() };
        this.synchronise_rooms();
        this
    }

    /// Synchronise the existing rooms, including name and messages
    fn synchronise_rooms(&self) {
        let rooms = Arc::clone(&self.rooms);
        let user = Arc::clone(&self.user);
        let _handle = tokio::spawn(async move {
            loop {
                let local_rooms = Arc::clone(&rooms);
                let on_room_load = move |new_room: DisplayRoom| {
                    let new_room_id = new_room.id();
                    if let Some(old_room) = safe_unlock(&local_rooms)
                        .iter_mut()
                        .find(|room| safe_unlock(room).id() == new_room_id)
                    {
                        safe_unlock(old_room).update_from(new_room);
                    } else {
                        safe_unlock(&local_rooms)
                            .push(Arc::new(Mutex::new(new_room)));
                    }
                };
                user.load_rooms(on_room_load).await.unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });
    }
}

impl Component for ChatPage {
    type ResponseData = Infallible;
    type UpdateState = Infallible;

    fn draw(&self, frame: &mut Frame<'_>, area: Rect) {
        let room_list_width = *ROOM_LIST_WIDTH;
        let constraints: &[Constraint] =
            if area.width >= room_list_width.saturating_mul(2) {
                &[Constraint::Length(*ROOM_LIST_WIDTH), Constraint::Fill(1)]
            } else if area.width > 40 {
                &[Constraint::Percentage(50), Constraint::Percentage(50)]
            } else {
                &[Constraint::Fill(1)]
            };

        let layout =
            Layout::new(Direction::Horizontal, constraints).split(area);

        if constraints.len() == 2 {
            self.menu.draw(frame, layout[0]);
            self.current_room.draw(frame, layout[1]);
        } else {
            self.current_room.draw(frame, layout[0]);
        }
    }

    async fn on_event(&mut self, event: Event) -> Option<Self::UpdateState> {
        let key_event = event.as_key_press_event()?;
        if key_event.code.is_char('k')
            && key_event.modifiers & KeyModifiers::CONTROL
                == KeyModifiers::CONTROL
        {
            let update_data =
                UpdateCurrentRoomPanel::Search(Arc::clone(&self.rooms));
            self.current_room.update(update_data);
            return None;
        }

        if let Some(index) = self.menu.on_event(event.clone()).await {
            let new_room = Arc::clone(&safe_unlock(&self.rooms)[index]);
            self.current_room.update(UpdateCurrentRoomPanel::NewRoom(new_room));
        } else {
            let CreateRoomAction(name) =
                self.current_room.on_event(event).await?;
            let room_name = if name.is_empty() { None } else { Some(name) };
            match self.user.create_room_with_name(room_name).await {
                Ok(new_matrix_room) => loop {
                    let tui_rooms = safe_unlock(&self.rooms);
                    let Some(tui_room) = tui_rooms.iter().find(|tui_room| {
                        safe_unlock(tui_room).id() == new_matrix_room.room_id()
                    }) else {
                        drop(tui_rooms);
                        thread::sleep(Duration::from_secs(1));
                        continue;
                    };
                    let current_room = Arc::clone(tui_room);
                    drop(tui_rooms);
                    self.current_room
                        .update(UpdateCurrentRoomPanel::NewRoom(current_room));
                    break;
                },
                Err(err) => self
                    .current_room
                    .update(UpdateCurrentRoomPanel::Error(err.to_string())),
            }
        }
        None
    }
}
