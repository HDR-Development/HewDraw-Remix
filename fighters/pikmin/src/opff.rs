// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn winged_pikmin_cancel(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, cat1: i32) {
    if [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_PIKMIN_STATUS_KIND_SPECIAL_HI_WAIT].contains(&status_kind) {
        if boma.is_cat_flag(Cat1::SpecialN) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_PIKMIN_STATUS_KIND_SPECIAL_HI_END, false);
        }
        if boma.check_airdodge_cancel() {
            VarModule::on_flag(boma.object(), vars::pikmin::instance::SPECIAL_HI_CANCEL_ESCAPE_AIR);
        }
    }
    if fighter.is_status(*FIGHTER_STATUS_KIND_ESCAPE_AIR)
    && fighter.is_situation(*SITUATION_KIND_AIR)
    && !StatusModule::is_changing(fighter.module_accessor)
    && CancelModule::is_enable_cancel(fighter.module_accessor)
    && VarModule::is_flag(boma.object(), vars::pikmin::instance::SPECIAL_HI_CANCEL_ESCAPE_AIR) {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
        let cancel_module = *(fighter.module_accessor as *mut BattleObjectModuleAccessor as *mut u64).add(0x128 / 8) as *const u64;
        *(((cancel_module as u64) + 0x1c) as *mut bool) = false;  // CancelModule::is_enable_cancel = false
    }
}

pub unsafe fn solimar_scaling(boma: &mut BattleObjectModuleAccessor, status_kind: i32, frame: f32) {
    if StatusModule::is_changing(boma) {
        return;
    }
    let pikmin_count = WorkModule::get_int(boma, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_NUM);
    if pikmin_count == 0 {
        let olimar_hand_scale = Vector3f{x: 1.5, y: 1.35, z: 1.35};
        let olimar_hand_midpoint_scale = Vector3f{x: 1.2, y: 1.17, z: 1.17};
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
            if frame > 5.0 && frame < 16.0 {
                ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("handr"), &olimar_hand_scale);
            } else if frame >= 16.0 && frame < 18.0 {
                ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("handr"), &olimar_hand_midpoint_scale);
            }
        }/* else if status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW4 {
            if frame > 10.0 && frame < 14.0 {
                ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("handr"), &olimar_hand_scale);
                ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("handl"), &olimar_hand_scale);
            } else if frame >= 14.0 && frame < 16.0 {
                ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("handl"), &olimar_hand_midpoint_scale);
                ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("handl"), &olimar_hand_midpoint_scale);
            }
        } else if (status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4 || status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4) {
            if frame > 11.0 && frame < 14.0 {
                ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("handl"), &olimar_hand_scale);
            } else if frame >= 14.0 && frame < 16.0 {
                ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("handl"), &olimar_hand_midpoint_scale);
            }
        }*/
    }
}

#[repr(C)]
struct TroopManager {
  _x0: u64,
  max_pikmin_count: usize, // always 3
  current_pikmin_count: usize,
  pikmin_objects: *mut *mut BattleObject,
  pikmin: [*mut BattleObject; 3],
  // remainder that we don't care about
  // funny blujay made this happen
}

unsafe fn pikmin_antenna_indicator(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_ENTRY
    || sv_information::is_ready_go() {
        let troops = WorkModule::get_int64(fighter.module_accessor, 0x100000C0);
        // 0x100000C0 = FIGHTER_PIKMIN_INSTANCE_WORK_INT_TROOPS_MANAGER_ADDRESS
        let troopmanager = troops as *const TroopManager;
        let count = (*troopmanager).current_pikmin_count;
        let pikmin;
        let pikmin_id;
        if count > 0 {
            pikmin = (*troopmanager).pikmin[0];
            pikmin_id = (*pikmin).battle_object_id;
        }
        else {
            pikmin = std::ptr::null_mut();
            pikmin_id = *BATTLE_OBJECT_ID_INVALID as u32;
        }
        let antenna_eff = WorkModule::get_int(fighter.module_accessor, 0x100000C4) as u32;
        // 0x100000C4 = FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_ANTENNA_EFFECT_HANDLE
        if pikmin_id != *BATTLE_OBJECT_ID_INVALID as u32
        && sv_battle_object::is_active(pikmin_id) {
            let variation = WorkModule::get_int((*pikmin).module_accessor, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
            let r_param;
            let g_param;
            let b_param;
            match variation {
                0 => {
                    r_param = "antenna.light_strong";
                    g_param = "antenna.light_weak";
                    b_param = "antenna.light_weak";
                }, // Red
                1 => {
                    r_param = "antenna.light_strong";
                    g_param = "antenna.light_strong";
                    b_param = "antenna.light_weak";
                }, // Yellow
                2 => {
                    r_param = "antenna.light_weak";
                    g_param = "antenna.light_weak";
                    b_param = "antenna.light_strong";
                }, // Blue
                3 => {
                    r_param = "antenna.light_medium_high";
                    g_param = "antenna.light_medium_high";
                    b_param = "antenna.light_medium_high";
                }, // White
                _ => {
                    r_param = "antenna.light_medium_low";
                    g_param = "antenna.light_weak";
                    b_param = "antenna.light_medium_high";
                }  // Purple
            };
            let r = ParamModule::get_float(fighter.battle_object, ParamType::Agent, r_param);
            let g = ParamModule::get_float(fighter.battle_object, ParamType::Agent, g_param);
            let b = ParamModule::get_float(fighter.battle_object, ParamType::Agent, b_param);
            let alpha = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "antenna.light_bright_alpha");
            EffectModule::set_alpha(fighter.module_accessor, antenna_eff, 1.0);
            EffectModule::set_rgb(fighter.module_accessor, antenna_eff, r, g, b);
        }
        else {
            // No Pikmin, make it transparent and grey.
            let rgb = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "antenna.light_medium_high");
            EffectModule::set_rgb(fighter.module_accessor, antenna_eff, rgb, rgb, rgb);
            let alpha = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "antenna.light_dim_alpha");
            EffectModule::set_alpha(fighter.module_accessor, antenna_eff, 0.05);
        }
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_PIKMIN_STATUS_KIND_SPECIAL_HI_END
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    winged_pikmin_cancel(fighter, boma, status_kind, cat[0]);
    solimar_scaling(boma, status_kind, frame);
    pikmin_antenna_indicator(fighter);
    fastfall_specials(fighter);
}

#[utils::macros::opff(FIGHTER_KIND_PIKMIN )]
pub fn pikmin_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		pikmin_frame(fighter)
    }
}

pub unsafe fn pikmin_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}