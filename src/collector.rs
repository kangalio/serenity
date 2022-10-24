// Or we'll get deprecation warnings from our own deprecated type (seriously Rust?)
#![allow(deprecated)]

use futures::future::pending;
use futures::{Stream, StreamExt as _};

use crate::client::bridge::gateway::{CollectorCallback, ShardMessenger};
use crate::model::prelude::*;

/// Fundamental collector function. All collector types in this module are just wrappers around this
/// function.
///
/// Example: creating a collector stream over removed reactions
/// ```rust
/// # use std::time::Duration;
/// # use futures::StreamExt as _;
/// # use serenity::model::Event;
/// # async fn _example(shard: &ShardMessenger) {
/// let stream = collect(shard, Some(Duration::from_secs(10)), |event| match event {
///     Event::ReactionRemove(event) => Some(event.reaction),
///     _ => None,
/// });
/// stream
///     .for_each(|reaction| async {
///         println!("{}: removed {}", reaction.channel_id, reaction.emoji);
///     })
///     .await;
/// # }
/// ```
pub fn collect<T: Send + 'static>(
    shard: &ShardMessenger,
    extractor: impl Fn(&Event) -> Option<T> + Send + Sync + 'static,
) -> impl Stream<Item = T> {
    let (sender, mut receiver) = tokio::sync::mpsc::unbounded_channel();

    shard.add_collector(CollectorCallback(Box::new(move |event| match extractor(event) {
        Some(item) => sender.send(item).is_ok(),
        None => !sender.is_closed(),
    })));

    futures::stream::poll_fn(move |cx| receiver.poll_recv(cx))
}

macro_rules! make_specific_collector {
    (
        $( #[ $($meta:tt)* ] )*
        $collector_type:ident, $item_type:ident,
        $extractor:pat => $value:ident,
        $( $filter_name:ident: $filter_type:ident = $filter_extractor:expr, )*
    ) => {
        #[doc = concat!("A [`", stringify!($collector_type), "`] receives [`", stringify!($item_type), "`]'s match the given filters for a set duration.")]
        $( #[ $($meta)* ] )*
        #[must_use]
        pub struct $collector_type {
            shard: ShardMessenger,
            duration: Option<std::time::Duration>,
            filter: Option<Box<dyn Fn(&$item_type) -> bool + Send + Sync>>,
            $( $filter_name: Option<$filter_type>, )*
        }

        impl $collector_type {
            /// Creates a new collector without any filters configured.
            pub fn new(shard: impl AsRef<ShardMessenger>) -> Self {
                Self {
                    shard: shard.as_ref().clone(),
                    duration: None,
                    filter: None,
                    $( $filter_name: None, )*
                }
            }

            /// Sets a duration for how long the collector shall receive interactions.
            pub fn timeout(mut self, duration: std::time::Duration) -> Self {
                self.duration = Some(duration);
                self
            }

            /// Sets a generic filter function.
            pub fn filter(mut self, filter: impl Fn(&$item_type) -> bool + Send + Sync + 'static) -> Self {
                self.filter = Some(Box::new(filter));
                self
            }

            $(
                #[doc = concat!("Filters [`", stringify!($item_type), "`]'s by a specific [`", stringify!($filter_type), "`].")]
                pub fn $filter_name(mut self, $filter_name: $filter_type) -> Self {
                    self.$filter_name = Some($filter_name);
                    self
                }
            )*

            #[doc = concat!("Returns a [`Stream`] over all collected [`", stringify!($item_type), "`].")]
            pub fn collect_stream(self) -> impl Stream<Item = $item_type> {
                collect(&self.shard, move |event| match event {
                    $extractor
                        if $( self.$filter_name.map_or(true, |x| $filter_extractor == Some(x)) && )*
                            self.filter.as_ref().map_or(true, |f| f($value)) =>
                    {
                        Some($value.clone())
                    },
                    _ => None,
                })
                // Need to Box::pin this, or else users have to `pin_mut!()` the stream to the stack
                .take_until(Box::pin(async move { match self.duration {
                    Some(d) => tokio::time::sleep(d).await,
                    None => pending::<()>().await,
                } }))
            }

            #[doc = concat!("Returns the next [`", stringify!($item_type), "`] which passes the filters.")]
            pub async fn collect_single(self) -> Option<$item_type> {
                self.collect_stream().next().await
            }
        }
    };
}

make_specific_collector!(
    ComponentInteractionCollector, ComponentInteraction,
    Event::InteractionCreate(InteractionCreateEvent {
        interaction: Interaction::Component(interaction),
    }) => interaction,
    author_id: UserId = Some(interaction.user.id),
    channel_id: ChannelId = Some(interaction.channel_id),
    guild_id: GuildId = interaction.guild_id,
    message_id: MessageId = Some(interaction.message.id),
);
make_specific_collector!(
    ModalInteractionCollector, ModalSubmitInteraction,
    Event::InteractionCreate(InteractionCreateEvent {
        interaction: Interaction::ModalSubmit(interaction),
    }) => interaction,
    author_id: UserId = Some(interaction.user.id),
    channel_id: ChannelId = Some(interaction.channel_id),
    guild_id: GuildId = interaction.guild_id,
    message_id: MessageId = interaction.message.as_ref().map(|m| m.id),
);
make_specific_collector!(
    ReactionCollector, Reaction,
    Event::ReactionAdd(ReactionAddEvent { reaction }) => reaction,
    author_id: UserId = reaction.user_id,
    channel_id: ChannelId = Some(reaction.channel_id),
    guild_id: GuildId = reaction.guild_id,
    message_id: MessageId = Some(reaction.message_id),
);
make_specific_collector!(
    MessageCollector, Message,
    Event::MessageCreate(MessageCreateEvent { message }) => message,
    author_id: UserId = Some(message.author.id),
    channel_id: ChannelId = Some(message.channel_id),
    guild_id: GuildId = message.guild_id,
);
make_specific_collector!(
    #[deprecated = "use the collect() function to collect arbitrary events"]
    EventCollector, Event,
    event => event,
);
