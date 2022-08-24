// status imports
use super::*;
use globals::*;

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
pub mod attack;
mod shield;
mod turn;
mod walk;
mod attackdash;
mod attackhi4;
mod attacklw4;
mod passive;
mod damagefall;
mod downdamage;
mod crawl;
mod cliff;
mod catchcut;
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
    println!("pre damageair");
    if VarModule::is_flag(fighter.battle_object, vars::common::status::IS_KNOCKDOWN_THROW) {
        println!("interrupt meteor");
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR);
        return 1.into();
    }
    call_original!(fighter)
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_DamageFlyCommon_init)]
pub unsafe fn damage_fly_common_init(fighter: &mut L2CFighterCommon) {
    ControlModule::set_command_life_extend(fighter.module_accessor, 5);
    if VarModule::is_flag(fighter.battle_object, vars::common::status::IS_KNOCKDOWN_THROW) {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FLY_REFLECT_D);
    }
    original!()(fighter)
}

#[smashline::common_status_script(status = FIGHTER_STATUS_KIND_DAMAGE_FLY, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END,
    symbol = "_ZN7lua2cpp16L2CFighterCommon20status_end_DamageFlyEv")]
pub unsafe fn damage_fly_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::set_command_life_extend(fighter.module_accessor, 0);
    original!()(fighter)
}

#[smashline::common_status_script(status = FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END,
    symbol = "_ZN7lua2cpp16L2CFighterCommon28status_end_DamageFlyReflectDEv")]
pub unsafe fn damage_fly_reflect_d_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::set_command_life_extend(fighter.module_accessor, 0);
    original!()(fighter)
}

#[smashline::common_status_script(status = FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_JUMP_BOARD, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END,
    symbol = "_ZN7lua2cpp16L2CFighterCommon36status_end_DamageFlyReflectJumpBoardEv")]
pub unsafe fn damage_fly_reflect_jump_board_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::set_command_life_extend(fighter.module_accessor, 0);
    original!()(fighter)
}

#[smashline::common_status_script(status = FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END,
    symbol = "_ZN7lua2cpp16L2CFighterCommon29status_end_DamageFlyReflectLREv")]
pub unsafe fn damage_fly_reflect_lr_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::set_command_life_extend(fighter.module_accessor, 0);
    original!()(fighter)
}

#[smashline::common_status_script(status = FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END,
    symbol = "_ZN7lua2cpp16L2CFighterCommon28status_end_DamageFlyReflectUEv")]
pub unsafe fn damage_fly_reflect_u_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::set_command_life_extend(fighter.module_accessor, 0);
    original!()(fighter)
}

#[smashline::common_status_script(status = FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END,
    symbol = "_ZN7lua2cpp16L2CFighterCommon24status_end_DamageFlyRollEv")]
pub unsafe fn damage_fly_roll_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::set_command_life_extend(fighter.module_accessor, 0);
    original!()(fighter)
}

