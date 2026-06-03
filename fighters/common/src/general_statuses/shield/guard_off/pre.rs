// status imports
use super::*;
use globals::*;

#[skyline::hook(replace = L2CFighterCommon_status_pre_GuardOff)]
unsafe fn status_pre_GuardOff(fighter: &mut L2CFighterCommon) -> L2CValue {
    // enable rolls in RoA mode
    if utils::game_modes::check_custom_mode(game_modes::CustomMode::RivalsOfAetherMode) {
        let stick_x = if fighter.is_button_on(Buttons::CStickOn) {
            fighter.right_stick_x()
        } else {
            fighter.left_stick_x()
        };
        let stick_x = stick_x * PostureModule::lr(fighter.module_accessor);
        let stick_y = fighter.stick_y();
        let stick_vertical = stick_y.abs() >= stick_x.abs() && stick_y < 0.0;
        if !stick_vertical && stick_x >= 0.4 {
            VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_PARRY_FOR_GUARD_OFF);
            fighter.set_status_kind_interrupt(*FIGHTER_STATUS_KIND_ESCAPE_F);
            return true.into();
        }
        if !stick_vertical && stick_x <= -0.4 {
            VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_PARRY_FOR_GUARD_OFF);
            fighter.set_status_kind_interrupt(*FIGHTER_STATUS_KIND_ESCAPE_B);
            return true.into();
        }
    }

    // vanilla
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_MOTION,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_GUARD_OFF_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_GUARD_OFF_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_GUARD_OFF_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        true,
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
    skyline::install_hook!(status_pre_GuardOff);
}
