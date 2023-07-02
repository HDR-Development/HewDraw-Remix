// status imports
use super::*;
use globals::*;

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon48sub_ftStatusUniqProcessGuardOn_execStatus_commonEv")]
unsafe fn sub_ftStatusUniqProcessGuardOn_execStatus_common(fighter: &mut L2CFighterCommon) {
    super::super::misc::sub_ftStatusUniqProcessGuardFunc_updateShield(fighter, false.into());
}

#[common_status_script(status = FIGHTER_STATUS_KIND_GUARD_ON, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS,
    symbol = "_ZN7lua2cpp16L2CFighterCommon41sub_ftStatusUniqProcessGuardOn_execStatusEv")]
unsafe fn ftStatusUniqProcessGuardOn_execStatus(fighter: &mut L2CFighterCommon) -> L2CValue {
    sub_ftStatusUniqProcessGuardOn_execStatus_common(fighter);
    L2CValue::I32(0)
}

#[common_status_script(status = FIGHTER_STATUS_KIND_GUARD_ON, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STOP, 
    symbol = "_ZN7lua2cpp16L2CFighterCommon39sub_ftStatusUniqProcessGuardOn_execStopEv")]
pub unsafe fn ftStatusUniqProcessGuardOn_execStop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let shield = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
    let scale = super::super::fighter_status_guard::calc_shield_scale(fighter, shield.into()).get_f32();
    ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("throw"), &Vector3f{ x: scale, y: scale, z: scale });
    sub_ftStatusUniqProcessGuardOn_execStop_Inner(fighter, FIGHTER_STATUS_GUARD_ON_WORK_FLOAT_DELAY_MUL.into());
    L2CValue::I32(0)
}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon45sub_ftStatusUniqProcessGuardOn_execStop_InnerEN3lib8L2CValueE")]
pub unsafe fn sub_ftStatusUniqProcessGuardOn_execStop_Inner(fighter: &mut L2CFighterCommon, arg: L2CValue) {
    if super::super::fighter_status_guard::check_hit_stop_delay_flick(fighter, arg).get_bool() {
        let clears = [
            *FIGHTER_PAD_CMD_CAT1_ESCAPE,
            *FIGHTER_PAD_CMD_CAT1_ESCAPE_F,
            *FIGHTER_PAD_CMD_CAT1_ESCAPE_B,
            *FIGHTER_PAD_CMD_CAT1_DASH,
            *FIGHTER_PAD_CMD_CAT1_TURN_DASH,
        ];
        for x in clears.iter() {
            ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *x);
        }
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
        fighter.global_table[CMD_CAT1].assign(&L2CValue::I32(cat1));
    }
}

pub fn install() {
    install_status_scripts!(
        ftStatusUniqProcessGuardOn_execStatus,
        ftStatusUniqProcessGuardOn_execStop
    );

    install_hooks!(
        sub_ftStatusUniqProcessGuardOn_execStatus_common,
        sub_ftStatusUniqProcessGuardOn_execStop_Inner
    );
}