use super::*;
use globals::*;
// status script import

unsafe extern "C" fn special_hi_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_FALL,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    return false.into();
}

unsafe extern "C" fn special_hi_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PIKACHU_GENERATE_ARTICLE_SPECIALUPDUMMY, false, -1);
    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PIKACHU_GENERATE_ARTICLE_SPECIALUPDUMMY, Hash40::new("special_air_hi_end"), false, -1.0);
    ArticleModule::set_frame(fighter.module_accessor, *FIGHTER_PIKACHU_GENERATE_ARTICLE_SPECIALUPDUMMY, 0.0);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_end"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_end_main_loop as *const () as _))
}

unsafe extern "C" fn special_hi_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return true.into();
    }

    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if VarModule::is_flag(fighter.battle_object, vars::pichu::status::SPECIAL_HI_QUICK_ATTACK_CANCEL)
        && fighter.is_cat_flag(Cat1::AirEscape) {
            // this is a hack
            fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_AIR.into(), false.into());
            VarModule::on_flag(fighter.battle_object, vars::common::instance::PERFECT_WAVEDASH);
            return true.into();
        }

        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return true.into();
        }
        if fighter.sub_air_check_fall_common().get_bool() {
            PostureModule::add_pos(fighter.module_accessor, &Vector3f::new(0.0, 5.0, 0.0));
            return true.into();
        }
    }

    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if MotionModule::is_end(fighter.module_accessor) {
        let status = if situation_kind == *SITUATION_KIND_GROUND {
            FIGHTER_STATUS_KIND_WAIT
        } else {
            FIGHTER_STATUS_KIND_FALL_SPECIAL
        };
        fighter.change_status(status.into(), false.into());
        return true.into();
    }
    
    if fighter.global_table[CURRENT_FRAME].get_i32() < 2 {
        // allow cancelling out of the status
        if VarModule::is_flag(fighter.battle_object, vars::pichu::status::SPECIAL_HI_QUICK_ATTACK_CANCEL) {
            CancelModule::enable_cancel(fighter.module_accessor);
        }

        if situation_kind == *SITUATION_KIND_GROUND {
            // disable gravity and place pichu in the air
            KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            StatusModule::set_situation_kind(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_AIR), false);
            fighter.global_table[PREV_SITUATION_KIND].assign(&L2CValue::I32(situation_kind));
            fighter.global_table[SITUATION_KIND].assign(&L2CValue::I32(*SITUATION_KIND_AIR));
            GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            GroundModule::set_correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
    } else if situation_kind == *SITUATION_KIND_GROUND {
        let next_status = if CancelModule::is_enable_cancel(fighter.module_accessor) {
            FIGHTER_STATUS_KIND_LANDING
        } else {
            FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL
        };
        fighter.change_status(next_status.into(), false.into());
        return true.into();
    }

    return false.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_END, special_hi_end_pre);
    agent.status(Main, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_END, special_hi_end_main);
}