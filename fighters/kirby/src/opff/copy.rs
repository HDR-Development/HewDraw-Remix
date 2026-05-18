use super::*;

// Copy Abilities

// Donkey Kong
unsafe fn donkey_nspecial_cancels(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_DONKEY_SPECIAL_N_CANCEL) {
        if fighter.is_situation(*SITUATION_KIND_AIR) {
            if fighter.get_int(*FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE) == *FIGHTER_DONKEY_SPECIAL_N_CANCEL_TYPE_AIR_ESCAPE_AIR {
                fighter.set_int(*FIGHTER_DONKEY_SPECIAL_N_CANCEL_TYPE_NONE, *FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
            }
        }
    }
}

// Link
unsafe fn bow_drift(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_LINK_SPECIAL_N) {
        if fighter.is_situation(*SITUATION_KIND_AIR) {
            if KineticModule::get_kinetic_type(fighter.module_accessor) != *FIGHTER_KINETIC_TYPE_FALL {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            }
        }
    }
}

// Samus & Dark Samus
unsafe fn samus_nspecial_cancels(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_SAMUS_SPECIAL_N_C) {
        if fighter.is_situation(*SITUATION_KIND_AIR) {
            if fighter.get_int(*FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE) == *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_AIR_ESCAPE_AIR {
                fighter.set_int(*FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_NONE, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
            }
        }
    }
}

// Fox
unsafe fn fox_drift_laser_landcancel(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_FOX_SPECIAL_N) {
        fighter.check_land_cancel(None);

        if fighter.is_situation(*SITUATION_KIND_AIR) {
            if KineticModule::get_kinetic_type(fighter.module_accessor) != *FIGHTER_KINETIC_TYPE_FALL {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            }
        }
    }
}

// Captain Falcon
unsafe fn repeated_falcon_punch_turnaround(fighter: &mut L2CFighterCommon) {
    if StatusModule::is_changing(fighter.module_accessor) {
        return;
    }
    let frame = fighter.motion_frame();
    if fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_CAPTAIN_SPECIAL_N_TURN)
    && 22.0 < frame && frame < 41.0
    && fighter.is_stick_backward()
    && fighter.stick_x().abs() > 0.1 {
        fighter.change_status_req(*FIGHTER_KIRBY_STATUS_KIND_CAPTAIN_SPECIAL_N_TURN, true);
    }
}

// Bowser
unsafe fn koopa_flame_cancel(fighter: &mut L2CFighterCommon) {
    if StatusModule::is_changing(fighter.module_accessor) {
        return;
    }
    if fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_KOOPA_SPECIAL_N) {
        let cooleddown = VarModule::countdown_int(fighter.battle_object, vars::koopa::instance::SPECIAL_N_FIREBALL_COOLDOWN, 0);
        if fighter.status_frame() < 23 && !cooleddown {
            if fighter.is_situation(*SITUATION_KIND_GROUND) && fighter.is_prev_situation(*SITUATION_KIND_AIR) {
                MotionModule::set_frame(fighter.module_accessor, 22.0, true);
            }
        }
    }
}

unsafe fn koopa_fireball_cooldown(fighter: &mut L2CFighterCommon) {
    if fighter.get_int(*FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == FIGHTER_KIND_KOOPA {
        let cooleddown = VarModule::countdown_int(fighter.battle_object, vars::koopa::instance::SPECIAL_N_FIREBALL_COOLDOWN, 0);
        let charged_effect = VarModule::get_int(fighter.battle_object, vars::koopa::instance::SPECIAL_N_FIREBALL_EFFECT_ID);
        // If cooling down, remove ready effect
        if !cooleddown {
            if charged_effect > 0 {
                VarModule::set_int(fighter.battle_object, vars::koopa::instance::SPECIAL_N_FIREBALL_EFFECT_ID, 0);
                if EffectModule::is_exist_effect(fighter.module_accessor, charged_effect as u32) {
                    EffectModule::kill(fighter.module_accessor, charged_effect as u32, false, false);
                }
            }
            return;
        }
        // Otherwise, spawn effect if effect does not exist
        else if (charged_effect <= 0 || !EffectModule::is_exist_effect(fighter.module_accessor, charged_effect as u32)) {
            if charged_effect <= 0 {
                let boma = fighter.boma();
                gimmick_flash(boma);
            }
            let pos = &Vector3f{x: 0.0, y: 5.0, z: 0.0};
            let rot = &Vector3f{x: 180.0, y: 0.0, z: 50.0};
            let handle = EffectModule::req_follow(fighter.module_accessor, Hash40::new("koopa_breath_m_fire"), Hash40::new("body"), pos, rot, 1.0, true, 0, 0, 0, 0, 0, false, false) as u32;
            VarModule::set_int(fighter.battle_object, vars::koopa::instance::SPECIAL_N_FIREBALL_EFFECT_ID,handle as i32);
        }
    }
}

// Zelda
unsafe fn nayru_drift_land_cancel(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_ZELDA_SPECIAL_N) {
        let landing_lag = 8.0;
        if fighter.check_land_cancel(Some(landing_lag)) {
            EffectModule::kill_kind(fighter.module_accessor, Hash40::new("zelda_nayru_l"), true, true);
            EffectModule::kill_kind(fighter.module_accessor, Hash40::new("zelda_nayru_r"), true, true);
            AttackModule::clear_all(fighter.module_accessor);
            fighter.on_flag(*FIGHTER_ZELDA_STATUS_SPECIAL_N_FLAG_REFLECTOR_END);
        }
    }
}

// Falco
unsafe fn falco_drift_laser_landcancel(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_FALCO_SPECIAL_N) {
        fighter.check_land_cancel(None);

        if fighter.is_situation(*SITUATION_KIND_AIR) {
            if KineticModule::get_kinetic_type(fighter.module_accessor) != *FIGHTER_KINETIC_TYPE_FALL {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            }
        }
    }
}

// Young Link
unsafe fn fire_arrow_drift(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_YOUNGLINK_SPECIAL_N) {
        if fighter.is_situation(*SITUATION_KIND_AIR) {
            if KineticModule::get_kinetic_type(fighter.module_accessor) != *FIGHTER_KINETIC_TYPE_FALL {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            }
        }
    }
}

// Sheik
unsafe fn sheik_nspecial_cancels(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_SHEIK_SPECIAL_N_CANCEL) {
        if fighter.is_situation(*SITUATION_KIND_AIR) {
            if fighter.get_int(*FT_SHEIK_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS) == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
                fighter.set_int(*STATUS_KIND_NONE, *FT_SHEIK_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS);
            }
        }
    }
}

