#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod acmd;

pub mod opff;
pub mod status;

// articles
mod fireball;
mod obakyumu;

use smash::{
    lib::{
        L2CValue,
        LuaConst,
    },
    app::{
        *,
        self,
        sv_animcmd::{
            frame,
            wait
        },
        lua_bind::*
    },
    hash40,
    lib::{
        lua_const::*
    },
    lua2cpp::*,
    phx::*
};
use smash_script::{
    *,
    macros::*
};
use smash_script::macros::ATTACK_ABS;
use utils::{
    *,
    util::*,
    ext::*,
    consts::*,
};
use smashline::*;
#[macro_use] extern crate smash_script;

pub unsafe fn reset_misfire_queue(fighter: &mut L2CFighterCommon) {
    let numerator = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "misfire.numerator");
    let denominator = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "misfire.denominator").min(32);

    // this bitflag represents the misfire queue
    // each 0 is a standard missile, each 1 is a misfire
    // the numerator above is the maximum number of misfires in the queue
    // the denominator above is the maximum length of the queue (capped to 32, because we are using an i32)
    let mut bitflag = 0b00000000000000000000000000000000;
    let mut num_misfire = 0;
    while (num_misfire < numerator) {
        // generate a random position between [0, denominator - 1] (inclusive)
        let position = app::sv_math::rand(hash40("fighter"), denominator);
        // if that bit is already set, skip it (prevents duplicates)
        if (bitflag & (1 << position)) != 0 {
            continue;
        }
        bitflag |= (1 << position); // set the bit at this position to 1
        num_misfire += 1;
    }
    // the result is a bit flag that looks something like: 0b00000000000000000001000000100000
    // in which case a misfire will happen at position 12 and 5
    //println!("Misfire bitflag on init: {:0denominator$b}", bitflag, denominator = (denominator as usize));
    VarModule::set_int(fighter.battle_object, vars::luigi::instance::SPECIAL_S_MISFIRE_BITFLAG, bitflag);
    VarModule::set_int(fighter.battle_object, vars::luigi::instance::SPECIAL_S_MISFIRE_COUNT, denominator - 1); // counts down from denominator (higher positions first)
}

pub unsafe fn calculate_misfire(fighter: &mut L2CFighterCommon) -> bool {
    // reset the queue and counter if the queue has not been initialized or the counter has dipped below zero
    if VarModule::get_int(fighter.battle_object, vars::luigi::instance::SPECIAL_S_MISFIRE_BITFLAG) == 0
    || VarModule::get_int(fighter.battle_object, vars::luigi::instance::SPECIAL_S_MISFIRE_COUNT) < 0 {
        reset_misfire_queue(fighter);
    }

    let bitflag = VarModule::get_int(fighter.battle_object, vars::luigi::instance::SPECIAL_S_MISFIRE_BITFLAG);
    let position = VarModule::get_int(fighter.battle_object, vars::luigi::instance::SPECIAL_S_MISFIRE_COUNT);
    VarModule::dec_int(fighter.battle_object, vars::luigi::instance::SPECIAL_S_MISFIRE_COUNT);
    //println!("Misfire queue: {:0position$b}", bitflag, position = (position as usize));
    let is_misfire = (bitflag & (1 << position)) != 0; // if the bit at this position is 1, we misfire
    return is_misfire; 
}

pub fn install() {
    let agent = &mut Agent::new("luigi");
    acmd::install(agent);
    opff::install(agent);
    status::install(agent);
    agent.install();

    fireball::install();
    obakyumu::install();
}