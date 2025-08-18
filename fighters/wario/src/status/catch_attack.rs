use super::*;

unsafe extern "C" fn catch_attack_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mask_flag = if VarModule::is_flag(fighter.battle_object, vars::wario::instance::PUMMEL_SKIP_STALE) {
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_CATCH_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64
    } else {
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_CATCH_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64
    };
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_MOTION,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        *FS_SUCCEEDS_KEEP_ATTACK_ABSOLUTE
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        true,
        false,
        mask_flag,
        (*FIGHTER_STATUS_ATTR_DISABLE_JUMP_BOARD_EFFECT | *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE) as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_CATCH as u32,
        0
    );
    
    return 0.into();
}

// Force opponent rotation
unsafe extern "C" fn catch_attack_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.boma();
    let mut vec =Vector3f{x: 0.0, y: 0.0, z: 0.0};
    let offset = ModelModule::joint_global_rotation(fighter.module_accessor,Hash40::new("throw"),&mut vec,false);
    let rot = Vector3f{x: vec.x, y: 0.0, z: 0.0};
    PostureModule::set_rot(boma.get_grabbed_opponent_boma(), &rot, 0);
    return false.into();
}

// Reset opponent rotation

unsafe extern "C" fn catch_attack_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.boma();
    PostureModule::set_rot(boma.get_grabbed_opponent_boma(), &Vector3f::zero(), 0);

    smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_CATCH_ATTACK)(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_CATCH_ATTACK, catch_attack_pre);
    agent.status(Exec, *FIGHTER_STATUS_KIND_CATCH_ATTACK, catch_attack_exec);
    agent.status(End, *FIGHTER_STATUS_KIND_CATCH_ATTACK, catch_attack_end);
}