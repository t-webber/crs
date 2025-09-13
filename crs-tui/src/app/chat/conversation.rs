//! Main chat page with the messages and the inputs to send messages

extern crate alloc;

use alloc::sync::Arc;
use core::convert::Infallible;
use std::sync::Mutex;

use crs_backend::room::DisplayRoom;
use ratatui::Frame;
use ratatui::crossterm::event::Event;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::Text;
use ratatui::widgets::{List, ListItem, Paragraph, Wrap};

use crate::ui::component::Component;
use crate::ui::input::Input;
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
    fn draw_conversation_content(&self, frame: &mut Frame<'_>, area: Rect) {
        match safe_unlock(&self.room).as_messages() {
            Ok(messages_bodies) => {
                let list = messages_bodies
                    .iter()
                    .map(|message| ListItem::new(message.as_body()));
                frame.render_widget(List::new(list), area);
            }
            Err(err) => {
                frame.render_widget(
                    Paragraph::new(Text::from(err.to_string()))
                        .style(Style::new().fg(Color::Red))
                        .wrap(Wrap { trim: true })
                        .alignment(Alignment::Center),
                    area,
                );
            }
        }
    }

    /// Checks if the messages of this room were correctly loaded
    fn has_errors(&self) -> bool {
        safe_unlock(&self.room).as_messages().is_err()
    }

    /// Open a new conversation for the given room
    pub fn new(room: Arc<Mutex<DisplayRoom>>) -> Self {
        let mut this = Self { room, message_prompt: Input::new() };
        if this.has_errors() {
            this.message_prompt.set_active(false);
        }
        this
    }

    /// Returns the room selected for the current conversation
    pub fn as_room(&self) -> &Arc<Mutex<DisplayRoom>> {
        &self.room
    }
}

impl Component for Conversation {
    type ResponseData = Infallible;
    type UpdateState = Infallible;

    fn draw(&self, frame: &mut Frame<'_>, area: Rect) {
        let layout = Layout::new(Direction::Vertical, [
            Constraint::Fill(1),
            Constraint::Length(Input::HEIGHT_WITHOUT_LABEL),
        ])
        .split(area);

        self.draw_conversation_content(frame, layout[0]);
        self.message_prompt.draw(frame, layout[1]);
    }

    async fn on_event(&mut self, event: Event) -> Option<Self::UpdateState> {
        self.message_prompt.on_event(event).await
    }
}
