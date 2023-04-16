use rustgie::types::destiny::{DamageType, DestinyItemSubType, DestinyItemType};
use rustgie::types::*;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

pub fn run(options: &[CommandDataOption]) -> String {
    let option = options
        .get(0)
        .expect("Expected user option")
        .resolved
        .as_ref()
        .expect("Expected user object");

    if let CommandDataOptionValue::User(user, _member) = option {
        format!("{}'s id is {}", user.tag(), user.id)
    } else {
        "Please provide a valid user".to_string()
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("foundry")
        .description("Filter for a weapon")
        .create_option(|option| {
            option
                .name("name")
                .description("Full or partial name of weapon")
                .kind(CommandOptionType::String)
                .required(false)
        })
        .create_option(|option| {
            option
                .name("weapon type")
                .description("Type of weapon")
                .kind(CommandOptionType::String)
                .add_string_choice("Auto Rifle", DestinyItemSubType::AutoRifle)
                .add_string_choice("Shotgun", DestinyItemSubType::Shotgun)
                .add_string_choice("Machine Gun", DestinyItemSubType::Machinegun)
                .add_string_choice("Hand Cannon", DestinyItemSubType::HandCannon)
                .add_string_choice("Rocket Launcher", DestinyItemSubType::HandCannon)
                .add_string_choice("Fusion Rifle", DestinyItemSubType::FusionRifle)
                .add_string_choice("Sniper Rifle", DestinyItemSubType::SniperRifle)
                .add_string_choice("Pulse Rifle", DestinyItemSubType::PulseRifle)
                .add_string_choice("Scout Rifle", DestinyItemSubType::ScoutRifle)
                .add_string_choice("Sidearm", DestinyItemSubType::Sidearm)
                .add_string_choice("Sword", DestinyItemSubType::Sword)
                .add_string_choice("Linear Fusion Rifle", DestinyItemSubType::FusionRifleLine)
                .add_string_choice("Grenade Launcher", DestinyItemSubType::GrenadeLauncher)
                .add_string_choice("Submachine Gun", DestinyItemSubType::SubmachineGun)
                .add_string_choice("Trace Rifle", DestinyItemSubType::TraceRifle)
                .add_string_choice("Bow", DestinyItemSubType::Bow)
                .add_string_choice("Glaive", DestinyItemSubType::Glaive)
                .required(false)
                .set_autocomplete(true)
        })
        .create_option(|option| {
            option
                .name("Energy Type")
                .kind(CommandOptionType::String)
                .add_string_choice("Kinetic", DamageType::Kinetic)
                .add_string_choice("Strand", DamageType::Strand)
                .add_string_choice("Stasis", DamageType::Stasis)
                .add_string_choice("Arc", DamageType::Arc)
                .add_string_choice("Solar", DamageType::Thermal)
                .add_string_choice("Void", DamageType::Void)
                .required(false).set_autocomplete(true)
        })
}
