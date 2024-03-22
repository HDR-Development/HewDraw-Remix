// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

mod copy;

// symbol-based call for the pikachu/pichu characters' common opff
extern "Rust" {
    fn gimmick_flash(boma: &mut BattleObjectModuleAccessor);
}

unsafe fn final_cutter_landing_bugfix(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI2)
    && MotionModule::frame(fighter.module_accessor) <= 2.0 {
        fighter.set_situation(L2CValue::I32(*SITUATION_KIND_AIR));
    }
}

unsafe fn horizontal_cutter(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI) {
        if (((fighter.is_situation(*SITUATION_KIND_GROUND) && fighter.status_frame() == 15)
            || (fighter.is_situation(*SITUATION_KIND_AIR) && fighter.status_frame() == 17))
            && ControlModule::get_stick_x(fighter.module_accessor).abs() >= 0.85)
            && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL_RAW) {
            if ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor) < 0.0 {
                REVERSE_LR(fighter);
            }
            StatusModule::change_status_request_from_script(fighter.module_accessor, statuses::kirby::SPECIAL_HI_H, false);
        }
    }
}

unsafe fn dash_attack_jump_cancels(boma: &mut BattleObjectModuleAccessor) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if boma.is_status(*FIGHTER_STATUS_KIND_ATTACK_DASH)
    && boma.is_situation(*SITUATION_KIND_AIR) {
        if MotionModule::frame(boma) >= 43.0 {
            boma.change_status_req(*FIGHTER_STATUS_KIND_FALL, true);
        }
    }
}

// unsafe fn disable_dash_attack_slideoff(fighter: &mut L2CFighterCommon) {
//     if fighter.is_status(*FIGHTER_STATUS_KIND_ATTACK_DASH) && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
//         VarModule::off_flag(fighter.battle_object, vars::common::status::ATTACK_DASH_ENABLE_AIR_FALL);
//         VarModule::off_flag(fighter.battle_object, vars::common::status::ATTACK_DASH_ENABLE_AIR_CONTINUE);
//     }
// }

// unsafe fn stone_control(fighter: &mut L2CFighterCommon) {
//     if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_LW) && fighter.status_frame() <= 30 {
//         fighter.clear_lua_stack();
//         lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 1.0, 0.0);
//         app::sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
//         fighter.clear_lua_stack();
//     }
// }

unsafe fn hammer_swing_drift_landcancel(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_ATTACK) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) && fighter.is_prev_situation(*SITUATION_KIND_AIR) {
            AttackModule::clear_all(fighter.module_accessor);
            MotionModule::change_motion_force_inherit_frame(fighter.module_accessor, Hash40::new("special_s"), 33.0, 1.0, 1.0);
            MotionModule::set_rate(fighter.module_accessor, (55.0 - 33.0)/25.0);    // equates to 17F landing lag
        }
        if fighter.is_situation(*SITUATION_KIND_AIR) {
            if KineticModule::get_sum_speed_y(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            }
        }
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    let copystatus = StatusModule::status_kind(fighter.module_accessor);
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        if fighter.is_status_one_of(&[
            *FIGHTER_STATUS_KIND_SPECIAL_N,
            *FIGHTER_STATUS_KIND_SPECIAL_S,
            *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_LOOP,
            *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_SPIT,
            *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_DRINK,
            *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_END,
            *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_SWALLOW,
            *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_EAT_FALL,
            *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_EAT_JUMP1,
            *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_EAT_JUMP2,
            *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_EAT_TURN_AIR,
            *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_EAT_WAIT_FALL,
            *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_EAT_WAIT_JUMP,
            *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_FALL,
            *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_JUMP,
            *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_ATTACK,
            *FIGHTER_KIRBY_STATUS_KIND_TRAIL_SPECIAL_N2,
            *FIGHTER_KIRBY_STATUS_KIND_TRAIL_SPECIAL_N3
            ])
        || (0x206..0x377).contains(&copystatus) {
            fighter.sub_air_check_dive();
            if fighter.is_flag(*FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
                if [*FIGHTER_KINETIC_TYPE_MOTION_AIR, *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE].contains(&KineticModule::get_kinetic_type(fighter.module_accessor)) {
                    fighter.clear_lua_stack();
                    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
                    let speed_y = app::sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);

                    fighter.clear_lua_stack();
                    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y, 0.0, 0.0, 0.0);
                    app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);

                    fighter.clear_lua_stack();
                    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                    app::sv_kinetic_energy::enable(fighter.lua_state_agent);

                    KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
                }
            }
        }
    }
}

// Shrink sword
unsafe fn cutter_size(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if ArticleModule::is_exist(boma, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER) {
        let article = ArticleModule::get_article(boma, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER);
        let article_id = smash::app::lua_bind::Article::get_battle_object_id(article) as u32;
        let article_boma = sv_battle_object::module_accessor(article_id);
        let article_motion = MotionModule::motion_kind(article_boma);
        if article_motion == hash40("special_hi2") {
            PostureModule::set_scale(article_boma, 0.8, false);
        }
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    final_cutter_landing_bugfix(fighter);
    horizontal_cutter(fighter);
    dash_attack_jump_cancels(boma);
    //disable_dash_attack_slideoff(fighter);
    //stone_control(fighter);
    hammer_swing_drift_landcancel(fighter);
    fastfall_specials(fighter);
    cutter_size(boma, status_kind);

    copy::kirby_copy_handler(fighter, boma, id, cat, status_kind, situation_kind, motion_kind, stick_x, stick_y, facing, frame);
}

pub extern "C" fn kirby_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		kirby_frame(fighter)
    }
}

pub unsafe fn kirby_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}
pub fn install() {
    smashline::Agent::new("kirby")
        .on_line(Main, kirby_frame_wrapper)
        .install();

}
