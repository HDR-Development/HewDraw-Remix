use super::*;


unsafe extern "C" fn special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
    special_lw_mot_helper(fighter);
    fighter.main_shift(special_lw_main_loop)
}

unsafe extern "C" fn special_lw_mot_helper(fighter: &mut L2CFighterCommon) {
    let cont = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
    let mot;
    let correct;
    if fighter.global_table[globals::SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        mot = Hash40::new("special_air_lw_hit");
        correct = *GROUND_CORRECT_KIND_AIR;
    }
    else {
        mot = Hash40::new("special_lw_hit");
        correct = *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP;
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_IGNORE_NORMAL);
    }
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(correct));
    if !cont {
        MotionModule::change_motion(
            fighter.module_accessor,
            mot,
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
    }
    else {
        MotionModule::change_motion_inherit_frame_keep_rate(
            fighter.module_accessor,
            mot,
            -1.0,
            1.0,
            0.0
        );
    }
    // <HDR>
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON) {
        ArticleModule::change_motion(
            fighter.module_accessor,
            *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON,
            mot,
            cont,
            -1.0
        );
    }
    // </HDR>
}

unsafe extern "C" fn special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        special_lw_mot_helper(fighter);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        let status = if fighter.global_table[globals::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            FIGHTER_STATUS_KIND_WAIT
        }
        else {
            FIGHTER_STATUS_KIND_FALL
        };
        fighter.change_status(status.into(), false.into());
        return 0.into();
    }
    // <HDR>
    // Commented out because we don't need it lmao
    // let rate = if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_SPECIAL_LW_FLAG_APPLY_POWERUP_MOTION_RATE) {
    //     1.0
    // }
    // else {
    //     WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("motion_rate"))
    // };
    // MotionModule::set_rate(fighter.module_accessor, rate);
    // </HDR>
    0.into()
}


unsafe extern "C" fn special_lw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[globals::STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_FINAL_VISUAL_ATTACK_OTHER {
        if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON) {
            ArticleModule::remove(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            VisibilityModule::set_whole(fighter.module_accessor, true);
        }
    }
    0.into()
}



pub fn install() {
    smashline::Agent::new("kamui")
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_main)
        .status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_end)
        .install();
}
