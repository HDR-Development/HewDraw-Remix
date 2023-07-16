// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn duck_jump_cancel(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_DUCKHUNT_STATUS_KIND_SPECIAL_HI_FLY) {
        let fuel_burn_rate = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "param_special_hi.fuel_burn_rate");
        let fuel = VarModule::get_int(fighter.battle_object, vars::duckhunt::instance::SPECIAL_HI_FUEL);
        VarModule::set_int(
            fighter.battle_object,
            vars::duckhunt::instance::SPECIAL_HI_FUEL,
            fuel - fuel_burn_rate,
        );
        if (fighter.status_frame() > 20 && fighter.is_cat_flag(Cat1::SpecialHi))
            || fuel <= 0
        {
            StatusModule::change_status_request_from_script(
                fighter.module_accessor,
                *FIGHTER_DUCKHUNT_STATUS_KIND_SPECIAL_HI_END,
                true,
            );
        }
    } else if fighter.is_situation(*SITUATION_KIND_GROUND) {
        let fuel_max = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "param_special_hi.fuel_max");
        if VarModule::get_int(fighter.battle_object, vars::duckhunt::instance::SPECIAL_HI_FUEL) < fuel_max {
            let fuel_recharge_rate = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "param_special_hi.fuel_recharge_rate");
            VarModule::add_int(fighter.battle_object, vars::duckhunt::instance::SPECIAL_HI_FUEL, fuel_recharge_rate);
        }
    }
}

unsafe fn fuel_reset(fighter: &mut L2CFighterCommon) {
    if fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_WIN,
        *FIGHTER_STATUS_KIND_LOSE,
        *FIGHTER_STATUS_KIND_ENTRY,
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_REBIRTH]) {
        let fuel_max = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "param_special_hi.fuel_max");
        VarModule::set_int(fighter.battle_object, vars::duckhunt::instance::SPECIAL_HI_FUEL, fuel_max);
    }
}

unsafe fn duck_jump_fuel_indicator(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    let fuel_max = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "param_special_hi.fuel_max") as f32;
    let low_fuel_threshold = fuel_max * 0.33;
    
    if fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_DUCKHUNT_STATUS_KIND_SPECIAL_HI_FLY, *FIGHTER_DUCKHUNT_STATUS_KIND_SPECIAL_HI_END])
    && VarModule::get_int(fighter.battle_object, vars::duckhunt::instance::SPECIAL_HI_FUEL) as f32 <= low_fuel_threshold
    && VarModule::get_int(fighter.battle_object, vars::duckhunt::instance::FUEL_EFFECT_HANDLER) == -1 {
        let handle = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_bomber_sweat"), Hash40::new("duckhead"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &Vector3f::zero(), 1.5, true, 0, 0, 0, 0, 0, true, true) as u32;
        VarModule::set_int(fighter.battle_object, vars::duckhunt::instance::FUEL_EFFECT_HANDLER, handle as i32);
    }
    else if VarModule::get_int(fighter.battle_object, vars::duckhunt::instance::FUEL_EFFECT_HANDLER) != -1 {
        let handle = VarModule::get_int(fighter.battle_object, vars::duckhunt::instance::FUEL_EFFECT_HANDLER) as u32;
        if !EffectModule::is_exist_effect(fighter.module_accessor, handle as u32)
        || VarModule::get_int(fighter.battle_object, vars::duckhunt::instance::SPECIAL_HI_FUEL) as f32 > low_fuel_threshold {
            VarModule::set_int(fighter.battle_object, vars::duckhunt::instance::FUEL_EFFECT_HANDLER, -1);
        }
    }
}

extern "Rust" {
    fn gimmick_flash(boma: &mut BattleObjectModuleAccessor);
}

unsafe fn gunman_timer(fighter: &mut L2CFighterCommon) {
    let timer = VarModule::get_int(fighter.object(), vars::duckhunt::instance::GUNMAN_TIMER);
    if  timer != 0 {
        VarModule::set_int(fighter.object(), vars::duckhunt::instance::GUNMAN_TIMER, (timer-1));
    }
    if timer == 1 {
        gimmick_flash(fighter);
    }
}

#[utils::macros::opff(FIGHTER_KIND_DUCKHUNT )]
pub fn duckhunt_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
        duck_jump_cancel(fighter);
        fuel_reset(fighter);
        duck_jump_fuel_indicator(fighter);
        gunman_timer(fighter);
    }
}

#[smashline::weapon_frame_callback(main)]
pub fn gunman_callback(weapon: &mut smash::lua2cpp::L2CFighterBase) {
    unsafe { 
        if weapon.kind() != WEAPON_KIND_DUCKHUNT_GUNMAN {
            return
        }
        let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        if weapon.is_status(*WEAPON_DUCKHUNT_GUNMAN_STATUS_KIND_READY) {
            let duckhunt = utils::util::get_battle_object_from_id(owner_id);
            let duckhunt_boma = &mut *(*duckhunt).module_accessor;

            if duckhunt_boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_STANDBY]) {
                return
            }

            if duckhunt_boma.is_cat_flag(Cat1::SpecialLw) && duckhunt_boma.is_button_trigger(Buttons::Special | Buttons::SpecialRaw) && WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE) > 25 {
                PLAY_STATUS(weapon, Hash40::new("se_duckhunt_special_l09"));
                let gunman_kind = WorkModule::get_int(weapon.boma(), *WEAPON_DUCKHUNT_GUNMAN_INSTANCE_WORK_ID_KIND);
                let lr = PostureModule::lr(weapon.boma());
                match gunman_kind {
                    0 => {
                        EFFECT_FOLLOW(weapon, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), 0.5*lr, 13.3, 0.74, 0, 0, 0, 1, true);
                        EFFECT_FOLLOW(weapon, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), 0.5*lr, 13.3, -0.78, 0, 0, 0, 1, true);
                    }
                    1 => {
                        EFFECT_FOLLOW(weapon, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), 0.5*lr, 15.66, 0.42, 0, 0, 0, 1, true);
                        EFFECT_FOLLOW(weapon, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), 0.5*lr, 15.66, -0.5, 0, 0, 0, 1, true);
                    }
                    2 => {
                        EFFECT_FOLLOW(weapon, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), 0.5*lr, 16.92, 0.26, 0, 0, 0, 1, true);
                        EFFECT_FOLLOW(weapon, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), 0.5*lr, 16.92, -1.29, 0, 0, 0, 1, true);
                    }
                    3 => {
                        EFFECT_FOLLOW(weapon, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), 0.5*lr, 10.9, 0.85, 0, 0, 0, 1, true);
                        EFFECT_FOLLOW(weapon, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), 0.5*lr, 10.9, -0.64, 0, 0, 0, 1, true);
                    }
                    4 => {
                        EFFECT_FOLLOW(weapon, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), 0.5*lr, 14.17, 0.4, 0, 0, 0, 1, true);
                        EFFECT_FOLLOW(weapon, Hash40::new("duckhunt_wildegunman_light"), Hash40::new("top"), 0.5*lr, 14.16, -1.36, 0, 0, 0, 1, true);
                    }
                    _ => {
                        return
                    }
                }
                WorkModule::set_int(weapon.module_accessor, 25, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
            }
            VarModule::set_int(duckhunt, vars::duckhunt::instance::GUNMAN_TIMER, 180);
        }
    }
}