use super::*;

extern "C" {
    fn palutena_special_n_init_common(fighter: &mut L2CFighterCommon) -> L2CValue;

    fn palutena_special_n_momentum_helper(fighter: &mut L2CFighterCommon, start: L2CValue);
}

unsafe extern "C" fn special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.is_situation(*SITUATION_KIND_GROUND) {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        FighterMotionModuleImpl::change_motion_kirby_copy(
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
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        FighterMotionModuleImpl::change_motion_kirby_copy(
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

    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_PALUTENA_STATUS_SPECIAL_N_FLAG_HAS_TARGET_ARTICLE);
    WorkModule::set_int(fighter.module_accessor, *BATTLE_OBJECT_CATEGORY_INVALID, *FIGHTER_PALUTENA_STATUS_SPECIAL_N_WORK_INT_TARGET_CATEGORY);

    fighter.main_shift(special_n_main_loop)
}

unsafe extern "C" fn special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }

    if MotionModule::is_end(fighter.module_accessor) {
        let status = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            FIGHTER_STATUS_KIND_WAIT
        }
        else {
            FIGHTER_STATUS_KIND_FALL
        };
        fighter.change_status(status.into(), false.into());
        return 0.into();
    }

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PALUTENA_STATUS_SPECIAL_N_FLAG_TARGET_EXIST) {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PALUTENA_STATUS_SPECIAL_N_FLAG_HAS_TARGET_ARTICLE) {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_AUTORETICLE, false, -1);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PALUTENA_STATUS_SPECIAL_N_FLAG_HAS_TARGET_ARTICLE);
        }
        let discharge_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("special_n_discharge_frame")) as f32;
        let shootspan_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("special_n_shootspan_frame")) as f32;
        let discharge_frame2 = discharge_frame + shootspan_frame;
        let discharge_frame3 = discharge_frame2 + shootspan_frame;
        let mut fire = false;
        let frame = MotionModule::frame(fighter.module_accessor);
        if frame >= discharge_frame && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PALUTENA_STATUS_SPECIAL_N_FLAG_SHOT_1ST_DONE) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PALUTENA_STATUS_SPECIAL_N_FLAG_SHOT_1ST_DONE);
            fire = true
        }
        else if frame >= discharge_frame2 && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PALUTENA_STATUS_SPECIAL_N_FLAG_SHOT_2ND_DONE) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PALUTENA_STATUS_SPECIAL_N_FLAG_SHOT_2ND_DONE);
            fire = true
        }
        else if frame >= discharge_frame3 && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PALUTENA_STATUS_SPECIAL_N_FLAG_SHOT_3RD_DONE) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PALUTENA_STATUS_SPECIAL_N_FLAG_SHOT_3RD_DONE);
            fire = true
        }

        if fire {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_AUTOAIMBULLET, false, -1);
        }
    }

    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        palutena_special_n_momentum_helper(fighter, false.into());
        if !fighter.is_situation(*SITUATION_KIND_GROUND) {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            FighterMotionModuleImpl::change_motion_inherit_frame_kirby_copy(
                fighter.module_accessor,
                Hash40::new("special_air_n"),
                -1.0,
                1.0,
                0.0,
                false,
                false
            );
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            FighterMotionModuleImpl::change_motion_inherit_frame_kirby_copy(
                fighter.module_accessor,
                Hash40::new("special_n"),
                -1.0,
                1.0,
                0.0,
                false,
                false
            );
        }
    }

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PALUTENA_STATUS_SPECIAL_N_FLAG_SIGHT_EFFECT_ON) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x3002e74dd7));
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_PALUTENA_STATUS_SPECIAL_N_FLAG_SIGHT_EFFECT_ON);
    }

    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Init, *FIGHTER_KIRBY_STATUS_KIND_PALUTENA_SPECIAL_N, palutena_special_n_init_common);
    agent.status(Main, *FIGHTER_KIRBY_STATUS_KIND_PALUTENA_SPECIAL_N, special_n_main);
}