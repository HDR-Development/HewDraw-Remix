// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
// symbol-based call for the pikachu/pichu characters' common opff
extern "Rust" {
    fn electric_rats_common(fighter: &mut smash::lua2cpp::L2CFighterCommon);
    fn gimmick_flash(boma: &mut BattleObjectModuleAccessor);
}

// handles pichu's charge increase
unsafe fn charge_state_increase(boma: &mut BattleObjectModuleAccessor) {
    MeterModule::update(boma.object(), false);
    if VarModule::get_int(boma.object(), vars::pichu::instance::CHARGE_LEVEL) == 0 {
        if MeterModule::level(boma.object()) >= 2 {
            VarModule::set_int(boma.object(), vars::common::instance::GIMMICK_TIMER, 900);
            MeterModule::reset(boma.object());
            VarModule::set_int(boma.object(), vars::pichu::instance::CHARGE_LEVEL, 1);
            //gimmick_flash(boma);
        }
    }
}

// handles pichu's charge decrease once at full charge
unsafe fn charge_state_decrease(boma: &mut BattleObjectModuleAccessor) {
    if VarModule::get_int(boma.object(), vars::pichu::instance::CHARGE_LEVEL) == 1 {
        if VarModule::get_int(boma.object(), vars::common::instance::GIMMICK_TIMER) > 0 
        && !boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW]) {
            VarModule::dec_int(boma.object(), vars::common::instance::GIMMICK_TIMER);
            if VarModule::get_int(boma.object(), vars::common::instance::GIMMICK_TIMER) == 870 {
                let handle = VarModule::get_int(boma.object(), vars::pichu::instance::CHARGE_EFFECT_HANDLER);
                EffectModule::set_scale(boma, handle as u32, &Vector3f{ x: 0.8, y: 0.8, z: 0.8 });
            }
            if VarModule::get_int(boma.object(), vars::common::instance::GIMMICK_TIMER) == 860 {
                let handle = VarModule::get_int(boma.object(), vars::pichu::instance::CHARGE_EFFECT_HANDLER);
                EffectModule::set_scale(boma, handle as u32, &Vector3f{ x: 0.7, y: 0.7, z: 0.7 });
            }
            if VarModule::get_int(boma.object(), vars::common::instance::GIMMICK_TIMER) == 850 {
                let handle = VarModule::get_int(boma.object(), vars::pichu::instance::CHARGE_EFFECT_HANDLER);
                EffectModule::set_scale(boma, handle as u32, &Vector3f{ x: 0.6, y: 0.6, z: 0.6 });
            }
            if VarModule::get_int(boma.object(), vars::common::instance::GIMMICK_TIMER) == 840 {
                let handle = VarModule::get_int(boma.object(), vars::pichu::instance::CHARGE_EFFECT_HANDLER);
                EffectModule::set_scale(boma, handle as u32, &Vector3f{ x: 0.5, y: 0.5, z: 0.5 });
            }
            if VarModule::get_int(boma.object(), vars::common::instance::GIMMICK_TIMER) == 828 {
                STOP_SE(get_fighter_common_from_accessor(boma), Hash40::new("vc_pichu_final01"));
            }
        }
        if VarModule::get_int(boma.object(), vars::common::instance::GIMMICK_TIMER) <= 0 {
            VarModule::set_int(boma.object(), vars::pichu::instance::CHARGE_LEVEL, 0);
        }
    }
}

// handles the damage multipliers
unsafe fn charge_state_damage_multipliers(boma: &mut BattleObjectModuleAccessor) {
    if VarModule::get_int(boma.object(), vars::pichu::instance::CHARGE_LEVEL) == 0 {
        VarModule::set_float(boma.object(), vars::pichu::instance::CHARGE_DAMAGE_MUL, 1.0);
        VarModule::set_float(boma.object(), vars::pichu::instance::CHARGE_RECOIL_MUL, 1.0);
    }
    else if VarModule::get_int(boma.object(), vars::pichu::instance::CHARGE_LEVEL) == 1 {
        VarModule::set_float(boma.object(), vars::pichu::instance::CHARGE_DAMAGE_MUL, 1.2);
        VarModule::set_float(boma.object(), vars::pichu::instance::CHARGE_RECOIL_MUL, 1.25);
    }
}

