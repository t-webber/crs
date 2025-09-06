use core::iter::{repeat, repeat_n};

use color_eyre::Result;
use ratatui::Frame;
use ratatui::crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style, Stylize as _};
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, Paragraph};

use crate::ui::Component;
use crate::ui::components::{Instructions, grid_center};

/// State differentiating the different inputs on the login page.
///
/// Defaults to [`Self::Username`]
#[derive(Default)]
enum CurrentInput {
    /// User is focused on the password input
    Password,
    /// User is focused on the username input
    #[default]
    Username,
}

/// UI handler for the login page
///
/// Refer to [`Component`] for usage
#[derive(Default)]
pub struct LoginPage {
    /// Current input that is being edited,
    current:  CurrentInput,
    /// Current text  hels by the input for the password
    password: String,
    /// Current text  hels by the input for the username
    username: String,
}

impl LoginPage {
    /// Returns the field currently being edited
    const fn as_current_mut(&mut self) -> &mut String {
        match self.current {
            CurrentInput::Password => &mut self.password,
            CurrentInput::Username => &mut self.username,
        }
    }

    /// Returns the instructions list for the footer of the popup
    fn instructions() -> Instructions<'static> {
        Instructions::new(&[(" Switch input", "Tab"), ("Submit", "Enter")])
    }

    /// Returns the border of the popup with the title and instructions
    /// integrated in the border.
    fn popup_border(instructions: Instructions<'static>) -> Block<'static> {
        let title = Line::from(" Login ".bold().style(Color::Green));
        Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.line.centered())
    }

    /// Fill the form with the current values
    #[expect(
        clippy::indexing_slicing,
        clippy::missing_asserts_for_indexing,
        reason = "len = 3"
    )]
    fn render_form(&self, frame: &mut Frame, area: Rect) {
        let form = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(3),
            ])
            .margin(2)
            .split(area);

        let (username_colour, password_colour) = match self.current {
            CurrentInput::Password => (Color::Reset, Color::Green),
            CurrentInput::Username => (Color::Green, Color::Reset),
        };

        let username_style = Style::default().fg(username_colour);
        let password_style = Style::default().fg(password_colour);

        let username = Paragraph::new(Text::from(self.username.as_str()))
            .block(Block::bordered().border_style(username_style));
        let masked_password: String =
            repeat_n('*', self.password.len()).collect();
        let password = Paragraph::new(Text::from(masked_password))
            .block(Block::bordered().border_style(password_style));

        frame.render_widget(Text::from("Username"), form[0]);
        frame.render_widget(username, form[1]);
        frame.render_widget(Text::from("Password"), form[3]);
        frame.render_widget(password, form[4]);
    }

    /// Toggles the current input into the other mode to edit the other field
    const fn toggle_field(&mut self) {
        self.current = match self.current {
            CurrentInput::Password => CurrentInput::Username,
            CurrentInput::Username => CurrentInput::Password,
        }
    }
}

impl Component for LoginPage {
    type UpdateState = SubmitLogin;

    fn draw(&self, frame: &mut Frame) {
        let instructions = Self::instructions();
        let minimum_width = instructions.width.saturating_add(2);
        let popup_border = Self::popup_border(instructions);

        let popup_area = grid_center(
            Constraint::Min(minimum_width),
            Constraint::Length(13),
            frame.area(),
        );

        self.render_form(frame, popup_area);

        let popup_dummy_content = Paragraph::new(Text::from(""));
        let popup_dummy = popup_dummy_content.centered().block(popup_border);

        frame.render_widget(popup_dummy, popup_area);
    }

    fn on_event(&mut self, event: Event) -> Result<Option<Self::UpdateState>> {
        let Event::Key(key_event): Event = event else { return Ok(None) };
        if key_event.kind != KeyEventKind::Press {
            return Ok(None);
        }
        match key_event.code {
            // Input handlers
            KeyCode::Char(ch) => self.as_current_mut().push(ch),
            KeyCode::Backspace => {
                self.as_current_mut().pop();
            }
            // Page handlers
            KeyCode::Tab | KeyCode::BackTab => self.toggle_field(),
            KeyCode::Enter => return Ok(Some(SubmitLogin)),
            _ => (),
        }
        Ok(None)
    }
}

/// Struct to represent the state in which the user just submitted the login
/// form
pub struct SubmitLogin;
