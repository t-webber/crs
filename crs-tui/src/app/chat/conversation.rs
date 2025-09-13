//! Main chat page with the messages and the inputs to send messages

extern crate alloc;

use alloc::sync::Arc;
use core::convert::Infallible;
use std::sync::Mutex;

use crs_backend::room::DisplayRoom;
use ratatui::Frame;
use ratatui::crossterm::event::{Event, KeyCode};
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::Text;
use ratatui::widgets::{
    Block, BorderType, Borders, List, ListItem, Paragraph, Wrap
};

use crate::ui::component::Component;
use crate::ui::input::Input;
use crate::ui::widgets::{
    InstructionsBuilder, fully_centered_content, linear_center, saturating_cast
};
use crate::utils::safe_unlock;

/// Conversation panel, with the all the messages and the input to send messages
pub struct Conversation {
    /// Message prompt to write the messages
    message_prompt: Input<'static>,
    /// Room currently being displayed
    room:           Arc<Mutex<DisplayRoom>>,
}

impl Conversation {
    /// Draw the contents of the conversation
    #[expect(clippy::arithmetic_side_effects, reason = "width >= 20")]
    fn draw_conversation_content(&self, frame: &mut Frame<'_>, area: Rect) {
        debug_assert!(area.width >= 20, "not wide enough");
        match safe_unlock(&self.room).as_messages() {
            Ok(messages_bodies) => {
                let list = messages_bodies
                    .iter()
                    .map(|message| ListItem::new(message.as_body()));
                frame.render_widget(List::new(list), area);
            }
            Err(err) => {
                let err_msg = err.to_string();
                let err_len = saturating_cast(err_msg.len());
                let rect =
                    fully_centered_content(err_len, area.width - 4, area);

                let err_widget = Paragraph::new(err_msg)
                    .style(Style::new().fg(Color::Red))
                    .wrap(Wrap { trim: true })
                    .alignment(Alignment::Center);

                frame.render_widget(err_widget, rect);
            }
        }
    }

    /// Displays the popup to ask if the users wants to accept the invitation
    fn draw_invation_box(frame: &mut Frame<'_>, area: Rect) {
        let center =
            linear_center(Constraint::Length(4), Direction::Vertical, area);

        let layout = Layout::new(Direction::Vertical, [
            Constraint::Length(1),
            Constraint::Length(3),
        ])
        .split(center);

        let message =
            Paragraph::new("You have a pending invitation for this room.")
                .centered();
        frame.render_widget(message, layout[0]);

        let accept = InstructionsBuilder::default()
            .text("Press")
            .key("Enter")
            .text("to accept and join")
            .build();

        let button = Paragraph::new(accept.line)
            .centered()
            .block(Block::new().borders(Borders::ALL));

        frame.render_widget(button, layout[1]);
    }

    /// Displays the name of the room at the top of the conversation component
    fn draw_room_name(&self, frame: &mut Frame<'_>, area: Rect) {
        let unknown = String::from("<unknown room>");

        let room = safe_unlock(&self.room);

        let room_name = room.as_name().unwrap_or(&unknown);
        let room_name_widget = Text::from(room_name.as_str())
            .style(Style::new().fg(Color::Yellow))
            .alignment(Alignment::Center);

        frame.render_widget(room_name_widget, area);
        drop(room);
    }

    /// Checks if the messages of this room were correctly loaded
    fn has_errors(&self) -> bool {
        safe_unlock(&self.room).as_messages().is_err()
    }

    /// Checks if the messages of this room were correctly loaded
    fn has_invitation(&self) -> bool {
        safe_unlock(&self.room).has_invitation()
    }

    /// Open a new conversation for the given room
    pub fn new(room: Arc<Mutex<DisplayRoom>>) -> Self {
        let mut this = Self { room, message_prompt: Input::new() };
        let has_errors = this.has_errors();
        this.message_prompt.set_active(!has_errors);
        this.message_prompt.set_error(has_errors);
        this
    }
}

impl Component for Conversation {
    type ResponseData = Infallible;
    type UpdateState = Infallible;

    fn draw(&self, frame: &mut Frame<'_>, area: Rect) {
        let layout = Layout::new(Direction::Vertical, [
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(Input::HEIGHT_WITHOUT_LABEL),
        ])
        .split(area);

        self.draw_room_name(frame, layout[0]);
        if self.has_invitation() {
            Self::draw_invation_box(frame, layout[1]);
        } else {
            self.draw_conversation_content(frame, layout[1]);
        }
        self.message_prompt.draw(frame, layout[2]);
    }

    async fn on_event(&mut self, event: Event) -> Option<Self::UpdateState> {
        let key_event = event.as_key_press_event()?;

        if self.has_invitation() {
            if key_event.code.is_enter() {
                let room = safe_unlock(&self.room).into_room();
                room.accept_invitation().await.unwrap();
            }

            return None;
        }

        if key_event.code.is_enter() {
            let message = self.message_prompt.take_value();
            let room = safe_unlock(&self.room).into_room();
            room.send_plain(&message).await.unwrap();
            return None;
        }

        if self.has_errors() {
            self.message_prompt.set_active(false);
            self.message_prompt.set_error(true);
            None
        } else {
            self.message_prompt.set_active(true);
            self.message_prompt.set_error(false);
            self.message_prompt.on_event(event).await
        }
    }
}
