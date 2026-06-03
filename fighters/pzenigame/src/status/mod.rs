use super::*;
use globals::*;
// status script import

mod landing_fall_special;
mod run;
mod special_n;
mod special_s;
mod special_lw;
mod special_hi;

unsafe extern "C" fn entry_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    reset_ptrainer_vars(fighter);
    smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_ENTRY)(fighter)
}

unsafe extern "C" fn dead_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if LinkModule::is_link(fighter.module_accessor, *FIGHTER_POKEMON_LINK_NO_PTRAINER) {
        let parent_id = LinkModule::get_parent_id(fighter.module_accessor, *FIGHTER_POKEMON_LINK_NO_PTRAINER, true) as u32;
        let object = utils::util::get_battle_object_from_id(parent_id);
        VarModule::set_int(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_STATE, *PLEDGE_STATE_WATER);
        VarModule::set_int(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_TIMER, 0);
        VarModule::on_flag(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_PAUSE_TIMER);
        VarModule::set_int(object, vars::ptrainer::instance::SPECIAL_LW_SWAP_TIMER, 0);
    }

    smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_DEAD)(fighter)
}

unsafe extern "C" fn rebirth_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if LinkModule::is_link(fighter.module_accessor, *FIGHTER_POKEMON_LINK_NO_PTRAINER) {
        let parent_id = LinkModule::get_parent_id(fighter.module_accessor, *FIGHTER_POKEMON_LINK_NO_PTRAINER, true) as u32;
        let object = utils::util::get_battle_object_from_id(parent_id);
        let pledge_state = VarModule::get_int(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_STATE);
        //println!("pledge_state: {}", pledge_state);
        if pledge_state == *PLEDGE_STATE_GRASS {
            let handle = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_status_speed_up"), Hash40::new("hip"), &Vector3f::new(0.7, 0.0, 0.0), &Vector3f::zero(), 0.7, true, 0, 0, 0, 0, 0, true, true) as u32;
            VarModule::set_int(fighter.object(), vars::pzenigame::instance::SPECIAL_N_PLEDGE_EFFECT_HANDLE, handle as i32);
            fighter.play_pledge_effect(*PLEDGE_STATE_GRASS);
        }
        else if pledge_state == *PLEDGE_STATE_FIRE {
            let handle = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_status_attack_up"), Hash40::new("hip"), &Vector3f::new(0.7, 0.0, 0.0), &Vector3f::zero(), 0.7, true, 0, 0, 0, 0, 0, true, true) as u32;
            VarModule::set_int(fighter.object(), vars::pzenigame::instance::SPECIAL_N_PLEDGE_EFFECT_HANDLE, handle as i32);
            fighter.play_pledge_effect(*PLEDGE_STATE_FIRE);
        }
        let pledge_duration_frame = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "param_special_lw.pledge_duration_frame");
        VarModule::set_int(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_TIMER, pledge_duration_frame);
        VarModule::off_flag(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_PAUSE_TIMER);
    }

    smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_REBIRTH)(fighter)
}

unsafe extern "C" fn win_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    reset_ptrainer_vars(fighter);
    smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_WIN)(fighter)
}

unsafe extern "C" fn lose_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    reset_ptrainer_vars(fighter);
    smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_LOSE)(fighter)
}

unsafe extern "C" fn reset_ptrainer_vars(fighter: &mut L2CFighterCommon) {
    if LinkModule::is_link(fighter.module_accessor, *FIGHTER_POKEMON_LINK_NO_PTRAINER) {
        let parent_id = LinkModule::get_parent_id(fighter.module_accessor, *FIGHTER_POKEMON_LINK_NO_PTRAINER, true) as u32;
        let object = utils::util::get_battle_object_from_id(parent_id);
        VarModule::set_int(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_STATE, *PLEDGE_STATE_NONE);
        VarModule::set_int(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_TIMER, 0);
        VarModule::off_flag(object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_PAUSE_TIMER);
        VarModule::set_int(object, vars::ptrainer::instance::SPECIAL_LW_SWAP_TIMER, 0);
    }
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_ENTRY, entry_main);
    agent.status(Main, *FIGHTER_STATUS_KIND_DEAD, dead_main);
    agent.status(Main, *FIGHTER_STATUS_KIND_REBIRTH, rebirth_main);
    agent.status(Main, *FIGHTER_STATUS_KIND_WIN, win_main);
    agent.status(Main, *FIGHTER_STATUS_KIND_LOSE, lose_main);
    
    landing_fall_special::install(agent);
    run::install(agent);
    special_n::install(agent);
    special_s::install(agent);
    special_lw::install(agent);
    special_hi::install(agent);
}