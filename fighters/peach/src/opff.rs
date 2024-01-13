use super::*;
use globals::*;
utils::import_noreturn!(common::opff::fighter_common_opff);
 
unsafe fn float_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if status_kind == *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR {
        let prev_status_0 = StatusModule::prev_status_kind(boma, 0);
        let prev_status_1 = StatusModule::prev_status_kind(boma, 1);
        if [*FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT, *FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT_START].contains(&prev_status_0)
            || [*FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT, *FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT_START].contains(&prev_status_1) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
        }
    }
}
unsafe fn wall_bounce(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if status_kind == *FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_JUMP {
        let lr = PostureModule::lr(boma);
        let frame = MotionModule::frame(boma) as i32;
        let mut touch_wall = false;
        if lr > 0.0 {
            touch_wall = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_RIGHT as u32);
        } else {
            touch_wall = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_LEFT as u32);
        };
        if touch_wall && (1..25).contains(&frame){
                VarModule::on_flag(boma.object(), vars::peach::instance::IS_WALLBOUNCE);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_HIT_END, true);
        }
    }else if status_kind == *FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_HIT_END {
        if VarModule::is_flag(boma.object(), vars::peach::instance::IS_WALLBOUNCE) {
            MotionModule::set_rate(boma, 0.6);
        }
    } else {
        VarModule::off_flag(boma.object(), vars::peach::instance::IS_WALLBOUNCE);
    }
}

unsafe fn parasol_ff(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    let direction = ControlModule::get_stick_y(boma);
    if status_kind == *FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_AIR_END && direction <= -0.5 {
        let vec = Vector3f{x: 0.0, y: -1.2, z: 0.0};
        let speed_y = KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);

        if speed_y > -1.5 {
            KineticModule::add_speed(boma, &vec);
        }
    }
}

unsafe fn up_special_freefall_land_cancel(fighter: &mut L2CFighterCommon) {
    if (fighter.is_prev_status(*FIGHTER_STATUS_KIND_FALL_SPECIAL) || fighter.is_prev_status(*FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_AIR_END)) 
    && fighter.is_status(*FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL) {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_LANDING, false);
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_PEACH_STATUS_KIND_SPECIAL_N_HIT,
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

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    float_cancel(boma, status_kind);
    wall_bounce(boma, status_kind);
    up_special_freefall_land_cancel(fighter);
    fastfall_specials(fighter);
    parasol_ff(boma, status_kind);
}

#[::utils::macros::opff(FIGHTER_KIND_PEACH )]
pub fn peach_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		peach_frame(fighter)
    }
}

pub unsafe fn peach_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}