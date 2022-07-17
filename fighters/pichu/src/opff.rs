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
    if 0 <= VarModule::get_int(boma.object(), vars::pichu::instance::CHARGE_LEVEL) && 2 > VarModule::get_int(boma.object(), vars::pichu::instance::CHARGE_LEVEL) {
        if MeterModule::level(boma.object()) >= 1 && !VarModule::is_flag(boma.object(), vars::pichu::instance::CHARGE_EFFECT_HALF) {
            VarModule::set_int(boma.object(), vars::pichu::instance::CHARGE_LEVEL, 1);
            VarModule::on_flag(boma.object(), vars::pichu::instance::CHARGE_EFFECT_HALF);
            gimmick_flash(boma);
        }
        if MeterModule::level(boma.object()) >= 2 {
            VarModule::set_int(boma.object(), vars::common::instance::GIMMICK_TIMER, 900);
            MeterModule::reset(boma.object());
            VarModule::set_int(boma.object(), vars::pichu::instance::CHARGE_LEVEL, 2);
            VarModule::on_flag(boma.object(), vars::pichu::instance::CHARGE_EFFECT_ON);
            VarModule::off_flag(boma.object(), vars::pichu::instance::CHARGE_EFFECT_HALF);
            gimmick_flash(boma);
        }
    }
}

// handles pichu's overcharged state
unsafe fn charge_state_overcharge(boma: &mut BattleObjectModuleAccessor) {
    if VarModule::get_int(boma.object(), vars::pichu::instance::CHARGE_LEVEL) == 2 {
        if VarModule::get_int(boma.object(), vars::common::instance::GIMMICK_TIMER) > 0 {
            VarModule::dec_int(boma.object(), vars::common::instance::GIMMICK_TIMER);
        }
        if VarModule::get_int(boma.object(), vars::common::instance::GIMMICK_TIMER) <= 0 {
            VarModule::set_int(boma.object(), vars::pichu::instance::CHARGE_LEVEL, 0);
            VarModule::off_flag(boma.object(), vars::pichu::instance::CHARGE_EFFECT_ON);
        }
    }
}

// handles the damage multipliers
unsafe fn charge_state_damage_multipliers(boma: &mut BattleObjectModuleAccessor) {
    if VarModule::get_int(boma.object(), vars::pichu::instance::CHARGE_LEVEL) == 0 {
        VarModule::set_float(boma.object(), vars::pichu::instance::CHARGE_DAMAGE_MUL, 1.0);
        VarModule::set_float(boma.object(), vars::pichu::instance::CHARGE_RECOIL_MUL, 1.0);
    }
    else if VarModule::get_int(boma.object(), vars::pichu::instance::CHARGE_LEVEL) == 2 {
        VarModule::set_float(boma.object(), vars::pichu::instance::CHARGE_DAMAGE_MUL, 1.2);
        VarModule::set_float(boma.object(), vars::pichu::instance::CHARGE_RECOIL_MUL, 1.25);
    }
}

// charge status resets on death and game end
unsafe fn charge_state_reset(boma: &mut BattleObjectModuleAccessor) {
    if lua_bind::FighterManager::is_result_mode(utils::singletons::FighterManager()) {
        VarModule::off_flag(boma.object(), vars::pichu::instance::CHARGE_EFFECT_ON);
        VarModule::off_flag(boma.object(), vars::pichu::instance::CHARGE_EFFECT_HALF);
        VarModule::set_int(boma.object(), vars::pichu::instance::CHARGE_LEVEL, 0);
        VarModule::set_int(boma.object(), vars::common::instance::GIMMICK_TIMER, 0);
        VarModule::set_int(boma.object(), vars::pichu::instance::CHARGE_EFFECT_HANDLER_1, -1);
        VarModule::set_int(boma.object(), vars::pichu::instance::CHARGE_EFFECT_HANDLER_2, -1);
        VarModule::set_int(boma.object(), vars::pichu::instance::CHARGE_EFFECT_HALF_HANDLER, -1);
        MeterModule::reset(boma.object());
    }
    
    if boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_WIN,
        *FIGHTER_STATUS_KIND_LOSE,
        *FIGHTER_STATUS_KIND_ENTRY,]) {
        VarModule::set_int(boma.object(), vars::common::instance::GIMMICK_TIMER, 0);
        VarModule::off_flag(boma.object(), vars::pichu::instance::CHARGE_EFFECT_ON);
        VarModule::off_flag(boma.object(), vars::pichu::instance::CHARGE_EFFECT_HALF);
        VarModule::set_int(boma.object(), vars::pichu::instance::CHARGE_LEVEL, 0);
        VarModule::set_int(boma.object(), vars::pichu::instance::CHARGE_EFFECT_HANDLER_1, -1);
        VarModule::set_int(boma.object(), vars::pichu::instance::CHARGE_EFFECT_HANDLER_2, -1);
        VarModule::set_int(boma.object(), vars::pichu::instance::CHARGE_EFFECT_HALF_HANDLER, -1);
        MeterModule::reset(boma.object());
    }

    if boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_REBIRTH]) {
            VarModule::set_int(boma.object(), vars::common::instance::GIMMICK_TIMER, 0);
        }
}

