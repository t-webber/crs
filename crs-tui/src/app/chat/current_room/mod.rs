//! Current display in the chat panel

mod discussion;
mod invited;
mod search;

extern crate alloc;
use alloc::sync::Arc;
use core::convert::Infallible;
use core::mem::take;
use std::sync::Mutex;

use crs_backend::room::DisplayRoom;
use ratatui::Frame;
use ratatui::crossterm::event::Event;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::Text;
use ratatui::widgets::{Paragraph, Wrap};

use crate::app::chat::current_room::discussion::Discussion;
use crate::app::chat::current_room::invited::{
    AcceptInvitation, InvitationPopup
};
use crate::app::chat::current_room::search::RoomSearch;
use crate::ui::component::Component;
use crate::ui::widgets::{
    InstructionsBuilder, fully_centered_content, saturating_cast
};
use crate::utils::safe_unlock;

/// Chat panel with the currently selected room, or the messages for errors and
/// popups if there are any.
#[derive(Default)]
pub struct CurrentRoom {
    /// Content currently displayed on the screen
    child:     CurrentRoomChild,
    /// Name of the room that is currently selected
    room_name: String,
}

impl CurrentRoom {
    /// Accept invitation and display the new status.
    async fn accept_invitation(&mut self, room: Arc<Mutex<DisplayRoom>>) {
        let room_handle = safe_unlock(&room).into_room();
        match room_handle.accept_invitation().await {
            Err(err) =>
                self.child = CurrentRoomChild::Error(err.to_string(), room),
            Ok(new_room) => {
                *safe_unlock(&room) = new_room;
                self.select_new_room(room);
            }
        }
    }

    /// Draws the error at the center of the chat panel
    #[expect(clippy::arithmetic_side_effects, reason = "width >= 20")]
    fn draw_error(err_msg: &str, frame: &mut Frame<'_>, area: Rect) {
        debug_assert!(area.width >= 20, "terminal too small");

        let err_len = saturating_cast(err_msg.len());
        let rect = fully_centered_content(err_len, area.width - 4, area);

        let err_widget = Paragraph::new(err_msg)
            .style(Style::new().fg(Color::Red))
            .wrap(Wrap { trim: true })
            .centered();

        frame.render_widget(err_widget, rect);
    }

    /// Displays the name of the room at the top of the chat panel
    fn draw_room_name(&self, frame: &mut Frame<'_>, area: Rect) {
        let room_name_widget = Text::from(self.room_name.as_str())
            .style(Style::new().fg(Color::Yellow))
            .alignment(Alignment::Center);

        frame.render_widget(room_name_widget, area);
    }

    /// Open a new room in the discussion panel
    fn select_new_room(&mut self, room: Arc<Mutex<DisplayRoom>>) {
        let room_handle = safe_unlock(&room);
        self.room_name = room_handle.as_name().map_or_else(
            |_| String::from("<unknown channel>"),
            ToOwned::to_owned,
        );
        if room_handle.has_invitation() {
            drop(room_handle);
            self.child = CurrentRoomChild::Invited(InvitationPopup, room);
        } else if let Err(err) = room_handle.as_messages() {
            let err_msg = err.to_string();
            drop(room_handle);
            self.child = CurrentRoomChild::Error(err_msg, room);
        } else if !self.child.room_is(&room_handle) {
            drop(room_handle);
            self.child = CurrentRoomChild::Discussion(Discussion::new(room));
        }
    }
}

impl Component for CurrentRoom {
    type ResponseData = UpdateCurrentRoomPanel;
    type UpdateState = Infallible;

    fn draw(&self, frame: &mut Frame<'_>, area: Rect) {
        let constraints: &[Constraint] = if self.child.is_discussion() {
            &[Constraint::Length(1), Constraint::Fill(1)]
        } else {
            &[Constraint::Length(1), Constraint::Fill(1), Constraint::Length(1)]
        };

        let layout = Layout::new(Direction::Vertical, constraints).split(area);
        self.draw_room_name(frame, layout[0]);

        match &self.child {
            CurrentRoomChild::Discussion(child) => child.draw(frame, layout[1]),
            CurrentRoomChild::Error(err_msg, _) =>
                Self::draw_error(err_msg, frame, layout[1]),
            CurrentRoomChild::Invited(child, _) => child.draw(frame, layout[1]),
            CurrentRoomChild::None => NoRoomSelected.draw(frame, layout[1]),
            CurrentRoomChild::Search(child, _) => child.draw(frame, layout[1]),
        }
    }

