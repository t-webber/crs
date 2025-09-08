//! Main page displayed with the chats

mod menu;
extern crate alloc;
use alloc::sync::Arc;
use std::sync::Mutex;

use backend::room::DisplayRoom;
use backend::user::User;
use ratatui::Frame;
use ratatui::crossterm::event::Event;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::Text;

use crate::app::chat::menu::RoomList;
use crate::ui::component::Component;

/// This page renders and gives the user an interface to list the chat and
/// communicate in those chats.
pub struct ChatPage {
    /// Menu with the list of rooms
    menu:      RoomList,
    /// Room currently opened in the chat panel
    open_room: usize,
    /// Rooms visible by the user
    rooms:     Arc<Mutex<Vec<DisplayRoom>>>,
    /// User to interact with matrix server
    user:      Arc<User>,
}

impl ChatPage {
    /// Create a new chat page with the given logged in user
    ///
    /// The rooms and their content will load in the background.
    pub fn new(user: Arc<User>) -> Self {
        let rooms = Arc::new(Mutex::new(vec![]));
        let rooms_adder = Arc::clone(&rooms);
        let user_adder = Arc::clone(&user);
        let _handle = tokio::spawn(async move {
            let on_room_load =
                move |room| rooms_adder.lock().unwrap().push(room);
            user_adder.load_rooms(on_room_load).await
        });
        let menu = RoomList::new(Arc::clone(&rooms));
        Self { rooms, user, open_room: 0, menu }
    }
}

impl Component for ChatPage {
    type ResponseData = ();
    type UpdateState = ();

    fn draw(&self, frame: &mut Frame, area: Rect) {
        let layout = Layout::new(Direction::Horizontal, [
            Constraint::Percentage(30),
            Constraint::Percentage(70),
        ])
        .split(area);

        self.menu.draw(frame, layout[0]);

        let rooms = self.rooms.lock().unwrap();

        let open_room_name = rooms.get(self.open_room).map_or_else(
            || Text::from("Loading..."),
            |room| match room.as_name() {
                Ok(name) => Text::from(name.as_str()),
                Err(err) => Text::from(format!("Failed to load room: {err}"))
                    .style(Style::default().fg(Color::Red)),
            },
        );

        frame.render_widget(open_room_name, layout[1]);
        drop(rooms);
    }

    async fn on_event(&mut self, event: Event) -> Option<Self::UpdateState> {
        self.open_room = self.menu.on_event(event).await?;
        None
    }
}
