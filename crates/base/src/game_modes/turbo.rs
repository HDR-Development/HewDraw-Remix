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

    //println!("doing turbo update!");
    for object_id in util::get_all_active_battle_object_ids() {
        let object = util::get_battle_object_from_id(object_id);
        if !object.is_null() {
            let fighter = util::get_fighter_common_from_accessor(&mut *(*object).module_accessor);
            handle_turbo(fighter);
        }
    }
}

unsafe fn handle_turbo(fighter: &mut L2CFighterCommon) {
    //println!("doing turbo logic");
    if AttackModule::is_infliction_status(fighter.boma(), *COLLISION_KIND_MASK_HIT) {
        // enable turbo behavior
        CancelModule::enable_cancel(fighter.boma());
        //println!("enabled cancelling!");

        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            fighter.sub_wait_ground_check_common(false.into());
        } else {
            fighter.sub_air_check_fall_common();
        }
    }
}