    #[expect(clippy::unreachable, reason = "just checked monothread data")]
    async fn on_event(&mut self, event: Event) -> Option<Self::UpdateState> {
        match &mut self.child {
            CurrentRoomChild::Discussion(discussion) => {
                discussion.on_event(event).await;
            }

            CurrentRoomChild::Invited(invitation_popup, _) => {
                let AcceptInvitation = invitation_popup.on_event(event).await?;
                match take(&mut self.child) {
                    CurrentRoomChild::Invited(_, room) =>
                        self.accept_invitation(room).await,
                    _ => unreachable!(),
                }
            }

            CurrentRoomChild::Search(search, _) =>
                if event.as_key_press_event()?.code.is_caps_lock() {
                    match take(&mut self.child) {
                        CurrentRoomChild::Search(_, Some(room)) =>
                            self.select_new_room(room),
                        CurrentRoomChild::Search(_, None) => (),
                        _ => unreachable!(),
                    }
                } else if let Some(new_room) = search.on_event(event).await {
                    self.select_new_room(new_room);
                },

            CurrentRoomChild::None | CurrentRoomChild::Error(..) => (),
        }
        None
    }

    fn update(&mut self, response_data: Self::ResponseData) {
        match response_data {
            UpdateCurrentRoomPanel::NewRoom(new_room) =>
                self.select_new_room(new_room),

            UpdateCurrentRoomPanel::Search(room_list) => {
                let old_room = self.child.take_room();
                self.child = CurrentRoomChild::Search(
                    RoomSearch::new(room_list),
                    old_room,
                );
            }
        }
    }
}

/// Type of the content displayed in the chat panel
#[derive(Default)]
enum CurrentRoomChild {
    /// A valid room discussion is open and running
    Discussion(Discussion),
    /// An error occurred and needs to be displayed
    Error(String, Arc<Mutex<DisplayRoom>>),
    /// The current roomed hasn't been joined by the user yet, but it has a
    /// pending invitation.
    Invited(InvitationPopup, Arc<Mutex<DisplayRoom>>),
    /// No room was selected yet.
    #[default]
    None,
    /// Search bar to find and select a room.
    ///
    /// This member stores the old room that the user had before entering search
    /// mode.
    Search(RoomSearch, Option<Arc<Mutex<DisplayRoom>>>),
}

impl CurrentRoomChild {
    /// Checks if current state is a discussion, meaning the client can interact
    /// with the room without issues.
    const fn is_discussion(&self) -> bool {
        matches!(self, Self::Discussion(_))
    }

    /// Checks if a room is open, and if so, that is matches the provided id.
    fn room_is(&self, other: &DisplayRoom) -> bool {
        if let Self::Discussion(room) = self {
            room.room_is(other)
        } else {
            false
        }
    }

    /// Clear the [`CurrentRoomChild`] and return the underlying room if it
    /// exists.
    fn take_room(&mut self) -> Option<Arc<Mutex<DisplayRoom>>> {
        match take(self) {
            Self::Discussion(discussion) => Some(discussion.room()),
            Self::Invited(_, room) | Self::Error(_, room) => Some(room),
            Self::None => None,
            Self::Search(_, room) => room,
        }
    }
}

/// Default component for when no room is selected.
struct NoRoomSelected;

impl Component for NoRoomSelected {
    type ResponseData = Infallible;
    type UpdateState = Infallible;

    fn draw(&self, frame: &mut Frame<'_>, area: Rect) {
        let instructions = InstructionsBuilder::default()
            .text("  Use")
            .key("Up")
            .text("and")
            .key("Down")
            .text("to find the conversation, then")
            .key("Right")
            .text("to open it here. You can also press")
            .key("C-k")
            .text("to search a chat by name.")
            .build();

        let rect = fully_centered_content(instructions.width, area.width, area);

        let paragraph = Paragraph::new(instructions.line)
            .wrap(Wrap { trim: true })
            .centered();

        frame.render_widget(paragraph, rect);
    }
}

/// Data send to this panel to update it.
pub enum UpdateCurrentRoomPanel {
    /// A new room was selected from the room list menu.
    NewRoom(Arc<Mutex<DisplayRoom>>),
    /// A search was requested on a set of rooms.
    Search(Arc<Mutex<Vec<Arc<Mutex<DisplayRoom>>>>>),
}
