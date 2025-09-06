use color_eyre::Result;
use ratatui::Frame;
use ratatui::crossterm::event::Event;
use ratatui::layout::Constraint;
use ratatui::style::{Color, Stylize as _};
use ratatui::symbols::border;
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
    current:   CurrentInput,
    /// Current text  hels by the input for the password
    password:  String,
    /// Current text  hels by the input for the username
    user_name: String,
}

impl Component for LoginPage {
    type UpdateState = SubmitLogin;

    fn draw(&self, frame: &mut Frame) {
        let title = Line::from(" Login ".bold().style(Color::Green));
        let instructions =
            Instructions::new(&[(" Switch input", "Tab"), ("Submit", "Enter")]);

        let popup_area = grid_center(
            Constraint::Min(instructions.width.saturating_add(2)),
            Constraint::Min(10),
            frame.area(),
        );

        let popup_border = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.line.centered())
            .border_set(border::THICK);

        let popup =
            Paragraph::new(Text::from("hi")).centered().block(popup_border);

        frame.render_widget(popup, popup_area);
    }

    fn on_event(&mut self, event: Event) -> Result<Option<Self::UpdateState>> {
        todo!()
    }
}

/// Struct to represent the state in which the user just submitted the login
/// form
pub struct SubmitLogin;
