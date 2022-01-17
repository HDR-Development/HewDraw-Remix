use super::*;
use globals::*;

//===================================================================
//== MOMENTUM TRANSFER
//== The chonky meat of the code; includes some status script hooks
//===================================================================

/* Moves that should bypass the momentum logic (in terms of the jump status script) */
const MOMENTUM_EXCEPTION_MOVES: [smash::lib::LuaConst ; 1] = [
    FIGHTER_SONIC_STATUS_KIND_SPIN_JUMP
];

//Jump (runs once at the beginning of the status)
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_Jump_sub)]
pub unsafe fn status_jump_sub_hook(fighter: &mut L2CFighterCommon, param_2: L2CValue, param_3: L2CValue) -> L2CValue {
    let boma = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let mut l2c_agent = L2CAgent::new(fighter.lua_state_agent);
    let fighter_kind = get_kind(boma);
    //println!("Pre-jump horizontal velocity: {}", KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN));

    if !MOMENTUM_EXCEPTION_MOVES.iter().any(|x| *x == StatusModule::status_kind(boma) ) {
        l2c_agent.clear_lua_stack();
        l2c_agent.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_CONTROL as u64));
        l2c_agent.push_lua_stack(&mut L2CValue::new_num(calc_melee_momentum(fighter, false, false, false)));
        sv_kinetic_energy::set_speed(fighter.lua_state_agent);
        l2c_agent.clear_lua_stack();
        //println!("Post-jump horizontal velocity: {}", KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN));
        curr_momentum[get_player_number(boma)] = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN); // Set the current momentum to what was just calculated
    }

    original!()(fighter, param_2, param_3)

}

//Aerials (runs once at the beginning of the status)
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_attack_air_common)]
pub unsafe fn status_attack_air_hook(fighter: &mut L2CFighterCommon, param_1: L2CValue){
    let boma = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let fighter_kind = get_kind(boma);
    let ratio = VarModule::get_float(fighter.object(), vars::common::JUMP_SPEED_RATIO);
    let jump_speed_x_max = WorkModule::get_param_float(boma, hash40("run_speed_max"), 0) * ratio;

    let mut l2c_agent = L2CAgent::new(fighter.lua_state_agent);
    let is_speed_backward = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) * PostureModule::lr(boma) < 0.0;
    let prev_status_check = [*FIGHTER_STATUS_KIND_JUMP, *FIGHTER_STATUS_KIND_JUMP_SQUAT].contains(&StatusModule::prev_status_kind(boma, 0));
    let mut new_speed = clamp(VarModule::get_float(fighter.object(), vars::common::CURRENT_MOMENTUM), -jump_speed_x_max, jump_speed_x_max);

    if prev_status_check {
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, new_speed);
        app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
        fighter.clear_lua_stack();
    }

    original!()(fighter, param_1)
}

/*      FALL CHANGED KINETIC TYPE NEUTRAL SPECIAL MOMENTUM   */
/* called in hooks/function_hooks/djcancel.rs in the change_kinetic hook */
/* special cased cus putting these in momentum_transfer_helper didnt work... momentum seemed to be reset every frame */
pub unsafe fn change_kinetic_momentum_related(boma: &mut smash::app::BattleObjectModuleAccessor, kinetic_type: i32) -> Option<i32> { //spacie laser momentum conservation
    let status_kind = StatusModule::status_kind(boma);
    let prev_status_kind = StatusModule::prev_status_kind(boma, 0);
    let situation_kind = StatusModule::situation_kind(boma);
    let fighter_kind = smash::app::utility::get_kind(boma);
    if [*FIGHTER_KIND_CAPTAIN, *FIGHTER_KIND_FALCO, *FIGHTER_KIND_FOX, *FIGHTER_KIND_GAMEWATCH, *FIGHTER_KIND_WOLF].contains(&fighter_kind) && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N
        && situation_kind == *SITUATION_KIND_AIR && [*FIGHTER_STATUS_KIND_JUMP, *FIGHTER_STATUS_KIND_JUMP_SQUAT].contains(&prev_status_kind) {
        return Some(-1);
    }
    None
}