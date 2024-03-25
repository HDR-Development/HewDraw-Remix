#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

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
pub mod acmd;
pub mod status;
pub mod opff;
pub mod vtable_hook;
pub use status::krool_belly_damage_hook_impl;

// articles
mod backpack;
mod ironball;

pub fn install() {
    let agent = &mut Agent::new("krool");
    acmd::install(agent);
    status::install(agent);
    opff::install(agent);
    agent.install();

    backpack::install();
    ironball::install();

    // prevents shield break on belly
    use opff::*;
    skyline::patching::Patch::in_text(0xc04f00).data(0x1400001Eu32);
}