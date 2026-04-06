#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod acmd;

pub mod opff;
pub mod status;

mod water;

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

pub trait PokeExt {
    unsafe fn play_pledge_effect(&mut self, state: i32);
}
impl PokeExt for app::BattleObjectModuleAccessor {
    unsafe fn play_pledge_effect(&mut self, state: i32) {
        match state {
            2 /* GRASS */ => {
                for _ in 0..2 {
                    let grass_fx = EffectModule::req_follow(self, Hash40::new("sys_grass_landing"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
                    EffectModule::set_rgb(self, grass_fx, 0.5, 2.0, 0.5);
                    EffectModule::set_scale(self, grass_fx, &Vector3f::new(1.2, 1.4, 1.2));
                    EffectModule::set_rate(self, grass_fx, 0.6);
                }
            }
            3 /* FIRE */ => {
                let fire_fx = EffectModule::req_follow(self, Hash40::new("sys_damage_fire"), Hash40::new("top"), &Vector3f::new(0.5, 0.0, 0.0), &Vector3f::zero(), 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
                EffectModule::set_rgb(self, fire_fx, 1.0, 0.9, 0.9);
                EffectModule::set_scale(self, fire_fx, &Vector3f::new(1.2, 1.25, 1.2));
                EffectModule::set_rate(self, fire_fx, 0.5);
            }
            _ => println!("Invalid pledge state provided.")
        }
    }
}

pub fn install() {
    let agent = &mut Agent::new("pzenigame");
    acmd::install(agent);
    opff::install(agent);
    status::install(agent);
    agent.install();

    water::install();
}