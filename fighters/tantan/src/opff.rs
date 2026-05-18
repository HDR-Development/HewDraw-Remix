// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn recoil_cancel(fighter: &mut L2CFighterCommon) {
    // Since we check for the recoil cancel in exec, we need to transition in main or our acmd lags a frame behind
    if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
        VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
        let new_status = VarModule::get_int(fighter.battle_object, vars::tantan::status::RECOIL_CANCEL_STATUS);
        StatusModule::change_status_force(fighter.module_accessor, new_status, false);
    }
}

unsafe fn arms_switch_during_normals(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
        *FIGHTER_STATUS_KIND_ATTACK_AIR,
        *FIGHTER_STATUS_KIND_ATTACK_DASH,
        *FIGHTER_STATUS_KIND_ATTACK_S4,
        *FIGHTER_STATUS_KIND_ATTACK_HI4,
        *FIGHTER_STATUS_KIND_ATTACK_LW4])
    || (boma.is_motion(Hash40::new("attack_13"))) {
        if boma.is_cat_flag(Cat1::SpecialLw) {
            if !boma.is_in_hitlag() {
                WorkModule::on_flag(boma,*FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_CHANGE_PUNCH_R);
                boma.clear_commands(Cat1::SpecialLw); 
            }
        }
    }
}

unsafe fn double_dragon(boma: &mut BattleObjectModuleAccessor) {
    if WorkModule::is_flag(boma, *FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_DRAGONIZE_L) {
        let dragonEffect = VarModule::get_int(boma.object(),vars::tantan::instance::ARMR_DRAGONIZE_EFFECT_HANDLE) as u32;
        let armType =  WorkModule::get_int(boma, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_PUNCH_KIND_R);
        let bigScale = WorkModule::get_param_float(boma,hash40("param_private"),hash40("arm_l_big_scale"));
        ModelModule::set_joint_scale(boma, Hash40::new("pr1_have"), &Vector3f::new(bigScale, bigScale, bigScale));

        if !EffectModule::is_exist_effect(boma, dragonEffect) {
            let handle = EffectModule::req_follow(boma, Hash40::new("tantan_dragon_fire"), Hash40::new("pr1_gimmickc"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, bigScale, true, 0, 0, 0, 0, 0, false, false);
            VarModule::set_int(boma.object(),vars::tantan::instance::ARMR_DRAGONIZE_EFFECT_HANDLE,handle as i32);
        }
        else if !ArticleModule::is_exist(boma, *FIGHTER_TANTAN_GENERATE_ARTICLE_SPIRALRIGHT)
        && armType == 0 {
            EffectModule::set_scale(boma, dragonEffect, &Vector3f::new(1.0, 1.0, 1.0));
        }
        else{
            EffectModule::set_scale(boma, dragonEffect, &Vector3f::zero());
        }
    }
    else {
        let dragonEffect = VarModule::get_int(boma.object(),vars::tantan::instance::ARMR_DRAGONIZE_EFFECT_HANDLE) as u32;
        if dragonEffect > 0 {
            ModelModule::set_joint_scale(boma, Hash40::new("pr1_main"), &Vector3f::new(1.0, 1.0, 1.0));
            EffectModule::kill(boma, dragonEffect, false, false);
            VarModule::set_int(boma.object(), vars::tantan::instance::ARMR_DRAGONIZE_EFFECT_HANDLE, 0);
        }
    }
}

unsafe fn fsmash_effect_translation(boma: &mut BattleObjectModuleAccessor) {
    if !boma.is_status(*FIGHTER_STATUS_KIND_ATTACK_S4) { return; }
    if AttackModule::is_attack(boma, 0, false) {
        let startFrame = 18.0;
        let newpos = 2.0 * (MotionModule::frame(boma) - startFrame);
        ModelModule::set_joint_translate(boma, Hash40::new("pl1_muzzle_eff"), &Vector3f::new(0.0, newpos, 0.0), false, false);
        ModelModule::set_joint_translate(boma, Hash40::new("pr1_muzzle_eff"), &Vector3f::new(0.0, newpos, 0.0), false, false);
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    recoil_cancel(fighter);
    arms_switch_during_normals(boma);
    double_dragon(boma);
    fsmash_effect_translation(boma);
    
    //Prevent B Jab
    WorkModule::off_flag(boma, *FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_ATTACK_COMBO_ENABLE);
}

pub extern "C" fn tantan_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		tantan_frame(fighter);
    }
}

pub unsafe fn tantan_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, tantan_frame_wrapper);
}