// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn krown_reflect_fix(weapon: &mut smash::lua2cpp::L2CFighterBase) {
    // this opff runs once after reflection
    let boma = weapon.module_accessor;
    if !weapon.is_status(*WEAPON_KROOL_CROWN_STATUS_KIND_THROW) || !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_REFLECTOR | *COLLISION_KIND_MASK_PARRY) {
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
    if weapon.is_status(*WEAPON_KROOL_CROWN_STATUS_KIND_THROW) && weapon.motion_frame() <= 40.0 { // F40 is the frame that animation reverses direction
        PostureModule::reverse_lr(boma);
        PostureModule::update_rot_y_lr(boma);
    }

    // restart status
    weapon.change_status(WEAPON_KROOL_CROWN_STATUS_KIND_THROW.into(), false.into());
}

pub unsafe extern "C" fn krool_crown_frame(weapon: &mut smash::lua2cpp::L2CFighterBase) {
    krown_reflect_fix(weapon);
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, krool_crown_frame);
}