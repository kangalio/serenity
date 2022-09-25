use serenity::builder::*;
use serenity::model::prelude::interaction::application_command::*;
use serenity::model::prelude::*;
use serenity::prelude::*;

const IMAGE_URL: &str = "https://raw.githubusercontent.com/serenity-rs/serenity/current/logo.png";
const IMAGE_URL_2: &str = "https://rustacean.net/assets/rustlogo.png";

async fn message(ctx: &Context, msg: Message) -> Result<(), serenity::Error> {
    let guild_id = msg.guild_id.unwrap();
    if msg.content == "register" {
        guild_id
            .create_application_command(
                &ctx,
                CreateApplicationCommand::new("editembeds").description("test command"),
            )
            .await?;
        guild_id
            .create_application_command(
                &ctx,
                CreateApplicationCommand::new("editattachments").description("test command"),
            )
            .await?;
    } else if msg.content == "edit" {
        let mut msg = msg
            .channel_id
            .send_message(
                &ctx,
                CreateMessage::new().add_file(CreateAttachment::url(ctx, IMAGE_URL).await?),
            )
            .await?;
        // This falsely triggered a MODEL_TYPE_CONVERT Discord error pre-PR
        msg.edit(&ctx, EditMessage::new().add_existing_attachment(msg.attachments[0].id.get()))
            .await?;
    } else if msg.content == "createsticker" {
        let hmm = "https://apng.onevcat.com/assets/elephant.png";
        let mut attachment = CreateAttachment::url(ctx, hmm).await?;
        // attachment.filename = "lol.png".into();
        guild_id
            .create_sticker(ctx, CreateSticker::new("sticker", "tags", "description", attachment))
            .await?;
    } else {
        return Ok(());
    }

    msg.react(&ctx, '✅').await?;
    Ok(())
}

async fn interaction(
    ctx: &Context,
    interaction: ApplicationCommandInteraction,
) -> Result<(), serenity::Error> {
    if interaction.data.name == "editembeds" {
        interaction
            .create_interaction_response(
                &ctx,
                CreateInteractionResponse::new().interaction_response_data(
                    CreateInteractionResponseData::new()
                        .content("hi")
                        .embed(CreateEmbed::new().description("hi")),
                ),
            )
            .await?;

        // This falsely deleted the embed pre-PR
        interaction
            .edit_original_interaction_response(&ctx, EditInteractionResponse::new().content("hi2"))
            .await?;
    } else if interaction.data.name == "editattachments" {
        interaction
            .create_interaction_response(
                &ctx,
                CreateInteractionResponse::new().interaction_response_data(
                    CreateInteractionResponseData::new()
                        .add_file(CreateAttachment::url(ctx, IMAGE_URL).await?),
                ),
            )
            .await?;

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        let msg = interaction
            .edit_original_interaction_response(
                &ctx,
                EditInteractionResponse::new()
                    .add_attachment(CreateAttachment::url(ctx, IMAGE_URL_2).await?),
            )
            .await?;

        // interaction
        //     .edit_original_interaction_response(
        //         &ctx,
        //         EditInteractionResponse::new()
        //             .keep_specific_attachments(vec![msg.attachments[1].id.get()]),
        //     )
        //     .await?;
    }

    Ok(())
}

struct Handler;
#[serenity::async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        message(&ctx, msg).await.unwrap();
    }

    async fn interaction_create(&self, ctx: Context, i: Interaction) {
        if let Interaction::ApplicationCommand(i) = i {
            interaction(&ctx, i).await.unwrap();
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), serenity::Error> {
    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    Client::builder(token, intents).event_handler(Handler).await?.start().await
}
