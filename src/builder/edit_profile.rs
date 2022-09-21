use super::CreateAttachment;
#[cfg(feature = "http")]
use crate::http::Http;
#[cfg(feature = "http")]
use crate::internal::prelude::*;
#[cfg(feature = "http")]
use crate::model::user::CurrentUser;

/// A builder to edit the current user's settings, to be used in conjunction with
/// [`CurrentUser::edit`].
#[derive(Clone, Debug, Default, Serialize)]
#[must_use]
pub struct EditProfile {
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<String>,
}

impl EditProfile {
    /// Equivalent to [`Self::default`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Edit the current user's profile with the fields set.
    ///
    /// # Errors
    ///
    /// Returns an [`Error::Http`] if an invalid value is set. May also return an [`Error::Json`]
    /// if there is an error in deserializing the API response.
    #[cfg(feature = "http")]
    pub async fn execute(self, http: impl AsRef<Http>) -> Result<CurrentUser> {
        http.as_ref().edit_profile(&self).await
    }

    /// Set the avatar of the current user.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # #[cfg(all(feature = "client", feature = "cache", feature = "gateway"))]
    /// # {
    /// # use serenity::builder::EditProfile;
    /// # use serenity::prelude::*;
    /// # use serenity::model::prelude::*;
    /// #
    /// # struct Handler;
    /// #
    /// # #[serenity::async_trait]
    /// # impl EventHandler for Handler {
    /// #     async fn message(&self, context: Context, _: Message) {
    /// // assuming a `context` has been bound
    /// let mut user = context.cache.current_user().clone();
    ///
    /// let builder =
    ///     EditProfile::new().avatar(&context, "./my_image.jpg").await.expect("Failed to read image.");
    /// let _ = user.edit(&context, builder).await;
    /// #     }
    /// # }
    /// # }
    /// ```
    pub fn avatar(mut self, avatar: CreateAttachment<'_>) -> Self {
        self.avatar = Some(Some(crate::utils::encode_image(&avatar.data)));
        self
    }

    /// Delete the current user's avatar, resetting it to the default logo.
    pub fn delete_avatar(mut self) -> Self {
        self.avatar = Some(None);
        self
    }

    /// Modifies the current user's username.
    ///
    /// When modifying the username, if another user has the same _new_ username and current
    /// discriminator, a new unique discriminator will be assigned. If there are no available
    /// discriminators with the requested username, an error will occur.
    pub fn username(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self
    }
}
