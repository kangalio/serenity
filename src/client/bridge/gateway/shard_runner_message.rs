use tokio_tungstenite::tungstenite::Message;

#[cfg(feature = "collector")]
use crate::collector::{
    ComponentInteractionFilter,
    EventFilter,
    MessageFilter,
    ModalInteractionFilter,
    ReactionFilter,
};
use crate::gateway::ActivityData;
pub use crate::gateway::ChunkGuildFilter;
use crate::model::id::GuildId;
use crate::model::user::OnlineStatus;

/// A message to send from a shard over a WebSocket.
#[derive(Clone, Debug)]
pub enum ShardRunnerMessage {
    /// Indicates that the client is to send a member chunk message.
    ChunkGuild {
        /// The IDs of the [`Guild`] to chunk.
        ///
        /// [`Guild`]: crate::model::guild::Guild
        guild_id: GuildId,
        /// The maximum number of members to receive [`GuildMembersChunkEvent`]s
        /// for.
        ///
        /// [`GuildMembersChunkEvent`]: crate::model::event::GuildMembersChunkEvent
        limit: Option<u16>,
        /// A filter to apply to the returned members.
        filter: ChunkGuildFilter,
        /// Optional nonce to identify [`GuildMembersChunkEvent`] responses.
        ///
        /// [`GuildMembersChunkEvent`]: crate::model::event::GuildMembersChunkEvent
        nonce: Option<String>,
    },
    /// Indicates that the client is to close with the given status code and
    /// reason.
    ///
    /// You should rarely - if _ever_ - need this, but the option is available.
    /// Prefer to use the [`ShardManager`] to shutdown WebSocket clients if you
    /// are intending to send a 1000 close code.
    ///
    /// [`ShardManager`]: super::ShardManager
    Close(u16, Option<String>),
    /// Indicates that the client is to send a custom WebSocket message.
    Message(Message),
    /// Indicates that the client is to update the shard's presence's activity.
    SetActivity(Option<ActivityData>),
    /// Indicates that the client is to update the shard's presence in its
    /// entirety.
    SetPresence(Option<ActivityData>, OnlineStatus),
    /// Indicates that the client is to update the shard's presence's status.
    SetStatus(OnlineStatus),
    /// Sends a new filter for events to the shard.
    #[cfg(feature = "collector")]
    SetEventFilter(EventFilter),
    /// Sends a new filter for messages to the shard.
    #[cfg(feature = "collector")]
    SetMessageFilter(MessageFilter),
    /// Sends a new filter for reactions to the shard.
    #[cfg(feature = "collector")]
    SetReactionFilter(ReactionFilter),
    /// Sends a new filter for component interactions to the shard.
    #[cfg(feature = "collector")]
    SetComponentInteractionFilter(ComponentInteractionFilter),
    /// Sends a new filter for modal interactions to the shard.
    #[cfg(feature = "collector")]
    SetModalInteractionFilter(ModalInteractionFilter),
}
