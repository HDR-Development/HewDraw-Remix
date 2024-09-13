use super::*;
use globals::*;
// status script import

mod air_lasso;
mod attack_air_lw;
mod attack_dash;
mod appeal;
mod entry;
mod guard;
mod jump;
mod rebirth;
mod recreate_table;
mod special_s;

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    VarModule::on_flag(fighter.battle_object, vars::pickel::instance::CYCLE_MATERIAL);
    VarModule::on_flag(fighter.battle_object, vars::pickel::instance::TABLE_ENABLE_RESPAWN);
    VarModule::off_flag(fighter.battle_object, vars::pickel::instance::DAMAGE_FLY_RESET_ROT);
    VarModule::set_int(fighter.battle_object, vars::pickel::instance::MATERIAL_INDEX, 0);
    VarModule::set_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER, 0);
    VarModule::set_int(fighter.battle_object, vars::pickel::instance::DAMAGE_FLY_HITSTUN_TIMER, 0);
    VarModule::set_float(fighter.battle_object, vars::pickel::instance::DAMAGE_FLY_STORED_DAMAGE, 0.0);
    VarModule::set_float(fighter.battle_object, vars::pickel::instance::TABLE_CURRENT_LIFE, 20.0);
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);
    
    air_lasso::install(agent);
    attack_air_lw::install(agent);
    attack_dash::install(agent);
    appeal::install(agent);
    entry::install(agent);
    guard::install(agent);
    jump::install(agent);
    rebirth::install(agent);
    recreate_table::install(agent);
    special_s::install(agent);
}