// Mr. Game and Watch
unsafe fn chef_drift_land_cancel(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_GAMEWATCH_SPECIAL_N) {
        if fighter.status_frame() == 18 {
            let air_accel_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_mul"), 0);
            let air_accel_x_add = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_add"), 0);
            sv_kinetic_energy!(controller_set_accel_x_mul, fighter, air_accel_x_mul * 0.5);
            sv_kinetic_energy!(controller_set_accel_x_add, fighter, air_accel_x_add * 0.5);
        }
        if fighter.is_situation(*SITUATION_KIND_AIR) {
            if fighter.is_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_REQUEST_DIVE_EFFECT) {
                fighter.off_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_REQUEST_DIVE_EFFECT);
            }
            if !fighter.is_flag(*FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
                if KineticModule::get_sum_speed_y(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0
                && fighter.stick_y() < WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("attack_lw4_stick_y")) {
                    fighter.on_flag(*FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
                    fighter.on_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_REQUEST_DIVE_EFFECT);
                }
            }
        }
        let landing_lag = 6.0;
        fighter.check_land_cancel(Some(landing_lag));

        if StatusModule::is_changing(fighter.module_accessor) {
            let nspec_halt = Vector3f{x: 0.9, y: 1.0, z: 1.0};
            KineticModule::mul_speed(fighter.module_accessor, &nspec_halt, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            if fighter.is_situation(*SITUATION_KIND_AIR) {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            }
        }
    }
}

// Dark Pit
unsafe fn pitb_bow_lc(fighter: &mut L2CFighterCommon) {
    if fighter.get_int(*FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == FIGHTER_KIND_PITB {
        if fighter.is_status_one_of(&[
            *FIGHTER_KIRBY_STATUS_KIND_PIT_SPECIAL_N_SHOOT,
            *FIGHTER_KIRBY_STATUS_KIND_PIT_SPECIAL_N_CHARGE,
            *FIGHTER_KIRBY_STATUS_KIND_PIT_SPECIAL_N_DIR,
            *FIGHTER_KIRBY_STATUS_KIND_PIT_SPECIAL_N_TURN
        ]) {
            if fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_PIT_SPECIAL_N_SHOOT) {
                let landing_lag = 7.0;
                fighter.check_land_cancel(Some(landing_lag));
            }
        }
    }
}

// Wario
unsafe fn bite_early_throw_turnaround(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_WARIO_SPECIAL_N_BITE)
    && !StatusModule::is_changing(fighter.module_accessor) {
        if fighter.is_pad_flag(PadFlag::SpecialTrigger) {
            fighter.change_status_req(*FIGHTER_KIRBY_STATUS_KIND_WARIO_SPECIAL_N_BITE_END, false);
        }
    }
    if fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_WARIO_SPECIAL_N_BITE_END) {
        if fighter.status_frame() < 7 {
            if PostureModule::lr(fighter.module_accessor) * fighter.stick_x() < 0.0 {
                PostureModule::reverse_lr(fighter.module_accessor);
                PostureModule::update_rot_y_lr(fighter.module_accessor);
            }
        }
    }
}

unsafe extern "C" fn pledge_timer(fighter: &mut L2CFighterCommon) {
    if VarModule::get_int(fighter.battle_object, vars::kirby::instance::SPECIAL_N_PTRAINER_PLEDGE_STATE) != *PLEDGE_STATE_NONE {
        if VarModule::get_int(fighter.battle_object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_TIMER) < 0 {
            VarModule::set_int(fighter.battle_object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_TIMER, 0);
            VarModule::set_int(fighter.battle_object, vars::kirby::instance::SPECIAL_N_PTRAINER_PLEDGE_STATE, *PLEDGE_STATE_NONE);

            // kill pledge effects
            let handle = VarModule::get_int(fighter.battle_object, vars::kirby::instance::SPECIAL_N_PTRAINER_PLEDGE_EFFECT_HANDLE) as u32;
            EffectModule::kill(fighter.module_accessor, handle, false, false);
            VarModule::set_int(fighter.battle_object, vars::kirby::instance::SPECIAL_N_PTRAINER_PLEDGE_EFFECT_HANDLE, -1);
            utils::ui::UiManager::set_ptrainer_meter_enable(fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32, false);
        } else {
            if VarModule::get_int(fighter.battle_object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_TIMER) == 1800 {
                pledge_init_effects(fighter);
            }
            VarModule::dec_int(fighter.battle_object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_TIMER);
            pledge_update_ui(fighter);
        }
    }
}

// Pokemon Trainer
unsafe extern "C" fn pledge_init_effects(fighter: &mut L2CFighterCommon) {
    let pledge_state = VarModule::get_int(fighter.battle_object, vars::kirby::instance::SPECIAL_N_PTRAINER_PLEDGE_STATE);
    if pledge_state == *PLEDGE_STATE_WATER {
        let handle = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_status_defense_up"), Hash40::new("hip"), &Vector3f::new(0.7, 0.0, 0.0), &Vector3f::zero(), 0.7, true, 0, 0, 0, 0, 0, true, true) as u32;
        VarModule::set_int(fighter.battle_object, vars::kirby::instance::SPECIAL_N_PTRAINER_PLEDGE_EFFECT_HANDLE, handle as i32);
        let water_fx = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_water_landing"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
        EffectModule::set_rgb(fighter.module_accessor, water_fx, 0.2, 0.55, 1.0);
        EffectModule::set_scale(fighter.module_accessor, water_fx, &Vector3f::new(0.6, 0.9, 0.6));
        EffectModule::set_rate(fighter.module_accessor, water_fx, 0.7);
    }
    else if pledge_state == *PLEDGE_STATE_GRASS {
        let handle = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_status_speed_up"), Hash40::new("hip"), &Vector3f::new(0.7, 0.0, 0.0), &Vector3f::zero(), 0.7, true, 0, 0, 0, 0, 0, true, true) as u32;
        VarModule::set_int(fighter.battle_object, vars::kirby::instance::SPECIAL_N_PTRAINER_PLEDGE_EFFECT_HANDLE, handle as i32);
        for _ in 0..2 {
            let grass_fx = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_grass_landing"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
            EffectModule::set_rgb(fighter.module_accessor, grass_fx, 0.5, 2.0, 0.5);
            EffectModule::set_scale(fighter.module_accessor, grass_fx, &Vector3f::new(1.2, 1.4, 1.2));
            EffectModule::set_rate(fighter.module_accessor, grass_fx, 0.6);
        }
    }
    else if pledge_state == *PLEDGE_STATE_FIRE {
        let handle = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_status_attack_up"), Hash40::new("hip"), &Vector3f::new(0.7, 0.0, 0.0), &Vector3f::zero(), 0.7, true, 0, 0, 0, 0, 0, true, true) as u32;
        VarModule::set_int(fighter.battle_object, vars::kirby::instance::SPECIAL_N_PTRAINER_PLEDGE_EFFECT_HANDLE, handle as i32);
        let fire_fx = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_damage_fire"), Hash40::new("top"), &Vector3f::new(0.5, 0.0, 0.0), &Vector3f::zero(), 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
        EffectModule::set_rgb(fighter.module_accessor, fire_fx, 1.0, 0.9, 0.9);
        EffectModule::set_scale(fighter.module_accessor, fire_fx, &Vector3f::new(1.2, 1.25, 1.2));
        EffectModule::set_rate(fighter.module_accessor, fire_fx, 0.5);
    }
}

