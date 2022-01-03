use std::env;

use serenity::{
    async_trait,
    model::{
        gateway::Ready,
        id::GuildId,
        interactions::{
            application_command::{
                ApplicationCommand,
                ApplicationCommandInteraction,
                ApplicationCommandInteractionDataOptionValue,
                ApplicationCommandOptionType,
            },
            Interaction,
            InteractionResponseType,
        },
    },
    prelude::*,
};

async fn interaction_handler(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
) -> Result<(), serenity::Error> {
    match command.data.name.as_str() {
        "ping" => {
            command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|m| m.content("Hey, I'm alive!"))
                })
                .await?;
        },
        "id" => {
            let options = command
                .data
                .options
                .get(0)
                .expect("Expected user option")
                .resolved
                .as_ref()
                .expect("Expected user object");

            let content = if let ApplicationCommandInteractionDataOptionValue::User(user, _member) =
                options
            {
                format!("{}'s id is {}", user.tag(), user.id)
            } else {
                "Please provide a valid user".to_string()
            };

            command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|m| m.content(content))
                })
                .await?;
        },
        "images" => {
            let ferris_eyes = "https://raw.githubusercontent.com/serenity-rs/serenity/current/examples/e09_create_message_builder/ferris_eyes.png";
            let serenity_logo =
                "https://raw.githubusercontent.com/serenity-rs/serenity/current/logo.png";

            command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|m| m.add_file(ferris_eyes))
                })
                .await?;
            command
                .create_followup_message(&ctx.http, |response| response.add_file(serenity_logo))
                .await?;
            command
                .edit_original_interaction_response(&ctx.http, |response| {
                    response.add_file(serenity_logo);
                    response.0.insert("attachments", serde_json::json! { [
                        { "filename": "idk.png", "description": "poopoo" }
                    ] });
                    response
                })
                .await?;
        },
        _ => {
            command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|m| m.content("not implemented :("))
                })
                .await?;
        },
    }

    Ok(())
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            if let Err(why) = interaction_handler(&ctx, &command).await {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| {
                    command.name("ping").description("A ping command")
                })
                .create_application_command(|command| {
                    command.name("id").description("Get a user id").create_option(|option| {
                        option
                            .name("id")
                            .description("The user to lookup")
                            .kind(ApplicationCommandOptionType::User)
                            .required(true)
                    })
                })
                .create_application_command(|command| {
                    command.name("images").description("Outputs some nice images")
                })
                .create_application_command(|command| {
                    command
                        .name("welcome")
                        .description("Welcome a user")
                        .create_option(|option| {
                            option
                                .name("user")
                                .description("The user to welcome")
                                .kind(ApplicationCommandOptionType::User)
                                .required(true)
                        })
                        .create_option(|option| {
                            option
                                .name("message")
                                .description("The message to send")
                                .kind(ApplicationCommandOptionType::String)
                                .required(true)
                                .add_string_choice(
                                    "Welcome to our cool server! Ask me if you need help",
                                    "pizza",
                                )
                                .add_string_choice("Hey, do you want a coffee?", "coffee")
                                .add_string_choice(
                                    "Welcome to the club, you're now a good person. Well, I hope.",
                                    "club",
                                )
                                .add_string_choice(
                                    "I hope that you brought a controller to play together!",
                                    "game",
                                )
                        })
                })
                .create_application_command(|command| {
                    command
                        .name("numberinput")
                        .description("Test command for number input")
                        .create_option(|option| {
                            option
                                .name("int")
                                .description("An integer from 5 to 10")
                                .kind(ApplicationCommandOptionType::Integer)
                                .min_int_value(5)
                                .max_int_value(10)
                                .required(true)
                        })
                        .create_option(|option| {
                            option
                                .name("number")
                                .description("A float from -3.3 to 234.5")
                                .kind(ApplicationCommandOptionType::Number)
                                .min_number_value(-3.3)
                                .max_number_value(234.5)
                                .required(true)
                        })
                })
        })
        .await;

        println!("I now have the following guild slash commands: {:#?}", commands);

        let guild_command =
            ApplicationCommand::create_global_application_command(&ctx.http, |command| {
                command.name("wonderful_command").description("An amazing command")
            })
            .await;

        println!("I created the following global slash command: {:#?}", guild_command);
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // The Application Id is usually the Bot User Id.
    let application_id: u64 = env::var("APPLICATION_ID")
        .expect("Expected an application id in the environment")
        .parse()
        .expect("application id is not a valid id");

    // Build our client.
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .application_id(application_id)
        .await
        .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
