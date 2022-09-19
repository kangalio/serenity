use crate::model::{Colour, Timestamp};

/// Represents a rich embed which allows using richer markdown, multiple fields
/// and more. This was heavily inspired by [slack's attachments].
///
/// You can include an attachment in your own message by a user or a bot, or in
/// a webhook.
///
/// **Note**: Maximum amount of characters you can put is 256 in a field name,
/// 1024 in a field value, and 2048 in a description.
///
/// [Discord docs](https://discord.com/developers/docs/resources/channel#embed-object).
///
/// [slack's attachments]: https://api.slack.com/docs/message-attachments
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Embed {
    /// Information about the author of the embed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<EmbedAuthor>,
    /// The colour code of the embed.
    #[serde(rename = "color")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colour: Option<Colour>,
    /// The description of the embed.
    ///
    /// The maximum value for this field is 2048 unicode codepoints.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The array of fields.
    ///
    /// The maximum number of fields is 25.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<EmbedField>,
    /// Footer information for the embed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<EmbedFooter>,
    /// Image information of the embed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<EmbedImage>,
    /// The type of the embed. For embeds not generated by Discord's backend,
    /// this will always be "rich".
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Provider information for the embed.
    ///
    /// For example, if the embed [`Self::kind`] is `"video"`, the provider might
    /// contain information about YouTube.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<EmbedProvider>,
    /// Thumbnail information of the embed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<EmbedThumbnail>,
    /// Timestamp information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<Timestamp>,
    /// The title of the embed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The URL of the embed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The embed's video information.
    ///
    /// This is present if the [`Self::kind`] is `"video"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<EmbedVideo>,
    #[doc(hidden)]
    #[serde(default, skip)]
    pub __non_exhaustive: (),
}

/// An author object in an embed.
///
/// [Discord docs](https://discord.com/developers/docs/resources/channel#embed-object-embed-author-structure).
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct EmbedAuthor {
    /// The name of the author.
    pub name: String,
    /// The URL of the author.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The URL of the author icon.
    ///
    /// This only supports HTTP(S) and attachments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// A proxied URL of the author icon.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_icon_url: Option<String>,
}

/// A field object in an embed.
///
/// [Discord docs](https://discord.com/developers/docs/resources/channel#embed-object-embed-field-structure).
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct EmbedField {
    /// The name of the field.
    ///
    /// The maximum length of this field is 512 unicode codepoints.
    pub name: String,
    /// The value of the field.
    ///
    /// The maximum length of this field is 1024 unicode codepoints.
    pub value: String,
    /// Indicator of whether the field should display as inline.
    #[serde(default)]
    pub inline: bool,
}

impl EmbedField {
    /// Creates a new embed field.
    ///
    /// **Note**: Refer to the [`Self::name`] and [`Self::value`] documentation for maximum
    /// lengths.
    pub fn new(name: impl Into<String>, value: impl Into<String>, inline: bool) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
            inline,
        }
    }
}

/// Footer information for an embed.
///
/// [Discord docs](https://discord.com/developers/docs/resources/channel#embed-object-embed-footer-structure).
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct EmbedFooter {
    /// The associated text with the footer.
    pub text: String,
    /// The URL of the footer icon.
    ///
    /// This only supports HTTP(S) and attachments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// A proxied URL of the footer icon.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_icon_url: Option<String>,
}

/// An image object in an embed.
///
/// [Discord docs](https://discord.com/developers/docs/resources/channel#embed-object-embed-image-structure).
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct EmbedImage {
    /// Source URL of the image.
    ///
    /// This only supports HTTP(S) and attachments.
    pub url: String,
    /// A proxied URL of the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_url: Option<String>,
    /// The height of the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u64>,
    /// The width of the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u64>,
}

/// The provider of an embed.
///
/// [Discord docs](https://discord.com/developers/docs/resources/channel#embed-object-embed-provider-structure).
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct EmbedProvider {
    /// The name of the provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The URL of the provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// The dimensions and URL of an embed thumbnail.
///
/// [Discord docs](https://discord.com/developers/docs/resources/channel#embed-object-embed-thumbnail-structure).
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct EmbedThumbnail {
    /// The source URL of the thumbnail.
    ///
    /// This only supports HTTP(S) and attachments.
    pub url: String,
    /// A proxied URL of the thumbnail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_url: Option<String>,
    /// The height of the thumbnail in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u64>,
    /// The width of the thumbnail in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u64>,
}

/// Video information for an embed.
///
/// [Discord docs](https://discord.com/developers/docs/resources/channel#embed-object-embed-video-structure).
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct EmbedVideo {
    /// The source URL of the video.
    pub url: String,
    /// A proxied URL of the thumbnail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_url: Option<String>,
    /// The height of the video in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u64>,
    /// The width of the video in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u64>,
}
