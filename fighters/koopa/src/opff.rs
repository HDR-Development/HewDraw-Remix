// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

use vars::koopa::{
    instance::*,
    status::*
};

// symbol-based call for the pikachu/pichu characters' common opff
extern "Rust" {
    fn gimmick_flash(boma: &mut BattleObjectModuleAccessor);
}
 
unsafe fn bowser_bomb(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_LW_G]) {
        if boma.status_frame() >= 14 && boma.status_frame() < 30 {
            let stick_x = boma.stick_x();
            if stick_x != 0.0 {
                let motion_vec = x_motion_vec(1.0, stick_x);
                KineticModule::add_speed_outside(boma, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
            }
        }
        if boma.status_frame() >= 20 && boma.status_frame() < 30 {
            if boma.is_situation(*SITUATION_KIND_AIR) {
                boma.check_jump_cancel(false, false);
            }
        }
    }
}

// Bowser Flame Cancel
unsafe fn flame_cancel(boma: &mut BattleObjectModuleAccessor) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if boma.is_status(*FIGHTER_STATUS_KIND_SPECIAL_N) {
        if boma.motion_frame() < 22.0 && !boma.is_motion_one_of(&[Hash40::new("special_n_max"), Hash40::new("special_air_n_max")]) {
            if boma.is_situation(*SITUATION_KIND_GROUND) && StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR {
                MotionModule::set_frame(boma, 22.0, true);
            }
        }
    }
}

unsafe fn fireball_cooldown(boma: &mut BattleObjectModuleAccessor) {
    //Ignore cooldown during respawn,death,entry and nspecial
    if boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_ENTRY, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_REBIRTH,
        *FIGHTER_STATUS_KIND_WIN, *FIGHTER_STATUS_KIND_LOSE,
        *FIGHTER_STATUS_KIND_SPECIAL_N
    ]) {
        return;
    }

    let charged_effect = VarModule::get_int(boma.object(), vars::koopa::instance::SPECIAL_N_FIREBALL_EFFECT_ID);
    //If cooling down, remove ready effect
    if VarModule::get_int(boma.object(), vars::koopa::instance::SPECIAL_N_FIREBALL_COOLDOWN) > 0 {
        VarModule::dec_int(boma.object(), vars::koopa::instance::SPECIAL_N_FIREBALL_COOLDOWN);
        if charged_effect > 0 {
            VarModule::set_int(boma.object(), vars::koopa::instance::SPECIAL_N_FIREBALL_EFFECT_ID, 0);
            if EffectModule::is_exist_effect(boma, charged_effect as u32) {
                EffectModule::kill(boma, charged_effect as u32, false,false);
            }
        }
        return;
    }
    //Otherwise, spawn effect if effect does not exist
    else if (charged_effect <= 0 || !EffectModule::is_exist_effect(boma, charged_effect as u32)) {
        if (charged_effect <= 0) {
            gimmick_flash(boma);
        }
        let pos = &Vector3f{x: 0.0, y: 1.0, z: 0.0};
        let rot = &Vector3f{x: 180.0, y: 0.0, z: 50.0};
        let handle = EffectModule::req_follow(boma, Hash40::new("koopa_breath_m_fire"), Hash40::new("jaw"), pos, rot, 1.0, true, 0, 0, 0, 0, 0, false, false) as u32;
        EffectModule::set_scale(boma, handle, &Vector3f::new(1.1, 1.3, 1.1));
        VarModule::set_int(boma.object(), vars::koopa::instance::SPECIAL_N_FIREBALL_EFFECT_ID, handle as i32);
    }
}

// opff for handling the "excellent" punch 
unsafe fn ex_punch(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_ATTACK_S4_HOLD) {
        if fighter.status_frame() == 51 { // indicates start of "excellent" frame window
            VarModule::on_flag(boma.object(), ATTACK_S4_EXCELLENT_PUNCH);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_level_up"), Hash40::new("handr"), 3, 0, 0, 0, 0, 0, 0.4, true);
            LAST_EFFECT_SET_RATE(fighter, 3.0);
        } else if fighter.status_frame() == 58 { // window ends
            VarModule::off_flag(boma.object(), ATTACK_S4_EXCELLENT_PUNCH);
        }
    }
    if fighter.is_status(*FIGHTER_STATUS_KIND_ATTACK_S4) 
    && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
    && VarModule::is_flag(boma.object(), ATTACK_S4_EXCELLENT_PUNCH) {
        VarModule::off_flag(boma.object(), ATTACK_S4_EXCELLENT_PUNCH);
        SlowModule::set_whole(boma, 8, 25);
        PLAY_SE(fighter, Hash40::new("se_common_criticalhit"));
        PLAY_SE(fighter, Hash40::new("se_koopa_final06")); // excellent sfx
        EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_fire"), Hash40::new("handr"), 3, 0, 0, 0, 0, 0, 1.0, true);
        EffectModule::req_screen(boma, Hash40::new("bg_criticalhit"), false, true, true);
    }
}

pub unsafe fn initialize_fireball(fighter: &mut L2CFighterCommon) {
    if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_INIT) {
        return;
    }
    //Grant fireball during training mode
    if is_training_mode() {
        VarModule::set_int(fighter.battle_object, vars::koopa::instance::SPECIAL_N_FIREBALL_COOLDOWN, 0);
    }
    else {
        VarModule::set_int(fighter.battle_object, vars::koopa::instance::SPECIAL_N_FIREBALL_COOLDOWN, MAX_COOLDOWN);
    }
    VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_INIT);
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status(*FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_A) && fighter.motion_frame() >= 60.0 {
        fighter.sub_air_check_dive();
    }
}

pub unsafe fn moveset(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    bowser_bomb(boma);
    flame_cancel(boma);
    fireball_cooldown(boma);
    ex_punch(fighter, boma);
    initialize_fireball(fighter);
    fastfall_specials(fighter);
}

pub extern "C" fn koopa_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		koopa_frame(fighter)
    }
}

pub unsafe fn koopa_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, koopa_frame_wrapper);
}