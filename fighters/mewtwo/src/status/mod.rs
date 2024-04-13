use super::*;
use globals::*;
// status script import

mod jump_aerial;
mod attack_air;
mod float;
mod fall;
mod special_n;

extern "Rust" {
    #[link_name = "float_check_air_jump"]
    fn float_check_air_jump(fighter: &mut L2CFighterCommon, float_status: L2CValue) -> L2CValue;
    #[link_name = "float_check_air_jump_aerial"]
    fn float_check_air_jump_aerial(fighter: &mut L2CFighterCommon, float_status: L2CValue) -> L2CValue;
}

unsafe extern "C" fn air_jump_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    float_check_air_jump(fighter, statuses::mewtwo::FLOAT.into())
}

unsafe extern "C" fn air_jump_aerial_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    float_check_air_jump_aerial(fighter, statuses::mewtwo::FLOAT.into())
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    // uncomment the next five lines to enable float
    // fighter.global_table[0x32].assign(&L2CValue::Ptr(air_jump_uniq as *const () as _));
    // fighter.global_table[0x33].assign(&L2CValue::Ptr(air_jump_aerial_uniq as *const () as _));
    // VarModule::set_int(fighter.battle_object, vars::common::instance::FLOAT_DURATION, 60);
    // VarModule::on_flag(fighter.battle_object, vars::common::instance::OMNI_FLOAT);
    // VarModule::set_int(fighter.battle_object, vars::common::instance::FLOAT_STATUS_KIND, statuses::mewtwo::FLOAT);
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start); 

    jump_aerial::install(agent);
    attack_air::install(agent);
    float::install(agent);
    fall::install(agent);
    special_n::install(agent);
}