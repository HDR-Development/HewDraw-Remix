use super::*;
use globals::*;
// status script import

mod jump;
mod catch;

mod attack;
mod attack_air;
mod attack_jump_aerial;
mod attack_landing;
mod attack_s3;
mod attack_s4;

mod special_n;
mod special_hi;

/// Re-enables the ability to use aerial specials when connecting to ground or cliff
unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    //remove double dragon effect
    if fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_ENTRY,*FIGHTER_STATUS_KIND_DEAD,*FIGHTER_STATUS_KIND_REBIRTH,
        *FIGHTER_STATUS_KIND_WIN,*FIGHTER_STATUS_KIND_LOSE]) || !sv_information::is_ready_go() {
        let dragonEffect = VarModule::get_int(fighter.object(),vars::tantan::instance::DRAGONIZE_R_EFFECT_HANDLE) as u32;
        if EffectModule::is_exist_effect(fighter.module_accessor, dragonEffect){
            EffectModule::kill(fighter.module_accessor, dragonEffect, false,false);
        }
        VarModule::set_int(fighter.object(),vars::tantan::instance::DRAGONIZE_R_EFFECT_HANDLE,0);
    }
    true.into()
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    // set the callbacks on fighter init              
    fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));   
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);

    jump::install(agent);
    catch::install(agent);

    attack::install(agent);
    attack_air::install(agent);
    attack_jump_aerial::install(agent);
    attack_landing::install(agent);
    attack_s3::install(agent);
    attack_s4::install(agent);

    special_n::install(agent);
    special_hi::install(agent);
}
