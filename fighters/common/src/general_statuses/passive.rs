// status imports
use super::*;
use globals::*;
// This file contains code related to teching

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_check_passive_button_for_damage,
            status_pre_passive,
            status_pre_passivefb,
            sub_uniq_process_Passive_init
        );
    }
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_check_passive_button_for_damage)]
pub unsafe fn sub_check_passive_button_for_damage(fighter: &mut L2CFighterCommon, trigger_frame: L2CValue) -> L2CValue {
    let is_valid_tech_input = fighter.sub_check_passive_button(trigger_frame).get_bool();
    return L2CValue::Bool(is_valid_tech_input)
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_pre_Passive)]
pub unsafe fn status_pre_passive(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_UNIQ, // Originally *FIGHTER_KINETIC_TYPE_PASSIVE_GROUND
        *GROUND_CORRECT_KIND_GROUND as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        true,
        *FIGHTER_TREADED_KIND_ENABLE,
        false,
        false,
        false,
        0,
        *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE as u32,
        0,
        0
    );
    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_pre_PassiveFB)]
pub unsafe fn status_pre_passivefb(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_UNIQ, // Originally *FIGHTER_KINETIC_TYPE_PASSIVE_GROUND_MOTION
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        false,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        true,
        *FIGHTER_TREADED_KIND_ENABLE,
        false,
        false,
        false,
        0,
        (
            *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE
        ) as u32,
        0,
        0
    );
    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_uniq_process_Passive_init)]
pub unsafe fn sub_uniq_process_Passive_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_PASSIVE, *FIGHTER_STATUS_KIND_PASSIVE_FB]) {
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
        let damage_speed_x = smash::app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
        let damage_speed_y = smash::app::sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
        smash::app::sv_kinetic_energy::clear_speed(fighter.lua_state_agent);
        
        KineticModule::unable_energy_all(fighter.module_accessor);
    
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, ENERGY_MOTION_RESET_TYPE_GROUND_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
        app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_GROUND, damage_speed_x, damage_speed_y, 0.0, 0.0, 0.0);
        app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    
        let ground_brake = WorkModule::get_param_float(fighter.module_accessor, hash40("ground_brake"), 0);
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ground_brake, 0.0);
        app::sv_kinetic_energy::set_brake(fighter.lua_state_agent);
    
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    }
    call_original!(fighter)
}