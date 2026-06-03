use super::*;

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_MewtwoThrown_check_damage,
        );
    }
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_FighterStatusUniqProcessMewtwoThrown_check_damage)]
unsafe fn status_MewtwoThrown_check_damage(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.get_grabber_boma().is_status(*FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_S_THROW) {
        return call_original!(fighter);
    }
    // make spin match grab duration manually
    let frame = MotionModule::frame(fighter.module_accessor);
    MotionModule::set_rate(fighter.module_accessor, (28.0-frame)/(51.0-frame));
    // allow pushing thru platforms
    GroundModule::set_passable_check(fighter.module_accessor, true);
    if CaptureModule::check_damage_thrown(fighter.module_accessor) != 0 {
        return 1.into();
    } // bypass release set speed/status
    0.into()
}