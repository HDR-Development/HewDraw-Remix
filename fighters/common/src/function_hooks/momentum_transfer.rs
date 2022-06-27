use super::*;
use globals::*;
use crate::misc::calc_melee_momentum;

//===================================================================
//== MOMENTUM TRANSFER
//== The chonky meat of the code; includes some status script hooks
//===================================================================

pub fn install() {
    skyline::nro::add_hook(nro_main).unwrap();
}

fn nro_main(nro: &skyline::nro::NroInfo) {
    match nro.name {
        "common" => {
            skyline::install_hooks!(
                // lua2cpp_common.nro hooks here
                status_attack_air_hook
            );
        }
        _ => (),
    }
}

//Aerials (runs once at the beginning of the status)
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_attack_air_common)]
pub unsafe fn status_attack_air_hook(fighter: &mut L2CFighterCommon, param_1: L2CValue){
    let boma = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let fighter_kind = boma.kind();
    let ratio = VarModule::get_float(fighter.object(), vars::common::instance::JUMP_SPEED_RATIO);

    // get the multiplier for any special mechanics that require additional jump speed max (meta quick, etc)
    let mut jump_speed_max_mul = VarModule::get_float(fighter.object(), vars::common::instance::JUMP_SPEED_MAX_MUL);
    match jump_speed_max_mul {
        // if its not between 0.1 and 3.0, it is likely not a real value and we should ignore it
        0.1..=3.0 => {},
        _ => { jump_speed_max_mul = 1.0 }
    }

    let mut jump_speed_x_max = WorkModule::get_param_float(boma, hash40("run_speed_max"), 0) * ratio * jump_speed_max_mul;

    let mut l2c_agent = L2CAgent::new(fighter.lua_state_agent);
    let new_speed = VarModule::get_float(fighter.object(), vars::common::instance::CURRENT_MOMENTUM).clamp(-jump_speed_x_max, jump_speed_x_max);

    if StatusModule::prev_status_kind(boma, 0) == *FIGHTER_STATUS_KIND_JUMP
    || (fighter_kind == *FIGHTER_KIND_SONIC && StatusModule::prev_status_kind(boma, 0) == *FIGHTER_SONIC_STATUS_KIND_SPIN_JUMP) {
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
    let fighter_kind = boma.kind();
    if (([*FIGHTER_KIND_CAPTAIN, *FIGHTER_KIND_FALCO, *FIGHTER_KIND_FOX, *FIGHTER_KIND_GAMEWATCH, *FIGHTER_KIND_WOLF].contains(&fighter_kind) && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N))
        && situation_kind == *SITUATION_KIND_AIR && [*FIGHTER_STATUS_KIND_JUMP, *FIGHTER_STATUS_KIND_JUMP_SQUAT].contains(&prev_status_kind) {
        return Some(-1);
    }
    None
}