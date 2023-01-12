use super::*;
use globals::*;
use smashline::*;


pub fn install() {
  install_status_scripts!(
     pre_dash,
	 sonic_speciallw_pre,
     special_hi_jump_main
  );
}


#[status_script(agent = "sonic", status = FIGHTER_STATUS_KIND_DASH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn pre_dash(fighter: &mut L2CFighterCommon) -> L2CValue {
	let ground_brake = WorkModule::get_param_float(fighter.module_accessor, hash40("ground_brake"), 0);
	let mut initial_speed = VarModule::get_float(fighter.battle_object, vars::common::instance::CURR_DASH_SPEED);

	if ![*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH].contains(&StatusModule::prev_status_kind(fighter.module_accessor, 0)) {
		// println!("not after dash/turndash");
		initial_speed = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN);
	}

	// println!("dash total speed: {}", initial_speed);
	
	fighter.clear_lua_stack();
	lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, initial_speed);
	app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);

	VarModule::set_float(fighter.battle_object, vars::common::instance::CURR_DASH_SPEED, initial_speed);
	VarModule::set_float(fighter.battle_object, vars::common::instance::MOONWALK_SPEED, 0.0);

    original!(fighter)
}

#[status_script(agent = "sonic", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn sonic_speciallw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_pre_SpecialNCommon();
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
        *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    0.into()
}

// FIGHTER_SONIC_STATUS_KIND_SPECIAL_HI_JUMP

#[status_script(agent = "sonic", status = FIGHTER_SONIC_STATUS_KIND_SPECIAL_HI_JUMP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_hi_jump_main(arg1: f32, fighter: &mut L2CFighterCommon) -> L2CValue {
    let accel_recover_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_jump_accel_recover_frame")) as i32;
    WorkModule::set_int(fighter.module_accessor, accel_recover_frame, *FIGHTER_SONIC_STATUS_SPECIAL_HI_JUMP_WORK_INT_RECOVER_GRAVITY_COUNTER);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi"), 0.0, 1.0, false, 0.0, false, false);
    StatusModule::set_situation_kind(fighter.module_accessor, app::SituationKind(*SITUATION_KIND_AIR), true);
	fighter.main_shift(special_hi_jump_main_loop)
}

pub unsafe extern "C" fn special_hi_jump_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[CURRENT_FRAME].get_i32() > 0 {
        let recover_gravity_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_SONIC_STATUS_SPECIAL_HI_JUMP_WORK_INT_RECOVER_GRAVITY_COUNTER);
        if recover_gravity_count >= 0 {
            WorkModule::dec_int(fighter.module_accessor, *FIGHTER_SONIC_STATUS_SPECIAL_HI_JUMP_WORK_INT_RECOVER_GRAVITY_COUNTER);
        }
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if !CancelModule::is_enable_cancel(fighter.module_accessor) || (CancelModule::is_enable_cancel(fighter.module_accessor) && !fighter.sub_wait_ground_check_common(L2CValue::Bool(false)).get_bool() && !fighter.sub_air_check_fall_common().get_bool()) {
        if !MotionModule::is_end(fighter.module_accessor) {
            return 0.into();
        }
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    else {
        if !fighter.sub_wait_ground_check_common(L2CValue::Bool(false)).get_bool() {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 1.into();
            }
        }
    }
    0.into()
}