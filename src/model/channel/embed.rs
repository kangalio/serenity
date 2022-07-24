use crate::model::Timestamp;
#[cfg(feature = "utils")]
use crate::utils::Colour;

/// Represents a rich embed which allows using richer markdown, multiple fields
/// and more. This was heavily inspired by [slack's attachments].
///
/// You can include an attachment in your own message by a user or a bot, or in
/// a webhook.
///
/// **Note**: Maximum amount of characters you can put is 256 in a field name,
/// 1024 in a field value, and 2048 in a description.
///
/// [slack's attachments]: https://api.slack.com/docs/message-attachments
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Embed {
    /// Information about the author of the embed.
    pub author: Option<EmbedAuthor>,
    /// The colour code of the embed.
    #[cfg(feature = "utils")]
    #[serde(rename = "color")]
    pub colour: Option<Colour>,
    /// The colour code of the embed.
    #[cfg(not(feature = "utils"))]
    #[serde(default, rename = "color")]
    pub colour: u32,
    /// The description of the embed.
    ///
    /// The maximum value for this field is 2048 unicode codepoints.
    pub description: Option<String>,
    /// The array of fields.
    ///
    /// The maximum number of fields is 25.
    #[serde(default)]
    pub fields: Vec<EmbedField>,
    /// Footer information for the embed.
    pub footer: Option<EmbedFooter>,
    /// Image information of the embed.
    pub image: Option<EmbedImage>,
    /// The type of the embed. For embeds not generated by Discord's backend,
    /// this will always be "rich".
    #[serde(rename = "type")]
    pub kind: Option<String>,
    /// Provider information for the embed.
    ///
    /// For example, if the embed [`Self::kind`] is `"video"`, the provider might
    /// contain information about YouTube.
    pub provider: Option<EmbedProvider>,
    /// Thumbnail information of the embed.
    pub thumbnail: Option<EmbedThumbnail>,
    /// Timestamp information.
    pub timestamp: Option<Timestamp>,
    /// The title of the embed.
    pub title: Option<String>,
    /// The URL of the embed.
    pub url: Option<String>,
    /// The embed's video information.
    ///
    /// This is present if the [`Self::kind`] is `"video"`.
    pub video: Option<EmbedVideo>,
}

/// An author object in an embed.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct EmbedAuthor {
    /// The name of the author.
    pub name: String,
    /// The URL of the author.
    pub url: Option<String>,
    /// The URL of the author icon.
    ///
    /// This only supports HTTP(S) and attachments.
    pub icon_url: Option<String>,
    /// A proxied URL of the author icon.
    pub proxy_icon_url: Option<String>,
}

/// A field object in an embed.
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
    pub fn new<T, U>(name: T, value: U, inline: bool) -> Self
    where
        T: Into<String>,
        U: Into<String>,
    {
        Self::_new(name.into(), value.into(), inline)
    }

    pub(crate) const fn _new(name: String, value: String, inline: bool) -> Self {
        Self {
            name,
            value,
            inline,
        }
    }
}

/// Footer information for an embed.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct EmbedFooter {
    /// The associated text with the footer.
    pub text: String,
    /// The URL of the footer icon.
    ///
    /// This only supports HTTP(S) and attachments.
    pub icon_url: Option<String>,
    /// A proxied URL of the footer icon.
    pub proxy_icon_url: Option<String>,
}

/// An image object in an embed.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct EmbedImage {
    /// Source URL of the image.
    ///
    /// This only supports HTTP(S) and attachments.
    pub url: String,
    /// A proxied URL of the image.
    pub proxy_url: Option<String>,
    /// The height of the image.
    pub height: Option<u64>,
    /// The width of the image.
    pub width: Option<u64>,
}

/// The provider of an embed.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct EmbedProvider {
    /// The name of the provider.
    pub name: Option<String>,
    /// The URL of the provider.
    pub url: Option<String>,
}

/// The dimensions and URL of an embed thumbnail.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct EmbedThumbnail {
    /// The source URL of the thumbnail.
    ///
    /// This only supports HTTP(S) and attachments.
    pub url: String,
    /// A proxied URL of the thumbnail.
    pub proxy_url: Option<String>,
    /// The height of the thumbnail in pixels.
    pub height: Option<u64>,
    /// The width of the thumbnail in pixels.
    pub width: Option<u64>,
}

/// Video information for an embed.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct EmbedVideo {
    /// The source URL of the video.
    pub url: String,
    /// A proxied URL of the thumbnail.
    pub proxy_url: Option<String>,
    /// The height of the video in pixels.
    pub height: Option<u64>,
    /// The width of the video in pixels.
    pub width: Option<u64>,
}
