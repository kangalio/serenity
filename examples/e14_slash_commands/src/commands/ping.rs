use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::ResolvedOption;

pub fn run(_options: &[ResolvedOption]) -> String {
    "Hey, I'm alive!".to_string()
}

pub fn register() -> CreateApplicationCommand {
    CreateApplicationCommand::new("ping", "A ping command")
}
