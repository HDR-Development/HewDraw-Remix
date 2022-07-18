use super::*;
use globals::*;
// status script import
 
#[status_script(agent = "rockman", status = FIGHTER_STATUS_KIND_WALK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn pre_walk(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ground_brake = WorkModule::get_param_float(fighter.module_accessor, hash40("ground_brake"), 0);

	let mut initial_speed = VarModule::get_float(fighter.battle_object, vars::common::instance::CURR_DASH_SPEED);

	if ![*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH].contains(&StatusModule::prev_status_kind(fighter.module_accessor, 0)) {
		// println!("not after dash/turndash");
		initial_speed = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN);
	}

	// println!("walk initial speed: {}", initial_speed);

	VarModule::set_float(fighter.battle_object, vars::common::instance::CURR_DASH_SPEED, initial_speed);

    original!(fighter)
}

#[status_script(agent = "rockman", status = FIGHTER_STATUS_KIND_WALK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn walk(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WALK_FLAG_SLIP);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_SPECIAL);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ITEM);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_CATCH);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ATTACK);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ESCAPE);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_GUARD);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND);
    WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN);
    WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_WALK);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_SLIP);

    fighter.sub_shift_status_main(L2CValue::Ptr(walk_main as *const () as _))
}

unsafe extern "C" fn walk_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Walk_Main();
    0.into()
}

pub fn install() {
    install_status_scripts!(
        pre_walk,
        walk
    );
}