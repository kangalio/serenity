use super::{
    CreateAllowedMentions,
    CreateAttachment,
    CreateComponents,
    CreateEmbed,
    ExistingAttachment,
};
#[cfg(feature = "http")]
use crate::constants;
#[cfg(feature = "http")]
use crate::http::Http;
#[cfg(feature = "http")]
use crate::internal::prelude::*;
#[cfg(feature = "http")]
use crate::model::prelude::*;
#[cfg(feature = "http")]
use crate::utils::check_overflow;

#[derive(Clone, Debug, Default, Serialize)]
#[must_use]
pub struct EditInteractionResponse<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    embeds: Option<Vec<CreateEmbed>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_mentions: Option<CreateAllowedMentions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    components: Option<CreateComponents>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attachments: Option<Vec<ExistingAttachment>>,

    #[serde(skip)]
    files: Vec<CreateAttachment<'a>>,
}

impl<'a> EditInteractionResponse<'a> {
    /// Equivalent to [`Self::default`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Edits the initial interaction response. Does not work for ephemeral messages.
    ///
    /// The `application_id` used will usually be the bot's [`UserId`], except if the bot is very
    /// old.
    ///
    /// **Note**: Message contents must be under 2000 unicode code points, and embeds must be under
    /// 6000 code points.
    ///
    /// # Errors
    ///
    /// Returns an [`Error::Model`] if the message content is too long. May also return an
    /// [`Error::Http`] if the API returns an error, or an [`Error::Json`] if there is an error in
    /// deserializing the API response.
    #[cfg(feature = "http")]
    pub async fn execute(mut self, http: impl AsRef<Http>, token: &str) -> Result<Message> {
        self.check_length()?;
        let files = std::mem::take(&mut self.files);
        http.as_ref().edit_original_interaction_response(token, &self, files).await
    }

    #[cfg(feature = "http")]
    fn check_length(&self) -> Result<()> {
        if let Some(content) = &self.content {
            check_overflow(content.chars().count(), constants::MESSAGE_CODE_LIMIT)
                .map_err(|overflow| Error::Model(ModelError::MessageTooLong(overflow)))?;
        }

        if let Some(embeds) = &self.embeds {
            check_overflow(embeds.len(), constants::EMBED_MAX_COUNT)
                .map_err(|_| Error::Model(ModelError::EmbedAmount))?;
            for embed in embeds {
                embed.check_length()?;
            }
        }

        Ok(())
    }

    /// Sets the `InteractionApplicationCommandCallbackData` for the message.

    /// Set the content of the message.
    ///
    /// **Note**: Message contents must be under 2000 unicode code points.
    #[inline]
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    /// Adds an embed for the message.
    ///
    /// Embeds from the original message creation are not kept and must be re-added.
    pub fn add_embed(mut self, embed: CreateEmbed) -> Self {
        self.embeds.get_or_insert(Vec::new()).push(embed);
        self
    }

    /// Adds multiple embeds to the message.
    ///
    /// Embeds from the original message creation are not kept and must be re-added.
    pub fn add_embeds(mut self, embeds: Vec<CreateEmbed>) -> Self {
        self.embeds.get_or_insert(Vec::new()).extend(embeds);
        self
    }

    /// Sets a single embed to include in the message
    ///
    /// Calling this will overwrite the embed list. To append embeds, call [`Self::add_embed`]
    /// instead.
    pub fn embed(mut self, embed: CreateEmbed) -> Self {
        self.embeds = Some(vec![embed]);
        self
    }

    /// Sets the embeds for the message.
    ///
    /// **Note**: You can only have up to 10 embeds per message.
    ///
    /// Calling this will overwrite the embed list. To append embeds, call [`Self::add_embeds`]
    /// instead.
    pub fn embeds(mut self, embeds: Vec<CreateEmbed>) -> Self {
        self.embeds = Some(embeds);
        self
    }

    /// Set the allowed mentions for the message.
    pub fn allowed_mentions(mut self, allowed_mentions: CreateAllowedMentions) -> Self {
        self.allowed_mentions = Some(allowed_mentions);
        self
    }

    /// Sets the components of this message.
    pub fn components(mut self, components: CreateComponents) -> Self {
        self.components = Some(components);
        self
    }

    /// Adds a new attachment for the message.
    ///
    /// This can be called multiple times.
    ///
    /// Existing attachments are kept. To reset them, or only keep a subset, see
    /// [`Self::keep_specific_attachments()`].
    pub fn add_attachment(mut self, attachment: impl Into<CreateAttachment<'a>>) -> Self {
        let fake_attachment_id = self.files.len();
        self.files.push(attachment.into());
        self.attachments.get_or_insert(Vec::new()).push(ExistingAttachment {
            id: fake_attachment_id as _,
            description: Some("my ass".into()),
        });
        self
    }

    /// Only keeps a specified subset of existing attachments. By default, all existing attachments
    /// are kept.
    ///
    /// New attachments must be included in this list too, or they will not be sent (TODO: correct?)
    pub fn keep_specific_attachments(mut self, ids: Vec<u64>) -> Self {
        self.attachments = Some(
            ids.into_iter()
                .map(|id| ExistingAttachment {
                    id,
                    description: None,
                })
                .collect(),
        );

        self
    }
}
