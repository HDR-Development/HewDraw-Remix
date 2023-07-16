// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn charge_handle(fighter: &mut L2CFighterCommon) {
    if fighter.is_motion_one_of(&[
        Hash40::new("attack_air_lw"),
        Hash40::new("special_n3_start"),
        Hash40::new("special_air_n3_start"),
        Hash40::new("special_hi1"),
        Hash40::new("special_air_hi1")]) {
        let is_hold =
            if fighter.is_motion(Hash40::new("attack_air_lw")) {
                ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
            } else {
                ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
            };
        let charge = VarModule::get_float(fighter.battle_object, vars::miigunner::status::CURRENT_CHARGE);
        let mut charge_start_frame = 0.0;
        let mut charge_end_frame = 0.0;
        let mut max_charge_frames = 0.0;

        match MotionModule::motion_kind(fighter.module_accessor) {
            _ if fighter.is_motion(Hash40::new("attack_air_lw")) => {
                charge_start_frame = 10.0;
                charge_end_frame = 14.0;
                max_charge_frames = 20.0;
            },
            _ if fighter.is_motion_one_of(&[Hash40::new("special_n3_start"), Hash40::new("special_air_n3_start")]) => {
                charge_start_frame = 10.0;
                charge_end_frame = 15.0;
                max_charge_frames = 20.0;
            },
            _ if fighter.is_motion_one_of(&[Hash40::new("special_hi1"), Hash40::new("special_air_hi1")]) => {
                charge_start_frame = 6.0;
                charge_end_frame = 8.0;
                max_charge_frames = 20.0;
            },
            _ => {}
        }

        if (charge_start_frame..charge_end_frame).contains(&fighter.motion_frame()) && charge < max_charge_frames && is_hold {
            if fighter.is_motion(Hash40::new("attack_air_lw")) {
                let handle = VarModule::get_int64(fighter.battle_object, vars::miigunner::instance::LUNAR_LAUNCH_EFF_HANDLER);
                EffectModule::set_scale(fighter.module_accessor, handle as u32, &Vector3f::new(0.75 + 0.018 * charge, 0.75 + 0.018 * charge, 0.75 + 0.018 * charge));
            }
            else if fighter.is_motion_one_of(&[Hash40::new("special_hi1"), Hash40::new("special_air_hi1")]) {
                let motion_vec = if charge <= 10.0 { Vector3f{ x: 1.0, y: 0.55, z: 1.0 } } else { Vector3f{ x: 1.0, y: 0.35, z: 1.0 } };
                KineticModule::mul_speed(fighter.module_accessor, &motion_vec, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                let handle = VarModule::get_int64(fighter.battle_object, vars::miigunner::instance::LUNAR_LAUNCH_EFF_HANDLER);
                EffectModule::set_rate(fighter.module_accessor, handle as u32, 1.0/fighter.motion_frame());
            }
            let motion_rate = (charge_end_frame - charge_start_frame)/max_charge_frames;
            MotionModule::set_rate(fighter.module_accessor, motion_rate);
            VarModule::set_float(fighter.battle_object, vars::miigunner::status::CURRENT_CHARGE, charge + 1.0);
        }
        else {
            VarModule::on_flag(fighter.battle_object, vars::miigunner::status::IS_CHARGE_FINISHED);
            if fighter.is_motion(Hash40::new("attack_air_lw")) {
                let handle = VarModule::get_int64(fighter.battle_object, vars::miigunner::instance::LUNAR_LAUNCH_EFF_HANDLER);
                EffectModule::set_rate(fighter.module_accessor, handle as u32, 1.0);
                MotionModule::set_rate(fighter.module_accessor, 1.0);
            }
            else if fighter.is_motion_one_of(&[Hash40::new("special_n3_start"), Hash40::new("special_air_n3_start")]) {
                VarModule::set_float(fighter.battle_object, vars::miigunner::instance::GRENADE_CHARGE, charge);
                MotionModule::set_rate(fighter.module_accessor, 1.0);
            }
            else if fighter.is_motion_one_of(&[Hash40::new("special_hi1"), Hash40::new("special_air_hi1")]) {
                let handle = VarModule::get_int64(fighter.battle_object, vars::miigunner::instance::LUNAR_LAUNCH_EFF_HANDLER);
                EffectModule::set_rate(fighter.module_accessor, handle as u32, 1.0);
                MotionModule::set_rate(fighter.module_accessor, 1.0);
            }
        }
    }
}
 