unsafe extern "C" fn pledge_update_ui(fighter: &mut L2CFighterCommon) {
    if VarModule::get_int(fighter.battle_object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_TIMER) > 0 {
        let timer = VarModule::get_int(fighter.battle_object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_TIMER) as f32;
        let pledge = VarModule::get_int(fighter.battle_object, vars::kirby::instance::SPECIAL_N_PTRAINER_PLEDGE_STATE);
        utils::ui::UiManager::set_ptrainer_meter_enable(fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32, true);
        utils::ui::UiManager::set_ptrainer_meter_info(
            fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32,
            timer,
            1800.0,
            0.0,
            0.0,
            pledge,
            false
        );
    }
}

// Diddy Kong
unsafe fn peanut_popgun_ac(fighter: &mut L2CFighterCommon) {
    if StatusModule::is_changing(fighter.module_accessor) {
        return;
    }
    if fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_DIDDY_SPECIAL_N_SHOOT) && fighter.status_frame() > 5 {
        fighter.check_airdodge_cancel();
    }
}

unsafe fn diddy_nspecial_cancels(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_DIDDY_SPECIAL_N_CHARGE) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            if fighter.is_cat_flag(Cat2::StickEscape) {
                VarModule::set_int(fighter.battle_object, vars::diddy::status::SPECIAL_N_CANCEL_TYPE, vars::diddy::SPECIAL_N_CANCEL_TYPE_ESCAPE);
                fighter.change_status(statuses::kirby::DIDDY_SPECIAL_N_CANCEL.into(), true.into());
            }
            else if fighter.is_cat_flag(Cat2::StickEscapeF) {
                VarModule::set_int(fighter.battle_object, vars::diddy::status::SPECIAL_N_CANCEL_TYPE, vars::diddy::SPECIAL_N_CANCEL_TYPE_ESCAPE_F);
                fighter.change_status(statuses::kirby::DIDDY_SPECIAL_N_CANCEL.into(), true.into());
            }
            else if fighter.is_cat_flag(Cat2::StickEscapeB) {
                VarModule::set_int(fighter.battle_object, vars::diddy::status::SPECIAL_N_CANCEL_TYPE, vars::diddy::SPECIAL_N_CANCEL_TYPE_ESCAPE_B);
                fighter.change_status(statuses::kirby::DIDDY_SPECIAL_N_CANCEL.into(), true.into());
            }
            else if (fighter.is_cat_flag(Cat1::JumpButton) || (ControlModule::is_enable_flick_jump(fighter.module_accessor) && fighter.is_cat_flag(Cat1::Jump) && fighter.sub_check_button_frick().get_bool())) {
                VarModule::set_int(fighter.battle_object, vars::diddy::status::SPECIAL_N_CANCEL_TYPE, vars::diddy::SPECIAL_N_CANCEL_TYPE_GROUND_JUMP);
                fighter.change_status(statuses::kirby::DIDDY_SPECIAL_N_CANCEL.into(), true.into());
            }
            if fighter.sub_check_command_guard().get_bool() {
                VarModule::set_int(fighter.battle_object, vars::diddy::status::SPECIAL_N_CANCEL_TYPE, vars::diddy::SPECIAL_N_CANCEL_TYPE_GUARD);
                fighter.change_status(statuses::kirby::DIDDY_SPECIAL_N_CANCEL.into(), true.into());
            }
        }
        else {
            if fighter.is_cat_flag(Cat1::AirEscape) {
                VarModule::set_int(fighter.battle_object, vars::diddy::status::SPECIAL_N_CANCEL_TYPE, vars::diddy::SPECIAL_N_CANCEL_TYPE_ESCAPE_AIR);
                fighter.change_status(statuses::kirby::DIDDY_SPECIAL_N_CANCEL.into(), true.into());
            }
            else if (fighter.is_cat_flag(Cat1::JumpButton) || (ControlModule::is_enable_flick_jump(fighter.module_accessor) && fighter.is_cat_flag(Cat1::Jump)))
            && fighter.get_num_used_jumps() < fighter.get_jump_count_max() {
                VarModule::set_int(fighter.battle_object, vars::diddy::status::SPECIAL_N_CANCEL_TYPE, vars::diddy::SPECIAL_N_CANCEL_TYPE_JUMP_AERIAL);
                fighter.change_status(statuses::kirby::DIDDY_SPECIAL_N_CANCEL_JUMP.into(), true.into());
            }
        }
    }
}

// Lucas
unsafe fn lucas_offense_charge(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE) {
        if fighter.is_status_one_of(&[
            *FIGHTER_STATUS_KIND_ATTACK_HI4,
            *FIGHTER_STATUS_KIND_ATTACK_LW4,
            *FIGHTER_STATUS_KIND_ATTACK_S4, 
            *FIGHTER_KIRBY_STATUS_KIND_LUCAS_SPECIAL_N_FIRE
        ]) {
            //println!("In swing! Status of release: {} Reflective: {}", VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF));
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                VarModule::off_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF);
            }
        }
        else if !fighter.is_status_one_of(&[
            *FIGHTER_STATUS_KIND_ATTACK_HI4,
            *FIGHTER_STATUS_KIND_ATTACK_LW4,
            *FIGHTER_STATUS_KIND_ATTACK_S4, 
            *FIGHTER_STATUS_KIND_ATTACK_HI4_START,
            *FIGHTER_STATUS_KIND_ATTACK_LW4_START,
            *FIGHTER_STATUS_KIND_ATTACK_S4_START, 
            *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD,
            *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD,
            *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD,
            *FIGHTER_KIRBY_STATUS_KIND_LUCAS_SPECIAL_N_END])
        && VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF) {
            VarModule::off_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF);
            VarModule::set_float(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_CHARGE_LEVEL, 0.0);
            VarModule::off_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE);
        }
    } 
}

