#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod acmd;

pub mod opff;
pub mod status;

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
use utils::{
    *,
    util::*,
    ext::*,
    consts::*,
};
use smashline::*;
#[macro_use] extern crate smash_script;

pub const KOOPA_MAX_COOLDOWN : i32 = 900;
pub const LUCAS_CHARGE_TIME : i32 = 120;
static mut BAYONET_EGGS:[i32;8] = [0; 8]; //I have no idea why varmod doesn't work with this, so this will have to do

pub fn install() {
    let agent = &mut Agent::new("kirby");
    acmd::install(agent);
    opff::install(agent);
    status::install(agent);
    agent.install();

    let whitelist_articles = [
        (*FIGHTER_KIND_PALUTENA, *WEAPON_KIND_PALUTENA_EXPLOSIVEFLAME),
        (*FIGHTER_KIND_PALUTENA, *WEAPON_KIND_PALUTENA_EXPLOSIVEFLAME_RESERVE)
    ];
    for (fighter_id, article_id) in whitelist_articles.iter() {
        smashline::whitelist_kirby_copy_article(*fighter_id, *article_id);
    }
}
