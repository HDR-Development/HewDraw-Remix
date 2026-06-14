use utils::consts::vars::wario;

use super::*;

// FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_DRIVE

pub unsafe extern "C" fn special_s_drive_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.main_shift(special_s_drive_main_loop)
}

unsafe extern "C" fn special_s_drive_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Jump cancel
    if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP)
    || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_SPECIAL_S_FLAG_RESERVE_JUMP) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_SPECIAL_S_FLAG_RESERVE_JUMP);

        fighter.change_status(FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_ESCAPE_START.into(), false.into());
        return 0.into();
    }

    // Aerial neutral cancel (sends into tumble)
    let bike = ArticleModule::get_article(fighter.module_accessor, *FIGHTER_WARIO_GENERATE_ARTICLE_WARIOBIKE);
    let bike_id = smash::app::lua_bind::Article::get_battle_object_id(bike) as u32;
    let bike_boma = sv_battle_object::module_accessor(bike_id);

    if StatusModule::situation_kind(bike_boma) == *SITUATION_KIND_AIR
    && (ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
        || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
        || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL))
    {
        VarModule::on_flag(fighter.battle_object, wario::instance::SPECIAL_S_CANCEL);

        fighter.change_status(FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_ESCAPE_START.into(), false.into());
        return 0.into();
    }

    if (ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI)
    || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW)
    || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R)
    || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L))
    && {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1daca540be));        
        fighter.pop_lua_stack(1).get_bool()
    } {
        fighter.change_status(FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_APPEAL.into(), false.into());
        return 0.into();
    }

    0.into()
}

// FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_ESCAPE_START

pub unsafe extern "C" fn special_s_escape_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    LinkModule::send_event_parents(fighter.module_accessor, *FIGHTER_WARIO_LINK_NO_BIKE, Hash40::new_raw(0x1f9a545917));

    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_escape_start"), 0.0, 1.0, false, 0.0, false, false);

    fighter.main_shift(special_s_escape_start_main_loop)
}

unsafe extern "C" fn special_s_escape_start_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        if VarModule::is_flag(fighter.battle_object, wario::instance::SPECIAL_S_CANCEL) {            
            fighter.change_status(FIGHTER_STATUS_KIND_DAMAGE_FALL.into(), true.into());
        }
        else {
            fighter.change_status(FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_ESCAPE.into(), false.into());
        }
    }

    0.into()
}

pub unsafe extern "C" fn special_s_escape_start_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let bike = ArticleModule::get_article(fighter.module_accessor, *FIGHTER_WARIO_GENERATE_ARTICLE_WARIOBIKE);
    let bike_id = smash::app::lua_bind::Article::get_battle_object_id(bike) as u32;
    let bike_boma = sv_battle_object::module_accessor(bike_id);

    VarModule::off_flag(fighter.battle_object, wario::instance::SPECIAL_S_CANCEL);

    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_DRIVE, special_s_drive_main);
    agent.status(Main, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_ESCAPE_START, special_s_escape_start_main);
    agent.status(End, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_ESCAPE_START, special_s_escape_start_end);
}