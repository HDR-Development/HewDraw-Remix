// status imports
use super::*;
use globals::*;
use utils::game_modes::CustomMode;

macro_rules! interrupt {
    () => { return L2CValue::I32(1); };
    ($fighter:ident, $status:expr, $repeat:expr) => {{ $fighter.change_status($status.into(), $repeat.into()); interrupt!(); }}
}

mod airdodge;
mod dash;
mod jumpsquat;
pub mod jump;
mod footstool;
mod run;
mod attack;
mod shield;
mod turn;
mod walk;
mod passive;
mod damagefall;
mod downdamage;
mod crawl;
mod cliff;
mod catch;
mod damage;
mod escape;
mod dead;
// mod damageflyreflect;
mod down;
mod float;
// [LUA-REPLACE-REBASE]
// [SHOULD-CHANGE]
// Reimplement the whole status script (already done) instead of doing this.
// Technically this is fine, but I'd rather remove the code which transitions to airdodge instead
// of deleting the input

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_wait_common_Main)]
pub unsafe fn sub_wait_common_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
	let turn_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("turn_stick_x"));

	if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_AIR {
		if fighter.global_table[STICK_X].get_f32() * PostureModule::lr(fighter.module_accessor) > turn_stick_x {
			//println!("no turn");
			WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN);
		}
		else {
			WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN);
		}
		if !fighter.sub_wait_ground_check_common(L2CValue::Bool(false)).get_bool() {
			if !fighter.sub_ground_check_ottotto().get_bool() {
				return 0.into();
			}
		}
	}
	else {
		interrupt!(fighter, *FIGHTER_STATUS_KIND_FALL, false);
	}
	1.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_pre_DamageAir)]
pub unsafe fn status_pre_DamageAir(fighter: &mut L2CFighterCommon) -> L2CValue {
    //println!("knockback units: {}", DamageModule::reaction(fighter.module_accessor, 0));
    
    fighter.clear_lua_stack();
    lua_args!(fighter, hash40("angle"));
    sv_information::damage_log_value(fighter.lua_state_agent);
    let angle = fighter.pop_lua_stack(1).get_f32();
    let degrees = angle.to_degrees();
    let meteor_vector_min = WorkModule::get_param_int(fighter.module_accessor, hash40("battle_object"), hash40("meteor_vector_min")) as f32;
    let meteor_vector_max = WorkModule::get_param_int(fighter.module_accessor, hash40("battle_object"), hash40("meteor_vector_max")) as f32; 
    
    if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_KNOCKDOWN_THROW)
    || (degrees >= meteor_vector_min && degrees <= meteor_vector_max && DamageModule::reaction(fighter.module_accessor, 0) >= 65.0) {
        //println!("forced tumble");
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR);
        return 1.into();
    }

    // Checks whether you have successfully CC'd into non-tumble knockback
    // This is so we can apply half hitstun upon landing from a CC'd attack
    if fighter.is_prev_status_one_of(&[*FIGHTER_STATUS_KIND_SQUAT, *FIGHTER_STATUS_KIND_SQUAT_WAIT]) {
        VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_CC_NON_TUMBLE);
    }
    else {
        VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_CC_NON_TUMBLE);
    }
    call_original!(fighter)
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_DamageFlyCommon_init)]
pub unsafe fn damage_fly_common_init(fighter: &mut L2CFighterCommon) {
    // ControlModule::set_command_life_extend(fighter.module_accessor, 5);
    if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_KNOCKDOWN_THROW) {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FLY_REFLECT_D);
    }
    VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_KNOCKDOWN_THROW);
    original!()(fighter)
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_wait_common_Main, 
            damage_fly_common_init, 
            status_pre_DamageAir,
            status_Landing_MainSub,
            status_LandingStiffness,
            status_pre_LandingLight,
            status_LandingAttackAirSub,
            status_pre_landing_fall_special,
            sub_air_transition_group_check_air_attack_hook,
            // sub_transition_group_check_air_lasso,
            sub_transition_group_check_ground_jump_mini_attack,
            change_status_jump_mini_attack,
            sub_transition_group_check_ground_attack,
            sub_transition_group_check_air_escape,
            sub_transition_group_check_ground_escape,
            sub_transition_group_check_ground_guard,
            sub_transition_group_check_ground,
            sys_line_status_system_control_hook,
            status_FallSub_hook,
            super_jump_punch_main_hook,
            sub_cliff_uniq_process_exec_fix_pos,
            end_pass_ground,
            virtual_ftStatusUniqProcessDamage_exec_common,
            FighterStatusDamage__correctDamageVector,
            FighterStatusDamage__correctDamageVectorEffect,
            sub_fighter_pre_end_status,
            sub_is_dive,
        );
    }
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_LandingStiffness)]
pub unsafe fn status_LandingStiffness(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_STATUS_KIND] == FIGHTER_STATUS_KIND_DAMAGE_AIR
    && VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_CC_NON_TUMBLE) {
        // halve hitstun on non-tumble landing if CC'd
        // if halved hitstun is less than your heavy landing lag value, use your heavy landing lag value
        let hitstun = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME);
        let landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("landing_frame"), 0);
        WorkModule::set_float(fighter.module_accessor, (hitstun * 0.5).max(landing_frame), *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME);
    }
    VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_CC_NON_TUMBLE);
    original!()(fighter)
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_pre_LandingLight)]
pub unsafe fn status_pre_LandingLight(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE);
        ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE_F);
        ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE_B);
    }
    original!()(fighter)
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_LandingAttackAirSub)]
pub unsafe fn status_LandingAttackAirSub(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE);
        ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE_F);
        ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE_B);
    }
    original!()(fighter)
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_pre_landing_fall_special)]
pub unsafe fn status_pre_landing_fall_special(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE);
        ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE_F);
        ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE_B);
    }
    original!()(fighter)
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_Landing_MainSub)]
pub unsafe fn status_Landing_MainSub(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);

    if StatusModule::prev_status_kind(boma, 0) == *FIGHTER_STATUS_KIND_ESCAPE_AIR || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE);
        ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE_F);
        ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE_B);
    }
    original!()(fighter)
}

