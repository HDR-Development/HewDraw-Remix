use super::*;
use globals::*;
// status script import

mod attack_air;
mod attack_lw3;
mod float;
mod special_hi;
mod special_n;
mod special_s;

extern "Rust" {
    #[link_name = "float_check_air_jump"]
    fn float_check_air_jump(fighter: &mut L2CFighterCommon, float_status: L2CValue) -> L2CValue;
    #[link_name = "float_check_air_jump_aerial"]
    fn float_check_air_jump_aerial(fighter: &mut L2CFighterCommon, float_status: L2CValue) -> L2CValue;
}

unsafe extern "C" fn air_jump_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    float_check_air_jump(fighter, statuses::samusd::FLOAT.into())
}

unsafe extern "C" fn air_jump_aerial_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    float_check_air_jump_aerial(fighter, statuses::samusd::FLOAT.into())
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    fighter.global_table[0x32].assign(&L2CValue::Ptr(air_jump_uniq as *const () as _));
    fighter.global_table[0x33].assign(&L2CValue::Ptr(air_jump_aerial_uniq as *const () as _));
    let float_duration = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "param_float.float_duration");
    VarModule::set_int(fighter.battle_object, vars::common::instance::FLOAT_DURATION, float_duration);
    VarModule::on_flag(fighter.battle_object, vars::common::instance::OMNI_FLOAT);
    VarModule::set_int(fighter.battle_object, vars::common::instance::FLOAT_STATUS_KIND, statuses::samusd::FLOAT);
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);

    attack_air::install(agent);
    attack_lw3::install(agent);
    float::install(agent);
    special_hi::install(agent);
    special_n::install(agent);
    special_s::install(agent);
}