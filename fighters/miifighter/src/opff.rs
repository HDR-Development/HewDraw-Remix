utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

// Feint Jump Jump Cancel
unsafe fn feint_jump_jc(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_motion_one_of(&[Hash40::new("special_lw2_start"),Hash40::new("special_air_lw2_start")]) {
        if MotionModule::frame(boma) > 31.0 {
            if !boma.is_in_hitlag() {
                boma.check_jump_cancel(false, false, true);
            }
        }
    }
}

// TODO: create cancel animation for aerial EQF cancel
// Prevents aerial EQF cancel from grabbing ledge for first 7f
unsafe fn eqf_cancel_ledgegrab_lockout(fighter: &mut L2CFighterCommon) {
    if fighter.is_prev_status(*FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW1_AIR)
    && fighter.is_status(*FIGHTER_STATUS_KIND_FALL_SPECIAL) {
        if StatusModule::is_changing(fighter.module_accessor) {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
        }
        if fighter.status_frame() == 6 {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP);
        }
    }
}

unsafe fn boiling_punt_timer(fighter: &mut L2CFighterCommon) {
    if VarModule::get_int(fighter.battle_object, vars::miifighter::instance::SPECIAL_LW3_STAGE) > 1 {
        let timer = VarModule::get_int(fighter.battle_object, vars::miifighter::instance::SPECIAL_LW3_TIMER);
        if !VarModule::countdown_int(fighter.battle_object, vars::miifighter::instance::SPECIAL_LW3_TIMER, 0) {
            if timer % 60 == 0 {
                let vec1 = Vector4f{ x: 0.85, y: 0.85, z: 0.85, w: 0.2};
                let vec2 = Vector4f{ x: 0.85, y: 0.0, z: 0.0, w: 0.5};
                ColorBlendModule::set_main_color(fighter.module_accessor, &vec1, &vec2, 1.0, 1.0, 7, true);
            }
            if timer % 60 == 52 {
                ColorBlendModule::cancel_main_color(fighter.module_accessor, 0);
            }
        }
        else {
            VarModule::set_int(fighter.battle_object, vars::miifighter::instance::SPECIAL_LW3_TIMER, 300);
        }
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && (
        fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_N])
        || ([*FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_N_2,
            *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_HI_2
            ].contains(&WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_WAZA_CUSTOMIZE_TO))
            && fighter.is_status_one_of(&[
                *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_N2_MISS,
                *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_N2_FINISH,
                *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_N2_FINISH_MISS,
                *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_HI2_END
            ])
        )
        || ([*FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_S_3,
            *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_HI_3
            ].contains(&WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_WAZA_CUSTOMIZE_TO))
            && fighter.is_status_one_of(&[
                *FIGHTER_STATUS_KIND_SPECIAL_HI,
                *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_S3_THROW,
            ])
        )
    )
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    feint_jump_jc(boma);
    eqf_cancel_ledgegrab_lockout(fighter);
    boiling_punt_timer(fighter);
    fastfall_specials(fighter);
}

pub extern "C" fn miifighter_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
        miifighter_frame(fighter)
    }
}

pub unsafe fn miifighter_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, miifighter_frame_wrapper);
}