#[skyline::hook(replace = L2CFighterCommon_sub_transition_group_check_air_attack)]
unsafe fn sub_air_transition_group_check_air_attack_hook(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND] == FIGHTER_STATUS_KIND_CLIFF_ROBBED {
        false.into()
    } else {
        call_original!(fighter)
    }
}

#[skyline::hook(replace = L2CFighterCommon_sub_transition_group_check_air_lasso)]
unsafe fn sub_transition_group_check_air_lasso(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        // Disable Airdodging if you're pressing Grab.
        let is_guard_buffered = ControlModule::get_trigger_count(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD as u8) < ControlModule::get_command_life_count_max(fighter.module_accessor) as i32;  // checks if Guard input was pressed within max tap buffer window
        let is_attack_buffered = ControlModule::get_trigger_count(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK as u8) < ControlModule::get_command_life_count_max(fighter.module_accessor) as i32;  // checks if Attack input was pressed within max tap buffer window
        // original line
        // if cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_AIR_LASSO != 0 {
        // Split the Air Lasso check into two inputs, so that if the buffer gets cleared and you're still holding Shield,
        // you will never get an air tether. That's the theory, anyway.
        // check_button_on_trriger check is here strictly to preserve frame-perfect DJCZ tech, as get_trigger_count does not correctly update when X + Z + direction are input on same frame...
        if (ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) || is_guard_buffered)
        && (ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) || is_attack_buffered)
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_AIR_LASSO) {
            let air_lasso = WorkModule::get_param_int(fighter.module_accessor, hash40("air_lasso_type"), 0);
            if air_lasso != *FIGHTER_AIR_LASSO_TYPE_NONE
            && !LinkModule::is_link(fighter.module_accessor, *FIGHTER_LINK_NO_CONSTRAINT) {
                fighter.change_status(FIGHTER_STATUS_KIND_AIR_LASSO.into(), true.into());
                return true.into();
            }
        }
    }
    false.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_transition_group_check_ground_jump_mini_attack)]
unsafe fn sub_transition_group_check_ground_jump_mini_attack(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        if fighter.global_table[0x52].get_bool() {
            let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[0x52].get_ptr());
            return callable(fighter);
        }
        // Disable the grab button from being used to perform the "short hop aerial macro"
        let cat1 = fighter.global_table[CMD_CAT1].get_i32();
        if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH == 0 // Added "No Grab" check
        && cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0
        && fighter.sub_check_button_jump().get_bool() {
            fighter.change_status_jump_mini_attack(false.into());
            return true.into();
        }
    }
    false.into()
}

