use super::*;

#[no_mangle]
unsafe fn attack_air_float_pre(fighter: &mut L2CFighterCommon, float_status: L2CValue) -> L2CValue {
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    let keep = if prev_status_kind == float_status.get_i32() {
        (
            *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
            *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
            *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT
        )
    }
    else {
        (
            *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_AIR_FLAG,
            *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_AIR_INT,
            *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_AIR_FLOAT
        )
    };
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_MOTION_FALL,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        keep.0,
        keep.1,
        keep.2,
        *FS_SUCCEEDS_KEEP_VISIBILITY | *FS_SUCCEEDS_KEEP_ATTACK | *FS_SUCCEEDS_KEEP_EFFECT
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP as u64,
        *FIGHTER_STATUS_ATTR_CLEAR_MOTION_ENERGY as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_AIR as u32,
        0
    );
    0.into()
}

#[no_mangle]
unsafe fn attack_air_float_main(fighter: &mut L2CFighterCommon, float_status: L2CValue) -> L2CValue {
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    if prev_status_kind != float_status.get_i32() {
        return smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_ATTACK_AIR)(fighter);
    }

    let motion = MotionModule::motion_kind(fighter.module_accessor);

    let log = match motion {
        0xc3a4e2597 => Some(FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_N),
        0xc3495ada5 => Some(FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_F),
        0xc33f869bc => Some(FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_B),
        0xdde67d935 => Some(FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_HI),
        0xd40042152 => Some(FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_LW),
        _ => None
    };

    if let Some(log) = log {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b94de0d96), FIGHTER_LOG_ACTION_CATEGORY_KEEP, log);
    }

    // fighter.status_AttackAir_Main_common();
    WorkModule::set_int64(fighter.module_accessor, motion as i64, *FIGHTER_STATUS_ATTACK_AIR_WORK_INT_MOTION_KIND);

    // allow fast fall during float release aerials
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_fall_common_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_sub_fall_common_uniq as *const () as _));

    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_status_AttackAir_Main as *const () as _))
}

#[skyline::hook(replace = L2CFighterCommon_status_AttackAir_Main_common)]
unsafe extern "C" fn status_attackair_main_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !CancelModule::is_enable_cancel(fighter.module_accessor)
    && VarModule::is_flag(fighter.battle_object, vars::common::instance::OMNI_FLOAT)
    && !VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_FLOAT)
    && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
        let mut dive_cont_value = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("dive_cont_value"));
        let mut dive_flick_frame_value = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("dive_flick_frame_value"));
        if fighter.left_stick_y() <= dive_cont_value
        && VarModule::get_int(fighter.battle_object, vars::common::instance::LEFT_STICK_FLICK_Y) < dive_flick_frame_value {
            let status_kind = VarModule::get_int(fighter.battle_object, vars::common::instance::FLOAT_STATUS_KIND);
            if status_kind != 0 {
                VarModule::on_flag(fighter.battle_object, vars::common::status::FLOAT_INHERIT_AERIAL);
                fighter.change_status(status_kind.into(), true.into());
                return 0.into();
            }
        }
    }
    original!()(fighter)
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_attackair_main_common
        );
    }
}
pub fn install() {
    skyline::nro::add_hook(nro_hook);
}