// charge status resets on death and game end
unsafe fn charge_state_reset(boma: &mut BattleObjectModuleAccessor) {
    if lua_bind::FighterManager::is_result_mode(utils::singletons::FighterManager()) {
        VarModule::set_int(boma.object(), vars::pichu::instance::CHARGE_LEVEL, 0);
        VarModule::set_int(boma.object(), vars::common::instance::GIMMICK_TIMER, 0);
        VarModule::set_int(boma.object(), vars::pichu::instance::CHARGE_EFFECT_HANDLER, -1);
        MeterModule::reset(boma.object());
    }
    
    if boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_WIN,
        *FIGHTER_STATUS_KIND_LOSE,
        *FIGHTER_STATUS_KIND_ENTRY,]) {
        VarModule::set_int(boma.object(), vars::common::instance::GIMMICK_TIMER, 0);
        VarModule::set_int(boma.object(), vars::pichu::instance::CHARGE_LEVEL, 0);
        VarModule::set_int(boma.object(), vars::pichu::instance::CHARGE_EFFECT_HANDLER, -1);
        MeterModule::reset(boma.object());
    }

    if boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_REBIRTH]) {
            VarModule::set_int(boma.object(), vars::common::instance::GIMMICK_TIMER, 0);
        }
}

// handles the effects of pichu's charged state
unsafe fn charge_state_effects(boma: &mut BattleObjectModuleAccessor) {
    if VarModule::get_int(boma.object(), vars::pichu::instance::CHARGE_LEVEL) == 1
    && VarModule::get_int(boma.object(), vars::pichu::instance::CHARGE_EFFECT_HANDLER) == -1 {
        app::FighterUtil::flash_eye_info(boma);
        let handle = EffectModule::req_follow(boma, Hash40::new("pichu_final_hold"), Hash40::new("waist"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &Vector3f::zero(), 0.9, true, 0, 0, 0, 0, 0, true, true) as u32;
        VarModule::set_int(boma.object(), vars::pichu::instance::CHARGE_EFFECT_HANDLER, handle as i32);
        PLAY_SE(get_fighter_common_from_accessor(boma), Hash40::new("vc_pichu_final01"));
        PLAY_SE(get_fighter_common_from_accessor(boma), Hash40::new("se_pichu_final02"));
    }
    else if VarModule::get_int(boma.object(), vars::pichu::instance::CHARGE_LEVEL) == 0 && VarModule::get_int(boma.object(), vars::pichu::instance::CHARGE_EFFECT_HANDLER) != -1 {
        let handle = VarModule::get_int(boma.object(), vars::pichu::instance::CHARGE_EFFECT_HANDLER) as u32;
        EffectModule::kill(boma, handle, false, false);
        VarModule::set_int(boma.object(), vars::pichu::instance::CHARGE_EFFECT_HANDLER, -1);
    }
}

unsafe fn discharge_momentum(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_LW_HIT) && VarModule::is_flag(fighter.battle_object, vars::pichu::instance::IS_CHARGE_ATTACK) {
        let air_brake_x = WorkModule::get_param_float(fighter.module_accessor, smash::hash40("air_brake_x"), 0);
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, air_brake_x, 0.0);
        app::sv_kinetic_energy::set_brake(fighter.lua_state_agent);
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    charge_state_increase(boma);
    charge_state_decrease(boma);
    charge_state_damage_multipliers(boma);
    charge_state_reset(boma);
    charge_state_effects(boma);
    discharge_momentum(fighter);
}

#[utils::macros::opff(FIGHTER_KIND_PICHU )]
pub fn pichu_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		pichu_frame(fighter);
        electric_rats_common(fighter);
    }
}

pub unsafe fn pichu_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}