//! Loads, parses and handles room messages

#![allow(clippy::missing_docs_in_private_items, reason = "cf json reference")]

use matrix_sdk::Room;
use matrix_sdk::deserialized_responses::TimelineEvent;
use matrix_sdk::room::MessagesOptions;
use matrix_sdk::ruma::{UInt, UserId};
use serde::{Deserialize, Serialize as Serialise}; // ignore-spell
use serde_json::Value;

#[derive(Serialise, Deserialize)]
struct Content {
    body: Option<String>,
}

#[derive(Serialise, Deserialize)]
struct Message {
    content: Content,
    sender:  String,
}

/// Message from a room
///
/// A message can represent anything: a rule, a reaction, an action (e.g.
/// someone joined), etc.
pub struct DisplayMessage {
    body:   String,
    sender: String,
}

impl DisplayMessage {
    /// Returns the body of the message
    #[must_use]
    pub fn as_body(&self) -> &str {
        &self.body
    }

    /// Returns the sender of the message
    #[must_use]
    pub fn as_sender(&self) -> &str {
        &self.sender
    }

    async fn try_from(
        message: Message,
        room: &Room,
    ) -> Result<Option<Self>, matrix_sdk::Error> {
        if let Some(body) = message.content.body
            && let user_id = UserId::parse(message.sender)?
            && let Some(member) = room.get_member(&user_id).await?
            && let Some(name) = member.display_name()
        {
            Ok(Some(Self { body, sender: name.to_owned() }))
        } else {
            Ok(None)
        }
    }
}

async fn parse_message(
    room: &Room,
    event: TimelineEvent,
) -> matrix_sdk::Result<Option<DisplayMessage>> {
    let json = event.into_raw();
    let value = json.deserialize_as::<Value>()?;
    let message: Message = serde_json::from_value(value)?;
    DisplayMessage::try_from(message, room).await
}

/// Loads and parses the messages of a room
///
/// # Errors
///
/// For connection errors
///
/// # Panics
///
/// For serialisation errors
pub async fn get_room_messages(
    room: &Room,
) -> Result<Vec<DisplayMessage>, matrix_sdk::Error> {
    let mut opts = MessagesOptions::forward();
    opts.limit = UInt::MAX;

    let events = room.messages(opts).await?.chunk;

    let mut messages = Vec::with_capacity(events.len());
    for event in events {
        if let Some(message) = parse_message(room, event).await? {
            messages.push(message);
        }
    }

    Ok(messages)
}
