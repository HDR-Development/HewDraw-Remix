use super::*;
use globals::*;
utils::import!(common::djc::attack_air_main_status);
// status script import

mod attack_air;
mod attack_lw4;
mod special_n;
mod special_hi;

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    let charge_time = ParamModule::get_int(fighter.object(), ParamType::Agent, "attack_up_charge_time");
    VarModule::set_int(fighter.object(), charge_time, vars::lucas::instance::SPECIAL_N_OFFENSE_UP_CHARGE_LEVEL);
    VarModule::off_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE);
    VarModule::off_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_INIT);
    VarModule::off_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF);
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);

    attack_air::install(agent);
    attack_lw4::install(agent);
    special_n::install(agent);
    special_hi::install(agent);
}