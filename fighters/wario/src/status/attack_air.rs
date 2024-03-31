use super::*;

// FIGHTER_STATUS_KIND_ATTACK_AIR

unsafe extern "C" fn attack_air_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let dairAnim = Hash40::new("attack_air_lw");
    let dairRiseAnim = Hash40::new("attack_air_lw2");
    
    if MotionModule::motion_kind(fighter.module_accessor) != dairAnim.hash{
        return false.into();
    }
    if (AttackModule::is_infliction_status(fighter.module_accessor,  *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD)){
        MotionModule::change_motion(fighter.module_accessor, dairRiseAnim, 18.0, 1.0, false, 0.0, false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_machstamp"),false,true);
        AttackModule::clear_all(fighter.module_accessor);
    }
    
    return false.into();
}

// FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR

unsafe extern "C" fn landing_attack_air_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    EFFECT_OFF_KIND(fighter, Hash40::new("sys_merikomi"),false,true);
    return false.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Exec, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_exec);
    agent.status(End, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, landing_attack_air_end);
}