unsafe fn nspecial_cancels(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status(*FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N1_CANCEL) {
        if boma.is_situation(*SITUATION_KIND_AIR) {
            if WorkModule::get_int(boma, *FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_WORK_INT_CANCEL_STATUS) == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
                WorkModule::set_int(boma, *STATUS_KIND_NONE, *FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_WORK_INT_CANCEL_STATUS);
            }
        }
    }
}

unsafe fn reflector_jc(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status_one_of(&[
        *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW1_HIT,
        *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW1_END,
        *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW1_LOOP]) {
        if !boma.is_in_hitlag() {
            if (boma.is_status(*FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW3_HOLD) && boma.status_frame() > 3)
                || !boma.is_status(*FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW3_HOLD)
            {
                boma.check_jump_cancel(false);
            }
        }
        if boma.is_status(*FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW3_HOLD) {
            if PostureModule::lr(boma) * ControlModule::get_stick_x(boma) < 0.0 {
                PostureModule::reverse_lr(boma);
                PostureModule::update_rot_y_lr(boma);
            }
        }
    }
}

unsafe fn laser_blaze_ff_land_cancel(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_motion_one_of(&[
        Hash40::new("special_air_n2_start"),
        Hash40::new("special_air_n2_loop"),
        Hash40::new("special_air_n2_end"),
        Hash40::new("special_n2_start"),
        Hash40::new("special_n2_loop"),
        Hash40::new("special_n2_end") ]) {
        if boma.is_situation(*SITUATION_KIND_GROUND) && boma.is_prev_situation(*SITUATION_KIND_AIR) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, false);
        }
        if boma.is_situation(*SITUATION_KIND_AIR) {
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
            if boma.is_cat_flag(Cat2::FallJump) && ControlModule::get_stick_y(boma) < -0.66
                && KineticModule::get_sum_speed_y(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
                WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
            }
        }
    }
}

unsafe fn remove_homing_missiles(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status(*FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_1_GROUND) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_2_GROUND, false);
    }
    else if boma.is_status(*FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_1_AIR) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_2_AIR, false);
    }
}

unsafe fn missile_land_cancel(boma: &mut BattleObjectModuleAccessor) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if boma.is_status_one_of(&[
        *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_1_AIR,
        *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_2_AIR ]) {
        if boma.is_situation(*SITUATION_KIND_GROUND) && boma.is_prev_situation(*SITUATION_KIND_AIR) {
            if boma.status_frame() > 23 {
                MotionModule::set_frame(boma, 40.0, false);
            }
        }
    }
}

unsafe fn arm_rocket_airdash(fighter: &mut L2CFighterCommon) {
    if StatusModule::is_changing(fighter.module_accessor) {
        return;
    }
    if fighter.is_status(*FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_HI3_RUSH) && fighter.status_frame() > 16 {
        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_HI3_RUSH_END, false);
    }
    if fighter.is_status(*FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_HI3_RUSH_END) && fighter.status_frame() > 11 {
        VarModule::on_flag(fighter.battle_object, vars::common::instance::UP_SPECIAL_CANCEL);
        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
    }
}

/// Allow uncharged or slightly charged Lunar Launch to be actionable
unsafe fn lunar_launch_actionability(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI) {
        if fighter.status_frame() >= 35 && VarModule::get_float(fighter.battle_object, vars::miigunner::status::CURRENT_CHARGE) <= 10.0 {
            // if already used once this airtime
            if VarModule::is_flag(fighter.battle_object, vars::miigunner::instance::LUNAR_LAUNCH_AIR_USED) {
                VarModule::on_flag(fighter.battle_object, vars::common::instance::UP_SPECIAL_CANCEL);
            }
            else {
                VarModule::on_flag(fighter.battle_object, vars::miigunner::instance::LUNAR_LAUNCH_AIR_USED);
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
            }
        }
    }
}

