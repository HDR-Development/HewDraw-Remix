use super::*;
use globals::*;
// status script import

mod appeal;
mod attack;
mod attack_air;
mod attack_s3;
mod attack_s4;
mod special_hi;
mod special_lw;
mod special_s;
mod throw;

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    VarModule::set_int(fighter.battle_object, vars::packun::instance::CURRENT_STANCE, 0);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("heada"), true);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("headb"), false);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("heads"), false);
}

unsafe extern "C" fn entry_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_ENTRY)(fighter);
    VarModule::set_int(fighter.battle_object, vars::packun::instance::CURRENT_STANCE, 0);

    return ret;
}

unsafe extern "C" fn win_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_WIN)(fighter);
    VarModule::set_int(fighter.battle_object, vars::packun::instance::CURRENT_STANCE, 0);

    return ret;
}

unsafe extern "C" fn lose_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_LOSE)(fighter);
    VarModule::set_int(fighter.battle_object, vars::packun::instance::CURRENT_STANCE, 0);

    return ret;
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);

    appeal::install(agent);
    attack::install(agent);
    attack_air::install(agent);
    attack_s3::install(agent);
    attack_s4::install(agent);
    special_s::install(agent);
    special_hi::install(agent);
    special_lw::install(agent);
    throw::install(agent);

    agent.status(Main, *FIGHTER_STATUS_KIND_ENTRY, entry_main);
    agent.status(Main, *FIGHTER_STATUS_KIND_WIN, win_main);
    agent.status(Main, *FIGHTER_STATUS_KIND_LOSE, lose_main);
}