#[skyline::hook(replace = L2CFighterCommon_change_status_jump_mini_attack)]
unsafe fn change_status_jump_mini_attack(fighter: &mut L2CFighterCommon, arg: L2CValue) -> L2CValue {
    if fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_ATTACK_100,
        *FIGHTER_STATUS_KIND_ATTACK_DASH,
        *FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
        *FIGHTER_STATUS_KIND_ATTACK_S4_START,
        *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD,
        *FIGHTER_STATUS_KIND_ATTACK_HI4_START,
        *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD,
        *FIGHTER_STATUS_KIND_ATTACK_LW4_START,
        *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD
    ]) {
        VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_ATTACK_CANCEL);
    }
    call_original!(fighter, arg)
}

#[skyline::hook(replace = L2CFighterCommon_sub_transition_group_check_air_escape)]
unsafe fn sub_transition_group_check_air_escape(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[0x2F].get_bool() {
        let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[0x2F].get_ptr());
        if callable(fighter).get_bool() {
            return true.into();
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        let cat1 = fighter.global_table[CMD_CAT1].get_i32();
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR)
                    // Disable Airdodging if you're pressing Attack.
        && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N == 0
        && cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE != 0)
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR) {
            fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_AIR.into(), true.into());
            return true.into();
        }
    }
    false.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_transition_group_check_ground_attack)]
unsafe fn sub_transition_group_check_ground_attack(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_button_on(Buttons::Catch) {
        return false.into()
    }
    call_original!(fighter)
}

#[skyline::hook(replace = L2CFighterCommon_sub_transition_group_check_ground_escape)]
unsafe fn sub_transition_group_check_ground_escape(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_cat_flag(Cat1::JumpButton)
    || fighter.is_cat_flag(Cat1::Jump)
    || fighter.is_button_on(Buttons::Catch) {
        return false.into()
    }
    call_original!(fighter)
}

#[skyline::hook(replace = L2CFighterCommon_sub_transition_group_check_ground_guard)]
unsafe fn sub_transition_group_check_ground_guard(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_cat_flag(Cat1::JumpButton)
    || fighter.is_cat_flag(Cat1::Jump)
    || fighter.is_button_on(Buttons::Catch) {
        return false.into()
    }
    // if fighter.is_button_on(Buttons::Parry) {
    //     VarModule::on_flag(fighter.object(), vars::common::instance::IS_PARRY_FOR_GUARD_OFF);
    //     fighter.change_status(FIGHTER_STATUS_KIND_GUARD_OFF.into(), true.into());
    //     return true.into()
    // }
    call_original!(fighter)
}

#[skyline::hook(replace = L2CFighterCommon_sub_transition_group_check_ground)]
unsafe fn sub_transition_group_check_ground(fighter: &mut L2CFighterCommon, to_squat_wait: L2CValue) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        let cat2 = fighter.global_table[CMD_CAT2].get_i32();
        if cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI != 0
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_U)
        && {
            fighter.clear_lua_stack();
            lua_args!(fighter, Hash40::new_raw(0x1daca540be));
            sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
            fighter.pop_lua_stack(1).get_bool()
        } {
            fighter.change_status(FIGHTER_STATUS_KIND_APPEAL.into(), false.into());
            return true.into();
        }
        if cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_LW != 0
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_LW)
        && {
            fighter.clear_lua_stack();
            lua_args!(fighter, Hash40::new_raw(0x1daca540be));
            sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
            fighter.pop_lua_stack(1).get_bool()
        } {
            fighter.change_status(FIGHTER_STATUS_KIND_APPEAL.into(), false.into());
            return true.into();
        }
        if cat2 & (*FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L | *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R) != 0
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_S)
        && {
            fighter.clear_lua_stack();
            lua_args!(fighter, Hash40::new_raw(0x1daca540be));
            sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
            fighter.pop_lua_stack(1).get_bool()
        } {
            fighter.change_status(FIGHTER_STATUS_KIND_APPEAL.into(), false.into());
            return true.into();
        }
        let cat1 = fighter.global_table[CMD_CAT1].get_i32();
        if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH != 0
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH) {
            fighter.change_status(FIGHTER_STATUS_KIND_TURN_DASH.into(), true.into());
            return true.into();
        }
        if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH != 0
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH) {
            fighter.change_status(FIGHTER_STATUS_KIND_DASH.into(), true.into());
            return true.into();
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT)
        && fighter.sub_check_command_squat().get_bool() {
            let status = if to_squat_wait.get_bool() {
                FIGHTER_STATUS_KIND_SQUAT_WAIT
            }
            else {
                FIGHTER_STATUS_KIND_SQUAT
            };
            fighter.change_status(status.into(), true.into());
            return true.into();
        }
        // Vanilla uses TURN cat flag, which makes turnarounds bufferable
        // which was more of a hinderance within our engine, so this makes turnarounds unbufferable
        if fighter.left_stick_x() * PostureModule::lr(fighter.module_accessor) <= WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("turn_stick_x"))
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN) {
            fighter.change_status(FIGHTER_STATUS_KIND_TURN.into(), true.into());
            return true.into();
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_WALK)
        && fighter.sub_check_command_walk().get_bool() {
            fighter.change_status(FIGHTER_STATUS_KIND_WALK.into(), true.into());
            return true.into();
        }
    }
    false.into()
}