unsafe fn lucas_offense_effect_handler(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE) && !VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_INIT) 
    && (VarModule::get_int(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_EFFECT_HANDLE1) == -1 || VarModule::get_int(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_EFFECT_HANDLE2) == -1) {
        // The case is that Lucas is in Offense Up, has cleared past `pkfr_hold` effects, yet he does not have his hand effects. //
        let handle = EffectModule::req_follow(fighter.module_accessor, Hash40::new("lucas_pkfr_hold"), Hash40::new("handl"), &Vector3f{x: -2.0, y: 0.0, z: 0.0}, &Vector3f::zero(), 0.3, true, 0, 0, 0, 0, 0, true, true) as u32;
        VarModule::set_int(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_EFFECT_HANDLE1, handle as i32);
        let handle2 = EffectModule::req_follow(fighter.module_accessor, Hash40::new("lucas_pkfr_hold"), Hash40::new("handr"), &Vector3f{x: -2.0, y: 0.0, z: 0.0}, &Vector3f::zero(), 0.3, true, 0, 0, 0, 0, 0, true, true) as u32;
        VarModule::set_int(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_EFFECT_HANDLE2, handle2 as i32);
        let handle3 = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_status_defense_up"), Hash40::new("hip"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &Vector3f::zero(), 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
        VarModule::set_int(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_EFFECT_HANDLE3, handle3 as i32);
    }
    else if !VarModule::is_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE) 
    && (VarModule::get_int(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_EFFECT_HANDLE1) != -1 || VarModule::get_int(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_EFFECT_HANDLE2) != -1) {
        // The case is that Lucas is no longer in Offence Up, and his hand effects NEED TO BE CLEARED. //
        let handle = VarModule::get_int(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_EFFECT_HANDLE1) as u32;
        EffectModule::kill(fighter.module_accessor, handle, false, false);
        let handle2 = VarModule::get_int(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_EFFECT_HANDLE2) as u32;
        EffectModule::kill(fighter.module_accessor, handle2, false, false);
        let handle3 = VarModule::get_int(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_EFFECT_HANDLE3) as u32;
        EffectModule::kill(fighter.module_accessor, handle3, false, false);
        VarModule::set_int(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_EFFECT_HANDLE1, -1);
        VarModule::set_int(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_EFFECT_HANDLE2, -1);
        VarModule::set_int(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_EFFECT_HANDLE3, -1);
    }
}

// Lucario
unsafe fn magic_series_lucario(fighter: &mut L2CFighterCommon) {
    // Dont use magic series if we're already in cancel frames, if we're in hitlag, or if we didn't connect
    if CancelModule::is_enable_cancel(fighter.module_accessor) 
    || fighter.is_in_hitlag() 
    || !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD)
    || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_PARRY) {
        return;
    }
    
    // Tilt cancels
    if fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_DASH]) {
        if fighter.is_cat_flag(Cat1::AttackS3) {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S3, false);
        }
        if fighter.is_cat_flag(Cat1::AttackHi3) {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_HI3, false);
        }
        if fighter.is_cat_flag(Cat1::AttackLw3) {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_LW3, false);
        }
    }

    // Smash cancels
    if fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_DASH]) {
        if fighter.is_cat_flag(Cat1::AttackS4) {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S4_START, true);
        }
        if fighter.is_cat_flag(Cat1::AttackHi4) {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, true);
        }
        if fighter.is_cat_flag(Cat1::AttackLw4) {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, true);
        }
    }

    // Special cancels
    if fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_ATTACK, 
        *FIGHTER_STATUS_KIND_ATTACK_DASH, 
        *FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
        *FIGHTER_STATUS_KIND_ATTACK_S4,
        *FIGHTER_STATUS_KIND_ATTACK_HI4,
        *FIGHTER_STATUS_KIND_ATTACK_LW4,
        *FIGHTER_STATUS_KIND_ATTACK_AIR
    ]) {
        if fighter.is_cat_flag(Cat1::SpecialN) {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_N, false);
        }
        if fighter.is_cat_flag(Cat1::SpecialS) {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_S, false);
        }
        if fighter.is_cat_flag(Cat1::SpecialHi) {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_HI, false);
        }
        if fighter.is_cat_flag(Cat1::SpecialLw) {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_LW, false);
        }
    }
}

// Lucario
unsafe fn lucario_correct_jump_cancel_kind(fighter: &mut L2CFighterCommon) {
    if fighter.is_prev_status(*FIGHTER_KIRBY_STATUS_KIND_LUCARIO_SPECIAL_N_CANCEL) {
        if fighter.is_situation(*SITUATION_KIND_AIR)
        && fighter.is_status(*FIGHTER_STATUS_KIND_JUMP_AERIAL) {
            WorkModule::dec_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FLY, false);
        }
    }
}

// Mewtwo
unsafe fn mewtwo_correct_jump_cancel_kind(fighter: &mut L2CFighterCommon) {
    if fighter.is_prev_status(*FIGHTER_KIRBY_STATUS_KIND_MEWTWO_SPECIAL_N_CANCEL) {
        if fighter.is_situation(*SITUATION_KIND_AIR)
        && fighter.is_status(*FIGHTER_STATUS_KIND_JUMP_AERIAL) {
            WorkModule::dec_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FLY, false);
        }
    }
}

// Toon Link
unsafe fn heros_bow_drift(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_TOONLINK_SPECIAL_N) {
        if fighter.is_situation(*SITUATION_KIND_AIR) {
            if KineticModule::get_kinetic_type(fighter.module_accessor) != *FIGHTER_KINETIC_TYPE_FALL {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            }
        }
    }
}

// Wolf
unsafe fn wolf_drift_airdodge_cancel(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_WOLF_SPECIAL_N) {
        if fighter.status_frame() > 17 {
            fighter.check_airdodge_cancel();
        }
        if fighter.is_situation(*SITUATION_KIND_AIR) {
            if KineticModule::get_kinetic_type(fighter.module_accessor) != *FIGHTER_KINETIC_TYPE_FALL {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            }
        }
    }
}

// Mega Man
unsafe fn blade_toss_ac(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_ROCKMAN_SPECIAL_N) {
        if fighter.status_frame() > 16 {
            fighter.check_airdodge_cancel();
        }
    }
}

// Wii Fit Trainer
unsafe fn wiifit_nspecial_cancels(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_WIIFIT_SPECIAL_N_CANCEL) {
        if fighter.is_situation(*SITUATION_KIND_AIR) {
            if fighter.get_int(*FIGHTER_WIIFIT_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE) == *FIGHTER_WIIFIT_SPECIAL_N_CANCEL_TYPE_AIR_ESCAPE_AIR {
                fighter.set_int(*FIGHTER_WIIFIT_SPECIAL_N_CANCEL_TYPE_NONE, *FIGHTER_WIIFIT_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
                //ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
            }
        }
    }
}

