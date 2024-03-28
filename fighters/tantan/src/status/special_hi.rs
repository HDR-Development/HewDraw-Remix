use super::*;

// up special

unsafe extern "C" fn tantan_special_hi_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if (fighter.motion_frame() <= 30.0)
    && !fighter.is_button_on(Buttons::Special)
    {
        fighter.change_status_req(*FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_AIR, false);
        return 0.into();
    }
    return smashline::original_status(Exec, fighter, *FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_GROUND)(fighter);
}

unsafe extern "C" fn tantan_special_hi_air_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_prev_status(*FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_GROUND){
        StatusModule::init_settings(
            fighter.module_accessor,
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
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI as u64,
            0,
            *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
            0
        );
        return 0.into();
    }
    else
    {
        return smashline::original_status(Pre, fighter, *FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_AIR)(fighter);
    }
}

unsafe extern "C" fn tantan_special_hi_air_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick = Vector2f::new(
        fighter.stick_x(),
        fighter.stick_y()        
    );
    let angle = ((stick.x)*-10.0*PostureModule::lr(fighter.module_accessor))-5.0;
    WorkModule::set_float(fighter.module_accessor, angle, *FIGHTER_TANTAN_INSTANCE_WORK_ID_FLOAT_ATTACK_SHIFT_ANGLE_L);
    if (fighter.motion_frame()>=10.0)
    {
        fighter.set_joint_rotate("claviclel", Vector3f::new(0.0, angle, 0.0));
    }
    return 0.into();
}

unsafe extern "C" fn tantan_special_hi_air_reach_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let angle = WorkModule::get_float(fighter.module_accessor,*FIGHTER_TANTAN_INSTANCE_WORK_ID_FLOAT_ATTACK_SHIFT_ANGLE_L);
    fighter.set_joint_rotate("claviclel", Vector3f::new(0.0, angle, 0.0));
    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Exec,*FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_GROUND,tantan_special_hi_exec,);
    agent.status(Pre,*FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_AIR,tantan_special_hi_air_pre,);
    agent.status(Exec,*FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_AIR,tantan_special_hi_air_exec);
    agent.status(Exec,*FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_AIR_REACH,tantan_special_hi_air_reach_exec);
}