// This function runs once per frame for all battle objects, immediately before MAIN status
// Processes of this function include:
//   1. Incrementing CURRENT_FRAME counter in global table
//   2. Calling INIT status
//   3. Calling sub statuses
//   4. Updating situation kind in global table
#[skyline::hook(replace = smash::lua2cpp::L2CFighterBase_sys_line_status_system_control)]
pub unsafe fn sys_line_status_system_control_hook(fighter: &mut L2CFighterBase) -> L2CValue {
    if VarModule::has_var_module(fighter.battle_object)
    && VarModule::is_flag(fighter.battle_object, vars::common::instance::CHECK_CHANGE_MOTION_ONLY)
    {
        // When we are calling MAIN status for the sole purpose of changing motion kind upon contact with a surface,
        // there is no need to increment the CURRENT_FRAME counter,
        // or run sub statuses (which are often used to increment various counters used during a status)
        // So we are only updating situation kind, then returning
        // MAIN status will then be called after returning
        VarModule::off_flag(fighter.battle_object, vars::common::instance::CHECK_CHANGE_MOTION_ONLY);
        VarModule::on_flag(fighter.battle_object, vars::common::instance::FLUSH_EFFECT_ACMD);
        let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
        fighter.global_table[SITUATION_KIND].assign(&L2CValue::I32(situation_kind));
        fighter.global_table[0xD].assign(&L2CValue::Bool(false));
        0.into()
    }
    else {
        call_original!(fighter)
    }
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_FallSub)]
pub unsafe fn status_FallSub_hook(fighter: &mut L2CFighterCommon, arg2: L2CValue) {
    call_original!(fighter, arg2);
    let move_speed = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN);
    if move_speed * PostureModule::lr(fighter.module_accessor) < 0.0
    && MotionModule::motion_kind(fighter.module_accessor) != hash40("fall") {
        // Avoid runfall/walkfall animation if you were moving backwards out of dash
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("fall"), 0.0, 1.0, false, 0.0, false, false);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_RUN_FALL);
    }
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_super_jump_punch_main)]
pub unsafe fn super_jump_punch_main_hook(fighter: &mut L2CFighterCommon) {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return;
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_FALL_SPECIAL) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS) {
            if fighter.global_table[PREV_SITUATION_KIND] == SITUATION_KIND_AIR
            && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND
            && MotionModule::trans_move_speed(fighter.module_accessor).y < 0.0
            {
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
            }
        }
        else {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_CHANGE_KINE)
            && fighter.global_table[PREV_SITUATION_KIND] == SITUATION_KIND_AIR
            && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND
            {
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
            }
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        let new_status = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_STATUS_KIND_END);
        fighter.change_status_req(new_status, false);
    }
}

