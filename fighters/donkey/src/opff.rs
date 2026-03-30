// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn dash_attack_air_cancel(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status(*FIGHTER_STATUS_KIND_ATTACK_DASH)
    && boma.is_situation(*SITUATION_KIND_AIR)
    && MotionModule::frame(boma) >= 26.0 {
        boma.check_jump_cancel(false, false);
    }
}

extern "Rust" {
    fn gimmick_flash(boma: &mut BattleObjectModuleAccessor);
}

unsafe fn barrel_pull(boma: &mut BattleObjectModuleAccessor) {
    // barrel pull
    if boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_STATUS_KIND_GUARD_OFF, *FIGHTER_STATUS_KIND_WAIT])
    && ItemModule::is_have_item(boma, 0) && ItemModule::get_have_item_kind(boma, 0) == *ITEM_KIND_BARREL {
        boma.change_status_req(*FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP, false);
    }
    if boma.is_status(*FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP) {
        if boma.is_situation(*SITUATION_KIND_AIR) {
            KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_FALL {
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
            }
        }
        else {
            // manual grounded grab
            if boma.status_frame() == 6
            && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CATCH) {
                VarModule::on_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK);
                boma.change_status_req(*FIGHTER_STATUS_KIND_SPECIAL_LW, false);
            }
        }
    }
}

// DK Headbutt aerial stall
unsafe fn headbutt_aerial_stall(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status(*FIGHTER_STATUS_KIND_SPECIAL_S) {
        if boma.is_situation(*SITUATION_KIND_AIR)
        && !VarModule::is_flag(boma.object(), vars::common::instance::SPECIAL_STALL_USED) {
            if boma.status_frame() < 26 {
                if KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) < 0.0 {
                    VarModule::on_flag(boma.object(), vars::common::instance::SPECIAL_STALL);
                    KineticModule::mul_speed(boma, &Vector3f::new(1.0, 0.0, 1.0), *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                }
            }
        }
    }
    if VarModule::is_flag(boma.object(), vars::common::instance::SPECIAL_STALL)
    && (!boma.is_status(*FIGHTER_STATUS_KIND_SPECIAL_S) || (boma.is_status(*FIGHTER_STATUS_KIND_SPECIAL_S) && boma.status_frame() >= 26)) {
        VarModule::on_flag(boma.object(), vars::common::instance::SPECIAL_STALL_USED);
        VarModule::off_flag(boma.object(), vars::common::instance::SPECIAL_STALL);
    }
    if VarModule::is_flag(boma.object(), vars::common::instance::SPECIAL_STALL_USED)
    && boma.is_situation(*SITUATION_KIND_GROUND) {
        VarModule::off_flag(boma.object(), vars::common::instance::SPECIAL_STALL_USED);
    }
}

// prevent donkey kong from carrying/throwing steve's blocks
pub unsafe fn remove_pickelobject(fighter: &mut L2CFighterCommon) {
    if ItemModule::get_have_item_kind(fighter.boma(), 0) == *ITEM_KIND_PICKELOBJECT {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 0, 10, 15, 0, 0, 0, 1, false);
        LAST_EFFECT_SET_COLOR(fighter, 0.6, 0.6, 0.6);
        ItemModule::remove_item(fighter.boma(), 0);
        MotionModule::set_rate(fighter.boma(), 0.1);
        PLAY_SE(fighter, Hash40::new("se_common_famicom_hit"));
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    dash_attack_air_cancel(boma);
    barrel_pull(boma);
    headbutt_aerial_stall(boma);
    remove_pickelobject(fighter);
    fastfall_specials(fighter);
}

pub extern "C" fn donkey_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		donkey_frame(fighter);
    }
}

pub unsafe fn donkey_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, donkey_frame_wrapper);
}