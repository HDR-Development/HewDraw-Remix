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
unsafe fn charge_state_increase(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, frame: f32) {
    MeterModule::update(boma.object(), false);
    if VarModule::get_int(boma.object(), vars::pichu::instance::CHARGE_LEVEL) == 0 {
        if MeterModule::level(boma.object()) >= 2 {
            VarModule::set_int(boma.object(), vars::common::instance::GIMMICK_TIMER, 900);
            VarModule::on_flag(boma.object(), vars::pichu::instance::CHARGE_EFFECT_ON);
            MeterModule::reset(boma.object());
            gimmick_flash(boma);
            VarModule::set_int(boma.object(), vars::pichu::instance::CHARGE_LEVEL, 1);
        }
    }
}

// handles pichu's overcharged state
unsafe fn charge_state_overcharge(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, frame: f32) {
    if VarModule::get_int(boma.object(), vars::pichu::instance::CHARGE_LEVEL) == 1 {
        if VarModule::get_int(boma.object(), vars::common::instance::GIMMICK_TIMER) > 0 {
            VarModule::inc_int(boma.object(), vars::pichu::instance::CHARGE_EFFECT_REFRESH_TIMER);
            if VarModule::get_int(boma.object(), vars::pichu::instance::CHARGE_EFFECT_REFRESH_TIMER) > 15 {
                VarModule::on_flag(boma.object(), vars::pichu::instance::CHARGE_EFFECT_ON);
                VarModule::set_int(boma.object(), vars::pichu::instance::CHARGE_EFFECT_REFRESH_TIMER, 0);
            }
            VarModule::dec_int(boma.object(), vars::common::instance::GIMMICK_TIMER);
        }
        if VarModule::get_int(boma.object(), vars::common::instance::GIMMICK_TIMER) <= 0 {
            VarModule::on_flag(boma.object(), vars::pichu::instance::CHARGE_EFFECT_OFF);
            VarModule::set_int(boma.object(), vars::pichu::instance::CHARGE_LEVEL, 0);
            gimmick_flash(boma);
        }
    }
}

// handles the damage multipliers
unsafe fn charge_state_damage_multipliers(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, frame: f32) {
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
unsafe fn charge_state_reset(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, frame: f32) {
    if lua_bind::FighterManager::is_result_mode(utils::singletons::FighterManager()) {
        VarModule::set_int(boma.object(), vars::pichu::instance::CHARGE_EFFECT_REFRESH_TIMER, 0);
        VarModule::on_flag(boma.object(), vars::pichu::instance::CHARGE_EFFECT_OFF);
        VarModule::set_int(boma.object(), vars::pichu::instance::CHARGE_LEVEL, 0);
        VarModule::set_int(boma.object(), vars::common::instance::GIMMICK_TIMER, 0);
        MeterModule::reset(boma.object());
    }
    
    if boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_WIN,
        *FIGHTER_STATUS_KIND_LOSE,
        *FIGHTER_STATUS_KIND_ENTRY,]) {
        VarModule::set_int(boma.object(), vars::common::instance::GIMMICK_TIMER, 0);
        VarModule::set_int(boma.object(), vars::pichu::instance::CHARGE_EFFECT_REFRESH_TIMER, 0);
        VarModule::on_flag(boma.object(), vars::pichu::instance::CHARGE_EFFECT_OFF);
        VarModule::set_int(boma.object(), vars::pichu::instance::CHARGE_LEVEL, 0);
        MeterModule::reset(boma.object());
    }

    if boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_REBIRTH]) {
            VarModule::set_int(boma.object(), vars::common::instance::GIMMICK_TIMER, 0);
        }
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    charge_state_increase(boma, id, status_kind, frame);
    charge_state_overcharge(boma, id, status_kind, frame);
    charge_state_damage_multipliers(boma, id, status_kind, frame);
    charge_state_reset(boma, id, status_kind, frame);
}

#[utils::macros::opff(FIGHTER_KIND_PICHU )]
pub fn pichu_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		pichu_frame(fighter);
        charge_state_effects(fighter);
        electric_rats_common(fighter);
    }
}

pub unsafe fn pichu_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

unsafe fn charge_state_effects(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if VarModule::is_flag(fighter.battle_object, vars::pichu::instance::CHARGE_EFFECT_ON) && !fighter.is_status_one_of(&[*FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_END]) {
        EFFECT_FOLLOW(fighter, Hash40::new("pichu_elec2"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 1.1, true);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pichu_cheek"), Hash40::new("head"), 0, 0, 0, 0, -90, -90, 1, true);
        VarModule::off_flag(fighter.battle_object, vars::pichu::instance::CHARGE_EFFECT_ON);
    }
    if VarModule::is_flag(fighter.battle_object, vars::pichu::instance::CHARGE_EFFECT_OFF) || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_FALL_SPECIAL]) {
        EFFECT_OFF_KIND(fighter, Hash40::new("pichu_elec2"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("pichu_cheek"), false, true);
        VarModule::off_flag(fighter.battle_object, vars::pichu::instance::CHARGE_EFFECT_OFF);
    }
}