use super::*;
use globals::*;

unsafe extern "C" fn special_hi_main(agent: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(agent.battle_object, vars::gamewatch::instance::UP_SPECIAL_FREEFALL) {
        let cancel_module = *(agent.module_accessor as *mut BattleObjectModuleAccessor as *mut u64).add(0x128 / 8) as *const u64;
        *(((cancel_module as u64) + 0x1c) as *mut bool) = false;  // CancelModule::is_enable_cancel = false
    }
    WorkModule::on_flag(agent.module_accessor, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_FLAG_SPECIAL_HI);
    ArticleModule::generate_article(agent.module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_RESCUE, false, -1);
    let motion = if agent.is_situation(*SITUATION_KIND_GROUND) { Hash40::new("special_hi") } else { Hash40::new("special_air_hi") };
    ArticleModule::change_motion(agent.module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_RESCUE, motion, false, -1.0);
    MotionModule::change_motion(agent.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);
    if agent.is_situation(*SITUATION_KIND_GROUND) {
        agent.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    agent.sub_shift_status_main(L2CValue::Ptr(special_hi_main_loop as *const () as _))
}

unsafe fn special_hi_main_loop(agent: &mut L2CFighterCommon) -> L2CValue {
    agent.sub_transition_group_check_air_cliff();
    if MotionModule::is_end(agent.module_accessor) {
        let control = ControlModule::get_attack_air_kind(agent.module_accessor);
        WorkModule::set_int(agent.module_accessor, control, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_HI_WORK_INT_ATTACK_AIR_KIND);
        if VarModule::is_flag(agent.battle_object, vars::gamewatch::instance::UP_SPECIAL_FREEFALL) {
            agent.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
        }
        else {
            VarModule::on_flag(agent.battle_object, vars::gamewatch::instance::UP_SPECIAL_PARACHUTE);
            agent.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        return 1.into()
    }
    if agent.status_frame() > 31 && agent.is_cat_flag(Cat1::SpecialAny) {
        VarModule::on_flag(agent.battle_object, vars::gamewatch::instance::UP_SPECIAL_PARACHUTE);
        agent.change_status(statuses::gamewatch::SPECIAL_HI_OPEN.into(), true.into());
    }
    if agent.is_situation(*SITUATION_KIND_GROUND) {
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_HI_FLAG_LANDING_ENABLE) {
            let status = if WorkModule::get_param_float(agent.module_accessor, hash40("param_special_hi"), hash40("landing_frame")) > 0.0
                { FIGHTER_STATUS_KIND_LANDING } else { FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL };
            agent.change_status(status.into(), true.into());
            return 1.into()
        }
    }
    return 0.into()
}

unsafe extern "C" fn special_hi_exit(agent: &mut L2CFighterCommon) -> L2CValue {
    VarModule::on_flag(agent.battle_object, vars::gamewatch::instance::UP_SPECIAL_FREEFALL);
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_main);
    agent.status( Exit, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_exit);
}
