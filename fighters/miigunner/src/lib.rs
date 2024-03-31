#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod acmd;

pub mod opff;
pub mod status;

// articles

mod attackairf_bullet;
mod bottomshoot;
mod fullthrottle;
mod grenadelauncher;
mod gunnercharge;
mod rapidshot_bullet;
mod stealthbomb;
mod stealthbomb_s;
mod supermissile;

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

pub fn install() {
    let agent = &mut Agent::new("miigunner");
    acmd::install(agent);
    opff::install(agent);
    status::install(agent);
    agent.install();

    attackairf_bullet::install();
    bottomshoot::install();
    fullthrottle::install();
    grenadelauncher::install();
    gunnercharge::install();
    rapidshot_bullet::install();
    stealthbomb::install();
    stealthbomb_s::install();
    supermissile::install();
}