// I honestly don't know why this function was needed in vanilla in the first place
// Forces situation kind changes during ledge actions, even though situation kind automatically changes based on character position
// Also forces ECB shape changes, while stubbing this doesn't affect ECB shape whatsoever
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_cliff_uniq_process_exec_fix_pos)]
pub unsafe fn sub_cliff_uniq_process_exec_fix_pos(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[STATUS_KIND] != FIGHTER_STATUS_KIND_CLIFF_WAIT
    && fighter.global_table[STATUS_KIND] != FIGHTER_STATUS_KIND_CLIFF_CATCH
    && fighter.global_table[STATUS_KIND] != FIGHTER_STATUS_KIND_CLIFF_ATTACK
    && fighter.global_table[STATUS_KIND] != FIGHTER_STATUS_KIND_CLIFF_CLIMB
    && fighter.global_table[STATUS_KIND] != FIGHTER_STATUS_KIND_CLIFF_ESCAPE
    && fighter.global_table[STATUS_KIND] != FIGHTER_STATUS_KIND_CLIFF_JUMP1
    {
        return;
    }
    if fighter.global_table[STATUS_KIND] != FIGHTER_STATUS_KIND_CLIFF_WAIT {
        if !GroundModule::is_status_cliff(fighter.module_accessor) {
            if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
                if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_CLIFF {
                    return;
                }
                StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), false);
                let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
                fighter.global_table[PREV_SITUATION_KIND].assign(&L2CValue::I32(situation_kind));
                fighter.global_table[SITUATION_KIND].assign(&L2CValue::I32(*SITUATION_KIND_AIR));
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
                return;
            }
            let correct = GroundModule::get_correct(fighter.module_accessor);
            if correct != *GROUND_CORRECT_KIND_CLIFF {
                return;
            }
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
            app::sv_kinetic_energy::set_ground_trans(fighter.lua_state_agent);
            GroundModule::set_shape_flag(fighter.module_accessor, *GROUND_CORRECT_SHAPE_RHOMBUS_MODIFY_FLAG_FRONT_FIX as u16, false);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CLIFF_FLAG_TO_GROUND);
        }
        else {
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
            let is_cliff_ground_trans = app::sv_kinetic_energy::is_cliff_ground_trans(fighter.lua_state_agent);
            if !is_cliff_ground_trans {
                return;
            }
            let mut tra_out = Vector3f::zero();
            MotionModule::trans_tra(fighter.module_accessor, &mut tra_out as *mut Vector3f, true, true);
            if tra_out.z < 0.4  // 0.3 in vanilla
            || tra_out.y < -0.02  // -0.03 in vanilla
            {
                return;
            }
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
            app::sv_kinetic_energy::set_ground_trans(fighter.lua_state_agent);
            GroundModule::set_shape_flag(fighter.module_accessor, *GROUND_CORRECT_SHAPE_RHOMBUS_MODIFY_FLAG_FRONT_FIX as u16, false);
            StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), false);
            let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
            fighter.global_table[PREV_SITUATION_KIND].assign(&L2CValue::I32(situation_kind));
            fighter.global_table[SITUATION_KIND].assign(&L2CValue::I32(*SITUATION_KIND_GROUND));
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            GroundModule::set_status_ground(fighter.module_accessor);
            GroundModule::leave_cliff(fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CLIFF_FLAG_TO_GROUND);
        }
    }
    else {
        let is_touch_down = GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
        if !is_touch_down {
            return;
        }
        StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), false);
        let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
        fighter.global_table[PREV_SITUATION_KIND].assign(&L2CValue::I32(situation_kind));
        fighter.global_table[SITUATION_KIND].assign(&L2CValue::I32(*SITUATION_KIND_GROUND));
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    }
    return;
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_end_pass_ground)]
pub unsafe fn end_pass_ground(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_STATUS_KIND] != FIGHTER_STATUS_KIND_DASH
    && (fighter.kind() != *FIGHTER_KIND_RYU || fighter.global_table[PREV_STATUS_KIND] != FIGHTER_RYU_STATUS_KIND_DASH_BACK)
    && (fighter.kind() != *FIGHTER_KIND_KEN || fighter.global_table[PREV_STATUS_KIND] != FIGHTER_RYU_STATUS_KIND_DASH_BACK)
    && (fighter.kind() != *FIGHTER_KIND_DOLLY || fighter.global_table[PREV_STATUS_KIND] != FIGHTER_DOLLY_STATUS_KIND_DASH_BACK)
    && (fighter.kind() != *FIGHTER_KIND_DEMON || fighter.global_table[PREV_STATUS_KIND] != FIGHTER_DEMON_STATUS_KIND_DASH_BACK)
    {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
    }
    call_original!(fighter)
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_virtual_ftStatusUniqProcessDamage_exec_common)]
pub unsafe fn virtual_ftStatusUniqProcessDamage_exec_common(fighter: &mut L2CFighterCommon) {
    // Adding FIGHTER_STATUS_KIND_DAMAGE_AIR to this check allows for DI on non-tumble knockback
    if [*FIGHTER_STATUS_KIND_DAMAGE,
        *FIGHTER_STATUS_KIND_DAMAGE_AIR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
        *FIGHTER_STATUS_KIND_SAVING_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_LUIGI_FINAL_SHOOT,
        ].contains(&fighter.global_table[STATUS_KIND].get_i32())
    {
        fighter.ftStatusUniqProcessDamageFly_exec_common();
    }
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_FighterStatusDamage__correctDamageVector)]
pub unsafe fn FighterStatusDamage__correctDamageVector(fighter: &mut L2CFighterCommon) -> L2CValue {
    match utils::game_modes::get_custom_mode() {
        Some(modes) => {
            if modes.contains(&CustomMode::Smash64Mode) {
                return 0.into();
            }
        },
        _ => {}
    }
    call_original!(fighter)
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_FighterStatusDamage__correctDamageVectorEffect)]
pub unsafe fn FighterStatusDamage__correctDamageVectorEffect(fighter: &mut L2CFighterCommon) -> L2CValue {
    match utils::game_modes::get_custom_mode() {
        Some(modes) => {
            if modes.contains(&CustomMode::Smash64Mode) {
                return 0.into();
            }
        },
        _ => {}
    }
    if fighter.global_table[STATUS_KIND_INTERRUPT] != FIGHTER_STATUS_KIND_DAMAGE_AIR {
        return call_original!(fighter);
    }
    // This allows us to call the blue DI line effect on non-tumble knockback
    // Currently not able to be done by reimplementing this function
    // because an inner function returns multiple L2CValues
    // which is not currently supported by skyline-smash
    fighter.global_table[STATUS_KIND_INTERRUPT].assign(&L2CValue::I32(*FIGHTER_STATUS_KIND_DAMAGE_FLY));
    let ret = call_original!(fighter);
    fighter.global_table[STATUS_KIND_INTERRUPT].assign(&L2CValue::I32(*FIGHTER_STATUS_KIND_DAMAGE_AIR));
    ret
}

