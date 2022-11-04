use serde::{Deserialize, Deserializer, Serializer};

use crate::model::id::{ChannelId, EmojiId};

/// Information relating to a guild's welcome screen.
///
/// [Discord docs](https://discord.com/developers/docs/resources/guild#welcome-screen-object).
#[derive(Clone, Debug)]
pub struct GuildWelcomeScreen {
    /// The server description shown in the welcome screen.
    pub description: Option<String>,
    /// The channels shown in the welcome screen.
    ///
    /// **Note**: There can only be only up to 5 channels.
    pub welcome_channels: Vec<GuildWelcomeChannel>,
}

/// A channel shown in the [`GuildWelcomeScreen`].
///
/// [Discord docs](https://discord.com/developers/docs/resources/guild#welcome-screen-object-welcome-screen-channel-structure).
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct GuildWelcomeChannel {
    /// The channel Id.
    pub channel_id: ChannelId,
    /// The description shown for the channel.
    pub description: String,
    /// The emoji shown, if there is one.
    pub emoji: Option<GuildWelcomeChannelEmoji>,
}

/// A [`GuildWelcomeScreen`] emoji.
///
/// [Discord docs](https://discord.com/developers/docs/resources/guild#welcome-screen-object-welcome-screen-channel-structure).
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum GuildWelcomeChannelEmoji {
    /// A custom emoji.
    Custom { id: EmojiId, name: String },
    /// A unicode emoji.
    Unicode(String),
}
