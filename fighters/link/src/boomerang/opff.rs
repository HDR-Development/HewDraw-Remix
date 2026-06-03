// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn boomerang_reflect_fix(weapon: &mut smash::lua2cpp::L2CFighterBase) {
    // this opff runs once after reflection
    let boma = weapon.module_accessor;
    if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_REFLECTOR | *COLLISION_KIND_MASK_PARRY) {
        return;
    }
    AttackModule::clear_inflict_kind_status(boma);

    // update position, accounting for boomerang-weirdness
    let top_pos = *PostureModule::pos(boma);
    let rot_pos = &mut Vector3f{ x: 0.0, y: 0.0, z: 0.0 };
    ModelModule::joint_global_position(boma, Hash40::new("rot"), rot_pos, false);
    PostureModule::set_pos(boma, &Vector3f{ x: rot_pos.x, y: top_pos.y, z: top_pos.z });

    // update facing direction depending on whether it is traveling forwards or backwards
    let lr = PostureModule::lr(boma);
    let speed_x = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
    if weapon.is_status(*WN_LINK_BOOMERANG_STATUS_KIND_FLY) {
        PostureModule::reverse_lr(boma);
        PostureModule::update_rot_y_lr(boma);
    }

    // lifetime fix
    weapon.set_int_from_param(*WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_LIFE, "boomerang", "life");

    // restart status
    weapon.change_status(WN_LINK_BOOMERANG_STATUS_KIND_FLY.into(), false.into());
}

pub unsafe extern "C" fn link_boomerang_frame(weapon: &mut smash::lua2cpp::L2CFighterBase) {
    boomerang_reflect_fix(weapon);
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, link_boomerang_frame);
}