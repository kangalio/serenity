#[cfg(feature = "http")]
use crate::http::{CacheHttp, Http};
#[cfg(feature = "http")]
use crate::internal::prelude::*;
#[cfg(feature = "http")]
use crate::model::prelude::*;

/// Edits a [`StageInstance`].
#[derive(Clone, Debug, Default, Serialize)]
#[must_use]
pub struct EditStageInstance {
    #[serde(skip_serializing_if = "Option::is_none")]
    topic: Option<String>,
}

impl EditStageInstance {
    /// Edits the stage instance
    ///
    /// # Errors
    ///
    /// Returns [`ModelError::InvalidChannelType`] if the channel is not a stage channel.
    ///
    /// Returns [`Error::Http`] if the channel is not a stage channel, or there is no stage
    /// instance currently.
    #[cfg(feature = "http")]
    #[inline]
    pub async fn execute(
        self,
        cache_http: impl CacheHttp,
        channel_id: ChannelId,
    ) -> Result<StageInstance> {
        #[cfg(feature = "cache")]
        {
            if let Some(cache) = cache_http.cache() {
                if let Some(channel) = cache.guild_channel(channel_id) {
                    if channel.kind != ChannelType::Stage {
                        return Err(Error::Model(ModelError::InvalidChannelType));
                    }
                }
            }
        }

        self._execute(cache_http.http(), channel_id).await
    }

    #[cfg(feature = "http")]
    async fn _execute(self, http: &Http, channel_id: ChannelId) -> Result<StageInstance> {
        http.edit_stage_instance(channel_id.into(), &self).await
    }

    /// Sets the topic of the stage channel instance.
    pub fn topic(mut self, topic: impl Into<String>) -> Self {
        self.topic = Some(topic.into());
        self
    }
}
