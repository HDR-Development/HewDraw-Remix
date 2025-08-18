// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;
 
unsafe fn nspecial_cancels(boma: &mut BattleObjectModuleAccessor) {
    //PM-like neutral-b canceling
    if boma.is_status(*FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_CANCEL) {
        if boma.is_situation(*SITUATION_KIND_AIR) {
            if WorkModule::get_int(boma, *FIGHTER_REFLET_STATUS_SPECIAL_N_HOLD_INT_NEXT_STATUS) == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
                WorkModule::set_int(boma, *STATUS_KIND_NONE, *FIGHTER_REFLET_STATUS_SPECIAL_N_HOLD_INT_NEXT_STATUS);
                ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
            }
        }
    }
    // Allow charge jump cancel even when left stick is down
    if boma.is_status(*FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_HOLD) {
        if boma.is_input_jump() && boma.get_num_used_jumps() < boma.get_jump_count_max() {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_JUMP_CANCEL, false);
            WorkModule::set_int(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, *FIGHTER_REFLET_STATUS_SPECIAL_N_HOLD_INT_NEXT_STATUS);
        }
    }
}

// move to hi_main
unsafe fn elwind_cost(fighter: &mut L2CFighterCommon) {
    //cost of each elwind 1 -> 2
    //if fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_HI_2]) 
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI)
    && StatusModule::is_changing(fighter.module_accessor) {
        fighter.dec_int(*FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT);
        if fighter.get_int(*FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT) <= 0 {
            FighterSpecializer_Reflet::set_flag_to_table(fighter.module_accessor as *mut app::FighterModuleAccessor, *FIGHTER_REFLET_MAGIC_KIND_EL_WIND, true, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THROWAWAY_TABLE);
        }
    }
}

unsafe fn levin_leniency(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    // move to attack_air
    if boma.is_motion_one_of(&[
        Hash40::new("attack_air_n"),
        Hash40::new("attack_air_f"),
        Hash40::new("attack_air_b"),
        Hash40::new("attack_air_hi"),
        Hash40::new("attack_air_lw"),
    ])
    && VarModule::get_int(fighter.battle_object, vars::reflet::instance::ATTACK_AIR_LEVIN_LENIENCY) > 0 {
        VarModule::dec_int(fighter.battle_object, vars::reflet::instance::ATTACK_AIR_LEVIN_LENIENCY);
        if !fighter.is_flag(*FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) 
        && boma.is_button_on(Buttons::Smash | Buttons::SpecialRaw | Buttons::Guard) {
            let levin = *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT;
            if WorkModule::get_int(boma, levin) > 0 {
                if WorkModule::get_int(boma, levin) == 1 {
                    app::FighterSpecializer_Reflet::set_flag_to_table(fighter.module_accessor as *mut app::FighterModuleAccessor, *FIGHTER_REFLET_MAGIC_KIND_SWORD, true, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THROWAWAY_TABLE);
                }
                fighter.on_flag(*FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON);
                VisibilityModule::set_int64(boma, Hash40::new("sword").hash as i64, Hash40::new("sword_thunder").hash as i64);
                WorkModule::dec_int(boma, levin);
            }
        }
    }
}

// Lengthen sword
unsafe fn sword_length(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status(*FIGHTER_STATUS_KIND_ATTACK_HI4)
    && WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
        if boma.status_frame() <= 14 {
            ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("sword"), &Vector3f::new(1.0, 1.075, 1.0475));
        }
    }
    else {
        let long_sword_scale = Vector3f{x: 1.0, y: 1.175, z: 1.0475};
        ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("haver"), &long_sword_scale);
    }
}

// upB freefalls after one use per airtime
unsafe fn up_special_freefall(fighter: &mut L2CFighterCommon) {
    // move to hi_end
    if fighter.is_prev_status(*FIGHTER_STATUS_KIND_SPECIAL_HI) {
        if StatusModule::is_changing(fighter.module_accessor) {
            VarModule::on_flag(fighter.battle_object, vars::reflet::instance::SPECIAL_HI_ENABLE_FREEFALL);
        }
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_HOLD,
        *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_SHOOT,
        *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_CANCEL,
        *FIGHTER_REFLET_STATUS_KIND_SPECIAL_LW_END
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    nspecial_cancels(boma);
    elwind_cost(fighter);
    levin_leniency(fighter, boma);
    sword_length(boma);
    up_special_freefall(fighter);
    fastfall_specials(fighter);
}

pub extern "C" fn reflet_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
        reflet_frame(fighter)
    }
}

pub unsafe fn reflet_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, reflet_frame_wrapper);
}