// Greninja
unsafe fn max_water_shuriken_dc(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_GEKKOUGA_SPECIAL_N_MAX_SHOT) {
        if fighter.status_frame() > 12 {
            fighter.check_dash_cancel();
        }
    }
}

// Robin
unsafe fn reflet_nspecial_cancels(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_REFLET_SPECIAL_N_CANCEL) {
        if fighter.is_situation(*SITUATION_KIND_AIR) {
            if fighter.get_int(*FIGHTER_REFLET_STATUS_SPECIAL_N_HOLD_INT_NEXT_STATUS) == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
                fighter.set_int(*STATUS_KIND_NONE, *FIGHTER_REFLET_STATUS_SPECIAL_N_HOLD_INT_NEXT_STATUS);
                ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
            }
        } else if fighter.get_int(*FIGHTER_REFLET_STATUS_SPECIAL_N_HOLD_INT_NEXT_STATUS) != *FIGHTER_STATUS_KIND_JUMP_SQUAT {
            fighter.set_int(*STATUS_KIND_NONE, *FIGHTER_REFLET_STATUS_SPECIAL_N_HOLD_INT_NEXT_STATUS);
        }
    }
}

// Bowser Jr.
unsafe fn clown_cannon_shield_cancel(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_KOOPAJR_SPECIAL_N_HOLD) {
        if fighter.status_frame() > 16 {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                if fighter.is_situation(*SITUATION_KIND_GROUND) {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_GUARD_ON, true);
                }
            }
        }
    }
}

// Ryu and Ken
unsafe fn check_special_cancels(fighter: &mut L2CFighterCommon) {
    // Dont use cancels if we're already in cancel frames, if we're in hitlag, or if we didn't connect
    if CancelModule::is_enable_cancel(fighter.module_accessor) 
    || fighter.is_in_hitlag() {
        return;
    }
    if !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD)
    || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_PARRY) {
        return;
    }
    if !fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_ATTACK,
        *FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
        *FIGHTER_STATUS_KIND_ATTACK_AIR,
    ]) {
        return;
    }
    
    let terms = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND,
    ];
    let mut enableds = [false; 10];
    for x in 0..terms.len() {
        enableds[x] = WorkModule::is_enable_transition_term(fighter.module_accessor, terms[x]);
    }
    for val in terms.iter() {
        WorkModule::enable_transition_term(fighter.module_accessor, *val);
    }
    if !fighter.is_situation(*SITUATION_KIND_GROUND) {
        fighter.sub_transition_group_check_air_special()
    } else {
        fighter.sub_transition_group_check_ground_special()
    };
    for x in 0..terms.len() {
        if !enableds[x] {
            WorkModule::unable_transition_term(fighter.module_accessor, terms[x]);
        }
    }
}

// Ken
unsafe fn ken_air_hado_distinguish(fighter: &mut L2CFighterCommon) {
    if !fighter.is_status_one_of(&[
        *FIGHTER_KIRBY_STATUS_KIND_RYU_SPECIAL_N,
        *FIGHTER_KIRBY_STATUS_KIND_RYU_SPECIAL_N_COMMAND,
        *FIGHTER_KIRBY_STATUS_KIND_RYU_SPECIAL_N2_COMMAND,
        *FIGHTER_KIRBY_STATUS_KIND_KEN_SPECIAL_N,
        *FIGHTER_KIRBY_STATUS_KIND_KEN_SPECIAL_N_COMMAND,
        *FIGHTER_KIRBY_STATUS_KIND_KEN_SPECIAL_N2_COMMAND,
    ]) { return; }

    // set VarModule flag on f12 - this flag changes hado properties
    if fighter.status_frame() == 12 && fighter.is_motion_one_of(&[
        Hash40::new("ken_special_air_n"),
    ]) {
        VarModule::on_flag(fighter.battle_object, vars::shotos::instance::SPECIAL_N_HADOKEN_AIR);
    }
    // after frame 13, disallow changing from aerial to grounded hadoken
    // instead, we enter a landing animation
    if (fighter.status_frame() > 13 || fighter.is_motion_one_of(&[
        Hash40::new("ken_special_air_n_empty"),
        Hash40::new("ken_special_n_empty"),
    ]))
    {
        let landing_lag: Option<f32> = if fighter.motion_frame() < 70.0 { // the autocancel frame
            Some(14.0)
        } else {
            None
        };

        fighter.check_land_cancel(landing_lag);
    }
}

unsafe fn ken_hado_landcancel(fighter: &mut L2CFighterCommon) {
    if !fighter.is_status_one_of(&[
        *FIGHTER_KIRBY_STATUS_KIND_RYU_SPECIAL_N,
        *FIGHTER_KIRBY_STATUS_KIND_RYU_SPECIAL_N_COMMAND,
        *FIGHTER_KIRBY_STATUS_KIND_RYU_SPECIAL_N2_COMMAND,
        *FIGHTER_KIRBY_STATUS_KIND_KEN_SPECIAL_N,
        *FIGHTER_KIRBY_STATUS_KIND_KEN_SPECIAL_N_COMMAND,
        *FIGHTER_KIRBY_STATUS_KIND_KEN_SPECIAL_N2_COMMAND,
    ]) {
        return;
    }

    let landing_lag = 14.0;
    fighter.check_land_cancel(Some(landing_lag));
}

// Cloud
unsafe fn cloud_special_n_hold(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_CLOUD_SPECIAL_N) {
        if fighter.check_hold_input(0, 10, Buttons::SpecialAll) {
            VarModule::on_flag(fighter.battle_object, vars::cloud::status::SPECIAL_N_HOLD);
        }
    }
}

// Simon
unsafe fn axe_drift(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_SIMON_SPECIAL_N) {
        if fighter.is_situation(*SITUATION_KIND_AIR) {
            if KineticModule::get_kinetic_type(fighter.module_accessor) != *FIGHTER_KINETIC_TYPE_FALL {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            }
        }
    }
}

// incineroar
unsafe fn lariat_ledge_slipoff(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_GAOGAEN_SPECIAL_N) {
        GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_KEEP));
        fighter.sub_transition_group_check_air_cliff();
    }
}

// Mii Gunner
unsafe fn miigunner_nspecial_cancels(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_MIIGUNNER_SPECIAL_N1_CANCEL) {
        if fighter.is_situation(*SITUATION_KIND_AIR) {
            if fighter.get_int(*FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_WORK_INT_CANCEL_STATUS) == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
                fighter.set_int(*STATUS_KIND_NONE, *FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_WORK_INT_CANCEL_STATUS);
            }
        }
    }
}

