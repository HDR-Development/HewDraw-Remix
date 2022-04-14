// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    magic_series(fighter, boma, id, cat, status_kind, situation_kind, motion_kind, stick_x, stick_y, facing, frame);
    special_fadc_super_cancels(boma);
    target_combos(boma);
}

// symbol-based call for the shotos' common opff
extern "Rust" {
    fn shotos_common(fighter: &mut smash::lua2cpp::L2CFighterCommon);
}

#[utils::macros::opff(FIGHTER_KIND_RYU )]
pub fn ryu_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		ryu_frame(fighter);
        shotos_common(fighter);
    }
}

pub unsafe fn ryu_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

unsafe fn special_fadc_super_cancels(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_S,
                               *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND,
                               *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_END,
                               *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP,
                               *FIGHTER_STATUS_KIND_SPECIAL_HI,
                               *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND,
                               *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_JUMP]){
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD){
            VarModule::on_flag(boma.object(), vars::shotos::IS_ENABLE_FADC);
        }
        if VarModule::is_flag(boma.object(), vars::shotos::IS_ENABLE_FADC){
            if boma.is_cat_flag(Cat1::SpecialLw){
                if !StopModule::is_stop(boma) {
                    if MeterModule::drain(boma.object(), 1){
                        boma.change_status_req(*FIGHTER_STATUS_KIND_SPECIAL_LW, true);
                    }
                }
            }
            if boma.is_cat_flag(Cat4::SpecialNCommand | Cat4::SpecialHiCommand){
                if !StopModule::is_stop(boma){
                    if MeterModule::drain(boma.object(), 10) {
                        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL);
                        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_IS_DISCRETION_FINAL_USED);
                        boma.change_status_req(*FIGHTER_STATUS_KIND_FINAL, true);
                    } 
                }
            }
        }
    }
    else{
        VarModule::off_flag(boma.object(), vars::shotos::IS_ENABLE_FADC);
    }
}

// Target combos:
// 1: Prox jab into far heavy jab
// 2: Prox ftilt into light ftilt
unsafe fn target_combos(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_motion(Hash40::new("attack_11_near_s")){
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD){
            if boma.is_cat_flag(Cat1::AttackS3) {
                if !StopModule::is_stop(boma){
                    VarModule::on_flag(boma.object(), vars::shotos::IS_TARGET_COMBO_1);
                    MotionModule::change_motion(boma, Hash40::new("attack_11_s"), -1.0, 1.0, false, 0.0, false, false);
                }
            }
        }
    }
    if boma.is_motion(Hash40::new("attack_near_w")){
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD){
            if boma.is_cat_flag(Cat1::AttackS3) && !VarModule::is_flag(boma.object(), vars::shotos::IS_TARGET_COMBO_2) {
                VarModule::on_flag(boma.object(), vars::shotos::IS_TARGET_COMBO_2);     
                return;
            }
            if VarModule::is_flag(boma.object(), vars::shotos::IS_TARGET_COMBO_2){
                if !StopModule::is_stop(boma){
                    MotionModule::change_motion(boma, Hash40::new("attack_s3_s_w"), -1.0, 1.0, false, 0.0, false, false);
                }
            }
        }
    }
    if VarModule::is_flag(boma.object(), vars::shotos::IS_TARGET_COMBO_2){
        /*
        WorkModule::off_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_NEAR_OPPONENT);
        WorkModule::on_flag(boma, *FIGHTER_RYU_STATUS_ATTACK_FLAG_RELEASE_BUTTON);
        WorkModule::on_flag(boma, *FIGHTER_RYU_STATUS_ATTACK_FLAG_WEAK);
        WorkModule::on_flag(boma, *FIGHTER_RYU_STATUS_ATTACK_FLAG_WEAK_BRANCH_FRAME_FIRST);
        */
        //ControlModule::clear_command(boma, true);
        //ControlModule::reset_button(boma);
    }
    /*
    if boma.is_motion(Hash40::new("attack_s3_s_s")){
        if VarModule::is_flag(boma.object(), vars::shotos::IS_TARGET_COMBO_2){
            MotionModule::change_motion(boma, Hash40::new("attack_s3_s_w"), -1.0, 1.0, false, 0.0, false, false);
        }
    }
    */
    if !(boma.is_motion_one_of(&[Hash40::new("attack_11_near_s"),
                                 Hash40::new("attack_11_s")])){
        VarModule::off_flag(boma.object(), vars::shotos::IS_TARGET_COMBO_1);
    }
    
    if !(boma.is_motion_one_of(&[Hash40::new("attack_near_w"),
                                 Hash40::new("attack_s3_s_s"),
                                 Hash40::new("attack_s3_w")])){
        VarModule::off_flag(boma.object(), vars::shotos::IS_TARGET_COMBO_2);
    }
    
}

