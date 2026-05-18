use super::*;

// FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_END

unsafe extern "C" fn special_s_end_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Init, fighter, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_END)(fighter);

    // Reduce speed on shield
    let prev_inflict_status = VarModule::get_int(fighter.battle_object, vars::common::instance::PREV_STATUS_INFLICT_STATUS);
    if prev_inflict_status & *COLLISION_KIND_MASK_SHIELD != 0 || prev_inflict_status & *COLLISION_KIND_MASK_PARRY != 0 {
        let shield_hit_end_speed_x = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_s.shield_hit_end_speed_x");
        let lr = PostureModule::lr(fighter.module_accessor);
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            shield_hit_end_speed_x * lr,
            0.0
        );
    }

    ret
}

pub fn install(agent: &mut Agent) {
    agent.status(Init, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_END, special_s_end_init);
}