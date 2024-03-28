use super::*;

// FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI2_RUSH

unsafe extern "C" fn special_hi2_rush_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.battle_object, vars::miiswordsman::instance::SKYWARD_SLASH_DASH_HIT);
    smashline::original_status(Pre, fighter, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI2_RUSH)(fighter)
}

// not running for some reason

unsafe extern "C" fn special_hi2_rush_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
        VarModule::on_flag(fighter.battle_object, vars::miiswordsman::instance::SKYWARD_SLASH_DASH_HIT);
        //println!("SSD Hit");
    }
    return 0.into()
}

// FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI2_RUSH_END
// not running for some reason

unsafe extern "C" fn special_hi2_rush_end_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, vars::miiswordsman::instance::SKYWARD_SLASH_DASH_HIT) && !VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        //println!("SSD Success");
        if MotionModule::frame(fighter.module_accessor) >= 30.0 {
            //println!("SSD Fall Act");
            VarModule::off_flag(fighter.battle_object, vars::miiswordsman::instance::SKYWARD_SLASH_DASH_HIT);
            VarModule::on_flag(fighter.battle_object, vars::common::instance::UP_SPECIAL_CANCEL);
            fighter.change_status(
                L2CValue::I32(*FIGHTER_STATUS_KIND_FALL),
                L2CValue::Bool(false)
            );
            return 1.into()
        }
    }
    return 0.into()
}

// FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI2_BOUND

pub unsafe extern "C" fn special_hi2_bound_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("landing_frame"), 0);
    WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI2_RUSH, special_hi2_rush_pre);
    agent.status(Exec, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI2_RUSH, special_hi2_rush_exec);
    agent.status(Exec, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI2_RUSH_END, special_hi2_rush_end_exec);

    agent.status(End, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI2_BOUND, special_hi2_bound_end);
}