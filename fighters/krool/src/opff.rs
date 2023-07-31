// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn jetpack_cancel(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, cat1: i32) {
    if status_kind == *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI {
        let fuel_burn_rate = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "param_special_hi.fuel_burn_rate");
        let fuel = VarModule::get_int(fighter.battle_object, vars::krool::instance::SPECIAL_HI_FUEL);
        VarModule::set_int(fighter.battle_object, vars::krool::instance::SPECIAL_HI_FUEL, fuel - fuel_burn_rate);
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) || fuel <= 0 {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_AIR_END, true);
        }
    } else if fighter.is_situation(*SITUATION_KIND_GROUND) {
        let fuel_max = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "param_special_hi.fuel_max");
        if VarModule::get_int(fighter.battle_object, vars::krool::instance::SPECIAL_HI_FUEL) < fuel_max {
            let fuel_recharge_rate = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "param_special_hi.fuel_recharge_rate");
            VarModule::add_int(fighter.battle_object, vars::krool::instance::SPECIAL_HI_FUEL, fuel_recharge_rate);
        }
    }
}

unsafe fn fuel_reset(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_WIN,
        *FIGHTER_STATUS_KIND_LOSE,
        *FIGHTER_STATUS_KIND_ENTRY,
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_REBIRTH]) {
        let fuel_max = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "param_special_hi.fuel_max");
        VarModule::set_int(fighter.battle_object, vars::krool::instance::SPECIAL_HI_FUEL, fuel_max);
        VarModule::set_float(fighter.battle_object, vars::krool::instance::STORED_DAMAGE, 0.0);
        VarModule::off_flag(fighter.battle_object, vars::krool::instance::BLUNDERBUSS_GRAB);
    }
}

pub unsafe fn armored_charge(fighter: &mut L2CFighterCommon, motion_kind: u64) {
    if fighter.is_motion_one_of(&[
        Hash40::new("attack_s3_s"),
        Hash40::new("attack_s3_hi"),
        Hash40::new("attack_s3_lw"),
        Hash40::new("attack_hi3"),
        Hash40::new("attack_lw3"),
        Hash40::new("special_lw"),
        Hash40::new("special_air_lw") ]) {
        let is_hold =
            if [hash40("special_lw"), hash40("special_air_lw")].contains(&motion_kind) {
                ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
            }
            else {
                ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
            };
        let charge = VarModule::get_int(fighter.battle_object, vars::krool::status::CURRENT_CHARGE);
        let mut charge_start_frame = 0.0;
        let mut charge_end_frame = 0.0;
        let mut eff_offset = Vector3f::zero();
        // due to what I presume is internal rounding error, the current amount of 20.0 equates to 18 frames
        let max_charge_frames = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_waist.max_charge_frames");

        match MotionModule::motion_kind(fighter.module_accessor) {
            _ if [hash40("attack_s3_s"), hash40("attack_s3_hi"), hash40("attack_s3_lw")].contains(&motion_kind) => {
                charge_start_frame = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_waist.attack_s3_charge_start");
                charge_end_frame = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_waist.attack_s3_charge_end");
                eff_offset = Vector3f::new(3.0, 0.0, 5.0);
            },
            _ if motion_kind == hash40("attack_hi3") => {
                charge_start_frame = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_waist.attack_hi3_charge_start");
                charge_end_frame = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_waist.attack_hi3_charge_end");
                eff_offset = Vector3f::new(3.0, 0.0, 3.0);
            },
            _ if motion_kind == hash40("attack_lw3") => {
                charge_start_frame = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_waist.attack_lw3_charge_start");
                charge_end_frame = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_waist.attack_lw3_charge_end");
                eff_offset = Vector3f::new(3.0, 0.0, 0.0);
            },
            _ if [hash40("special_lw"), hash40("special_air_lw")].contains(&motion_kind) => {
                charge_start_frame = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_waist.special_lw_charge_start");
                charge_end_frame = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_waist.special_lw_charge_end");
                eff_offset = Vector3f::new(3.0, 0.0, 3.0);
            },
            _ => {}
        }

        if (charge_start_frame..charge_end_frame).contains(&fighter.motion_frame()) && charge < (max_charge_frames as i32) && is_hold {
            if [hash40("special_lw"), hash40("special_air_lw")].contains(&motion_kind) {
                if fighter.motion_frame() >= 1.18 {
                    VarModule::on_flag(fighter.battle_object, vars::krool::status::GUT_CHECK_CHARGED);
                }
                MotionModule::set_rate(fighter.module_accessor, 0.01);
                VarModule::set_int(fighter.battle_object, vars::krool::status::CURRENT_CHARGE, charge + 1);
            }
            else {
                if fighter.motion_frame() == charge_start_frame {
                    // if WorkModule::get_float(fighter.module_accessor, 0x4d) >= 1.0 {
                    //     WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_ON);
                    // }
                    let facing = eff_offset.z * PostureModule::lr(fighter.module_accessor);
                    smash_script::macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_level_up"), Hash40::new("hip"), eff_offset.x, eff_offset.y, facing, 0, 0, 0, 0.55, true);
                    smash_script::macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_krool_rnd_attack"));
                }
                let motion_rate = (charge_end_frame - charge_start_frame)/max_charge_frames;
                MotionModule::set_rate(fighter.module_accessor, motion_rate);
                VarModule::set_int(fighter.battle_object, vars::krool::status::CURRENT_CHARGE, charge + 1);
            }
        } else {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
        }
    }
}