// Piranha Plant
unsafe fn packun_ptooie_stance(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_SWALLOW_WAIT) {
        let opponent_boma = fighter.get_grabbed_opponent_boma();
        if opponent_boma.kind() == *FIGHTER_KIND_PACKUN {
            let new_stance = VarModule::get_int(opponent_boma.object(), vars::packun::instance::CURRENT_STANCE);
            VarModule::set_int(fighter.battle_object, vars::packun::instance::CURRENT_STANCE, new_stance);
        }
    }
}

// Hero
unsafe fn brave_nspecial_cancels(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_BRAVE_SPECIAL_N_CANCEL)
    && fighter.is_situation(*SITUATION_KIND_AIR)
    && fighter.get_int(*FIGHTER_BRAVE_STATUS_SPECIAL_N_HOLD_INT_NEXT_STATUS) == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
        fighter.set_int(*STATUS_KIND_NONE, *FIGHTER_BRAVE_STATUS_SPECIAL_N_HOLD_INT_NEXT_STATUS);
        ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
    }
}

// Banjo and Kazooie
unsafe fn blue_eggs_land_cancels(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_BUDDY_SPECIAL_N) {
        let landing_lag = 12.0;
        fighter.check_land_cancel(Some(landing_lag));
    }
}

unsafe fn indicator_breegull_fatigue(fighter: &mut L2CFighterCommon) {
    if StatusModule::is_changing(fighter.module_accessor) {
        return;
    }
	let eggs_shot = WorkModule::get_int(fighter.module_accessor, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_SPECIAL_N_BAKYUN_BULLET_SHOOT_COUNT);
    let eggs_Weakest = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("bakyun_power_down_2_num"));
    let eggs_Weak = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("bakyun_power_down_1_num"));
	if eggs_shot >= eggs_Weak && !fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_BUDDY_SPECIAL_N_SHOOT_END) {
		let sweatRate = if eggs_shot < eggs_Weakest { 25.0 } else { 15.0 };
		let sweatSize = if eggs_shot < eggs_Weakest { 0.625 } else { 0.9 };
		let modulo = fighter.motion_frame() % sweatRate;
		if modulo < 1.0 {
			EFFECT_FOLLOW(fighter, Hash40::new("buddy_special_s_sweat"), Hash40::new("top"), 0, 8.5, 7.5, 0, 0, 0, sweatSize, true);
		}
	}
}

unsafe fn breegull_bayonet(fighter: &mut L2CFighterCommon) {
    let motion = MotionModule::motion_kind(fighter.module_accessor);
    let motion_partial = MotionModule::motion_kind_partial(fighter.module_accessor,*FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY);
    if fighter.is_status_one_of(&[
        *FIGHTER_KIRBY_STATUS_KIND_BUDDY_SPECIAL_N_SHOOT,
        *FIGHTER_KIRBY_STATUS_KIND_BUDDY_SPECIAL_N_SHOOT_WALK_F,
        *FIGHTER_KIRBY_STATUS_KIND_BUDDY_SPECIAL_N_SHOOT_WALK_B,
        *FIGHTER_KIRBY_STATUS_KIND_BUDDY_SPECIAL_N_SHOOT_TURN,
        *FIGHTER_KIRBY_STATUS_KIND_BUDDY_SPECIAL_N_SHOOT_LANDING
    ]) 
    && fighter.is_situation(*SITUATION_KIND_GROUND) 
    && !VarModule::is_flag(fighter.battle_object, vars::kirby::instance::BUDDY_SPECIAL_N_BAYONET_ACTIVE) {
        if motion_partial == hash40("buddy_special_n_shoot_upper_fire") {
            let frame_partial = MotionModule::frame_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY);
            let disable_frame = 3.0; //frame before egg fires
            let disable_bayonet = (!CancelModule::is_enable_cancel(fighter.module_accessor) && frame_partial >= disable_frame);
            VarModule::set_flag(fighter.battle_object, vars::buddy::instance::SPECIAL_N_BAYONET_DISABLE,disable_bayonet);
        }
        else {
            VarModule::off_flag(fighter.battle_object, vars::buddy::instance::SPECIAL_N_BAYONET_DISABLE);
        }
        let is_csticking = ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4 != 0;
        if (is_csticking && !VarModule::is_flag(fighter.battle_object, vars::buddy::instance::SPECIAL_N_BAYONET_DISABLE)) {
            VarModule::on_flag(fighter.battle_object, vars::kirby::instance::BUDDY_SPECIAL_N_BAYONET_ACTIVE);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BUDDY_STATUS_SPECIAL_N_FLAG_PRECEDE_SHOOT);
            fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_BUDDY_SPECIAL_N_SHOOT_JUMP_SQUAT.into(), false.into());            
        }
    }
}

// Steve
unsafe fn pickel_mining(fighter: &mut L2CFighterCommon) { 
    if fighter.get_int(*FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_PICKEL {
        if VarModule::get_int(fighter.battle_object, vars::kirby::instance::SPECIAL_N_PICKEL_MATERIAL_INDEX) as i32 > 99 {
            VarModule::set_int(fighter.battle_object, vars::kirby::instance::SPECIAL_N_PICKEL_MATERIAL_INDEX, 0);
        }
        
        // wait 2 frames before letting the material table advance, preventing any jumps in entries
        if !VarModule::is_flag(fighter.battle_object, vars::kirby::instance::SPECIAL_N_PICKEL_CYCLE_MATERIAL) {
            if VarModule::get_int(fighter.battle_object, vars::kirby::status::SPECIAL_N_PICKEL_MINING_TIMER) == 0 {
                VarModule::on_flag(fighter.battle_object, vars::kirby::instance::SPECIAL_N_PICKEL_CYCLE_MATERIAL);
            } else {
                VarModule::dec_int(fighter.battle_object, vars::kirby::status::SPECIAL_N_PICKEL_MINING_TIMER);
            }
        }
    }
}

// Sephiroth
unsafe fn edge_nspecial_cancels(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_EDGE_SPECIAL_N_CANCEL) {
        if fighter.is_situation(*SITUATION_KIND_AIR) {
            if fighter.get_int(*FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS) == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
                fighter.set_int(*STATUS_KIND_NONE, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS);
                ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
            }
        }
    }
}

