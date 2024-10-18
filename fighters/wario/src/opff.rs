// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn bite_early_throw_turnaround(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status(*FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_BITE)
    && !StatusModule::is_changing(boma) {
        if boma.is_pad_flag(PadFlag::SpecialTrigger) {
            boma.change_status_req(*FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_BITE_END, false);
        }
    }
    if boma.is_status(*FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_BITE_END) {
        if boma.status_frame() < 7 {
            if PostureModule::lr(boma) * boma.stick_x() < 0.0 {
                PostureModule::reverse_lr(boma);
                PostureModule::update_rot_y_lr(boma);
            }
        }
    }
}

// Wario Bthrow Movement
unsafe fn bthrow_movement(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status(*FIGHTER_STATUS_KIND_THROW) {
        if boma.is_motion(Hash40::new("throw_b")) {
            if boma.is_situation(*SITUATION_KIND_GROUND) {
                let currentFrame = MotionModule::frame(boma);
                let maxFrame = 46.0;
                if boma.stick_x() != 0.0 
                && currentFrame < maxFrame {
                    let motion_vec = x_motion_vec(1.0, boma.stick_x());
                    KineticModule::add_speed_outside(boma, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
                }
            }
        }
    }
}

unsafe fn dash_attack_air_cancel(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status(*FIGHTER_STATUS_KIND_ATTACK_DASH)
    && boma.is_situation(*SITUATION_KIND_AIR)
    && MotionModule::frame(boma) >= 30.0 {
        CancelModule::enable_cancel(boma);
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_END,
        *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_OPEN_WAIT,
        *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_BITE_START,
        *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_BITE_END,
        *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_EAT,
        *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_BITE,
        *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_BOMB,
        *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_FOOD,
        *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_ITEM,
        *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_LARGE,
        *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_BUMP,
        *FIGHTER_WARIO_STATUS_KIND_SPECIAL_HI_JUMP
        ])
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
        if fighter.is_flag(*FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
            if [*FIGHTER_KINETIC_TYPE_MOTION_AIR, *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE].contains(&KineticModule::get_kinetic_type(fighter.module_accessor)) {
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
                let speed_y = app::sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);

                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y, 0.0, 0.0, 0.0);
                app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
                
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                app::sv_kinetic_energy::enable(fighter.lua_state_agent);

                KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
            }
        }
    }
}

pub unsafe fn moveset(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    bite_early_throw_turnaround(boma);
    bthrow_movement(boma);
    dash_attack_air_cancel(boma);
    fastfall_specials(fighter);
}

unsafe extern "C" fn garlic_meter(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        if !sv_information::is_ready_go() && fighter.status_frame() < 1 {
            return;
        }

        utils::ui::UiManager::set_garlic_meter_enable(fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32, true);
        utils::ui::UiManager::set_garlic_meter_info(
            fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32,
            (fighter.get_int(0x100000bf) as f32) / 60.0, //FIGHTER_WARIO_INSTANCE_WORK_ID_INT_GASS_COUNT
            fighter.get_param_float("param_special_lw", "gass_middle_time"),
            fighter.get_param_float("param_special_lw", "gass_large_time"),
            fighter.get_param_float("param_special_lw", "gass_max_time"),
        );
    }
}

pub extern "C" fn wario_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		wario_frame(fighter)
    }
}

pub unsafe fn wario_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, wario_frame_wrapper);
    agent.on_line(Main, garlic_meter);
}
