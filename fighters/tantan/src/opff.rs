// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

//TODO: Figure out how to cancel arm recoil with a tilt/aerial, and if it's worth implementing
unsafe fn recoil_cancel(boma: &mut BattleObjectModuleAccessor,status: i32,situation_kind: i32){

    if !VarModule::is_flag(boma.object(), vars::tantan::status::ARMS_ATTACK_CANCEL) {return;}

    let mut new_status = 0;
    if boma.is_cat_flag(Cat1::AttackS4) {
        new_status = *FIGHTER_STATUS_KIND_ATTACK_S4_START;
    } else if boma.is_cat_flag(Cat1::AttackHi4) {
        new_status = *FIGHTER_STATUS_KIND_ATTACK_HI4_START;
    } else if boma.is_cat_flag(Cat1::AttackLw4) {
        new_status = *FIGHTER_STATUS_KIND_ATTACK_LW4_START;
    } else if boma.is_cat_flag(Cat1::AttackS3) {
        new_status = *FIGHTER_STATUS_KIND_ATTACK_S3;
    } else if boma.is_cat_flag(Cat1::AttackHi3) {
        new_status = *FIGHTER_STATUS_KIND_ATTACK_HI3;
    } else if boma.is_cat_flag(Cat1::AttackLw3) {
        new_status = *FIGHTER_STATUS_KIND_ATTACK_LW3;
    } else if boma.is_cat_flag(Cat1::AttackN) {
        new_status = *FIGHTER_STATUS_KIND_ATTACK;
    }
    if (new_status>0){
        if (situation_kind!=*SITUATION_KIND_AIR) {
            StatusModule::change_status_request_from_script(boma, new_status, false);
        }
        else{
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
        }
        VarModule::off_flag(boma.object(), vars::tantan::status::ARMS_ATTACK_CANCEL);
    }
}

unsafe fn arms_switch_during_normals(boma: &mut BattleObjectModuleAccessor, cat1: i32, status_kind: i32, situation_kind: i32, motion_kind: u64) {
    if [*FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4].contains(&status_kind)
    || ([*FIGHTER_STATUS_KIND_ATTACK].contains(&status_kind) && motion_kind == hash40("attack_13")){
        if !boma.is_in_hitlag() {
            if boma.is_cat_flag(Cat1::SpecialLw) {
                WorkModule::on_flag(boma,*FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_CHANGE_PUNCH_R);
                boma.clear_commands(Cat1::SpecialLw); 
            }
        }
    }
}

unsafe fn double_dragon(boma: &mut BattleObjectModuleAccessor)
{
    let dragonEffect = VarModule::get_int(boma.object(),vars::tantan::instance::DRAGONIZE_R_EFFECT_HANDLE) as u32;
    let armType =  WorkModule::get_int(boma, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_PUNCH_KIND_R);
    if WorkModule::is_flag(boma, *FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_DRAGONIZE_L) {
        let bigScale = WorkModule::get_param_float(boma,hash40("param_private"),hash40("arm_l_big_scale"));
        ModelModule::set_joint_scale(boma, Hash40::new("pr1_have"), &Vector3f::new(bigScale, bigScale, bigScale));

        if !EffectModule::is_exist_effect(boma, dragonEffect) {
            let handle = EffectModule::req_follow(boma, Hash40::new("tantan_dragon_fire"), Hash40::new("pr1_gimmickc"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, bigScale, true, 0, 0, 0, 0, 0, false, false);
            VarModule::set_int(boma.object(),vars::tantan::instance::DRAGONIZE_R_EFFECT_HANDLE,handle as i32);
        }
        else if !ArticleModule::is_exist(boma, *FIGHTER_TANTAN_GENERATE_ARTICLE_SPIRALRIGHT)
        && armType==0 {
            EffectModule::set_scale(boma, dragonEffect, &Vector3f::new(1.0, 1.0, 1.0));
        }
        else{
            EffectModule::set_scale(boma, dragonEffect, &Vector3f::zero());
        }
    }
    else{
        ModelModule::set_joint_scale(boma, Hash40::new("pr1_main"), &Vector3f::new(1.0, 1.0, 1.0));
        if dragonEffect > 0 {
            EffectModule::kill(boma, dragonEffect, false,false);
            VarModule::set_int(boma.object(),vars::tantan::instance::DRAGONIZE_R_EFFECT_HANDLE,0);
        }
    }
}
unsafe fn fsmash_effect_translation(boma: &mut BattleObjectModuleAccessor, status_kind: i32)
{
    if status_kind != *FIGHTER_STATUS_KIND_ATTACK_S4 {
        return;
    }
    if AttackModule::is_attack(boma, 0, false) {
        let startFrame = 18.0;
        let newpos = 2.0*(MotionModule::frame(boma)-startFrame);
        ModelModule::set_joint_translate(boma, Hash40::new("pl1_muzzle_eff"), &Vector3f::new(0.0, newpos, 0.0),false,false);
        ModelModule::set_joint_translate(boma, Hash40::new("pr1_muzzle_eff"), &Vector3f::new(0.0, newpos, 0.0),false,false);
    }
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    recoil_cancel(boma,status_kind,situation_kind);
    arms_switch_during_normals(boma, cat[0], status_kind, situation_kind, motion_kind);
    double_dragon(boma);
    fsmash_effect_translation(boma,status_kind);
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
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, tantan_frame_wrapper);
}
