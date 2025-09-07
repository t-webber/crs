//! Every piece of UI derives from the same [`Component`] trait for better
//! organisation. This module defines the specifications of this trait.
//!
//! It defines how it communicates with the core app, in both directions.

use color_eyre::Result;
use ratatui::Frame;
use ratatui::crossterm::event::Event;

/// Trait to define components present in the app
///
/// An infinite loop is executed to re-redner the app. The UI is rendered, then
/// blocked until an [`Event`] occurs. After when this event occurs, the
/// component is updated then the screen is redrawn.
///
/// The struct deriving the component should contain the data needed for the
/// [`Self::draw`] method, and nothing else.
///
/// If events happen that depend on this component, the [`Self::on_event`] is
/// called. It returns an [`Self::UpdateState`] that is used to send information
/// to the parent component if need be.
#[expect(unused_variables, reason = "trait def")]
#[expect(clippy::arbitrary_source_item_ordering, reason = "chrological")]
pub trait Component {
    /// Renders the component on the given frame
    fn draw(&self, frame: &mut Frame);

    /// Data returned to the parent component on update
    ///
    /// It is a good practice to mark the types used here as `must_use`, to
    /// prevent the parent from discarding it.
    type UpdateState;

    /// Listens for events that are custom to this UI component
    ///
    /// # Returns
    ///
    /// - `Ok(Some(state))` if there is something to be done by the parent
    ///   component
    /// - `Ok(None)` if nothing else needs to be done
    /// - `Err(err)` if something unexpected occured
    fn on_event(&mut self, event: Event) -> Result<Option<Self::UpdateState>> {
        Ok(None)
    }

    /// Data provided by the parent in response to an update
    ///
    /// This is used to update the UI after the state has changed.
    type ResponseData;

    /// Executed after events were handled by the parent
    ///
    /// This method will update this component with the new states and values
    fn update(&mut self, response_data: Self::ResponseData) {}
}
