// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

pub extern "C" fn knife_bounce(weapon: &mut smash::lua2cpp::L2CFighterBase) {
    unsafe {
        if weapon.kind() != *WEAPON_KIND_RICHTER_AXE || !weapon.is_status(*WEAPON_SIMON_AXE_STATUS_KIND_FLY) { return; };
        if AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_HIT) {
            let owner = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
            ArticleModule::remove(owner, *FIGHTER_RICHTER_GENERATE_ARTICLE_AXE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
        else if AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_ATTACK | *COLLISION_KIND_MASK_SHIELD) {
            weapon.change_status(WEAPON_SIMON_AXE_STATUS_KIND_HOP.into(), false.into());
        }
        let owner = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        let speedx = KineticModule::get_sum_speed_x(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        let speedy = KineticModule::get_sum_speed_y(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        let mut pos = Vector3f::zero();
        ModelModule::joint_global_position(weapon.module_accessor, Hash40::new("axe"), &mut pos, false);
        let pos = Vector2f::new(pos.x, pos.y);
        let next_pos = Vector2f::new(pos.x + speedx, pos.y + speedy);
        if !GroundModule::line_segment_check(weapon.module_accessor, &pos, &next_pos, &Vector2f::zero(), &mut Vector2f::zero(), true).is_null() {
            weapon.change_status(WEAPON_SIMON_AXE_STATUS_KIND_HOP.into(), false.into());
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, knife_bounce);
}
