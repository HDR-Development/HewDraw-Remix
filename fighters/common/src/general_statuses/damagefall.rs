// status imports
use super::*;
use globals::*;
// This file contains code for tumble

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_DamageFall_Main
        );
    }
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_DamageFall_Main)]
unsafe fn status_DamageFall_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut should_tech = false;
    let stick_x = fighter.global_table[STICK_X].get_f32();
    let passive_fb_value = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("passive_fb_cont_value"));
    let trigger_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("air_escape_passive_trigger_frame"));

    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 0.into();
    }
    if fighter.check_damage_fall_transition().get_bool() {
        return 0.into();
    }
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_GROUND) {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_AIR) {
            should_tech = fighter.sub_check_passive_button(L2CValue::I32(trigger_frame)).get_bool();
        }
        else {
            should_tech = false;
        }
    }
    else {
        let ganon_special_s_passive_trigger_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("ganon_special_s_passive_trigger_frame"));

        should_tech = fighter.sub_check_passive_button(L2CValue::I32(ganon_special_s_passive_trigger_frame)).get_bool();
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_FB)
    && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND
    && app::FighterUtil::is_touch_passive_ground(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32)
    && should_tech
    && passive_fb_value <= stick_x.abs()
    && !FighterStopModuleImpl::is_damage_stop(fighter.module_accessor) {
        fighter.change_status(
            L2CValue::I32(*FIGHTER_STATUS_KIND_PASSIVE_FB),
            L2CValue::Bool(true)
        );
        return true.into();
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE)
    && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND
    && app::FighterUtil::is_touch_passive_ground(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32)
    && should_tech
    && !FighterStopModuleImpl::is_damage_stop(fighter.module_accessor) {
        fighter.change_status(
            L2CValue::I32(*FIGHTER_STATUS_KIND_PASSIVE),
            L2CValue::Bool(true)
        );
        return true.into();
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DOWN)
    && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
        fighter.change_status(
            L2CValue::I32(*FIGHTER_STATUS_KIND_DOWN),
            L2CValue::Bool(false)
        );
        return 0.into();
    }
    fighter.sub_damage_fall_uniq_process_exec_fix_pos();
    return 0.into()
}