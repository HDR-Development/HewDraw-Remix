// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn duck_jump_cancel(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_DUCKHUNT_STATUS_KIND_SPECIAL_HI_FLY)
    && fighter.motion_frame() > 20.0
    && fighter.is_cat_flag(Cat1::SpecialHi) {
        fighter.change_status_req(*FIGHTER_DUCKHUNT_STATUS_KIND_SPECIAL_HI_END, true);
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_DUCKHUNT_STATUS_KIND_SPECIAL_HI_END
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
        if fighter.is_flag(*FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
            if [*FIGHTER_KINETIC_TYPE_MOTION_AIR, *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE].contains(&KineticModule::get_kinetic_type(fighter.module_accessor)) {
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
                let speed_y = app::sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);

                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y, 0.0, 0.0, 0.0);
                app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
                
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                app::sv_kinetic_energy::enable(fighter.lua_state_agent);

                KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
            }
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
        gunman_timer(fighter);
        fastfall_specials(fighter);
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
            VarModule::set_int(duckhunt, vars::duckhunt::instance::GUNMAN_TIMER, 300);
        }
    }
}