use std::fmt;

use serde::{Deserialize, Serialize};

/// The available OAuth2 Scopes.
///
/// [Discord docs](https://discord.com/developers/docs/topics/oauth2#shared-resources-oauth2-scopes).
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Scope {
    /// For oauth2 bots, this puts the bot in the user's selected guild by default.
    Bot,
    /// Allows your app to use Slash Commands in a guild.
    ApplicationsCommands,
    /// Allows your app to update its Slash Commands via this bearer token - client credentials grant only.
    ApplicationsCommandsUpdate,
    /// Allows your app to update permissions for its commands in a guild a user has permissions to.
    ApplicationsCommandsPermissionsUpdate,

    /// Allows `/users/@me` without [`Self::Email`].
    Identify,
    /// Enables `/users/@me` to return an `email` field.
    Email,
    /// Allows `/users/@me/connections` to return linked third-party accounts.
    Connections,
    /// Allows `/users/@me/guilds` to return basic information about all of a user's guilds.
    Guilds,
    /// Allows `/guilds/{guild.id}/members/{user.id}` to be used for joining users to a guild.
    GuildsJoin,
    /// Allows `/users/@me/guilds/{guild.id}/member` to return a user's member information in a guild.
    GuildsMembersRead,
    /// Allows your app to join users to a group dm.
    GdmJoin,
    /// For local rpc server access, this allows you to control a user's local Discord client -
    /// requires Discord approval.
    Rpc,
    /// For local rpc server api access, this allows you to receive notifications pushed out to the user - requires Discord approval.
    RpcNotificationsRead,

    RpcVoiceRead,

    RpcVoiceWrite,

    RpcActivitiesWrite,
    /// This generates a webhook that is returned in the oauth token response for authorization code grants.
    WebhookIncomming, // TODO: fix misspelling
    /// For local rpc server api access, this allows you to read messages from all client channels
    /// (otherwise restricted to channels/guilds your app creates).
    MessagesRead,
    /// Allows your app to upload/update builds for a user's applications - requires Discord approval.
    ApplicationsBuildsUpload,
    /// Allows your app to read build data for a user's applications.
    ApplicationsBuildsRead,
    /// Allows your app to read and update store data (SKUs, store listings, achievements, etc.) for a user's applications.
    ApplicationsStoreUpdate,
    /// Allows your app to read entitlements for a user's applications.
    ApplicationsEntitlements,
    /// Allows your app to fetch data from a user's "Now Playing/Recently Played" list - requires Discord approval.
    ActivitiesRead,
    /// allows your app to update a user's activity - requires Discord approval (Not required for gamesdk activity manager!).
    ActivitiesWrite,
    /// Allows your app to know a user's friends and implicit relationships - requires Discord approval.
    RelactionshipsRead,
    /// Allows your app to see information about the user's DMs and group DMs - requires Discord approval.
    DmChannelsRead,
    /// Allows your app to connect to voice on user's behalf and see all the voice members - requires Discord approval.
    Voice,
}
