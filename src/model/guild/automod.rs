//! Auto moderation types
//!
//! [Discord docs](https://discord.com/developers/docs/resources/auto-moderation)

use std::fmt;

use serde::de::{Deserializer, Error, IgnoredAny, MapAccess};
use serde::ser::{SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};
use serde_value::{DeserializerError, Value};

use crate::model::id::{ChannelId, GuildId, RoleId, RuleId, UserId};

/// Configured auto moderation rule.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Rule {
    /// ID of the rule.
    pub id: RuleId,
    /// ID of the guild this rule belonfs to.
    pub guild_id: GuildId,
    /// Name of the rule.
    pub name: String,
    /// ID of the user which created the rule.
    pub creator_id: UserId,
    /// Event context in which the rule should be checked.
    pub event_type: EventType,
    /// Characterizes the type of content which can trigger the rule.
    pub trigger_type: TriggerType,
    /// Additional data for the trigger type.
    pub trigger_metadata: TriggerMetadata,
    /// Actions which will execute when the rule is triggered.
    pub actions: Vec<Action>,
    /// Whether the rule is enabled.
    pub enabled: bool,
    /// Roles that should not be affected by the rule.
    ///
    /// Maximum of 20.
    pub exempt_roles: Vec<RoleId>,
    /// Channels that should not be affected by the rule.
    ///
    /// Maximum of 50.
    pub exempt_channels: Vec<ChannelId>,
}

/// Indicates in what event context a rule should be checked.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(from = "u8", into = "u8")]
#[non_exhaustive]
pub enum EventType {
    MessageSend,
    Unknown(u8),
}

impl From<u8> for EventType {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::MessageSend,
            _ => Self::Unknown(value),
        }
    }
}

impl From<EventType> for u8 {
    fn from(value: EventType) -> Self {
        match value {
            EventType::MessageSend => 1,
            EventType::Unknown(unknown) => unknown,
        }
    }
}

/// Characterizes the type of content which can trigger the rule.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(from = "u8", into = "u8")]
#[non_exhaustive]
pub enum TriggerType {
    Keyword,
    HarmfulLink,
    Spam,
    KeywordPresent,
    Unknown(u8),
}

impl From<u8> for TriggerType {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Keyword,
            2 => Self::HarmfulLink,
            3 => Self::Spam,
            4 => Self::KeywordPresent,
            _ => Self::Unknown(value),
        }
    }
}

impl From<TriggerType> for u8 {
    fn from(value: TriggerType) -> Self {
        match value {
            TriggerType::Keyword => 1,
            TriggerType::HarmfulLink => 2,
            TriggerType::Spam => 3,
            TriggerType::KeywordPresent => 4,
            TriggerType::Unknown(unknown) => unknown,
        }
    }
}

/// Additional data used to determine whether a rule should be triggered.
///
/// Different fields are relevant based on the value of trigger_type.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TriggerMetadata {
    keyword_filter: Option<Vec<String>>,
    presets: Option<Vec<KeywordPresentType>>,
}

/// Internally pre-defined wordsets which will be searched for in content.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(from = "u8", into = "u8")]
#[non_exhaustive]
pub enum KeywordPresentType {
    Profanity,
    SexualContent,
    Slurs,
    Unknown(u8),
}

impl From<u8> for KeywordPresentType {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Profanity,
            2 => Self::SexualContent,
            3 => Self::Slurs,
            _ => Self::Unknown(value),
        }
    }
}

impl From<KeywordPresentType> for u8 {
    fn from(value: KeywordPresentType) -> Self {
        match value {
            KeywordPresentType::Profanity => 1,
            KeywordPresentType::SexualContent => 2,
            KeywordPresentType::Slurs => 3,
            KeywordPresentType::Unknown(unknown) => unknown,
        }
    }
}

/// An action which will execute whenever a rule is triggered.
#[derive(Clone, Debug, PartialEq)]
pub enum Action {
    /// Blocks the content of a message according to the rule.
    BlockMessage,
    /// Logs user content to a specified channel.
    Alert(ChannelId),
    /// Timeout user for a specified duration.
    ///
    /// Maximum of 2419200 seconds (4 weeks).
    ///
    /// A `Timeout` action can only be setup for [`Keyword`] rules.
    /// [`Permissions::MODERATE_MEMBERS`] permission is required to use the `Timeout` action type.
    ///
    /// [`Keyword`]: TriggerType::Keyword
    /// [`Permissions::MODERATE_MEMBERS`]: crate::model::Permissions::MODERATE_MEMBERS
    Timeout(u64),
    Unknown(u8),
}

#[derive(Deserialize, Serialize)]
#[serde(rename = "ActionMetadata")]
struct Alert {
    channel_id: ChannelId,
}

#[derive(Deserialize, Serialize)]
#[serde(rename = "ActionMetadata")]
struct Timeout {
    #[serde(rename = "duration_seconds")]
    duration: u64,
}