#[smashline::common_status_script(status = FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn damage_fly_meteor_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::set_command_life_extend(fighter.module_accessor, 0);
    original!()(fighter)
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_wait_common_Main, 
            damage_fly_common_init, 
            status_pre_DamageAir,
            status_Landing_MainSub,
            //status_pre_Landing,
            status_pre_LandingLight,
            status_LandingAttackAirSub,
            status_pre_landing_fall_special,
            sub_air_transition_group_check_air_attack_hook,
            // sub_transition_group_check_air_lasso,
            sub_transition_group_check_ground_jump_mini_attack,
            sub_transition_group_check_air_escape,
            sub_transition_group_check_ground_escape,
            sub_transition_group_check_ground_guard
        );
    }
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_pre_Landing)]
pub unsafe fn status_pre_Landing(fighter: &mut L2CFighterCommon) -> L2CValue {
    let id = VarModule::get_int(fighter.battle_object, vars::common::instance::COSTUME_SLOT_NUMBER) as usize;
    let mut fighter_pos = Vector3f {
        x: PostureModule::pos_x(fighter.module_accessor),
        y: PostureModule::pos_y(fighter.module_accessor),
        z: PostureModule::pos_z(fighter.module_accessor)
    };
    fighter_pos.y += VarModule::get_float(fighter.object(), vars::common::instance::ECB_Y_OFFSETS);
    VarModule::set_float(fighter.battle_object, vars::common::instance::GET_DIST_TO_FLOOR, GroundModule::get_distance_to_floor(fighter.module_accessor, &fighter_pos, fighter_pos.y, true));
    let dist = VarModule::get_float(fighter.battle_object, vars::common::instance::GET_DIST_TO_FLOOR);
    if (0.0 <= dist || VarModule::is_flag(fighter.battle_object, vars::common::instance::ENABLE_AIR_ESCAPE_MAGNET)) && dist < 0.1 {
        if dist != -1.0 {
            PostureModule::set_pos(fighter.module_accessor, &fighter_pos);
        }
    }
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
        && cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N == 0
        && cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE != 0
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR) {
            fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_AIR.into(), true.into());
            return true.into();
        }
    }
    false.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_transition_group_check_ground_escape)]
unsafe fn sub_transition_group_check_ground_escape(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_cat_flag(Cat1::JumpButton) || fighter.is_cat_flag(Cat1::Jump) {
        return false.into()
    }
    call_original!(fighter)
}

#[skyline::hook(replace = L2CFighterCommon_sub_transition_group_check_ground_guard)]
unsafe fn sub_transition_group_check_ground_guard(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_cat_flag(Cat1::JumpButton) || fighter.is_cat_flag(Cat1::Jump) {
        return false.into()
    }
    call_original!(fighter)
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
    attackdash::install();
    attackhi4::install();
    attacklw4::install();
    passive::install();
    damagefall::install();
    downdamage::install();
    crawl::install();
    cliff::install();
    catchcut::install();

    smashline::install_status_scripts!(
        damage_fly_end,
        damage_fly_reflect_d_end,
        damage_fly_reflect_jump_board_end,
        damage_fly_reflect_lr_end,
        damage_fly_reflect_u_end,
        damage_fly_roll_end,
        damage_fly_meteor_end
    );

    skyline::nro::add_hook(nro_hook);
}

pub fn general_mechanics_status_script_nro_hooks(nro: &skyline::nro::NroInfo) {
    match nro.name {
        "common" => {
            skyline::install_hooks!(
                //status_jump_squat_hook, //Smash4 shorthop aerials (aerials can be buffered out of jumpsquat - no shorthop aerial macro)
                status_main_jumpsquat_hook, //Melee shorthop aerials (no buffered aerials - no shorthop aerial macro)
            );
        },
        _ => (),
    }
}

/*
Thought process here... for smash4 you can buffer an aerial out of jumpsquat...
so we clear buffer right before jumpsquat (status_JumpSquat runs once right as you enter that status)
For melee, you can't buffer aerials in jumpsquat, so we clear the buffer just after jumpsquat (or in this case since status_end_JumpSquat just didn't cooperate, during Jumpsquat)
so that any aerials you buffered during JS aren't taken into account.
*/

//Smash4 style shorthop aerials
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_JumpSquat)]
pub unsafe fn status_jump_squat_hook(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) == 0 {
        ControlModule::clear_command(boma, true);
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_MINI_ATTACK);
    }
    original!()(fighter)
}

//Melee style shorthop aerials
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_JumpSquat_Main)] //prolly better to use status_end_JumpSquat but for some reason it seemed like it wasn't being called
pub unsafe fn status_main_jumpsquat_hook(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    if MotionModule::frame(boma) <= 3.0 && (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) == 0 { //during JS and you're not inputting an airdodge...
        ControlModule::clear_command(boma, true);
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_MINI_ATTACK);
    }
    original!()(fighter)
}
