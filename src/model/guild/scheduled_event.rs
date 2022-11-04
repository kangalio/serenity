use crate::model::prelude::*;

/// Information about a guild scheduled event.
///
/// [Discord docs](https://discord.com/developers/docs/resources/guild-scheduled-event#guild-scheduled-event-object).
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct ScheduledEvent {
    /// The Id of the scheduled event.
    pub id: ScheduledEventId,
    /// The Id of the guild that the event belongs to.
    pub guild_id: GuildId,
    /// The Id of the channel that the event belongs to, if any.
    pub channel_id: Option<ChannelId>,
    /// The Id of the User that created the scheduled event.
    ///
    /// Only `None` for events created before October 25th, 2021.
    pub creator_id: Option<UserId>,
    /// The name of the scheduled event.
    pub name: String,
    /// The description of the scheduled event, if any.
    pub description: Option<String>,
    /// The event's starting time.
    pub start_time: Timestamp,
    /// The event's ending time; optional.
    pub end_time: Option<Timestamp>,
    /// The event's status; either Scheduled, Active, Completed, or Canceled.
    pub status: ScheduledEventStatus,
    /// The User that created the event.
    ///
    /// Only `None` for events created before October 25th, 2021.
    pub creator: Option<User>,
    /// The type of the event, indicating if it will take place in a Stage Instance, a Voice
    /// Channel, or at some External location.
    pub kind: ScheduledEventType,
    /// Optional event location, only required for External events.
    pub metadata: Option<ScheduledEventMetadata>,
    /// Number of users interested in the event.
    ///
    /// Only populated if `with_user_count` is set to true provided when calling
    /// [`GuildId::scheduled_event`] or [`GuildId::scheduled_events`].
    pub user_count: Option<u64>,
    /// The hash of the event's cover image, if present.
    pub image: Option<String>,
}

enum_number! {
    /// [Discord docs](https://discord.com/developers/docs/resources/guild-scheduled-event#guild-scheduled-event-object-guild-scheduled-event-status).
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]

    #[non_exhaustive]
    pub enum ScheduledEventStatus {
        Scheduled = 1,
        Active = 2,
        Completed = 3,
        Canceled = 4,
        _ => Unknown(u8),
    }
}

enum_number! {
    /// [Discord docs](https://discord.com/developers/docs/resources/guild-scheduled-event#guild-scheduled-event-object-guild-scheduled-event-entity-types).
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]

    #[non_exhaustive]
    pub enum ScheduledEventType {
        StageInstance = 1,
        Voice = 2,
        External = 3,
        _ => Unknown(u8),
    }
}

/// [Discord docs](https://discord.com/developers/docs/resources/guild-scheduled-event#guild-scheduled-event-object-guild-scheduled-event-entity-metadata).
#[derive(Clone, Debug)]
pub struct ScheduledEventMetadata {
    // TODO: Change to `Option<String>` in next version.
    pub location: String,
}

/// [Discord docs](https://discord.com/developers/docs/resources/guild-scheduled-event#guild-scheduled-event-user-object).
#[derive(Clone, Debug)]
pub struct ScheduledEventUser {
    pub event_id: ScheduledEventId,
    pub user: User,
    pub member: Option<Member>,
}
