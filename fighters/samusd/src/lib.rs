#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod acmd;
pub mod status;
pub mod opff;

mod bomb;
mod cshot;
mod missile;
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

pub fn install() {
    let agent = &mut Agent::new("samusd");
    acmd::install(agent);
    status::install(agent);
    opff::install(agent);
    agent.install();

    bomb::install();
    cshot::install();
    missile::install();
    supermissile::install();
}
