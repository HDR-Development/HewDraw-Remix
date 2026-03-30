use super::*;
use globals::*;

extern "Rust" {
    fn gimmick_flash(boma: &mut BattleObjectModuleAccessor);
}

pub unsafe extern "C" fn pledge_meter(weapon: &mut L2CFighterBase) {
    let pledge_state = VarModule::get_int(weapon.battle_object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_STATE);
    let is_pledge_timer_paused = VarModule::is_flag(weapon.battle_object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_PAUSE_TIMER);
    if pledge_state != *PLEDGE_STATE_NONE && !is_pledge_timer_paused {
        if VarModule::get_int(weapon.battle_object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_TIMER) <= 0 {
            VarModule::set_int(weapon.battle_object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_TIMER, 0);
            VarModule::set_int(weapon.battle_object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_STATE, *PLEDGE_STATE_NONE);
            if LinkModule::is_link(weapon.module_accessor, *WEAPON_PTRAINER_PTRAINER_LINK_NO_POKEMON) {
                let poke_parent_id = LinkModule::get_parent_object_id(weapon.module_accessor, *WEAPON_PTRAINER_PTRAINER_LINK_NO_POKEMON) as u32;
                let poke_object = utils::util::get_battle_object_from_id(poke_parent_id);
                kill_pledge_effects(poke_object);
            }
        } else {
            VarModule::dec_int(weapon.battle_object, vars::ptrainer::instance::SPECIAL_N_PLEDGE_TIMER);
            if LinkModule::is_link(weapon.module_accessor, *WEAPON_PTRAINER_PTRAINER_LINK_NO_POKEMON) {
                let poke_parent_id = LinkModule::get_parent_object_id(weapon.module_accessor, *WEAPON_PTRAINER_PTRAINER_LINK_NO_POKEMON) as u32;
                let poke_object = utils::util::get_battle_object_from_id(poke_parent_id);
                update_pledge_ui(weapon, poke_object);
            }
        }
    }
    if VarModule::countdown_int(weapon.battle_object, vars::ptrainer::instance::SPECIAL_LW_SWAP_TIMER, 0) {
        if LinkModule::is_link(weapon.module_accessor, *WEAPON_PTRAINER_PTRAINER_LINK_NO_POKEMON) {
            let poke_parent_id = LinkModule::get_parent_object_id(weapon.module_accessor, *WEAPON_PTRAINER_PTRAINER_LINK_NO_POKEMON) as u32;
            let poke_object = utils::util::get_battle_object_from_id(poke_parent_id);
            gimmick_flash(&mut *(*poke_object).module_accessor);
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, pledge_meter);
}