use super::*;
use globals::*;

mod attack_air_lw;
mod entry;
mod guard;
mod jump;
mod rebirth;
mod recreate_table;
mod special_s;

// lets the "stuff" article generate in new statuses
#[skyline::hook(offset = 0xf13d5c, inline)]
unsafe fn stuff_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let new_shield_statuses = &[
        0x1B, // GUARD_ON
        0x1C // GUARD
        ];
    let status = *ctx.registers[0].x.as_ref();
    if new_shield_statuses.contains(&status) {
        *ctx.registers[0].x.as_mut() = 0x1E;
    } 
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    VarModule::on_flag(fighter.battle_object, vars::pickel::instance::SHOULD_CYCLE_MATERIAL);
    VarModule::off_flag(fighter.battle_object, vars::pickel::instance::SHOULD_RESET_ROT);
    VarModule::set_int(fighter.battle_object, vars::pickel::instance::MATERIAL_INDEX, 0);
    VarModule::set_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER, 0);
    VarModule::set_int(fighter.battle_object, vars::pickel::instance::HITSTUN_TIMER, 0);
    VarModule::set_float(fighter.battle_object, vars::pickel::instance::DAMAGE_TRACKER, 0.0);
    VarModule::set_float(fighter.battle_object, vars::pickel::instance::TABLE_HP_TRACKER, 20.0);
}

pub fn install(agent: &mut Agent) {
    skyline::install_hooks!(
        stuff_hook
    );
    agent.on_start(on_start);
    
    attack_air_lw::install(agent);
    entry::install(agent);
    guard::install(agent);
    jump::install(agent);
    rebirth::install(agent);
    recreate_table::install(agent);
    special_s::install(agent);
}