// Sora
unsafe fn trail_magic_handling(fighter: &mut L2CFighterCommon) {
    // Firaga Airdodge Cancel
    if fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_TRAIL_SPECIAL_N1_SHOOT) 
    && fighter.is_motion(Hash40::new("trail_special_air_n1")) 
    && fighter.motion_frame() > 2.0 {
        fighter.check_airdodge_cancel();
    }
    // thundaga land cancel
    if fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_TRAIL_SPECIAL_N3) {
        let landing_lag = 12.0; // 11F of landing lag plus one extra frame to subtract from the FAF to actually get that amount of lag
        fighter.check_land_cancel(Some(landing_lag));
    }
    // blizzaga jump cancel
    if (fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_TRAIL_SPECIAL_N2)
    && fighter.motion_frame() > 12.0) {
        fighter.check_jump_cancel(false, false, false);
    }

    // handles the cooldown timer between casting spells
    if VarModule::get_int(fighter.battle_object, vars::trail::instance::SPECIAL_N_MAGIC_TIMER) > 0 {
        VarModule::dec_int(fighter.battle_object, vars::trail::instance::SPECIAL_N_MAGIC_TIMER);

        // cycles and enables magic on the last frame of the cooldown window
        if VarModule::get_int(fighter.battle_object, vars::trail::instance::SPECIAL_N_MAGIC_TIMER) == 1 {
            let trail = fighter.global_table[0x4].get_ptr() as *mut Fighter;

            fighter.off_flag(*FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_MAGIC_SELECT_FORBID);

            // 0x2100000C is needed by FighterSpecializer_Trail::change_magic
            // for it to work properly
            if fighter.is_flag(0x2100000C) {
                FighterSpecializer_Trail::change_magic(trail);
            }
            else {
                fighter.on_flag(0x2100000C);
                FighterSpecializer_Trail::change_magic(trail);
                fighter.off_flag(0x2100000C);
            }

            VarModule::off_flag(fighter.battle_object, vars::trail::instance::DISABLE_SPECIAL_N);
        }
    }   
}

// cycles Kirby to firaga after copying Sora
unsafe fn trail_magic_cycle(fighter: &mut L2CFighterCommon) { 
    if fighter.is_motion(Hash40::new("special_n_drink"))
    && WorkModule::get_int(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_TRAIL {
        let magic_kind = WorkModule::get_int(fighter.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_SPECIAL_N_MAGIC_KIND);
        let kirby = fighter.global_table[0x4].get_ptr() as *mut Fighter;
        if magic_kind == *FIGHTER_TRAIL_SPECIAL_N_MAGIC_KIND_FIRE && fighter.status_frame() > 3 {
            WorkModule::on_flag(fighter.boma(), *FIGHTER_TRAIL_STATUS_SPECIAL_N1_FLAG_CHANGE_MAGIC);
            FighterSpecializer_Trail::change_magic(kirby); // cycles to thunder
        }
        else if magic_kind == *FIGHTER_TRAIL_SPECIAL_N_MAGIC_KIND_THUNDER && fighter.status_frame() > 4 {
            FighterSpecializer_Trail::change_magic(kirby); // cycles to "blizzard", which is now fire
        }
    }
}

// handles the speed and disappearance of blizzaga effects
unsafe fn trail_flower_frame(fighter: &mut L2CFighterCommon) {
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_TRAIL_GENERATE_ARTICLE_FLOWER) {
        let article = ArticleModule::get_article(fighter.module_accessor, *FIGHTER_TRAIL_GENERATE_ARTICLE_FLOWER);
        let article_id = smash::app::lua_bind::Article::get_battle_object_id(article) as u32;
        let article_boma = sv_battle_object::module_accessor(article_id);
        if MotionModule::motion_kind(article_boma) == hash40("special_n2") {
            let blizz_frame = MotionModule::frame(article_boma) as i32;
            if blizz_frame == 1 {
                MotionModule::set_rate(article_boma, 1.1);
            }
            if (12..64).contains(&blizz_frame)
            && !fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_TRAIL_SPECIAL_N2) {
                MotionModule::set_rate(article_boma, 1.7);
            }
            if (65..90).contains(&blizz_frame) {
                MotionModule::set_rate(article_boma, 1.1);
                ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_TRAIL_GENERATE_ARTICLE_FLOWER, app::ArticleOperationTarget(0));
            }
        }
    }
}

// No Copy Ability
unsafe fn reset_flags(fighter: &mut L2CFighterCommon) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) != FIGHTER_KIND_KOOPA {
        VarModule::set_int(fighter.battle_object, vars::koopa::instance::SPECIAL_N_FIREBALL_COOLDOWN, KOOPA_MAX_COOLDOWN);
    }
    if fighter.get_int(*FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) != FIGHTER_KIND_LUCAS
    || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_ENTRY])
    || !sv_information::is_ready_go() {
        VarModule::set_int(fighter.battle_object, vars::lucas::instance::SPECIAL_N_OFFENSE_UP_CHARGE_LEVEL, LUCAS_CHARGE_TIME);
        VarModule::off_flag(fighter.battle_object, vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE);
        VarModule::off_flag(fighter.battle_object, vars::lucas::instance::SPECIAL_N_OFFENSE_UP_INIT);
        VarModule::off_flag(fighter.battle_object, vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF);
        let handle = VarModule::get_int(fighter.battle_object, vars::lucas::instance::SPECIAL_N_OFFENSE_UP_EFFECT_HANDLE1) as u32;
        EffectModule::kill(fighter.module_accessor, handle, false, false);
        let handle2 = VarModule::get_int(fighter.battle_object, vars::lucas::instance::SPECIAL_N_OFFENSE_UP_EFFECT_HANDLE2) as u32;
        EffectModule::kill(fighter.module_accessor, handle2, false, false);
        let handle3 = VarModule::get_int(fighter.battle_object, vars::lucas::instance::SPECIAL_N_OFFENSE_UP_EFFECT_HANDLE3) as u32;
        EffectModule::kill(fighter.module_accessor, handle3, false, false);
        VarModule::set_int(fighter.battle_object, vars::lucas::instance::SPECIAL_N_OFFENSE_UP_EFFECT_HANDLE1, -1);
        VarModule::set_int(fighter.battle_object, vars::lucas::instance::SPECIAL_N_OFFENSE_UP_EFFECT_HANDLE2, -1);
        VarModule::set_int(fighter.battle_object, vars::lucas::instance::SPECIAL_N_OFFENSE_UP_EFFECT_HANDLE3, -1);
    }
}

