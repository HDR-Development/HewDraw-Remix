use smash::app::{self, BattleObject, BattleObjectModuleAccessor, lua_bind::*};
use smash::lib::lua_const::*;
use smash::phx::{Hash40, Vector4f};
use smash::hash40;
use smash::lua2cpp::L2CFighterCommon;
use crate::modules::*;
use utils_dyn::consts::*;
use utils_dyn::ext::*;
use smash::phx::Vector3f;
use crate::util;

pub unsafe fn update() {
    // skip this frame because the match hasnt started
    if !app::sv_information::is_ready_go() {
        return;
    }

    //println!("doing hitfall update!");
    for object_id in util::get_all_player_battle_object_ids() {
        let object = util::get_battle_object_from_id(object_id);
        if !object.is_null() {
            let fighter = util::get_fighter_common_from_accessor(&mut *(*object).module_accessor);
            fighter.check_hitfall();
        }
    }
}