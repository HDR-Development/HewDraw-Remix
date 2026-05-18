// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn side_special_freefall(fighter: &mut L2CFighterCommon) {
    if fighter.is_status_one_of(&[*FIGHTER_PACMAN_STATUS_KIND_SPECIAL_S_DASH, *FIGHTER_PACMAN_STATUS_KIND_SPECIAL_S_DASH]) {
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            VarModule::on_flag(fighter.battle_object, vars::pacman::status::SPECIAL_S_HIT);
        }
    }
    if fighter.is_status(*FIGHTER_PACMAN_STATUS_KIND_SPECIAL_S_RETURN) {
        if fighter.is_prev_status(*FIGHTER_PACMAN_STATUS_KIND_SPECIAL_S_DASH)
        && fighter.is_situation(*SITUATION_KIND_AIR)
        && CancelModule::is_enable_cancel(fighter.module_accessor)
        && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PACMAN_STATUS_SPECIAL_S_WORK_FLAG_EAT_POWER_ESA)
        && !VarModule::is_flag(fighter.battle_object, vars::pacman::instance::SPECIAL_S_GROUND_START) {
            fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
            let cancel_module = *(fighter.module_accessor as *mut BattleObjectModuleAccessor as *mut u64).add(0x128 / 8) as *const u64;
            *(((cancel_module as u64) + 0x1c) as *mut bool) = false;  // CancelModule::is_enable_cancel = false
        }

        if !StatusModule::is_changing(fighter.module_accessor)
        && fighter.is_prev_situation(*SITUATION_KIND_AIR)
        && fighter.is_situation(*SITUATION_KIND_GROUND) {
            if VarModule::is_flag(fighter.battle_object, vars::pacman::instance::SPECIAL_S_GROUND_START) {
                // prevent special landing from grounded version before transitioning into normal fall
                if fighter.status_frame() < 30 { return; }
            }
            else {
                // land cancel air version
                if VarModule::is_flag(fighter.battle_object, vars::pacman::status::SPECIAL_S_HIT) {
                    fighter.check_land_cancel(Some(10.0));
                    return;
                }
            }
            
            fighter.change_status_req(*FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, true);
        } 
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status(*FIGHTER_PACMAN_STATUS_KIND_SPECIAL_S_REFLECT_FALL) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

pub unsafe fn moveset(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    side_special_freefall(fighter);
    fastfall_specials(fighter);
}

pub extern "C" fn pacman_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		pacman_frame(fighter)
    }
}

pub unsafe fn pacman_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, pacman_frame_wrapper);
}