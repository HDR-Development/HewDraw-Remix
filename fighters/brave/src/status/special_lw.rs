use super::*;

unsafe extern "C" fn special_lw_start_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Pre, fighter, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_START)(fighter);
    let mut start_turn = *FIGHTER_STATUS_ATTR_START_TURN as u32;
    let facing = PostureModule::lr(fighter.module_accessor);
    let c_stick_override = fighter.is_button_on(Buttons::CStickOverride);
    let cstick = if c_stick_override {
        ControlModule::get_stick_x(fighter.module_accessor)
    } else {
        ControlModule::get_sub_stick_x(fighter.module_accessor)
    };
    if cstick.abs() > 0.2 && facing.signum() != cstick.signum() {
        start_turn = 0;
        PostureModule::reverse_lr(fighter.module_accessor);
    }
    let spell_kind = WorkModule::get_int(fighter.module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_DECIDE_COMMAND);
    let various_kind = WorkModule::get_int(fighter.module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_VARIOUS_KIND);
    let mask = if let Some(target) = smashline::api::get_target_function("lua2cpp_brave.nrs", 0x0398f0) {
        let get_special_lw_mask: fn(&mut L2CValue, L2CValue, L2CValue) = std::mem::transmute(target);
        let mask_l2c = &mut L2CValue::U64(0);
        get_special_lw_mask(mask_l2c, spell_kind.into(), various_kind.into());
        mask_l2c.get_u64()
    }
    else {
        0
    };
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        mask,
        start_turn,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );

    ret
}

unsafe extern "C" fn special_lw_steel_start_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let facing = PostureModule::lr(fighter.module_accessor);
    let c_stick_override = fighter.is_button_on(Buttons::CStickOverride);
    let cstick = if c_stick_override {
        ControlModule::get_stick_x(fighter.module_accessor)
    } else {
        ControlModule::get_sub_stick_x(fighter.module_accessor)
    };
    if cstick.abs() > 0.2 && facing.signum() != cstick.signum() {
        PostureModule::reverse_lr(fighter.module_accessor);
    }
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );

    return 0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_START, special_lw_start_pre);
    agent.status(Pre, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_STEEL_START, special_lw_steel_start_pre);
}