//! Login page to store pass credentials to the backend

use core::iter::repeat_n;
use core::mem::take;

use color_eyre::Result;
use ratatui::Frame;
use ratatui::crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style, Stylize as _};
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, Paragraph, Wrap};

use crate::ui::component::Component;
use crate::ui::widgets::{Instructions, grid_center};

/// State differentiating the different inputs on the login page.
///
/// Defaults to [`Self::Username`]
#[derive(Default)]
enum CurrentInput {
    /// User is focused on the password input
    Password,
    /// User just submitted, no data can be given
    Submitting,
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
    /// Current login error displayed
    error:    String,
    /// Current text  hels by the input for the password
    password: String,
    /// Current text  hels by the input for the username
    username: String,
}

impl LoginPage {
    /// Returns the instructions list for the footer of the popup
    fn instructions() -> Instructions<'static> {
        Instructions::new(&[(" Switch input", "Tab"), ("Submit", "Enter")])
    }

    /// Returns the field currently being edited
    fn on_input<F>(&mut self, action: F)
    where F: Fn(&mut String) {
        match self.current {
            CurrentInput::Password => action(&mut self.password),
            CurrentInput::Username => action(&mut self.username),
            CurrentInput::Submitting => (),
        }
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
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Length(3),
            ])
            .margin(2)
            .split(area);

        let (username_colour, password_colour) = match self.current {
            CurrentInput::Password => (Color::Reset, Color::Green),
            CurrentInput::Username => (Color::Green, Color::Reset),
            CurrentInput::Submitting => (Color::Reset, Color::Reset),
        };

        let username_style = Style::default().fg(username_colour);
        let password_style = Style::default().fg(password_colour);

        let username = Paragraph::new(Text::from(self.username.as_str()))
            .block(Block::bordered().border_style(username_style));
        let masked_password: String =
            repeat_n('*', self.password.len()).collect();
        let password = Paragraph::new(Text::from(masked_password))
            .block(Block::bordered().border_style(password_style));

        let error = Paragraph::new(self.error.as_str())
            .wrap(Wrap { trim: true })
            .style(Style::new().fg(Color::Red));

        frame.render_widget(Text::from("Username"), form[1]);
        frame.render_widget(username, form[2]);

        frame.render_widget(Text::from("Password"), form[4]);
        frame.render_widget(password, form[5]);
        frame.render_widget(error, form[6]);
    }

    /// Toggles the current input into the other mode to edit the other field
    const fn toggle_field(&mut self) {
        self.current = match self.current {
            CurrentInput::Password => CurrentInput::Username,
            CurrentInput::Username => CurrentInput::Password,
            CurrentInput::Submitting => return,
        }
    }
}

impl Component for LoginPage {
    type ResponseData = String;
    type UpdateState = LoginCredentials;

    fn draw(&self, frame: &mut Frame) {
        let instructions = Self::instructions();
        let minimum_width = instructions.width.saturating_add(2);
        let popup_border = Self::popup_border(instructions);

        let popup_area = grid_center(
            Constraint::Min(minimum_width),
            Constraint::Length(17),
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
            KeyCode::Char(ch) => self.on_input(|input| input.push(ch)),
            KeyCode::Backspace => self.on_input(|input| {
                input.pop();
            }),
            KeyCode::Tab | KeyCode::BackTab => self.toggle_field(),
            KeyCode::Enter => {
                let credentials = LoginCredentials::from(take(self));
                self.current = CurrentInput::Submitting;
                return Ok(Some(credentials));
            }
            _ => (),
        }
        Ok(None)
    }

    fn update(&mut self, response_data: Self::ResponseData) {
        self.error = response_data;
        self.current = CurrentInput::default();
    }
}

/// Struct to represent the state in which the user just submitted the login
/// form
pub struct LoginCredentials {
    /// Final password to send to server
    pub password: String,
    /// Username to send to server
    pub username: String,
}

impl From<LoginPage> for LoginCredentials {
    fn from(login_page: LoginPage) -> Self {
        Self { username: login_page.username, password: login_page.password }
    }
}
