use super::*;

// This file contains code for dash item throw

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            ItemThrowDashUniq,
            status_ItemThrowDash_Main,
            status_end_ItemThrowDash,
            bind_address_call_status_end_ItemThrowDash
        );
    }
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_ItemThrowDashUniq)]
unsafe fn ItemThrowDashUniq(fighter: &mut L2CFighterCommon, arg2: L2CValue) -> L2CValue {
    if !arg2.get_bool() {
        return 0.into();
    }

    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_STATUS_ITEM_THROW_WORK_INT_FRAME);

    // We check the current frame of the dash item throw (and add one because it our range is [min, max))
    // and if it is in the range we enable the transition terms (enabling DITCIT)
    // If we are outside of that range we just disable them

    // Add one because it is 0 based
    let current_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ITEM_THROW_WORK_INT_FRAME) + 1;
    let start_frame = ParamModule::get_int(fighter.battle_object, ParamType::Common, "ditcit.start_frame");
    let end_frame = ParamModule::get_int(fighter.battle_object, ParamType::Common, "ditcit.end_frame");
    if current_frame >= start_frame && current_frame < end_frame {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW);
    } 
    else {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW);
    }

    0.into()
}

unsafe fn is_catch_input(fighter: &mut L2CFighterCommon) -> bool {
    let cat1 = fighter.global_table[CMD_CAT1].get_i32();
    cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0 || ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_CATCH)
}

unsafe fn is_attackhi4_for_ditcit_input(fighter: &mut L2CFighterCommon) -> bool {
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    let cat1 = fighter.global_table[CMD_CAT1].get_i32();
    let pad_flag = fighter.global_table[PAD_FLAG].get_i32();

    let is_catch = is_catch_input(fighter);
    let is_tilt_input = !StatusModule::is_changing(fighter.module_accessor)
                            && cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3 != 0
                            && pad_flag & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0;

    return (stick_y >= 0.7 && is_catch) || is_tilt_input || cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 != 0
}

unsafe fn is_attacklw4_for_ditcit_input(fighter: &mut L2CFighterCommon) -> bool {
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    let cat1 = fighter.global_table[CMD_CAT1].get_i32();
    let pad_flag = fighter.global_table[PAD_FLAG].get_i32();

    let is_catch = is_catch_input(fighter);
    let is_tilt_input = !StatusModule::is_changing(fighter.module_accessor)
                            && cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3 != 0
                            && pad_flag & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0;

    return (stick_y <= -0.7 && is_catch) || is_tilt_input || cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4 != 0
}

unsafe fn is_attacks4_for_ditcit_input(fighter: &mut L2CFighterCommon) -> bool {
    let cat1 = fighter.global_table[CMD_CAT1].get_i32();
    let pad_flag = fighter.global_table[PAD_FLAG].get_i32();

    let is_tilt_input = !StatusModule::is_changing(fighter.module_accessor)
                            && cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3 != 0
                            && pad_flag & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0;

    return is_tilt_input || cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4 != 0
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_ItemThrowDash_Main)]
unsafe fn status_ItemThrowDash_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_changing(fighter.module_accessor) {
        VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_DITCIT);
    }

    if !CancelModule::is_enable_cancel(fighter.module_accessor)
    || (!fighter.sub_wait_ground_check_common(L2CValue::Bool(false)).get_bool() && !fighter.sub_air_check_fall_common().get_bool()) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ITEM_THROW_WORK_FLAG_KILLER) {
            fighter.change_status(FIGHTER_STATUS_KIND_KILLER.into(), false.into());
            return 0.into();
        }

        if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 0.into();
        }

        if MotionModule::is_end(fighter.module_accessor)
        && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            return 0.into();
        }

        // DITCIT
        if !StatusModule::is_changing(fighter.module_accessor)
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW)
        && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
            if is_attackhi4_for_ditcit_input(fighter)
            || is_attacklw4_for_ditcit_input(fighter)
            || is_attacks4_for_ditcit_input(fighter) {
                VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_DITCIT);
                fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), false.into());
                return 0.into();
            }
        }
    }

    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_end_ItemThrowDash)]
unsafe fn status_end_ItemThrowDash(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_bind_address_call_status_end_ItemThrowDash)]
unsafe fn bind_address_call_status_end_ItemThrowDash(fighter: &mut L2CFighterCommon, _agent: &mut L2CAgent) -> L2CValue {
    fighter.status_end_ItemThrowDash();
    0.into()
}