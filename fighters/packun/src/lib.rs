#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod acmd;

pub mod opff;
pub mod status;

// articles

mod spikeball;
mod poisonbreath;
mod firebreath;

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
    lib::lua_const::*,
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

pub unsafe fn SET_STANCE(fighter: &mut L2CFighterCommon, stance: i32, follow: bool) {
    if !(VarModule::get_int(fighter.battle_object, vars::packun::instance::CURRENT_STANCE) == stance) {
        VarModule::set_int(fighter.battle_object, vars::packun::instance::CURRENT_STANCE, stance);
        //VarModule::on_flag(fighter.battle_object, vars::packun::status::STANCE_INIT);

        if follow {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_level_up"), Hash40::new("top"), -2, 10, 0, 0, 0, 0, 0.4, true);
        }
        else {
            EFFECT(fighter, Hash40::new("sys_level_up"), Hash40::new("top"), -2, 10, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, true);
        }
        PLAY_SE(fighter, Hash40::new("se_packun_special_s02"));

        // Enable meshes for stances
        // HeadA is the normal head
        // HeadB is the poison head
        // HeadS is the spike head
        match stance {
            0 => {  // STANCE_PIRANHA
                EFFECT_FOLLOW(fighter, Hash40::new("sys_grass_landing"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, false);
                ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("heada"), true);
                ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("headb"), false);
                ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("heads"), false);
            },
            1 => {  // STANCE_PUTRID
                EFFECT_FOLLOW(fighter, Hash40::new("packun_poison_max"), Hash40::new("top"), 0, 15.5, 0, 0, 0, 0, 1.2, false);
                ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("headb"), true);
                ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("heada"), false);
                ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("heads"), false);
            },
            2 => {  // STANCE_PRICKLY
                EFFECT_FOLLOW(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, false);
                ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("heads"), true);
                ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("headb"), false);
                ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("heada"), false);
            },
            _ => {}
        }
    }
}

pub fn install() {
    let agent = &mut Agent::new("packun");
    acmd::install(agent);
    opff::install(agent);
    status::install(agent);
    agent.install();

    spikeball::install();
    poisonbreath::install();
    firebreath::install();

    smashline::clone_weapon("mario", *WEAPON_KIND_MARIO_FIREBALL, "packun", "firebreath", false);
}