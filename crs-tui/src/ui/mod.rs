//! Handles the display to the screen and event listening

use color_eyre::Result;
use crossterm::event::Event;
use ratatui::Frame;

/// Trait to define components present in the app
///
/// An infinite loop is executed to re-redner the app. The UI is rendered, then
/// blocked until an [`Event`] occurs. After when this event occurs, the
/// component is updated then the screen is redrawn.
///
/// The struct deriving the component should contain the data needed for the
/// [`Self::render`] method, and nothing else.
///
/// If events happen that depend on this component, the [`Self::on_event`] is
/// called. It returns an [`Self::UpdateState`] that is used to send information
/// to the parent component if need be.
pub trait Component {
    /// Data returned to the parent component on update
    ///
    /// It is a good practice to mark the types used here as `must_use`, to
    /// prevent the parent from discarding it.
    type UpdateState;

    /// Renders the component on the given frame
    fn draw(&self, frame: &mut Frame);

    /// Listens for events that are custom to this UI component
    ///
    /// # Returns
    ///
    ///   component
    /// - `Ok(Some(state))` if there is something to be done by the parent
    /// - `Ok(None)` if nothing else needs to be done
    /// - `Err(err)` if something unexpected occured
    #[expect(unused_variables, reason = "trait def")]
    fn on_event(&mut self, event: Event) -> Result<Option<Self::UpdateState>> {
        Ok(None)
    }
}


/// Screen currently displayed on the user interface
#[derive(Default)]
pub struct Screen;

impl Component for Screen {
    type UpdateState = ();

    fn draw(&self, _frame: &mut Frame) {}
}
