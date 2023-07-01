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

    // println!("doing airdash update!");
    for object_id in util::get_all_active_battle_object_ids() {
        let object = util::get_battle_object_from_id(object_id);
        if !object.is_null() {
            let fighter = util::get_fighter_common_from_accessor(&mut *(*object).module_accessor);
            check_airdash(fighter);
        }
    }
}

unsafe fn check_airdash(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_ESCAPE_AIR) {
        if fighter.status_frame() < 1 {
            let speed_x = KineticModule::get_sum_speed_x(fighter.boma(), *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            let speed_y = KineticModule::get_sum_speed_y(fighter.boma(), *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            // lets make sure not to divide by zero
            let speed_x_adjust = match speed_x {
                0.0 => 0.01,
                _ => 0.0
            };
            let angle = (speed_y/(speed_x + speed_x_adjust)).atan();

            let pos = Vector3f { x: 0., y: 3., z: 0.};
            let mut rot = Vector3f { x:0., y:0., z: (90. + 180. * angle/3.14159)};

            if speed_x > 0. {
                EffectModule::req_on_joint(fighter.boma(), Hash40::new("sys_whirlwind_r"), Hash40::new("top"),
                &pos, &rot, 0.75, &Vector3f{x:0.0, y:0.0, z:0.0}, &Vector3f{x:0.0, y:0.0, z:0.0}, false, 0, 0, 0);
            }else{
                rot = Vector3f { x:0., y:0., z: (-90. + 180. * angle/3.14159)};
                EffectModule::req_on_joint(fighter.boma(), Hash40::new("sys_whirlwind_l"), Hash40::new("top"),
                &pos, &rot, 0.75, &Vector3f{x:0.0, y:0.0, z:0.0}, &Vector3f{x:0.0, y:0.0, z:0.0}, false, 0, 0, 0);
            }
        }

        CancelModule::enable_cancel(fighter.boma());
        if fighter.is_situation(*SITUATION_KIND_AIR) {
            fighter.sub_air_check_fall_common();
        }
    }
}