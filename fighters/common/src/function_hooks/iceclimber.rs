use super::*;

pub mod FighterSpecializer_Popo {
    extern "C" {
        #[link_name = "\u{1}_ZN3app23FighterSpecializer_Popo23get_partner_motion_kindERNS_7FighterE"]
        pub fn get_partner_motion_kind(
            fighter: *mut smash::app::Fighter,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app23FighterSpecializer_Popo28get_main_fighter_status_kindERNS_7FighterE"]
        pub fn get_main_fighter_status_kind(
            fighter: *mut smash::app::Fighter,
        ) -> u64;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN3app23FighterSpecializer_Popo31enable_partner_catch_transitionERNS_7FighterE"]
        pub fn enable_partner_catch_transition(
            fighter: *mut smash::app::Fighter,
        );
    }
}

// #[skyline::hook(replace = FighterSpecializer_Popo::get_partner_motion_kind)]
// pub unsafe extern "C" fn get_partner_motion_kind(fighter: *mut smash::app::Fighter) -> u64 {
//     let ret = call_original!(fighter);
//     let boma = (&mut (*fighter).battle_object).module_accessor;
//     dbg!(MotionModule::motion_kind(boma));
//     if [
//         hash40("catch"),
//         hash40("catch_dash"),
//         hash40("catch_turn"),
//         hash40("catch_pull"),
//         hash40("catch_wait"),
//         hash40("catch_attack"),
//         hash40("catch_cut"),
//     ].contains(&ret) {
//         return hash40("wait");
//     }
//     return ret;
// }

// #[skyline::hook(replace = FighterSpecializer_Popo::get_main_fighter_status_kind)]
// pub unsafe extern "C" fn get_main_fighter_status_kind(fighter: *mut smash::app::Fighter) -> u64 {
//     let ret = call_original!(fighter);
//     let boma = (&mut (*fighter).battle_object).module_accessor;
//     dbg!(StatusModule::status_kind(boma));
//     if [
//         *FIGHTER_STATUS_KIND_CATCH as u64,
//         *FIGHTER_STATUS_KIND_CATCH_DASH as u64,
//         *FIGHTER_STATUS_KIND_CATCH_TURN as u64,
//         *FIGHTER_STATUS_KIND_CATCH_PULL as u64,
//         *FIGHTER_STATUS_KIND_CATCH_WAIT as u64,
//         *FIGHTER_STATUS_KIND_CATCH_ATTACK as u64,
//         *FIGHTER_STATUS_KIND_CATCH_CUT as u64,
//     ].contains(&ret) {
//         return *FIGHTER_STATUS_KIND_WAIT as u64;
//     }
//     return ret;
// }

// #[skyline::hook(replace = FighterSpecializer_Popo::enable_partner_catch_transition)]
// pub unsafe extern "C" fn enable_partner_catch_transition(fighter: *mut smash::app::Fighter) {
//     let boma = (&mut (*fighter).battle_object).module_accessor;
//     dbg!(StatusModule::status_kind(boma));
//     return;
// }

#[skyline::hook(offset = 0xfb63e0, inline)]
unsafe fn cheer_cancel(ctx: &mut skyline::hooks::InlineCtx) {
    *ctx.registers[24].w.as_mut() = 0x11d;
}

pub fn install() {
    unsafe {
        // skyline::patching::Patch::in_text(0xfb63e8).data(0x17FFFF8B); // cheer cancel
        skyline::install_hooks!(
            // enable_partner_catch_transition,
            cheer_cancel
        );
    }
}