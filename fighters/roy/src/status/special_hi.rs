use super::*;

unsafe extern "C" fn special_hi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );

    return 0.into();
}

unsafe extern "C" fn special_hi_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let prev_speed = KineticModule::get_sum_speed3f(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let mut stop_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut app::KineticEnergy;
    let mut gravity_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) as *mut app::KineticEnergy;
    let mut motion_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION) as *mut app::KineticEnergy;

    if !fighter.is_situation(*SITUATION_KIND_AIR) {
        let reset_speed_2f = Vector2f { x: 0.0, y: 0.0 };
        let reset_speed_3f = Vector3f { x: 0.0, y: 0.0, z: 0.0 };
        lua_bind::KineticEnergy::reset_energy(motion_energy, *ENERGY_MOTION_RESET_TYPE_GROUND_TRANS_IGNORE_NORMAL, &reset_speed_2f, &reset_speed_3f, fighter.module_accessor);
        lua_bind::KineticEnergy::enable(motion_energy);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
    else {
        let air_hi_start_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("air_hi_start_x_mul"));
        let reset_speed_2f = Vector2f { x: prev_speed.x * air_hi_start_x_mul, y: 0.0 };
        let reset_speed_3f = Vector3f { x: 0.0, y: 0.0, z: 0.0 };
        lua_bind::KineticEnergy::reset_energy(stop_energy, *ENERGY_STOP_RESET_TYPE_AIR, &reset_speed_2f, &reset_speed_3f, fighter.module_accessor);
        lua_bind::KineticEnergy::enable(stop_energy);

        // let reset_speed_2f = Vector2f { x: 0.0, y: prev_speed.y };
        // let reset_speed_3f = Vector3f { x: 0.0, y: 0.0, z: 0.0 };
        // lua_bind::KineticEnergy::reset_energy(gravity_energy, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, &reset_speed_2f, &reset_speed_3f, fighter.module_accessor);
        // lua_bind::KineticEnergy::enable(gravity_energy);

        let mut gravity_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) as *mut app::FighterKineticEnergyGravity;
        let start_air_accel_y = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_hi.start_air_accel_y");
        smash::app::lua_bind::FighterKineticEnergyGravity::set_accel(gravity_energy, -start_air_accel_y);

        let reset_speed_2f = Vector2f { x: 0.0, y: 0.0 };
        let reset_speed_3f = Vector3f { x: 0.0, y: 0.0, z: 0.0 };
        lua_bind::KineticEnergy::reset_energy(motion_energy, *ENERGY_MOTION_RESET_TYPE_AIR_TRANS_ANGLE, &reset_speed_2f, &reset_speed_3f, fighter.module_accessor);
        lua_bind::KineticEnergy::enable(motion_energy);

        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }

    0.into()
}


pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_pre);
    agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_init);
}