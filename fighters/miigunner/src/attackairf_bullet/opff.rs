use super::*;

unsafe extern "C" fn airfbullet_frame(weapon: &mut smash::lua2cpp::L2CFighterBase) {
    if AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_PARRY | *COLLISION_KIND_MASK_REFLECTOR) {
        StatusModule::change_status_request_from_script(weapon.module_accessor, *WEAPON_MIIGUNNER_ATTACKAIRF_BULLET_STATUS_KIND_FLY, false);
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("fly"), 0.0, 1.0, false, 0.0, false, false);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, airfbullet_frame);
}