pub unsafe fn restore_armor(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_LW) {
        if VarModule::is_flag(fighter.battle_object, vars::krool::status::GUT_CHECK_CHARGED)
            && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
            && !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
            WorkModule::set_float(fighter.module_accessor, 4.0, 0x4d);
            VarModule::set_float(fighter.battle_object, vars::krool::instance::STORED_DAMAGE, 0.0);
        }
    }
}

unsafe fn gut_shine(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_LW) {
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR) <= 1 {
            GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        }
    }
    if (fighter.is_status (*FIGHTER_STATUS_KIND_SPECIAL_LW) && fighter.status_frame() > 8)  // Allows for jump cancel on frame 10 if not charged
        && !VarModule::is_flag(fighter.battle_object, vars::krool::status::GUT_CHECK_CHARGED)
        && !fighter.is_in_hitlag() {
        fighter.check_jump_cancel(false);
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    armored_charge(fighter, motion_kind);
    restore_armor(fighter);
    gut_shine(fighter);
    jetpack_cancel(fighter, boma, status_kind, cat[0]);
    fuel_reset(fighter);
}

#[utils::macros::opff(FIGHTER_KIND_KROOL)]
pub fn krool_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
        krool_frame(fighter)
    }
}

pub unsafe fn krool_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

#[smashline::weapon_frame( agent = WEAPON_KIND_KROOL_BACKPACK, main)]
pub fn krool_backpack_frame(weapon: &mut smash::lua2cpp::L2CFighterBase) {
    unsafe {
        let boma = weapon.boma();
        let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        
        // upB low fuel indicator
        let fuel_max = ParamModule::get_int(owner_boma.object(), ParamType::Agent, "param_special_hi.fuel_max") as f32;
        let low_fuel_threshold = fuel_max * 0.33;
        if VarModule::get_int(owner_boma.object(), vars::krool::instance::SPECIAL_HI_FUEL) as f32 <= low_fuel_threshold
        && VarModule::get_int(owner_boma.object(), vars::krool::instance::FUEL_EFFECT_HANDLER) == -1 {
            let handle = EffectModule::req_follow(weapon.module_accessor, Hash40::new("krool_buckpack"), Hash40::new("backpack"), &Vector3f{x: -12.0, y: -1.5, z: -6.0}, &Vector3f::zero(), 1.5, true, 0, 0, 0, 0, 0, false, false) as u32;
            EffectModule::set_rgb(weapon.module_accessor, handle, 0.15, 0.15, 0.15);
            EffectModule::enable_sync_init_pos_last(weapon.module_accessor);
            VarModule::set_int(owner_boma.object(), vars::krool::instance::FUEL_EFFECT_HANDLER, handle as i32);
        }
    }
}