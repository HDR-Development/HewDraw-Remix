use super::*;

unsafe extern "C" fn rockman_airshooter_regular_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_airshooter"), hash40("life"));
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let speed_y = WorkModule::get_param_float(weapon.module_accessor, hash40("param_airshooter"), hash40("speed"));
    let accel_y = WorkModule::get_param_float(weapon.module_accessor, hash40("param_airshooter"), hash40("accel"));
    let limit_speed_y = WorkModule::get_param_float(weapon.module_accessor, hash40("param_airshooter"), hash40("speed"));
    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        0.0,
        speed_y
    );
    sv_kinetic_energy!(
        set_accel,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        0.0,
        accel_y
    );
    sv_kinetic_energy!(
        set_limit_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        0.0,
        limit_speed_y
    );
    0.into()
}

unsafe extern "C" fn rockman_airshooter_regular_exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if VarModule::is_flag(weapon.battle_object, vars::rockman_airshooter::status::MOVE) {
        let limit_speed_y = WorkModule::get_param_float(weapon.module_accessor, hash40("param_airshooter"), hash40("limit_speed"));
        sv_kinetic_energy!(
            set_limit_speed,
            weapon,
            WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
            0.0,
            limit_speed_y
        );
        VarModule::off_flag(weapon.battle_object, vars::rockman_airshooter::status::MOVE);
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(
            Init,
            *WEAPON_ROCKMAN_AIRSHOOTER_STATUS_KIND_REGULAR,
            rockman_airshooter_regular_init,
        );
    agent.status(Exec, *WEAPON_ROCKMAN_AIRSHOOTER_STATUS_KIND_REGULAR, rockman_airshooter_regular_exec);
}