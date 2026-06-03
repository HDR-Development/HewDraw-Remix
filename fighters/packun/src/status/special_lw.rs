use super::*;

unsafe extern "C" fn special_lw_bite_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
    && !VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
        let stance = VarModule::get_int(fighter.battle_object, vars::packun::instance::CURRENT_STANCE);
        SET_STANCE(fighter, (stance + 1) % 3, true);
        VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
    }

    return smashline::original_status(Exec, fighter, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_LW_BITE)(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(Exec, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_LW_BITE, special_lw_bite_exec);
}