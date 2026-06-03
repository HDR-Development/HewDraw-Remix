#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod acmd;

pub mod opff;
pub mod status;

pub mod bowarrow;

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

hitbox_templates!(
    pub PITB_SWORD_HITBOX = {
        extends: BASE_HITBOX,
        effect: "collision_attr_cutup",
        hit_sound: CollisionSound::Cutup,
        region: AttackRegion::Palutena,
    };
    pub PITB_SWORD_HITBOX_S = {
        extends: PITB_SWORD_HITBOX,
        sound_level: SoundLevel::S,
    };
    pub PITB_SWORD_HITBOX_M = {
        extends: PITB_SWORD_HITBOX,
        sound_level: SoundLevel::M,
    };
    pub PITB_SWORD_HITBOX_L = {
        extends: PITB_SWORD_HITBOX,
        effect: "collision_attr_sting",
        sound_level: SoundLevel::L,
    };
    pub PITB_KICK_HITBOX = {
        extends: BASE_HITBOX,
        effect: "collision_attr_normal",
        hit_sound: CollisionSound::Kick,
        region: AttackRegion::Kick,
    };
    pub PITB_KICK_HITBOX_S = {
        extends: PITB_KICK_HITBOX,
        sound_level: SoundLevel::S,
    };
    pub PITB_KICK_HITBOX_M = {
        extends: PITB_KICK_HITBOX,
        sound_level: SoundLevel::M,
    };
    pub PITB_KICK_HITBOX_L = {
        extends: PITB_KICK_HITBOX,
        sound_level: SoundLevel::L,
    };
);

pub fn install() {
    let agent = &mut Agent::new("pitb");
    acmd::install(agent);
    opff::install(agent);
    status::install(agent);
    agent.install();

    bowarrow::install();
}
