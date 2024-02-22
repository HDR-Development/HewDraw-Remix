// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn bite_early_throw_turnaround(boma: &mut BattleObjectModuleAccessor, status_kind: i32, stick_x: f32, facing: f32, frame: f32) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if status_kind == *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_BITE {
        if compare_mask(ControlModule::get_pad_flag(boma), *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER) {
            boma.change_status_req(*FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_BITE_END, false);
        }
    }
    if status_kind == *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_BITE_END {
        if frame < 7.0 {
            if facing * stick_x < 0.0 {
                PostureModule::reverse_lr(boma);
                PostureModule::update_rot_y_lr(boma);
            }
        }
    }
}

// Wario Bthrow Movement
unsafe fn bthrow_movement(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_THROW {
        if motion_kind == hash40("throw_b") {
            if situation_kind == *SITUATION_KIND_GROUND {
                let currentFrame = MotionModule::frame(boma);
                let maxFrame = 46.0;
                if stick_x != 0.0 
                && currentFrame < maxFrame {
                    let motion_vec = x_motion_vec(1.0, stick_x);
                    KineticModule::add_speed_outside(boma, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
                }
            }
        }
    }
}

unsafe fn dash_attack_air_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH
    && situation_kind == *SITUATION_KIND_AIR
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

    bite_early_throw_turnaround(boma, status_kind, stick_x, facing, frame);
    bthrow_movement(boma, status_kind, situation_kind, motion_kind, stick_x);
    dash_attack_air_cancel(boma, status_kind, situation_kind);
    fastfall_specials(fighter);
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
pub fn install() {
    smashline::Agent::new("wario")
        .on_line(Main, wario_frame_wrapper)
        .install();
}
