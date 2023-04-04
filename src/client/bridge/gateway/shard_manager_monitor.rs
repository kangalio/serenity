use std::sync::Arc;

use futures::channel::mpsc::{UnboundedReceiver as Receiver, UnboundedSender as Sender};
use futures::StreamExt;
use tokio::sync::Mutex;
use tracing::{debug, instrument, warn};

use super::{ShardManager, ShardManagerMessage};
use crate::client::bridge::gateway::ShardId;

/// The shard manager monitor monitors the shard manager and performs actions on it as received.
///
/// The monitor is essentially responsible for running in its own task and receiving
/// [`ShardManagerMessage`]s, such as whether to shutdown a shard or shutdown everything entirely.
#[derive(Debug)]
pub struct ShardManagerMonitor {
    /// An clone of the Arc to the manager itself.
    pub manager: Arc<Mutex<ShardManager>>,
    /// The mpsc Receiver channel to receive shard manager messages over.
    pub rx: Receiver<ShardManagerMessage>,
    /// The mpsc Sender channel to inform the manager that a shard has just properly shut down
    pub shutdown: Sender<ShardId>,
}
#[derive(Debug)]
pub enum ShardManagerError {
    /// Returned when a shard received an [`InvalidAuthentication`] error. An invalid token has
    /// been specified.
    ///
    /// [`InvalidAuthentication`]: crate::gateway::GatewayError::InvalidAuthentication
    InvalidToken,
    /// Returned when a shard received an [`InvalidGatewayIntents`] error.
    ///
    /// [`InvalidGatewayIntents`]: crate::gateway::GatewayError::InvalidGatewayIntents
    InvalidGatewayIntents,
    /// Returned when a shard received a [`DisallowedGatewayIntents`] error.
    ///
    /// [`DisallowedGatewayIntents`]: crate::gateway::GatewayError::DisallowedGatewayIntents
    DisallowedGatewayIntents,
}

type Result<T> = std::result::Result<T, ShardManagerError>;

impl ShardManagerMonitor {
    /// "Runs" the monitor, waiting for messages over the Receiver.
    ///
    /// This should be called in its own thread due to its blocking, looped nature.
    ///
    /// This will continue running until either:
    /// - a [`ShardManagerMessage::ShutdownAll`] has been received
    /// - an error is returned while receiving a message from the channel (probably indicating that
    /// the shard manager should stop anyway)
    #[instrument(skip(self))]
    pub async fn run(&mut self) -> Result<()> {
        debug!("Starting shard manager worker");

        while let Some(value) = self.rx.next().await {
            match value {
                ShardManagerMessage::ShutdownInitiated => break,
                ShardManagerMessage::ShardInvalidAuthentication => {
                    self.manager.lock().await.shutdown_all().await;
                    return Err(ShardManagerError::InvalidToken);
                },

                ShardManagerMessage::ShardInvalidGatewayIntents => {
                    self.manager.lock().await.shutdown_all().await;
                    return Err(ShardManagerError::InvalidGatewayIntents);
                },
                ShardManagerMessage::ShardDisallowedGatewayIntents => {
                    self.manager.lock().await.shutdown_all().await;
                    return Err(ShardManagerError::DisallowedGatewayIntents);
                },
            }
        }

        Ok(())
    }
}
