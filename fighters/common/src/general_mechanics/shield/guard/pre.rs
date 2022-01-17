// status imports
use super::*;
use globals::*;

#[common_status_script(status = FIGHTER_STATUS_KIND_GUARD, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE,
    symbol = "_ZN7lua2cpp16L2CFighterCommon16status_pre_GuardEv")]
unsafe fn status_pre_Guard(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_MOTION,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_GUARD_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_GUARD_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_GUARD_FLOAT,
        *FS_SUCCEEDS_KEEP_VISIBILITY
    );
    
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        0,
        *FIGHTER_STATUS_ATTR_DISABLE_SHIELD_RECOVERY as u32,
        0,
        0
    );
    
    L2CValue::I32(0)
}

pub fn install() {
    install_status_scripts!(
        status_pre_Guard
    );
}