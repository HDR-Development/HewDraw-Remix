use super::*;


unsafe extern "C" fn special_hi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_MOTION_CLIFF_MOVE,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );

    return 0.into();
}

pub unsafe extern "C" fn special_hi_rush_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH)(fighter);

    // Grounded Blade Rush shorten mechanic
    let dir_x = WorkModule::get_float(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLOAT_DECIDE_DIR_X);
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND
    && dir_x != 0.0 {
    //&& (dir_x != 0.0 || VarModule::is_flag(fighter.battle_object, vars::edge::instance::SPECIAL_HI_BLADE_DASH_NO_HITBOX)){
        let rush_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("rush_speed"));
        let stopEnergy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut app::KineticEnergyNormal;
        let vec2 = Vector2f{x: rush_speed * dir_x, y: 0.0};
        // let mut movement_mul = 1.0;
        // if VarModule::is_flag(fighter.battle_object, vars::edge::instance::SPECIAL_HI_BLADE_DASH_NO_HITBOX) {
        //     movement_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "blade_dash.fakeout_dash_movement_mul");;
        // }
        // let vec2 = Vector2f{x: rush_speed * dir_x * movement_mul, y: 0.0};
        app::lua_bind::KineticEnergyNormal::set_speed(stopEnergy, &vec2);
    }

    ret
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_pre);
    agent.status(Main, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH, special_hi_rush_main);
}