unsafe fn jab_cancels(boma: &mut BattleObjectModuleAccessor) {
    let mut new_status = 0;
    // Jab 1 cancels
    if boma.is_motion(Hash40::new("attack_11_w")) {
        if boma.is_cat_flag(Cat1::AttackS3) {
            new_status = *FIGHTER_STATUS_KIND_ATTACK_S3;
        } else if boma.is_cat_flag(Cat1::AttackHi3) {
            new_status = *FIGHTER_STATUS_KIND_ATTACK_HI3;
        } else if boma.is_cat_flag(Cat1::AttackLw3) {
            new_status = *FIGHTER_STATUS_KIND_ATTACK_LW3;
        }
        // Tilt cat flags override smash cat flags, need to check smashes separately after tilts so the smash input can be properly detecetd
        if boma.is_cat_flag(Cat1::AttackS4) {
            new_status = *FIGHTER_STATUS_KIND_ATTACK_S4_START;
        } else if boma.is_cat_flag(Cat1::AttackHi4) {
            new_status = *FIGHTER_STATUS_KIND_ATTACK_HI4_START;
        } else if boma.is_cat_flag(Cat1::AttackLw4) {
            new_status = *FIGHTER_STATUS_KIND_ATTACK_LW4_START;
        } 
        if !(  boma.is_cat_flag(Cat1::AttackS3)
            || boma.is_cat_flag(Cat1::AttackHi3)
            || boma.is_cat_flag(Cat1::AttackLw3)
            || boma.is_cat_flag(Cat1::AttackS4)
            || boma.is_cat_flag(Cat1::AttackHi4)
            || boma.is_cat_flag(Cat1::AttackLw4)  ){
            return;
        }
    }
    // Jab 2 cancels
    else if boma.is_motion(Hash40::new("attack_12")) {
        if boma.is_cat_flag(Cat1::AttackS4) {
            new_status = *FIGHTER_STATUS_KIND_ATTACK_S4_START;
        } else if boma.is_cat_flag(Cat1::AttackHi4) {
            new_status = *FIGHTER_STATUS_KIND_ATTACK_HI4_START;
        } else if boma.is_cat_flag(Cat1::AttackLw4) {
            new_status = *FIGHTER_STATUS_KIND_ATTACK_LW4_START;
        } else {
            return;
        }
    }
    else{
        return;
    }

    if !StopModule::is_stop(boma){
        VarModule::on_flag(boma.object(), vars::shotos::IS_MAGIC_SERIES_CANCEL);
        boma.change_status_req(new_status, false);
    }
}

unsafe fn tilt_cancels(boma: &mut BattleObjectModuleAccessor) {

    let mut new_status = 0;
    if boma.is_motion(Hash40::new("attack_hi3_w"))
    || boma.is_motion(Hash40::new("attack_lw3_w"))
    || boma.is_motion(Hash40::new("attack_lw3_s")) 
    || boma.is_motion(Hash40::new("attack_s3_s_w"))
    || boma.is_motion(Hash40::new("attack_near_w")) {
        if boma.is_cat_flag(Cat1::AttackS4) {
            new_status = *FIGHTER_STATUS_KIND_ATTACK_S4_START;
        } else if boma.is_cat_flag(Cat1::AttackHi4) {
            new_status = *FIGHTER_STATUS_KIND_ATTACK_HI4_START;
        } else if boma.is_cat_flag(Cat1::AttackLw4) {
            new_status = *FIGHTER_STATUS_KIND_ATTACK_LW4_START;
        } else {
            return;
        }
    }
    else if boma.is_motion(Hash40::new("attack_s3_s")) {
        if boma.is_cat_flag(Cat1::AttackS4) {
            new_status = *FIGHTER_STATUS_KIND_ATTACK_S4_START;
        } else {
            return;
        }
    }
    else{
        return;
    }

    if !StopModule::is_stop(boma){
        VarModule::on_flag(boma.object(), vars::shotos::IS_MAGIC_SERIES_CANCEL);
        boma.change_status_req(new_status, false);
    }
}

