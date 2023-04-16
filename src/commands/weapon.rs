use d2statfind::WeaponSlot;
use rustgie::types::destiny::{DamageType, DestinyItemSubType, DestinyItemType, TierType};
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
                .kind(CommandOptionType::Integer)
                .add_int_choice("Auto Rifle", DestinyItemSubType::AutoRifle as i32)
                .add_int_choice("Shotgun", DestinyItemSubType::Shotgun as i32)
                .add_int_choice("Machine Gun", DestinyItemSubType::Machinegun as i32)
                .add_int_choice("Hand Cannon", DestinyItemSubType::HandCannon as i32)
                .add_int_choice("Rocket Launcher", DestinyItemSubType::HandCannon as i32)
                .add_int_choice("Fusion Rifle", DestinyItemSubType::FusionRifle as i32)
                .add_int_choice("Sniper Rifle", DestinyItemSubType::SniperRifle as i32)
                .add_int_choice("Pulse Rifle", DestinyItemSubType::PulseRifle as i32)
                .add_int_choice("Scout Rifle", DestinyItemSubType::ScoutRifle as i32)
                .add_int_choice("Sidearm", DestinyItemSubType::Sidearm as i32)
                .add_int_choice("Sword", DestinyItemSubType::Sword as i32)
                .add_int_choice(
                    "Linear Fusion Rifle",
                    DestinyItemSubType::FusionRifleLine as i32,
                )
                .add_int_choice(
                    "Grenade Launcher",
                    DestinyItemSubType::GrenadeLauncher as i32,
                )
                .add_int_choice("Submachine Gun", DestinyItemSubType::SubmachineGun as i32)
                .add_int_choice("Trace Rifle", DestinyItemSubType::TraceRifle as i32)
                .add_int_choice("Bow", DestinyItemSubType::Bow as i32)
                .add_int_choice("Glaive", DestinyItemSubType::Glaive as i32)
                .required(false)
        })
        .create_option(|option| {
            option
                .name("Energy Type")
                .kind(CommandOptionType::Integer)
                .add_int_choice("Kinetic", DamageType::Kinetic as i32)
                .add_int_choice("Strand", DamageType::Strand as i32)
                .add_int_choice("Stasis", DamageType::Stasis as i32)
                .add_int_choice("Arc", DamageType::Arc as i32)
                .add_int_choice("Solar", DamageType::Thermal as i32)
                .add_int_choice("Void", DamageType::Void as i32)
                .required(false)
        })
        .create_option(|option| {
            option
                .name("Slot")
                .kind(CommandOptionType::Integer)
                .add_int_choice("Top", WeaponSlot::Top as i32)
                .add_int_choice("Middle", WeaponSlot::Middle as i32)
                .add_int_choice("Bottom", WeaponSlot::Bottom as i32)
        })
        .create_option(|option| {
            option
                .name("Adept")
                .kind(CommandOptionType::Boolean)
                .required(false)
        })
        .create_option(|option| {
            option
                .name("Craftable")
                .kind(CommandOptionType::Boolean)
                .required(false)
        })
        .create_option(|option| {
            option
                .name("Rarity")
                .kind(CommandOptionType::Integer)
                .add_int_choice("Legendary", TierType::Superior as i32)
                .add_int_choice("Exotic", TierType::Exotic as i32)
                .add_int_choice("Rare", TierType::Rare as i32)
                .add_int_choice("Uncommon", TierType::Common as i32)
                .add_int_choice("Common", TierType::Basic as i32)
                .required(false)
        })
}
