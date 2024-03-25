#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod acmd;

pub mod opff;
pub mod status;

// articles
mod fireball;
mod pump;
mod pumpwater;

use smash::{
    app::{
        self,
        lua_bind::*,
        sv_animcmd::{frame, wait},
        *,
    },
    hash40,
    lib::lua_const::*,
    lib::{L2CValue, LuaConst},
    lua2cpp::*,
    phx::*,
};
use smash_script::{macros::*, *};
use smashline::*;
use utils::{consts::*, ext::*, util::*, *};

pub fn install() {
    let agent = &mut Agent::new("mario");
    acmd::install(agent);
    status::install(agent);
    opff::install(agent);
    agent.install();

    fireball::install();
    pump::install();
    pumpwater::install();
}
