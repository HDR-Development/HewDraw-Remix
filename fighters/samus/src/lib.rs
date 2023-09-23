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
    util::*,
    ext::*,
    consts::*,
};
use smashline::*;

pub fn install(is_runtime: bool) {
    acmd::install();
    status::install();
    opff::install(is_runtime);
}


pub unsafe fn suit_effect(boma: *mut BattleObjectModuleAccessor, battle_object: *mut BattleObject) {
    let is_ice = VarModule::is_flag(battle_object, vars::samus::instance::ICE_MODE);
    if is_ice {
        let motion_kind_partial = MotionModule::motion_kind_partial(boma, *FIGHTER_SAMUS_MOTION_PART_SET_KIND_VISOR);
        if motion_kind_partial == hash40("invalid") {
            MotionModule::add_motion_partial(boma, *FIGHTER_SAMUS_MOTION_PART_SET_KIND_VISOR, Hash40::new("visor"), 10.0,0.0, false, false, 0.0, true, true, false); 
        }
        if ArticleModule::is_exist(boma, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN) {
            LinkModule::send_event_nodes(boma, *LINK_NO_ARTICLE, Hash40::new_raw(0x1c5609e30f), 0);
        }
    }
    else{
        MotionModule::remove_motion_partial(boma, *FIGHTER_SAMUS_MOTION_PART_SET_KIND_VISOR, false);
    }
}

pub unsafe fn is_ice(fighter: &mut L2CFighterCommon) -> bool {
    return VarModule::is_flag(fighter.battle_object, vars::samus::instance::ICE_MODE);
}