unsafe extern "C" fn pledge_init(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_SWALLOW_WAIT) {
        let opponent_boma = fighter.get_grabbed_opponent_boma();
        if [*FIGHTER_KIND_PZENIGAME, *FIGHTER_KIND_PFUSHIGISOU, *FIGHTER_KIND_PLIZARDON].contains(&opponent_boma.kind()) {
            if LinkModule::is_link(opponent_boma, *FIGHTER_POKEMON_LINK_NO_PTRAINER) {
                let parent_id = LinkModule::get_parent_id(opponent_boma, *FIGHTER_POKEMON_LINK_NO_PTRAINER, true) as u32;
                let object = utils::util::get_battle_object_from_id(parent_id);
                VarModule::off_flag(fighter.battle_object, vars::pfushigisou::instance::SPECIAL_N_SEED_FIRED);
                let pledge = VarModule::get_int(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_STATE);
                if pledge != *PLEDGE_STATE_NONE {
                    VarModule::set_int(fighter.battle_object, vars::kirby::instance::SPECIAL_N_PTRAINER_PLEDGE_STATE, pledge);
                    VarModule::set_int(fighter.battle_object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_TIMER, 1800);
                    let sanity_check = VarModule::get_int(fighter.battle_object, vars::kirby::instance::SPECIAL_N_PTRAINER_PLEDGE_STATE);
                    return;
                }
            }
            // The pokemon has no active pledge or the trainer was somehow not found, so Kirby will not get a pledge
            VarModule::set_int(fighter.battle_object, vars::kirby::instance::SPECIAL_N_PTRAINER_PLEDGE_STATE, *PLEDGE_STATE_NONE);
            VarModule::set_int(fighter.battle_object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_TIMER, 0);
            VarModule::off_flag(fighter.battle_object, vars::pfushigisou::instance::SPECIAL_N_SEED_FIRED);
        }
    }
}

unsafe extern "C" fn plant_meter(fighter: &mut L2CFighterCommon) {
    unsafe {
        if !sv_information::is_ready_go() {
            if fighter.status_frame() < 1 {
                return;
            }
            else {
                utils::ui::UiManager::set_ptrainer_meter_enable(fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32, false);
            }
        } 
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_PACKUN {
            utils::ui::UiManager::set_plant_meter_enable(fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32, true);
            utils::ui::UiManager::set_plant_meter_info(
                fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32,
                VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE)
            );
            return;
        }
        else {
            utils::ui::UiManager::set_plant_meter_enable(fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32, false);
        }
    }
}


unsafe fn bayo_air_special_cancels(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_ATTACK_AIR) {
        if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
            WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
        }
        if !fighter.is_in_hitlag() 
        && !StopModule::is_stop(fighter.module_accessor) {
            fighter.sub_transition_group_check_air_special();
            fighter.sub_transition_group_check_air_escape();
        }
    }
}


pub unsafe fn kirby_copy_handler(fighter: &mut L2CFighterCommon) {
    let inhaledstatus = StatusModule::status_kind(fighter.module_accessor);
    // enable copying flags when inhaling an opponent
    if (0x1e3..0x1f1).contains(&inhaledstatus) {
        packun_ptooie_stance(fighter);
        pledge_init(fighter);
        return;
    }
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_COPY) {
        reset_flags(fighter);
        return;
    }

    let copy = WorkModule::get_int(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA);
    match copy {
        // Donkey Kong
        0x1 => donkey_nspecial_cancels(fighter),
        // Link
        0x2 => bow_drift(fighter),
        // Samus
        0x3 => samus_nspecial_cancels(fighter),
        // Dark Samus
        0x4 => samus_nspecial_cancels(fighter),
        // Fox
        0x7 => fox_drift_laser_landcancel(fighter),
        // Captain Falcon
        0xB => repeated_falcon_punch_turnaround(fighter),
        // Bowser
        0xF => {
            koopa_flame_cancel(fighter);
            koopa_fireball_cooldown(fighter);
        },
        // Zelda
        0x11 => nayru_drift_land_cancel(fighter),
        // Falco
        0x14 => falco_drift_laser_landcancel(fighter),
        // Young Link
        0x17 => fire_arrow_drift(fighter),
        // Sheik
        0x10 => sheik_nspecial_cancels(fighter),
        // Mewtwo
        0x19 => mewtwo_correct_jump_cancel_kind(fighter),
        // Mr. Game & Watch
        0x1C => chef_drift_land_cancel(fighter),
        // Dark Pit
        0x1F => pitb_bow_lc(fighter),
        // Wario
        0x21 => bite_early_throw_turnaround(fighter),
        // Squirtle
        0x24 => {
            pledge_timer(fighter);
        },
        // Ivysaur
        0x25 => {
            pledge_timer(fighter);
        },
        // Charizard
        0x26 => {
            pledge_timer(fighter);
        },
        // Diddy Kong
        0x27 => {
            peanut_popgun_ac(fighter);
            diddy_nspecial_cancels(fighter);
        },
        // Lucas
        0x28 => {
            lucas_offense_charge(fighter);
            lucas_offense_effect_handler(fighter);
        },
        // Lucario
        0x2C => {
            magic_series_lucario(fighter);
            lucario_correct_jump_cancel_kind(fighter);
        },
        // Toon Link
        0x2E => heros_bow_drift(fighter),
        // Wolf
        0x2F => wolf_drift_airdodge_cancel(fighter),
        // Mega Man
        0x31 => blade_toss_ac(fighter),
        // Wii Fit Trainer
        0x32 => wiifit_nspecial_cancels(fighter),
        // Greninja
        0x35 => max_water_shuriken_dc(fighter),
        // Robin
        0x38 => reflet_nspecial_cancels(fighter),
        // Shulk
    //  0x39 => None
        // Bowser Jr.
        0x3A => clown_cannon_shield_cancel(fighter),
        // Ryu
        0x3C => check_special_cancels(fighter),
        // Ken
        0x3D => {
            check_special_cancels(fighter);
            ken_air_hado_distinguish(fighter);
            ken_hado_landcancel(fighter)
        },
        // Cloud
        0x3E => {
            cloud_special_n_hold(fighter);
        }
        // Simon
        0x43 => axe_drift(fighter),
        // Incineroar
        0x47 => lariat_ledge_slipoff(fighter),
        // Mii Gunner
        0x4A => miigunner_nspecial_cancels(fighter),
        // Piranha Plant
        0x51 => {
            packun_ptooie_stance(fighter);
        },
        // Hero
        0x53 => {
            brave_nspecial_cancels(fighter);
        },
        // Banjo & Kazooie
        0x54 => {
            blue_eggs_land_cancels(fighter);
            indicator_breegull_fatigue(fighter);
            breegull_bayonet(fighter);
        },
        // Terry
        0x55 => check_special_cancels(fighter),
        // Byleth
        //0x56 => master_nspecial_cancels(fighter),
        // Steve
        0x58 => pickel_mining(fighter),
        // Sephiroth
        0x59 => edge_nspecial_cancels(fighter),
        // Sora
        0x5D => {
            trail_magic_handling(fighter);
            trail_magic_cycle(fighter);
            trail_flower_frame(fighter);
        },
        // Bayonetta
        0x40 => bayo_air_special_cancels(fighter),
        _ => {}
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, plant_meter);
}