// status imports
use super::*;
use globals::*;

#[skyline::hook(replace = L2CFighterCommon_sub_ftStatusUniqProcessGuardOn_execStatus_common)]
unsafe fn sub_ftStatusUniqProcessGuardOn_execStatus_common(fighter: &mut L2CFighterCommon) {
    super::super::misc::sub_ftStatusUniqProcessGuardFunc_updateShield(fighter, false.into());
}

#[skyline::hook(replace = L2CFighterCommon_sub_ftStatusUniqProcessGuardOn_execStatus)]
unsafe fn ftStatusUniqProcessGuardOn_execStatus(fighter: &mut L2CFighterCommon) -> L2CValue {
    sub_ftStatusUniqProcessGuardOn_execStatus_common(fighter);
    L2CValue::I32(0)
}

#[skyline::hook(replace = L2CFighterCommon_sub_ftStatusUniqProcessGuardOn_execStop)]
pub unsafe fn ftStatusUniqProcessGuardOn_execStop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let shield = WorkModule::get_float(
        fighter.module_accessor,
        *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD
    );
    let scale = super::super::fighter_status_guard
        ::calc_shield_scale(fighter, shield.into())
        .get_f32();
    ModelModule::set_joint_scale(
        fighter.module_accessor,
        Hash40::new("throw"),
        &(Vector3f {
            x: scale,
            y: scale,
            z: scale,
        })
    );
    sub_ftStatusUniqProcessGuardOn_execStop_Inner(
        fighter,
        FIGHTER_STATUS_GUARD_ON_WORK_FLOAT_DELAY_MUL.into()
    );
    L2CValue::I32(0)
}

#[skyline::hook(replace = L2CFighterCommon_sub_ftStatusUniqProcessGuardOn_execStop_Inner)]
pub unsafe fn sub_ftStatusUniqProcessGuardOn_execStop_Inner(
    fighter: &mut L2CFighterCommon,
    arg: L2CValue
) {
    if super::super::fighter_status_guard::check_hit_stop_delay_flick(fighter, arg).get_bool() {
        let clears = [
            *FIGHTER_PAD_CMD_CAT1_ESCAPE,
            *FIGHTER_PAD_CMD_CAT1_ESCAPE_F,
            *FIGHTER_PAD_CMD_CAT1_ESCAPE_B,
            *FIGHTER_PAD_CMD_CAT1_DASH,
            *FIGHTER_PAD_CMD_CAT1_TURN_DASH,
        ];
        for x in clears.iter() {
            ControlModule::clear_command_one(
                fighter.module_accessor,
                *FIGHTER_PAD_COMMAND_CATEGORY1,
                *x
            );
        }
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
        fighter.global_table[CMD_CAT1].assign(&L2CValue::I32(cat1));
    }
}

pub fn install() {
    skyline::install_hooks!(
        sub_ftStatusUniqProcessGuardOn_execStatus_common,
        sub_ftStatusUniqProcessGuardOn_execStop_Inner,
        ftStatusUniqProcessGuardOn_execStatus,
        ftStatusUniqProcessGuardOn_execStop
    );
}
