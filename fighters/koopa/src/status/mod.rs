use super::*;
use globals::*;
// status script import

mod special_n;
mod special_s;
mod special_hi;
mod attack_s4_hold;
 
// AGENT INIT AND CALLBACKS
unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    //Remove fireball ready effect
    if fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_ENTRY,*FIGHTER_STATUS_KIND_DEAD,*FIGHTER_STATUS_KIND_REBIRTH,
        *FIGHTER_STATUS_KIND_WIN,*FIGHTER_STATUS_KIND_LOSE]) || !sv_information::is_ready_go() {
        EFFECT_OFF_KIND(fighter,Hash40::new("koopa_breath_m_fire"),false,false);
        VarModule::set_int(fighter.battle_object, vars::koopa::instance::FIREBALL_EFFECT_ID,0);
        VarModule::set_int(fighter.battle_object, vars::koopa::instance::FIREBALL_COOLDOWN_FRAME,MAX_COOLDOWN);
    }
    true.into()
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);

    special_n::install(agent);
    special_s::install(agent);
    special_hi::install(agent);
    attack_s4_hold::install(agent);
}