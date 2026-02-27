// status imports
use super::*;
use globals::*;
// This file contains code related to teching

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_check_passive_button_for_damage,
            status_pre_passive,
            status_Passive_Main,
            status_pre_passivefb,
            status_PassiveFB_Main
        );
    }
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_check_passive_button_for_damage)]
pub unsafe fn sub_check_passive_button_for_damage(fighter: &mut L2CFighterCommon, trigger_frame: L2CValue) -> L2CValue {
    let is_valid_tech_input = fighter.sub_check_passive_button(trigger_frame).get_bool();
    return L2CValue::Bool(is_valid_tech_input)
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_pre_Passive)]
pub unsafe fn status_pre_passive(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_GROUND_STOP, // Originally *FIGHTER_KINETIC_TYPE_PASSIVE_GROUND
        *GROUND_CORRECT_KIND_GROUND as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        true,
        *FIGHTER_TREADED_KIND_ENABLE,
        false,
        false,
        false,
        0,
        *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE as u32,
        0,
        0
    );
    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_Passive_Main)]
pub unsafe fn status_Passive_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor, Hash40::new_raw(motion_kind), true);
    let end_frame = MotionModule::end_frame(fighter.module_accessor);
    if cancel_frame > end_frame {
        if StatusModule::is_changing(fighter.module_accessor) {
            let mut motion_rate = end_frame / cancel_frame;
            if motion_rate < 1.0 {
                motion_rate += 0.001;
            }
            MotionModule::set_rate(fighter.module_accessor, motion_rate);
            MotionModule::set_whole_rate(fighter.module_accessor, 1.0);
        }
        
        let xlu_end_frame = FighterMotionModuleImpl::get_hit_normal_frame(fighter.module_accessor, Hash40::new_raw(motion_kind), true);
        if fighter.global_table[CURRENT_FRAME].get_f32() == xlu_end_frame {
            HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        }
    }
    
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL, true);
        return 0.into();
    }
    
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return 0.into();
    }
    
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_WAIT, false);
        return 0.into();
    }

    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_pre_PassiveFB)]
pub unsafe fn status_pre_passivefb(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_MOTION, // Originally *FIGHTER_KINETIC_TYPE_PASSIVE_GROUND_MOTION
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        false,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        true,
        *FIGHTER_TREADED_KIND_ENABLE,
        false,
        false,
        false,
        0,
        (
            *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE
        ) as u32,
        0,
        0
    );
    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_PassiveFB_Main)]
pub unsafe fn status_PassiveFB_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor, Hash40::new_raw(motion_kind), true);
    let end_frame = MotionModule::end_frame(fighter.module_accessor);
    if cancel_frame > end_frame {
        if StatusModule::is_changing(fighter.module_accessor) {
            let mut motion_rate = end_frame / cancel_frame;
            if motion_rate < 1.0 {
                motion_rate += 0.001;
            }
            MotionModule::set_rate(fighter.module_accessor, motion_rate);
            MotionModule::set_whole_rate(fighter.module_accessor, 1.0);
        }
        
        let xlu_end_frame = FighterMotionModuleImpl::get_hit_normal_frame(fighter.module_accessor, Hash40::new_raw(motion_kind), true);
        if fighter.global_table[CURRENT_FRAME].get_f32() == xlu_end_frame {
            HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        }
    }

    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL, true);
        return 0.into();
    }
    
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    || MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_WAIT, false);
        return 0.into();
    }

    0.into()
}