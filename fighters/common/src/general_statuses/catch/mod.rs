use super::*;
mod catchcut;
mod catchdash;
mod catchattack;

#[skyline::hook(replace = L2CFighterCommon_sub_transition_group_check_ground_catch)]
unsafe fn sub_transition_group_check_ground_catch(fighter: &mut L2CFighterCommon) -> L2CValue {
    // prevents c-stick grabs, making c-stick roll feel smooth
    if super::shield::misc::check_cstick_escape_oos(fighter).get_bool() || super::shield::misc::check_escape_oos(fighter).get_bool() {
        return false.into();
    }
    call_original!(fighter)
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(sub_transition_group_check_ground_catch);
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
    catchcut::install();
    catchdash::install();
    catchattack::install();
}
