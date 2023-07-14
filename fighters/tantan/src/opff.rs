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

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    recoil_cancel(boma,status_kind,situation_kind);
    arms_switch_during_normals(boma, cat[0], status_kind, situation_kind, motion_kind);
    double_dragon(boma);
    //Prevent B Jab
    WorkModule::off_flag(boma, *FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_ATTACK_COMBO_ENABLE);
}

#[utils::macros::opff(FIGHTER_KIND_TANTAN )]
pub fn tantan_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
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


/// prevents rocket from despawning in the blastzone
#[weapon_frame( agent = WEAPON_KIND_TANTAN_PUNCH1 )]
fn dragon_frame(weapon: &mut L2CFighterBase) {
    unsafe {
        let boma = weapon.module_accessor;

        let mut is_dragon = WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_IS_DRAGONIZE);
        if !WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_IS_LEFT)
        && !WorkModule::is_flag(boma, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_IS_KIRBY)
        {
            let minmin_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
            if sv_battle_object::is_active(minmin_id) {
                let minmin = utils::util::get_battle_object_from_id(minmin_id);
                let minmin_boma = &mut *(*minmin).module_accessor;
                let bigScale = WorkModule::get_param_float(minmin_boma,hash40("param_private"),hash40("arm_l_big_scale"));

                //Only update if previously was not dragonized
                if !is_dragon {
                    is_dragon = WorkModule::get_int(minmin_boma, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_ARM_L_BIG_FRAME) > 0;
                    WorkModule::set_flag(boma, is_dragon,*WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_IS_DRAGONIZE);  

                    if is_dragon {
                        let handle = EffectModule::req_follow(boma, Hash40::new("tantan_dragon_attack_fire"), Hash40::new("gimmickc"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, bigScale, true, 0, 0, 0, 0, 0, false, false);
                    }
                }
                if is_dragon {
                    PostureModule::set_scale(boma, bigScale, false);
                }
            }
        }
        if is_dragon {
            AttackModule::set_power_mul(boma, 1.15);
        }
    }
}