unsafe fn smash_cancels(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND);
    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND);
    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND);
    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_COMMAND1);

    let mut new_status = 0;    

    if !boma.is_status(*FIGHTER_STATUS_KIND_ATTACK_S4){
        // Jump cancel usmash
        if boma.is_status(*FIGHTER_STATUS_KIND_ATTACK_HI4)
        && boma.is_input_jump()
        && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
        {
            if !StopModule::is_stop(boma){
                VarModule::on_flag(boma.object(), vars::shotos::IS_MAGIC_SERIES_CANCEL);
                boma.change_status_req(*FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                return;
            }
        }
        // Special cancels
        if boma.is_cat_flag(Cat1::SpecialN) {
            new_status = *FIGHTER_STATUS_KIND_SPECIAL_N;
        } else if boma.is_cat_flag(Cat4::SpecialNCommand) {
            new_status = *FIGHTER_RYU_STATUS_KIND_SPECIAL_N_COMMAND;
        } else if boma.is_cat_flag(Cat1::SpecialN2Command) {
            new_status = *FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND;
        } else if boma.is_cat_flag(Cat1::SpecialS) {
            new_status = *FIGHTER_STATUS_KIND_SPECIAL_S;
        } else if boma.is_cat_flag(Cat4::SpecialSCommand) {
            new_status = *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND;
        } else if boma.is_cat_flag(Cat1::SpecialHi) {
            new_status = *FIGHTER_STATUS_KIND_SPECIAL_HI;
        } else if boma.is_cat_flag(Cat4::SpecialHiCommand) {
            new_status = *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND;
        } else if boma.is_cat_flag(Cat1::SpecialLw) {
            new_status = *FIGHTER_STATUS_KIND_SPECIAL_LW
        }
    }
    else{
        return;
    }
    if !StopModule::is_stop(boma){
        VarModule::on_flag(boma.object(), vars::shotos::IS_MAGIC_SERIES_CANCEL);
        boma.change_status_req(new_status, false);
    }
    
}

unsafe fn aerial_cancels(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_input_jump()
    && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
    && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX)
    {
        if !StopModule::is_stop(boma){
            boma.change_status_req(*FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
            return;
        }
    }

    let dir = boma.get_aerial();
    if dir == None {
        return;
    }
    match MotionModule::motion_kind(boma) {
        super::hash40!("attack_air_n") if matches!(dir, Some(AerialKind::Nair)) => return,
        super::hash40!("attack_air_f") if matches!(dir, Some(AerialKind::Nair) | Some(AerialKind::Fair)) => return,
        super::hash40!("attack_air_b") => return,
        super::hash40!("attack_air_hi") if !matches!(dir, Some(AerialKind::Bair) | Some(AerialKind::Dair)) => return,
        super::hash40!("attack_air_lw") if !matches!(dir, Some(AerialKind::Bair)) => return,
        _ => {
            if VarModule::get_int(boma.object(), vars::shotos::AIR_CHAIN_COMBO_NUM) < 3 && !StopModule::is_stop(boma) {
                VarModule::on_flag(boma.object(), vars::shotos::IS_MAGIC_SERIES_CANCEL);
                VarModule::inc_int(boma.object(), vars::shotos::AIR_CHAIN_COMBO_NUM);
                boma.change_status_req(*FIGHTER_STATUS_KIND_ATTACK_AIR, false);
            }
            
        }
    }
}

unsafe fn magic_flag_reset(boma: &mut BattleObjectModuleAccessor) {
    if !(boma.is_motion_one_of(&[Hash40::new("attack_12"),
                                 Hash40::new("attack_s3_s_w"),
                                 Hash40::new("attack_s3_s_s"),
                                 Hash40::new("attack_near_w"),
                                 Hash40::new("attack_hi3_w"),
                                 Hash40::new("attack_hi3_s"),
                                 Hash40::new("attack_lw3_w"),
                                 Hash40::new("attack_lw3_s"),
                                 Hash40::new("attack_s4"),
                                 Hash40::new("attack_hi4"),
                                 Hash40::new("attack_lw4")])
        || boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_ATTACK_AIR,
                                   *FIGHTER_STATUS_KIND_SPECIAL_N,
                                   *FIGHTER_RYU_STATUS_KIND_SPECIAL_N_COMMAND,
                                   *FIGHTER_STATUS_KIND_SPECIAL_S,
                                   *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND,
                                   *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_END,
                                   *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP,
                                   *FIGHTER_STATUS_KIND_SPECIAL_HI,
                                   *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND,
                                   *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_JUMP,
                                   *FIGHTER_STATUS_KIND_SPECIAL_LW,
                                   *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_ATTACK,
                                   *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_ATTACK_TURN])){
            VarModule::off_flag(boma.object(), vars::shotos::IS_MAGIC_SERIES_CANCEL);
    }
    // Reset air chain combo number
    if !boma.is_situation(*SITUATION_KIND_AIR){
        VarModule::set_int(boma.object(), 0, vars::shotos::AIR_CHAIN_COMBO_NUM);
    }
}

unsafe fn magic_series(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    
    magic_flag_reset(boma);

    if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) /*&& VarModule::is_flag(boma.object(), vars::shotos::IS_ENABLE_MAGIC_SERIES_CANCEL)*/ {
        return;
    }

    if boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_ATTACK]) {
        jab_cancels(boma);
        return;
    }

    if boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_LW3]) {
        tilt_cancels(boma);
        return;
    }

    if boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4]) {
        smash_cancels(boma);
        return;
    }

    // Aerial Cancels
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
        aerial_cancels(boma);
        return;
    }

}