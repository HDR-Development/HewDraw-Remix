use super::*;
use globals::*;
use smashline::*;

pub fn install() {
  install_status_scripts!(
      pre_special_hi,
      pre_special_hi_jump
  );
}

#[status_script(agent = "sonic", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn pre_special_hi(fighter: &mut L2CFighterCommon) -> L2CValue {
	StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_SONIC_SPECIAL_HI_COMP,
        *GROUND_CORRECT_KIND_NONE as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_SONIC_SPECIAL_HI_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_SONIC_SPECIAL_HI_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_SONIC_SPECIAL_HI_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP
        ) as u64,
        (
            *FIGHTER_STATUS_ATTR_START_TURN |
            *FIGHTER_STATUS_ATTR_DISABLE_INTERRUPT_WARP
        ) as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    0.into()
}
#[status_script(agent = "sonic", status = FIGHTER_SONIC_STATUS_KIND_SPECIAL_HI_JUMP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn pre_special_hi_jump(fighter: &mut L2CFighterCommon) -> L2CValue {
	StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_SONIC_SPECIAL_HI_JUMP,
        *GROUND_CORRECT_KIND_NONE as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_SONIC_SPECIAL_HI_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_SONIC_SPECIAL_HI_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_SONIC_SPECIAL_HI_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        0,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    0.into()
}