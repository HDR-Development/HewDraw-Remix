#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod acmd;

pub mod status;
pub mod opff;

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
    consts::*,
    ext::*
};
use smashline::*;

pub const FIGHTER_CLOUD_GENERATE_ARTICLE_METEOR: i32 = 2;
pub const WEAPON_CLOUD_METEOR_INSTANCE_WORK_ID_INT_ANGLE: i32 = 0x10000009;
pub const WEAPON_CLOUD_METEOR_INSTANCE_WORK_ID_INT_NUM: i32 = 0x1000000b;
pub const METEOR_THRU_PLAT: bool = false;
pub const METEOR_SPAWN: f32 = 50.0;
pub const METEOR_AMOUNT: i32 = 4;
pub const METEOR_LIFE: i32 = 40;
pub const METEOR_ANGLE_MIN: i32 = -60;
pub const METEOR_ANGLE_MAX: i32 = -60;
pub const METEOR_OFFSET_X: [f32; 4] = [0.0,5.0,5.0,-5.0];
pub const METEOR_OFFSET_Y: [f32; 4] = [0.0,5.0,-5.0,-5.0];

pub fn install() {
    acmd::install();
    opff::install();
    status::install();

    smashline::clone_weapon("sheik", "needle", "cloud","meteor",false);
}