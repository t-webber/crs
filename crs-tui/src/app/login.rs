//! Login page to send credentials to the backend to get an authenticated matrix
//! client.

use core::mem::take;

use ratatui::Frame;
use ratatui::crossterm::event::{Event, KeyCode};
use ratatui::layout::{Constraint, Direction, Layout, Margin, Rect};
use ratatui::style::{Color, Style, Stylize as _};
use ratatui::text::Line;
use ratatui::widgets::{Block, Paragraph, Wrap};

use crate::credentials::Credentials;
use crate::ui::component::Component;
use crate::ui::input::Input;
use crate::ui::prompt::ErrorMessage;
use crate::ui::widgets::{Instructions, InstructionsBuilder, grid_center};

/// Height reserved for the error message
const ERROR_HEIGHT: u16 = 4;

/// Height needed to display a border around the popup
const BORDER_HEIGHT: u16 = 2;

/// Vertical separation size between each input of the popup
const INPUT_SEPARATION_HEIGHT: u16 = 1;

/// Full height to reserve to display the popup
const POPUP_HEIGHT: u16 = 3 * Input::HEIGHT_WITH_LABEL
    + 3 * INPUT_SEPARATION_HEIGHT
    + ERROR_HEIGHT
    + BORDER_HEIGHT;

/// State differentiating the different inputs on the login page.
///
/// Defaults to [`Self::Username`]
#[derive(Default)]
enum CurrentInput {
    /// User is focused on the homeserver url input
    #[default]
    HomeServer,
    /// User is focused on the password input
    Password,
    /// User just submitted, no data can be given
    Submitting,
    /// User is focused on the username input
    Username,
}

/// UI handler for the login page
///
/// Refer to [`Component`] for usage
pub struct LoginPage {
    /// Current input that is being edited,
    current:    CurrentInput,
    /// Current login error displayed
    error:      String,
    /// Current text  held by the input for the homeserver url
    homeserver: Input<'static>,
    /// Current text  held by the input for the password
    password:   Input<'static>,
    /// Current text  held by the input for the username
    username:   Input<'static>,
}

impl LoginPage {
    /// Focus the next field underneath the current one
    const fn focus_field_down(&mut self) {
        match self.current {
            CurrentInput::HomeServer => self.focus_username(),
            CurrentInput::Username => self.focus_password(),
            CurrentInput::Password => self.focus_homeserver(),
            CurrentInput::Submitting => (),
        }
    }

    /// Focus the next field above the current one
    const fn focus_field_up(&mut self) {
        match self.current {
            CurrentInput::HomeServer => self.focus_password(),
            CurrentInput::Username => self.focus_homeserver(),
            CurrentInput::Password => self.focus_username(),
            CurrentInput::Submitting => (),
        }
    }

    /// Focus the homeserver input
    pub const fn focus_homeserver(&mut self) {
        self.homeserver.set_active(true);
        self.username.set_active(false);
        self.password.set_active(false);
        self.current = CurrentInput::HomeServer;
    }

    /// Focus the password input
    pub const fn focus_password(&mut self) {
        self.homeserver.set_active(false);
        self.username.set_active(false);
        self.password.set_active(true);
        self.current = CurrentInput::Password;
    }

    /// Focus the username input
    pub const fn focus_username(&mut self) {
        self.homeserver.set_active(false);
        self.username.set_active(true);
        self.password.set_active(false);
        self.current = CurrentInput::Username;
    }

    /// After sending a signin request, block the inputs for no data should be
    /// entered
    const fn freeze(&mut self) {
        self.current = CurrentInput::Submitting;
        self.homeserver.set_active(false);
    }