impl<'de> Deserialize<'de> for Action {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            Type,
            Metadata,
            Unknown(String),
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Action;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("automod rule action")
            }

            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut kind = None;
                let mut metadata = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Type => {
                            if kind.is_some() {
                                return Err(Error::duplicate_field("type"));
                            }
                            kind = Some(map.next_value()?);
                        },
                        Field::Metadata => {
                            if metadata.is_some() {
                                return Err(Error::duplicate_field("metadata"));
                            }
                            metadata = Some(map.next_value::<Value>()?);
                        },
                        Field::Unknown(_) => {
                            map.next_value::<IgnoredAny>()?;
                        },
                    }
                }
                let kind = kind.ok_or_else(|| Error::missing_field("type"))?;
                match kind {
                    ActionType::BlockMessage => Ok(Action::BlockMessage),
                    ActionType::Alert => {
                        let alert: Alert = metadata
                            .ok_or_else(|| Error::missing_field("metadata"))?
                            .deserialize_into()
                            .map_err(DeserializerError::into_error)?;
                        Ok(Action::Alert(alert.channel_id))
                    },
                    ActionType::Timeout => {
                        let timeout: Timeout = metadata
                            .ok_or_else(|| Error::missing_field("metadata"))?
                            .deserialize_into()
                            .map_err(DeserializerError::into_error)?;
                        Ok(Action::Timeout(timeout.duration))
                    },
                    ActionType::Unknown(unknown) => Ok(Action::Unknown(unknown)),
                }
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

impl Serialize for Action {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let has_metadata = matches!(self, Self::Alert(_) | Self::Timeout(_));

        let len = 1 + usize::from(has_metadata);
        let mut s = serializer.serialize_struct("Action", len)?;

        s.serialize_field("type", &self.kind())?;
        match *self {
            Self::Alert(channel_id) => {
                s.serialize_field("metadata", &Alert {
                    channel_id,
                })?;
            },
            Self::Timeout(duration) => {
                s.serialize_field("metadata", &Timeout {
                    duration,
                })?;
            },
            _ => {},
        }

        s.end()
    }
}

impl Action {
    #[must_use]
    pub fn kind(&self) -> ActionType {
        match self {
            Self::BlockMessage => ActionType::BlockMessage,
            Self::Alert(_) => ActionType::Alert,
            Self::Timeout(_) => ActionType::Timeout,
            Self::Unknown(unknown) => ActionType::Unknown(*unknown),
        }
    }
}

/// Type of [`Action`].
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(from = "u8", into = "u8")]
#[non_exhaustive]
pub enum ActionType {
    /// Blocks the content of a message according to the rule.
    BlockMessage,
    /// Logs user content to a specified channel.
    Alert,
    /// Timeout user for a specified duration.
    ///
    /// A `Timeout` action can only be setup for [`Keyword`] rules.
    /// [`Permissions::MODERATE_MEMBERS`] permission is required to use the `Timeout` action type.
    ///
    /// [`Keyword`]: TriggerType::Keyword
    /// [`Permissions::MODERATE_MEMBERS`]: crate::model::Permissions::MODERATE_MEMBERS
    Timeout,
    Unknown(u8),
}

impl From<u8> for ActionType {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::BlockMessage,
            2 => Self::Alert,
            3 => Self::Timeout,
            unknown => Self::Unknown(unknown),
        }
    }
}

impl From<ActionType> for u8 {
    fn from(value: ActionType) -> Self {
        match value {
            ActionType::BlockMessage => 1,
            ActionType::Alert => 2,
            ActionType::Timeout => 3,
            ActionType::Unknown(unknown) => unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_test::Token;

    use super::*;

    #[test]
    fn action_serde() {
        let value = Action::BlockMessage;

        serde_test::assert_tokens(&value, &[
            Token::Struct {
                name: "Action",
                len: 1,
            },
            Token::Str("type"),
            Token::U8(1),
            Token::StructEnd,
        ]);

        let value = Action::Alert(ChannelId(123));
        serde_test::assert_tokens(&value, &[
            Token::Struct {
                name: "Action",
                len: 2,
            },
            Token::Str("type"),
            Token::U8(2),
            Token::Str("metadata"),
            Token::Struct {
                name: "ActionMetadata",
                len: 1,
            },
            Token::Str("channel_id"),
            Token::NewtypeStruct {
                name: "ChannelId",
            },
            Token::Str("123"),
            Token::StructEnd,
            Token::StructEnd,
        ]);

        let value = Action::Timeout(1024);
        serde_test::assert_tokens(&value, &[
            Token::Struct {
                name: "Action",
                len: 2,
            },
            Token::Str("type"),
            Token::U8(3),
            Token::Str("metadata"),
            Token::Struct {
                name: "ActionMetadata",
                len: 1,
            },
            Token::Str("duration_seconds"),
            Token::U64(1024),
            Token::StructEnd,
            Token::StructEnd,
        ]);

        let value = Action::Unknown(123);
        serde_test::assert_tokens(&value, &[
            Token::Struct {
                name: "Action",
                len: 1,
            },
            Token::Str("type"),
            Token::U8(123),
            Token::StructEnd,
        ]);
    }
}