/// Resets Lunar Launch use count
unsafe fn lunar_launch_reset(fighter: &mut L2CFighterCommon) {
    if fighter.is_situation(*SITUATION_KIND_GROUND) || fighter.is_status(*FIGHTER_STATUS_KIND_CLIFF_CATCH) {
        VarModule::off_flag(fighter.battle_object, vars::miigunner::instance::LUNAR_LAUNCH_AIR_USED);
    }
}

/// Turn off funny slowed down smoke when hit
unsafe fn lunar_launch_effect_reset(fighter: &mut L2CFighterCommon) {
    if fighter.is_prev_status(*FIGHTER_STATUS_KIND_SPECIAL_HI) && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_DAMAGE,
        *FIGHTER_STATUS_KIND_DAMAGE_AIR,
        *FIGHTER_STATUS_KIND_DAMAGE_FALL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
        *FIGHTER_STATUS_KIND_DAMAGE_FALL ]) {
        EFFECT_OFF_KIND(fighter, Hash40::new("miigunner_bottom_shot"), false, false);
    }
}

unsafe fn stealth_burst_land_cancel(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status(*FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S2_END) {
        if boma.is_situation(*SITUATION_KIND_GROUND) && boma.is_prev_situation(*SITUATION_KIND_AIR) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, false);
        }
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    charge_handle(fighter);
    nspecial_cancels(boma);
    reflector_jc(boma);
    laser_blaze_ff_land_cancel(boma);
    remove_homing_missiles(boma);
    missile_land_cancel(boma);
	arm_rocket_airdash(fighter);
    lunar_launch_actionability(fighter);
    lunar_launch_reset(fighter);
    lunar_launch_effect_reset(fighter);
    stealth_burst_land_cancel(boma);
}

#[utils::macros::opff(FIGHTER_KIND_MIIGUNNER )]
pub fn miigunner_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		miigunner_frame(fighter)
    }
}

pub unsafe fn miigunner_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

#[smashline::weapon_frame(agent = WEAPON_KIND_MIIGUNNER_SUPERMISSILE)]
pub fn miigunner_missile_frame(weapon: &mut smash::lua2cpp::L2CFighterBase) {
    unsafe {
        let boma = weapon.boma();
        let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        // Ensure the boma's owner is Mii Gunner
        if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_MIIGUNNER {
            let gunner = utils::util::get_battle_object_from_id(owner_id);
            let gunner_boma = &mut *(*gunner).module_accessor;
            if StatusModule::status_kind(boma) == *WEAPON_MIIGUNNER_SUPERMISSILE_STATUS_KIND_STRAIGHT
            && gunner_boma.is_cat_flag(Cat1::SpecialS)
            && VarModule::is_flag(gunner, vars::miigunner::instance::DETONATE_READY) {
                if WorkModule::is_enable_transition_term_group(gunner_boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ATTACK)
                    || WorkModule::is_enable_transition_term_group(gunner_boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK)
                    || WorkModule::is_enable_transition_term_group(gunner_boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_SPECIAL)
                    || WorkModule::is_enable_transition_term_group(gunner_boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL)
                    || WorkModule::is_enable_transition_term_group(gunner_boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP) {
                    StatusModule::change_status_request_from_script(boma, *WEAPON_MIIGUNNER_SUPERMISSILE_STATUS_KIND_S_BURST, false);
                    VarModule::on_flag(gunner, vars::miigunner::status::MISSILE_DETONATE);
                    VarModule::off_flag(gunner, vars::miigunner::instance::DETONATE_READY);
                    gunner_boma.clear_commands(Cat1::SpecialS); // Clear command so Gunner doesn't immediately fire another missile
                }
            }
        }
    }
}