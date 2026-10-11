use super::*;

unsafe extern "C" fn special_hi_2_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let control = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_HI_FLAG_CONTROL);
    original_status(Exec, fighter, *FIGHTER_IKE_STATUS_KIND_SPECIAL_HI_2)(fighter);
    if control {
        sv_kinetic_energy!(
            set_limit_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            -1.0,
            -1.0
        );
        sv_kinetic_energy!(
            set_stable_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            100.0,
            100.0
        );
        sv_kinetic_energy!(
            mul_x_accel_add,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            2.0
        );
        sv_kinetic_energy!(
            mul_x_accel_mul,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            2.0
        );
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Exec, *FIGHTER_IKE_STATUS_KIND_SPECIAL_HI_2, special_hi_2_exec);
}
