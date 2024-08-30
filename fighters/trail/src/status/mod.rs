use super::*;
use globals::*;
// status script import

mod jump_aerial;
mod attack_s3;
mod attack_lw4;
mod attack_air;
mod special_s;
mod special_n;

unsafe extern "C" fn should_use_special_n_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, vars::trail::instance::DISABLE_SPECIAL_N) {
        false.into()
    } else {
        true.into()
    }
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    fighter.global_table[globals::USE_SPECIAL_N_CALLBACK].assign(&L2CValue::Ptr(should_use_special_n_callback as *const () as _));
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);
    
    jump_aerial::install(agent);
    attack_s3::install(agent);
    attack_lw4::install(agent);
    attack_air::install(agent);
    special_s::install(agent);
    special_n::install(agent);
}
