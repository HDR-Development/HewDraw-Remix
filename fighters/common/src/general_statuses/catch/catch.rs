use super::*;
use globals::*;

// This file contains code for dash grab

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_Catch_Main
        );
    }
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_Catch_Main)]
unsafe fn status_Catch_Main(fighter: &mut L2CFighterCommon) -> L2CValue {

    let is_tether_grab = [
        *FIGHTER_KIND_LUCAS,
        *FIGHTER_KIND_PACMAN,
        *FIGHTER_KIND_PICKEL,
        *FIGHTER_KIND_PIKMIN,
        *FIGHTER_KIND_SAMUS,
        *FIGHTER_KIND_SAMUSD,
        *FIGHTER_KIND_TOONLINK,
        *FIGHTER_KIND_YOSHI,
        *FIGHTER_KIND_YOUNGLINK,
    ].contains(&fighter.kind());

    if !is_tether_grab
    && ParamModule::has_param_module(fighter.battle_object) {
        let frame = fighter.global_table[CURRENT_FRAME].get_i32();
        // grab clanks are universally enabled on start_frame
        let start_frame = ParamModule::get_int(fighter.battle_object, ParamType::Common, "grab_rebound.catch_start_frame");
        if frame == start_frame {
            GrabModule::set_rebound(fighter.module_accessor, true);
        }
        // and are disabled when the grab ends
        if GrabModule::is_rebound(fighter.module_accessor) 
        && fighter.is_flag(*FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT) {
            GrabModule::set_rebound(fighter.module_accessor, false);
        }
    }

    call_original!(fighter)
}
