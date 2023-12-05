use super::*;
use globals::*;

#[status_script(agent = "gamewatch", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn gamewatch_special_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, vars::gamewatch::instance::UP_SPECIAL_FREEFALL) {
        let cancel_module = *(fighter.module_accessor as *mut BattleObjectModuleAccessor as *mut u64).add(0x128 / 8) as *const u64;
        *(((cancel_module as u64) + 0x1c) as *mut bool) = false;  // CancelModule::is_enable_cancel = false
    }
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_FLAG_SPECIAL_HI);
    ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_RESCUE, false, -1);
    let motion = if fighter.is_situation(*SITUATION_KIND_GROUND) { Hash40::new("special_hi") } else { Hash40::new("special_air_hi") };
    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_RESCUE, motion, false, -1.0);
    MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(gamewatch_special_hi_main_loop as *const () as _))
}

unsafe fn gamewatch_special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_transition_group_check_air_cliff();
    if MotionModule::is_end(fighter.module_accessor) {
        let control = ControlModule::get_attack_air_kind(fighter.module_accessor);
        WorkModule::set_int(fighter.module_accessor, control, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_HI_WORK_INT_ATTACK_AIR_KIND);
        if VarModule::is_flag(fighter.battle_object, vars::gamewatch::instance::UP_SPECIAL_FREEFALL) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
        }
        else {
            VarModule::on_flag(fighter.battle_object, vars::gamewatch::instance::UP_SPECIAL_PARACHUTE);
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        return 1.into()
    }
    if fighter.status_frame() > 31 && fighter.is_cat_flag(Cat1::SpecialAny) {
        VarModule::on_flag(fighter.battle_object, vars::gamewatch::instance::UP_SPECIAL_PARACHUTE);
        fighter.change_to_custom_status(statuses::gamewatch::SPECIAL_HI_OPEN, true, false);
    }
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_HI_FLAG_LANDING_ENABLE) {
            let status = if WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("landing_frame")) > 0.0
                { FIGHTER_STATUS_KIND_LANDING } else { FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL };
            fighter.change_status(status.into(), true.into());
            return 1.into()
        }
    }
    return 0.into()
}

#[status_script(agent = "gamewatch", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS)]
unsafe fn gamewatch_special_hi_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::on_flag(fighter.battle_object, vars::gamewatch::instance::UP_SPECIAL_FREEFALL);
    0.into()
}

pub fn install() {
    install_status_scripts!(
        gamewatch_special_hi_main,
        gamewatch_special_hi_exit,
    );
}