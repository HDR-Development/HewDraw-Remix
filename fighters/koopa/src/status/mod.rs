use super::*;
use globals::*;
// status script import

mod attack_lw3;
mod special_n;
mod special_s;
mod special_hi;
mod attack_s4;
mod fall_special;
mod squat;
 
// AGENT INIT AND CALLBACKS
unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    //Remove fireball ready effect
    if fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_ENTRY, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_REBIRTH,
    *FIGHTER_STATUS_KIND_WIN, *FIGHTER_STATUS_KIND_LOSE]) || !sv_information::is_ready_go() {
        EFFECT_OFF_KIND(fighter,Hash40::new("koopa_breath_m_fire"), false, false);
        VarModule::set_int(fighter.battle_object, vars::koopa::instance::SPECIAL_N_FIREBALL_EFFECT_ID, 0);
        VarModule::set_int(fighter.battle_object, vars::koopa::instance::SPECIAL_N_FIREBALL_COOLDOWN, MAX_COOLDOWN);
    }
    if [*SITUATION_KIND_GROUND, *SITUATION_KIND_CLIFF].contains(&fighter.global_table[SITUATION_KIND].get_i32())
    || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_GIMMICK_SPRING_JUMP]) {
        VarModule::off_flag(fighter.battle_object, vars::koopa::instance::SPECIAL_S_DISABLE_STALL);
    }
    true.into()
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));

    VarModule::off_flag(fighter.battle_object, vars::koopa::instance::SPECIAL_S_DISABLE_STALL);
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);

    attack_lw3::install(agent);
    special_n::install(agent);
    special_s::install(agent);
    special_hi::install(agent);
    attack_s4::install(agent);
    fall_special::install(agent);
    squat::install(agent);
}