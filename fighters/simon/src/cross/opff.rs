// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn cross_reflect_fix(weapon: &mut smash::lua2cpp::L2CFighterBase) {
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
    if weapon.is_status(*WEAPON_SIMON_CROSS_STATUS_KIND_TURN) {
        PostureModule::reverse_lr(boma);
        PostureModule::update_rot_y_lr(boma);
    }

    // lifetime fix
    let param_name = if weapon.is_flag(*WEAPON_SIMON_CROSS_INSTANCE_WORK_ID_FLAG_FLICK) {
        "life_flick"
    } else {
        "life"
    };
    let life = weapon.get_param_int("cross", param_name);
    weapon.set_int(life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    weapon.set_int(life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);

    // restart status
    weapon.change_status(WEAPON_SIMON_CROSS_STATUS_KIND_TURN.into(), false.into());
}

pub unsafe extern "C" fn simon_cross_frame(weapon: &mut smash::lua2cpp::L2CFighterBase) {
    cross_reflect_fix(weapon);
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, simon_cross_frame);
}