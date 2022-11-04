use serde::de::{Deserialize, Deserializer, Error as DeError};
use serde::ser::{Error as _, Serialize};

#[cfg(feature = "model")]
use crate::builder::{
    CreateInteractionResponse,
    CreateInteractionResponseFollowup,
    EditInteractionResponse,
};
#[cfg(feature = "collector")]
use crate::builder::{CreateQuickModal, QuickModalResponse};
#[cfg(feature = "collector")]
use crate::client::Context;
#[cfg(feature = "model")]
use crate::http::Http;
use crate::internal::prelude::*;
use crate::model::application::component::ComponentType;
use crate::model::application::interaction::add_guild_id_to_resolved;
use crate::model::channel::Message;
use crate::model::guild::Member;
#[cfg(feature = "model")]
use crate::model::id::MessageId;
use crate::model::id::{
    ApplicationId,
    ChannelId,
    GenericId,
    GuildId,
    InteractionId,
    RoleId,
    UserId,
};
use crate::model::user::User;
use crate::model::Permissions;

/// An interaction triggered by a message component.
///
/// [Discord docs](https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-interaction-structure).
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct ComponentInteraction {
    /// Id of the interaction.
    pub id: InteractionId,
    /// Id of the application this interaction is for.
    pub application_id: ApplicationId,
    /// The data of the interaction which was triggered.
    pub data: ComponentInteractionData,
    /// The guild Id this interaction was sent from, if there is one.
    pub guild_id: Option<GuildId>,
    /// The channel Id this interaction was sent from.
    pub channel_id: ChannelId,
    /// The `member` data for the invoking user.
    ///
    /// **Note**: It is only present if the interaction is triggered in a guild.
    pub member: Option<Member>,
    /// The `user` object for the invoking user.
    pub user: User,
    /// A continuation token for responding to the interaction.
    pub token: String,
    /// Always `1`.
    pub version: u8,
    /// The message this interaction was triggered by, if
    /// it is a component.
    pub message: Box<Message>,
    /// Permissions the app or bot has within the channel the interaction was sent from.
    pub app_permissions: Option<Permissions>,
    /// The selected language of the invoking user.
    pub locale: String,
    /// The guild's preferred locale.
    pub guild_locale: Option<String>,
}

#[cfg(feature = "model")]
impl ComponentInteraction {
    /// Gets the interaction response.
    ///
    /// # Errors
    ///
    /// Returns an [`Error::Http`] if there is no interaction response.
    pub async fn get_response(&self, http: impl AsRef<Http>) -> Result<Message> {
        http.as_ref().get_original_interaction_response(&self.token).await
    }

    /// Creates a response to the interaction received.
    ///
    /// **Note**: Message contents must be under 2000 unicode code points.
    ///
    /// # Errors
    ///
    /// Returns an [`Error::Model`] if the message content is too long. May also return an
    /// [`Error::Http`] if the API returns an error, or an [`Error::Json`] if there is an error in
    /// deserializing the API response.
    pub async fn create_response(
        &self,
        http: impl AsRef<Http>,
        builder: CreateInteractionResponse,
    ) -> Result<()> {
        builder.execute(http, self.id, &self.token).await
    }

    /// Edits the initial interaction response. Does not work for ephemeral messages.
    ///
    /// **Note**: Message contents must be under 2000 unicode code points.
    ///
    /// # Errors
    ///
    /// Returns an [`Error::Model`] if the message content is too long. May also return an
    /// [`Error::Http`] if the API returns an error, or an [`Error::Json`] if there is an error in
    /// deserializing the API response.
    pub async fn edit_response(
        &self,
        http: impl AsRef<Http>,
        builder: EditInteractionResponse,
    ) -> Result<Message> {
        builder.execute(http, &self.token).await
    }

    /// Deletes the initial interaction response.
    ///
    /// Does not work on ephemeral messages.
    ///
    /// # Errors
    ///
    /// May return [`Error::Http`] if the API returns an error.
    /// Such as if the response was already deleted.
    pub async fn delete_response(&self, http: impl AsRef<Http>) -> Result<()> {
        http.as_ref().delete_original_interaction_response(&self.token).await
    }

    /// Creates a followup response to the response sent.
    ///
    /// **Note**: Message contents must be under 2000 unicode code points.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Model`] if the content is too long. May also return [`Error::Http`] if the
    /// API returns an error, or [`Error::Json`] if there is an error in deserializing the
    /// response.
    pub async fn create_followup(
        &self,
        http: impl AsRef<Http>,
        builder: CreateInteractionResponseFollowup,
    ) -> Result<Message> {
        builder.execute(http, None, &self.token).await
    }

    /// Edits a followup response to the response sent.
    ///
    /// **Note**: Message contents must be under 2000 unicode code points.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Model`] if the content is too long. May also return [`Error::Http`] if the
    /// API returns an error, or [`Error::Json`] if there is an error in deserializing the
    /// response.
    pub async fn edit_followup(
        &self,
        http: impl AsRef<Http>,
        message_id: impl Into<MessageId>,
        builder: CreateInteractionResponseFollowup,
    ) -> Result<Message> {
        builder.execute(http, Some(message_id.into()), &self.token).await
    }

    /// Deletes a followup message.
    ///
    /// # Errors
    ///
    /// May return [`Error::Http`] if the API returns an error.
    /// Such as if the response was already deleted.
    pub async fn delete_followup<M: Into<MessageId>>(
        &self,
        http: impl AsRef<Http>,
        message_id: M,
    ) -> Result<()> {
        http.as_ref().delete_followup_message(&self.token, message_id.into()).await
    }

    /// Gets a followup message.
    ///
    /// # Errors
    ///
    /// May return [`Error::Http`] if the API returns an error.
    /// Such as if the response was deleted.
    pub async fn get_followup<M: Into<MessageId>>(
        &self,
        http: impl AsRef<Http>,
        message_id: M,
    ) -> Result<Message> {
        http.as_ref().get_followup_message(&self.token, message_id.into()).await
    }

    /// Helper function to defer an interaction.
    ///
    /// # Errors
    ///
    /// Returns an [`Error::Http`] if the API returns an error, or an [`Error::Json`] if there is
    /// an error in deserializing the API response.
    pub async fn defer(&self, http: impl AsRef<Http>) -> Result<()> {
        self.create_response(http, CreateInteractionResponse::Acknowledge).await
    }

    /// See [`CreateQuickModal`].
    ///
    /// # Errors
    ///
    /// See [`CreateQuickModal::execute()`].
    #[cfg(feature = "collector")]
    pub async fn quick_modal(
        &self,
        ctx: &Context,
        builder: CreateQuickModal,
    ) -> Result<Option<QuickModalResponse>> {
        builder.execute(ctx, self.id, &self.token).await
    }
}

#[derive(Clone, Debug)]
pub enum ComponentInteractionDataKind {
    Button,
    StringSelect { values: Vec<String> },
    UserSelect { values: Vec<UserId> },
    RoleSelect { values: Vec<RoleId> },
    MentionableSelect { values: Vec<GenericId> },
    ChannelSelect { values: Vec<ChannelId> },
    Unknown(u8),
}

/// A message component interaction data, provided by [`ComponentInteraction::data`]
///
/// [Discord docs](https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-message-component-data-structure).
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct ComponentInteractionData {
    /// The custom id of the component.
    pub custom_id: String,
    /// Type and type-specific data of this component interaction.
    pub kind: ComponentInteractionDataKind,
}
