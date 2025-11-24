// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe extern "C" fn bullet_frame(weapon: &mut L2CFighterBase) {
    if weapon.is_status(*WEAPON_BUDDY_BULLET_STATUS_KIND_FLY)
    && weapon.get_int(*WEAPON_BUDDY_BULLET_INSTANCE_WORK_ID_INT_TYPE) == *WEAPON_BUDDY_BULLET_TYPE_MISSILE
    && AttackModule::is_infliction(weapon.module_accessor, *COLLISION_KIND_MASK_REFLECTOR | *COLLISION_KIND_MASK_PARRY) {
        let life = weapon.get_param_int("param_bullet", "missile_life");
        weapon.set_int(*WEAPON_INSTANCE_WORK_ID_INT_LIFE, life);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, bullet_frame);
}