//! Defines macros to derive component traits

/// Derives  [`crate::ui::component::Component`] trait for newtypes
///
/// Note: newtypes are  tuple
/// structs with one type; cf
/// <https://doc.rust-lang.org/book/ch20-02-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types>.
#[macro_export]
macro_rules! derive_component {
    ($newtype:ty, $innertype:ty) => {
        impl $crate::ui::component::Component for $newtype {
            type ResponseData =
                <$innertype as $crate::ui::component::Component>::ResponseData;
            type UpdateState =
                <$innertype as $crate::ui::component::Component>::UpdateState;

            fn draw(
                &self,
                frame: &mut ratatui::Frame<'_>,
                area: ratatui::prelude::Rect,
            ) {
                self.0.draw(frame, area);
            }

            async fn on_event(
                &mut self,
                event: ratatui::crossterm::event::Event,
            ) -> Option<Self::UpdateState> {
                self.0.on_event(event).await
            }

            fn update(&mut self, response_data: Self::ResponseData) {
                self.0.update(response_data);
            }
        }
    };
}