unsafe fn charge_state_effects(boma: &mut BattleObjectModuleAccessor) {
    if VarModule::get_int(boma.object(), vars::pichu::instance::CHARGE_LEVEL) == 1 && !VarModule::is_flag(boma.object(), vars::pichu::instance::CHARGE_EFFECT_HALF) 
    && VarModule::get_int(boma.object(), vars::pichu::instance::CHARGE_EFFECT_HALF_HANDLER) == -1 {
        let handle_half = EffectModule::req_follow(boma, Hash40::new("pichu_final_sphere_start"), Hash40::new("top"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &Vector3f::zero(), 4.0, true, 0, 0, 0, 0, 0, true, true) as u32;
        VarModule::set_int(boma.object(), vars::pichu::instance::CHARGE_EFFECT_HALF_HANDLER, handle_half as i32);
    }
    else if VarModule::get_int(boma.object(), vars::pichu::instance::CHARGE_LEVEL) == 2 && !VarModule::is_flag(boma.object(), vars::pichu::instance::CHARGE_EFFECT_ON) 
    && (VarModule::get_int(boma.object(), vars::pichu::instance::CHARGE_EFFECT_HANDLER_1) == -1 || VarModule::get_int(boma.object(), vars::pichu::instance::CHARGE_EFFECT_HANDLER_2) == -1) {
        let handle_1 = EffectModule::req_follow(boma, Hash40::new("pichu_final_hold"), Hash40::new("top"), &Vector3f{x: 0.0, y: 5.0, z: 0.0}, &Vector3f::zero(), 0.75, true, 0, 0, 0, 0, 0, true, true) as u32;
        let handle_2 = EffectModule::req_follow(boma, Hash40::new("pichu_cheek"), Hash40::new("head"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &Vector3f::zero(), 1.0, true, 0, 0, 0, 90, 90, true, true) as u32;
        VarModule::set_int(boma.object(), vars::pichu::instance::CHARGE_EFFECT_HANDLER_1, handle_1 as i32);
        VarModule::set_int(boma.object(), vars::pichu::instance::CHARGE_EFFECT_HANDLER_2, handle_2 as i32);
        if VarModule::is_flag(boma.object(), vars::pichu::instance::CHARGE_EFFECT_HALF) && VarModule::get_int(boma.object(), vars::pichu::instance::CHARGE_EFFECT_HALF_HANDLER) != -1 {
            let handle_half = VarModule::get_int(boma.object(), vars::pichu::instance::CHARGE_EFFECT_HALF_HANDLER) as u32;
            EffectModule::kill(boma, handle_half, false, false);
            VarModule::set_int(boma.object(), vars::pichu::instance::CHARGE_EFFECT_HALF_HANDLER, -1);
        }
    }
    else if VarModule::get_int(boma.object(), vars::pichu::instance::CHARGE_LEVEL) == 0 && (VarModule::get_int(boma.object(), vars::pichu::instance::CHARGE_EFFECT_HANDLER_1) != -1 || VarModule::get_int(boma.object(), vars::pichu::instance::CHARGE_EFFECT_HANDLER_2) != -1) {
        let handle_1 = VarModule::get_int(boma.object(), vars::pichu::instance::CHARGE_EFFECT_HANDLER_1) as u32;
        let handle_2 = VarModule::get_int(boma.object(), vars::pichu::instance::CHARGE_EFFECT_HANDLER_2) as u32;
        EffectModule::kill(boma, handle_1, false, false);
        EffectModule::kill(boma, handle_2, false, false);
        VarModule::set_int(boma.object(), vars::pichu::instance::CHARGE_EFFECT_HANDLER_1, -1);
        VarModule::set_int(boma.object(), vars::pichu::instance::CHARGE_EFFECT_HANDLER_2, -1);
    }
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    charge_state_increase(boma);
    charge_state_overcharge(boma);
    charge_state_damage_multipliers(boma);
    charge_state_reset(boma);
    charge_state_effects(boma);
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
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}