use super::*;
use globals::*;


// FIGHTER_STATUS_KIND_SPECIAL_N

#[status_script(agent = "duckhunt", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_air_n"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
    }
    else {
        GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_n"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
    }
    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_DUCKHUNT_GENERATE_ARTICLE_RETICLE, Hash40::new("special_n"), true, -1.0);
    if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_DUCKHUNT_GENERATE_ARTICLE_CAN) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DUCKHUNT_INSTANCE_WORK_ID_FLAG_RELEASE_CAN);
        //ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_DUCKHUNT_GENERATE_ARTICLE_CAN, false, -1);
    }
    fighter.main_shift(special_n_main_loop)
}

unsafe extern "C" fn special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    // <HDR>
    if fighter.global_table[CURRENT_FRAME].get_i32() == 6 {
        if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_DUCKHUNT_GENERATE_ARTICLE_CAN) {
            WorkModule::enable_transition_term_forbid(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_DUCKHUNT_GENERATE_ARTICLE_CAN, false, -1);
        }
    }
    // </HDR>
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
            fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL, false);
        }
        else {
            fighter.change_status_req(*FIGHTER_STATUS_KIND_WAIT, false);
        }
        return 0.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
                GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n"), -1.0, 1.0, 0.0, false, false);
            }
            else {
                GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n"), -1.0, 1.0, 0.0, false, false);
            }
        }
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        special_n_main
    );
}