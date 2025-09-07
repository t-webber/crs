//! Main page displayed with the chats

extern crate alloc;
use alloc::sync::Arc;

use backend::room::DisplayRoom;
use backend::user::User;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::widgets::{List, ListItem};

use crate::ui::component::Component;

/// This page renders and gives the user an interface to list the chat and
/// communicate in those chats.
pub struct ChatPage {
    /// Rooms visible by the user
    rooms: Vec<DisplayRoom>,
    /// User to interact with matrix server
    user:  Arc<User>,
}

impl ChatPage {
    /// Create a new chat page with the given logged in user
    pub async fn new(user: Arc<User>) -> Self {
        let rooms = user.list_rooms().await;
        Self { rooms, user }
    }
}

impl Component for ChatPage {
    type ResponseData = ();
    type UpdateState = ();

    fn draw(&self, frame: &mut Frame, area: Rect) {
        if frame.area().width <= 30 {
            todo!()
        }

        let unknown = String::from("<unknown>");

        let name_list = self
            .rooms
            .iter()
            .map(|name| {
                ListItem::new(
                    name.as_name().as_ref().unwrap_or(&unknown).as_str(),
                )
            })
            .collect::<Vec<_>>();

        let list = List::new(name_list);

        frame.render_widget(list, area);
    }
}

/// Room displayed in the list of rooms
pub struct Room {
    /// Id of the room
    id:   String,
    /// Name of the room
    name: String,
}
