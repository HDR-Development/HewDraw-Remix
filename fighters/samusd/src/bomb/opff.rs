// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe extern "C" fn bomb_callback(weapon: &mut L2CFighterBase) {
    if AttackModule::is_infliction(weapon.module_accessor, *COLLISION_KIND_MASK_ATTACK | *COLLISION_KIND_MASK_SHIELD) 
    && !AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_HIT) {
        sv_battle_object::end_inhaled(weapon.battle_object_id as u32, true);
    } //die
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, bomb_callback);
}