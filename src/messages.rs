use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

use crate::models::CharacterSelect;

/// The header of each message contains ids
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Header {
    pub account_id: i32,
    pub message_id: Uuid,
}

/// The value of each message contains various content
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Value {
    Move(MoveMessage),
    Attack(AttackMessage),
    Initial(InitialMessage),
    Connect(ConnectMessage),
}

/// Each message is made up of a header and value
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Message {
    header: Header,
    value: Value
}

// --------------------------------------------------
// MESSAGE CONTENT
#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct InitialMessage {
    pub entities: Vec<CharacterSelect>
}

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct ConnectMessage {
    pub entity: CharacterSelect
}

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct MoveMessage {
    pub speed: f32,
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct AttackMessage {
    pub target: i32,
    pub ability: i32,
}
// --------------------------------------------------

impl Header {
    pub fn new(account_id: i32) -> Self {
        let message_id = Uuid::now_v7();
        Self { account_id, message_id }
    }
}

impl Message {

    pub fn Move(id: i32, speed: f32, x: f32, y: f32) -> Self {
        Self {
            header: Header::new(id),
            value: Value::Move(MoveMessage {
                speed, x, y
            })
        }
    }

    pub fn Initial(id: i32, entities: Vec<CharacterSelect>) -> Self {
        Self {
            header: Header::new(id),
            value: Value::Initial(InitialMessage { entities })
        }
    }

    pub fn Connect(id: i32, entity: CharacterSelect) -> Self {
        Self {
            header: Header::new(id),
            value: Value::Connect(ConnectMessage { entity })
        }
    }

    pub fn Attack(id: i32, target: i32, ability: i32) -> Self {
        Self {
            header: Header::new(id),
            value: Value::Attack(AttackMessage {
                target, ability
            })
        }
    }

    pub fn id(&self) -> Uuid {
        self.header.message_id.clone()
    }

    pub fn serialize(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }

    pub fn deserialize<'a>(bytes: &'a [u8]) -> serde_json::Result<Self> {
        serde_json::from_slice::<Self>(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_message() {
        let message = Message::Move(0,1.0,2.0,3.0);
        let result = message.serialize();
        assert!(result.is_ok());
    }

    #[test]
    fn test_deserialize_message() {
        let message = Message::Move(0,1.0,2.0,3.0);
        let result = message.serialize();
        assert!(result.is_ok());

        let string = result.unwrap();
        let message = Message::deserialize(string.as_bytes());
        assert!(message.is_ok());
    }

}