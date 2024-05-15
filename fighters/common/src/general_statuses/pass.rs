use super::*;
use globals::*;

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_Pass_Main_sub_hook,
        );
    }
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_Pass_Main_sub)]
pub unsafe fn status_Pass_Main_sub_hook(fighter: &mut L2CFighterCommon, arg1: L2CValue) -> L2CValue {
    let pass_frame = fighter.get_int(*FIGHTER_STATUS_PASS_WORK_INT_FRAME);
    if pass_frame > 0 {
        // skip direct cancels from restricted statuses
        let skip_cancels = fighter.is_prev_status_one_of(&[
            *FIGHTER_STATUS_KIND_GUARD,
            *FIGHTER_STATUS_KIND_GUARD_DAMAGE,
            *FIGHTER_STATUS_KIND_GUARD_ON,
            *FIGHTER_STATUS_KIND_ESCAPE_AIR,
            *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE
        ]);
        if skip_cancels {
            return 0.into();
        }
    }

    original!()(fighter, arg1)
}