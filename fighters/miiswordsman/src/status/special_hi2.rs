use super::*;

unsafe extern "C" fn special_hi2_rush_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.battle_object, vars::miiswordsman::instance::SPECIAL_HI2_DASH_HIT);
    smashline::original_status(Pre, fighter, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI2_RUSH)(fighter)
}

unsafe extern "C" fn special_hi2_rush_check_attack(fighter: &mut L2CFighterCommon, param_1: &L2CValue, param_2: &L2CValue) -> L2CValue {
    if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
        VarModule::on_flag(fighter.battle_object, vars::miiswordsman::instance::SPECIAL_HI2_DASH_HIT);
    }

    return 0.into();
}

unsafe extern "C" fn special_hi2_rush_end_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(End, fighter, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI2_RUSH_END)(fighter);
    if VarModule::is_flag(fighter.battle_object, vars::miiswordsman::instance::SPECIAL_HI2_DASH_HIT)
    && !VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK)
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        if fighter.status_frame() >= 18 {
            VarModule::off_flag(fighter.battle_object, vars::miiswordsman::instance::SPECIAL_HI2_DASH_HIT);
            VarModule::on_flag(fighter.battle_object, vars::common::instance::UP_SPECIAL_CANCEL);
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 1.into();
        }
    }

    ret
}

pub unsafe extern "C" fn special_hi2_bound_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("landing_frame"), 0);
    WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI2_RUSH, special_hi2_rush_pre);
    agent.status(CheckAttack, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI2_RUSH, special_hi2_rush_check_attack);

    agent.status(End, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI2_RUSH_END, special_hi2_rush_end_end);

    agent.status(End, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI2_BOUND, special_hi2_bound_end);
}