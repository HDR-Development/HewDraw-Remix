use super::*;
use globals::*;


// FIGHTER_STATUS_KIND_SPECIAL_HI

#[status_script(agent = "edge", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn edge_special_hi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_MOTION_CLIFF_MOVE,
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
        (
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI |
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
            *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON
        ) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    0.into()
}

// FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH

#[status_script(agent = "edge", status = FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn edge_special_hi_rush_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original!(fighter);

    // Grounded Blade Rush shorten mechanic
    let dir_x = WorkModule::get_float(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLOAT_DECIDE_DIR_X);
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND
    && dir_x != 0.0 {
        let rush_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("rush_speed"));
        let stopEnergy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut app::KineticEnergyNormal;
        let vec2 = Vector2f{x: rush_speed * dir_x, y: 0.0};
        app::lua_bind::KineticEnergyNormal::set_speed(stopEnergy, &vec2);
    }

    ret
}

pub fn install() {
    install_status_scripts!(
        edge_special_hi_pre,
        edge_special_hi_rush_main
    );
}