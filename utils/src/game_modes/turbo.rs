use smash::app::{self, BattleObject, BattleObjectModuleAccessor, lua_bind::*};
use smash::lib::lua_const::*;
use smash::phx::{Hash40, Vector4f};
use smash::hash40;
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
    for i in 0..8 {
        if let Some(object_id) = util::get_active_battle_object_id_from_entry_id(i) {
            let object = util::get_battle_object_from_id(object_id);
            if !object.is_null() {
                //println!("found object with id: {}", i);
                let boma = &mut *(*object).module_accessor;
                if boma.is_fighter() {
                    handle_turbo(boma);
                }
            }
        }

    }
}

unsafe fn handle_turbo(boma: &mut BattleObjectModuleAccessor) {
    //println!("doing turbo logic");
    if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
        // enable turbo behavior
        CancelModule::enable_cancel(boma);
        //println!("enabled cancelling!");
    }
}