use super::*;

pub unsafe extern "C" fn bomb_jump_g_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    smashline::original_status(Main, fighter, *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_G)(fighter);
    fighter.main_shift(special_lw_common_main_loop)
}

pub unsafe extern "C" fn special_ground_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);

    smashline::original_status(Main, fighter, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW)(fighter);

    if VarModule::is_flag(fighter.battle_object, vars::samus::instance::SPECIAL_LW_INPUT_FROM_CRAWL) {
        //Keep speed from crawl
        VarModule::on_flag(fighter.battle_object, vars::samus::status::SPECIAL_LW_INPUT_IN_CRAWL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_MV);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_MV_CONT);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_JUMP);

        let sp_lw_gr_ax_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("sp_lw_gr_ax_mul"));
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST, speed_x, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(controller_set_accel_x_add, fighter, sp_lw_gr_ax_mul);

        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        let stable_speed_x = sv_kinetic_energy::get_stable_speed_x(fighter.lua_state_agent);
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        let stable_speed_y = sv_kinetic_energy::get_stable_speed_y(fighter.lua_state_agent);
        fighter.clear_lua_stack();

        let sp_lw_gr_vx_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("sp_lw_gr_vx_mul"));
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, stable_speed_x*sp_lw_gr_vx_mul, stable_speed_y);

        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
    VarModule::off_flag(fighter.battle_object, vars::samus::instance::SPECIAL_LW_INPUT_FROM_CRAWL);

    fighter.main_shift(special_lw_common_main_loop)
}

pub unsafe extern "C" fn special_lw_common_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, vars::samus::status::SPECIAL_LW_INPUT_IN_CRAWL) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_JUMP);
    }
    if morph_force_crawl(fighter).get_i32() == 1 {
        return 1.into();
    };
    if !fighter.is_motion(Hash40::new("special_lw")) {
        let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor, Hash40::new("special_lw"), true) as i32;
        if fighter.status_frame() >= cancel_frame {
            fighter.change_status(FIGHTER_STATUS_KIND_SQUAT_WAIT.into(), false.into());
        }
    }

    /* Original */
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(FIGHTER_STATUS_KIND_WAIT.into(), FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.global_table[CURRENT_FRAME].get_f32() > 1.0 {
            if fighter.is_situation(*SITUATION_KIND_AIR) {
                fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_A.into(), false.into());
                return 1.into();
            }
        }
    }
    //Check bomb
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() != *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_G {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_WEAPON) {
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_WEAPON);
                let max_bomb = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), hash40("bomb_max_req"));
                if ArticleModule::get_active_num(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_BOMB) < max_bomb {
                    ArticleModule::generate_article_enable(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_BOMB, false, -1);
                    ArticleModule::shoot_exist(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_BOMB, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
                }
            }
        }
    }

    return 0.into();
}

unsafe extern "C" fn morph_force_crawl(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_CHK_CROUCH) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_CHK_CROUCH);
        let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
        let stick_y_sensitivity = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), Hash40::new_raw(0x10d088fec9).hash);
        if stick_y < stick_y_sensitivity {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_CHK_CROUCH);
            ControlModule::clear_command(fighter.module_accessor, false);
            let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor, Hash40::new("special_lw"), true) as i32;
            let lock_frame = (cancel_frame - fighter.status_frame()).max(0);
            VarModule::set_int(fighter.battle_object, vars::samus::instance::SPECIAL_LW_BOMB_LOCKOUT, lock_frame);
            fighter.change_status(FIGHTER_STATUS_KIND_SQUAT_WAIT.into(), false.into());
            return 1.into();
        }
    }

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_G, bomb_jump_g_main);
    agent.status(Main, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW, special_ground_lw_main);
}