    /// Instructions to displau in the login box's footer
    fn instructions() -> Instructions<'static> {
        InstructionsBuilder::default()
            .text(" Switch input")
            .key("Tab")
            .text("Submit")
            .key("Enter")
            .build()
    }

    /// Create a new login page with the given credentials
    ///
    /// They are use as defailt. They will be lost after a login attempt.
    pub fn new(credentials: Credentials<String>) -> Self {
        let mut this = Self::default();
        this.homeserver.set_value(credentials.homeserver);
        this.username.set_value(credentials.username);
        this.password.set_value(credentials.password);
        this
    }

    /// Create a new login page after a login failed.
    ///
    /// The given message will appear as error on the screen.
    pub fn new_with_login_err(error: String) -> Self {
        Self { error, ..Default::default() }
    }

    /// Returns the border of the popup with the title and instructions
    /// integrated in the border.
    fn popup_border(instructions_line: Line<'static>) -> Block<'static> {
        let title = Line::from(" Login ".bold().style(Color::Green));
        Block::bordered()
            .title(title.centered())
            .title_bottom(instructions_line.centered())
    }

    /// Fill the form with the current values
    #[expect(clippy::indexing_slicing, reason = "len = 3")]
    fn render_form(&self, frame: &mut Frame<'_>, area: Rect) {
        let input_height = Input::HEIGHT_WITH_LABEL;

        let margin = Margin::new(2, 1);

        let form = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(INPUT_SEPARATION_HEIGHT),
                Constraint::Length(input_height),
                Constraint::Length(INPUT_SEPARATION_HEIGHT),
                Constraint::Length(input_height),
                Constraint::Length(INPUT_SEPARATION_HEIGHT),
                Constraint::Length(input_height),
                Constraint::Length(ERROR_HEIGHT),
            ])
            .split(area.inner(margin));

        self.homeserver.draw(frame, form[1]);
        self.username.draw(frame, form[3]);
        self.password.draw(frame, form[5]);

        if matches!(self.current, CurrentInput::Submitting) {
            let submitting_text =
                Paragraph::new("Submitting...").fg(Color::Green).centered();

            frame.render_widget(submitting_text, form[6]);
        } else {
            let error = Paragraph::new(self.error.as_str())
                .wrap(Wrap { trim: true })
                .style(Style::new().fg(Color::Red))
                .centered();

            frame.render_widget(error, form[6]);
        }
    }
}

impl Component for LoginPage {
    type ResponseData = ErrorMessage;
    type UpdateState = Credentials<String>;

    fn draw(&self, frame: &mut Frame<'_>, area: Rect) {
        let instructions = Self::instructions();

        let minimum_width = instructions.width.saturating_add(2);

        let popup_area = grid_center(
            Constraint::Min(minimum_width),
            Constraint::Length(POPUP_HEIGHT),
            area,
        );

        let popup_border = Self::popup_border(instructions.line.centered());
        frame.render_widget(popup_border, popup_area);

        self.render_form(frame, popup_area);
    }

    async fn on_event(&mut self, event: Event) -> Option<Self::UpdateState> {
        let key_event = event.as_key_press_event()?;
        match key_event.code {
            KeyCode::Tab => self.focus_field_down(),
            KeyCode::BackTab => self.focus_field_up(),
            KeyCode::Enter => {
                if self.homeserver.is_empty()
                    || self.username.is_empty()
                    || self.password.is_empty()
                {
                    "Found empty field, but all are required"
                        .clone_into(&mut self.error);
                    return None;
                }
                let credentials = Credentials::from(take(self));
                self.freeze();
                return Some(credentials);
            }
            _ => {
                match self.current {
                    CurrentInput::HomeServer =>
                        self.homeserver.on_event(event).await,
                    CurrentInput::Username =>
                        self.username.on_event(event).await,
                    CurrentInput::Password =>
                        self.password.on_event(event).await,
                    CurrentInput::Submitting => return None,
                };
            }
        }
        None
    }

    fn update(&mut self, response_data: Self::ResponseData) {
        let ErrorMessage(error) = response_data;
        self.error = error;
        self.current = CurrentInput::default();
    }
}

impl From<LoginPage> for Credentials<String> {
    fn from(value: LoginPage) -> Self {
        Self {
            homeserver: value.homeserver.into_value(),
            username:   value.username.into_value(),
            password:   value.password.into_value(),
        }
    }
}

impl Default for LoginPage {
    fn default() -> Self {
        Self {
            homeserver: Input::default().with_label("Homeserver").with_active(),
            username:   Input::default().with_label("Username"),
            password:   Input::default().with_label("Password").with_hidden(),
            current:    CurrentInput::default(),
            error:      String::new(),
        }
    }
}
