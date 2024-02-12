use super::*;
use globals::*;
use utils::consts::vars::edge;

#[status_script(agent = "edge", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(Hash40::new("special_n_start").into(), Hash40::new("special_air_n_start").into(), false.into());
    fighter.sub_set_special_start_common_kinetic_setting(hash40("param_special_n").into());
    special_hi_set_kinetics(fighter, true);
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_EDGE_SPECIAL_N_NONE, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CHARGE_KIND);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_N_FLAG_REQUEST_SHOOT);
    WorkModule::set_int(fighter.module_accessor, -1, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_GUARD);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
    ControlModule::set_add_jump_mini_button_life(fighter.module_accessor, 8);
    fighter.main_shift(special_n_main_loop)
}

unsafe extern "C" fn special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(fighter.module_accessor) {
        fighter.sub_change_motion_by_situation(Hash40::new("special_n_start").into(), Hash40::new("special_air_n_start").into(), true.into());
        fighter.sub_exec_special_start_common_kinetic_setting(hash40("param_special_n").into());
        special_hi_set_kinetics(fighter, false);
    }
    if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_N_FLAG_REQUEST_SHOOT);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_N_FLAG_REQUEST_SHOOT) {
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CHARGE_KIND) != *FIGHTER_EDGE_SPECIAL_N_NONE {
            fighter.change_status(FIGHTER_EDGE_STATUS_KIND_SPECIAL_N_SHOOT.into(), false.into());
            return 0.into()
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        let status = if fighter.is_situation(*SITUATION_KIND_GROUND) { *FIGHTER_STATUS_KIND_WAIT } else { *FIGHTER_STATUS_KIND_FALL };
        fighter.change_status(status.into(), false.into());
    }
    let cancel_start_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("cancel_start_frame"));
    if cancel_start_frame > fighter.status_frame() {
        if fighter.is_situation(*SITUATION_KIND_AIR) {
            let mut speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            let unk1 = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), 0x18ecc76f9d);
            if unk1 < speed_y {
                speed_y = unk1;
            }
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed_y);
        }
    }
    else {
        if fighter.sub_check_command_guard().get_bool() {
            if fighter.is_situation(*SITUATION_KIND_GROUND) {
                WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS);
                fighter.change_status(FIGHTER_EDGE_STATUS_KIND_SPECIAL_N_CANCEL.into(), false.into());
                return 1.into()
            }
            else {
                if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR) {
                    WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS);
                    fighter.change_status(FIGHTER_EDGE_STATUS_KIND_SPECIAL_N_CANCEL.into(), false.into());
                    return 1.into()
                }
            }
        }
        else {
            if fighter.is_situation(*SITUATION_KIND_AIR) {
                if fighter.sub_check_jump_in_charging().get_bool() {
                    WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_AERIAL, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS);
                    fighter.change_status(FIGHTER_EDGE_STATUS_KIND_SPECIAL_N_CANCEL.into(), true.into());
                }
            }
            else {
                if (fighter.is_cat_flag(Cat1::JumpButton) || (ControlModule::is_enable_flick_jump(fighter.module_accessor) && fighter.is_cat_flag(Cat1::Jump) && fighter.sub_check_button_frick().get_bool())) {
                    WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS);
                    fighter.change_status(FIGHTER_EDGE_STATUS_KIND_SPECIAL_N_CANCEL.into(), true.into());
                }
                else {
                    if fighter.is_cat_flag(Cat2::StickEscapeF) {
                        WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_F, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS);
                        fighter.change_status(FIGHTER_EDGE_STATUS_KIND_SPECIAL_N_CANCEL.into(), true.into());
                    }
                    else if fighter.is_cat_flag(Cat2::StickEscapeB) {
                        WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_B, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS);
                        fighter.change_status(FIGHTER_EDGE_STATUS_KIND_SPECIAL_N_CANCEL.into(), true.into());
                    }
                }
            }
        }
    }

    return 0.into()
}

unsafe extern "C" fn special_hi_set_kinetics(fighter: &mut L2CFighterCommon, param_1: bool) {
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        if !param_1 && fighter.is_prev_situation(*SITUATION_KIND_AIR) {
            return;
        }
        sv_kinetic_energy!(set_needs_set_param, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, false);
        if KineticModule::is_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL) {
            sv_kinetic_energy!(set_needs_set_param, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, false);
        }
    }
}

pub fn install() {
    install_status_scripts!(
        special_n_main,
    );
}