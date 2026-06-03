// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe extern "C" fn supermissile_frame(weapon: &mut L2CFighterBase) {
    let h = weapon.get_int(*WEAPON_SAMUS_SUPERMISSILE_STATUS_STRAIGHT_WORK_ID_INT_FRAME);
    if AttackModule::is_infliction(weapon.module_accessor, *COLLISION_KIND_MASK_REFLECTOR | *COLLISION_KIND_MASK_PARRY) {
        let life = weapon.get_param_int("param_supermissile", "s_life");
        weapon.set_int(life, *WEAPON_SAMUS_SUPERMISSILE_STATUS_STRAIGHT_WORK_ID_INT_FRAME);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, supermissile_frame);
}