// Disables aerials canceling fast fall
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_fighter_pre_end_status)]
pub unsafe fn sub_fighter_pre_end_status(fighter: &mut L2CFighterCommon) {
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_is_dive)]
pub unsafe fn sub_is_dive(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
        return false.into();
    }

    let status_kind = fighter.global_table[STATUS_KIND_INTERRUPT].get_i32();
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
        return false.into();
    }

    if [*FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
        *FIGHTER_STATUS_KIND_SAVING_DAMAGE_FLY,
    ].contains(&status_kind) {
        return false.into();
    }

    // Disallows fastfall from ledge after a certain amount of regrabs
    let cliff_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_COUNT);
    let cliff_dive_count_max = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), 0x189f0b0c96);
    if cliff_count > cliff_dive_count_max {
        return false.into();
    }
    
    if !KineticModule::is_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL)
    || KineticModule::is_suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL) {
        return false.into();
    }

    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if speed_y >= 0.0 {
        return false.into();
    }

    let mut dive_cont_value = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("dive_cont_value"));
    let mut dive_flick_frame_value = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("dive_flick_frame_value"));
    if [*FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE,
        *FIGHTER_STATUS_KIND_CLIFF_CATCH,
        *FIGHTER_STATUS_KIND_CLIFF_WAIT,
    ].contains(&prev_status_kind) {
        dive_cont_value = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("cliff_dive_cont_value"));
        dive_flick_frame_value = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("cliff_dive_flick_frame_value"));
    }

    if fighter.left_stick_y() > dive_cont_value
    || VarModule::get_int(fighter.battle_object, vars::common::instance::LEFT_STICK_FLICK_Y) >= dive_flick_frame_value {
        return false.into();
    }

    let dive_speed_y = WorkModule::get_param_float(fighter.module_accessor, hash40("dive_speed_y"), 0);
    if speed_y < -dive_speed_y {
        return false.into();
    }
    
    true.into()
}

pub fn install() {
    airdodge::install();
    dash::install();
    jumpsquat::install();
    jump::install();
    footstool::install();
    run::install();
    attack::install();
    shield::install();
    turn::install();
    walk::install();
    passive::install();
    damagefall::install();
    downdamage::install();
    crawl::install();
    cliff::install();
    catch::install();
    damage::install();
    escape::install();
    dead::install();
    // damageflyreflect::install();
    down::install();

    skyline::nro::add_hook(nro_hook);
}