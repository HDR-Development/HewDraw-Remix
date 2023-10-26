use super::*;
use globals::*;

//=================================================================
//== KineticModule::change_kinetic
//== Note: Double jump cancels for Ness, Lucas, and Mewtwo
//== Note: This changes the kinetic energy, not the animation
//=================================================================
#[skyline::hook(replace=KineticModule::change_kinetic)]
unsafe fn change_kinetic_hook(boma: &mut BattleObjectModuleAccessor, kinetic_type: i32) -> i32 {
    let mut kinetic_type_new = kinetic_type;
    if boma.is_fighter() {
        let status_kind = StatusModule::status_kind(boma);
        let situation_kind = StatusModule::situation_kind(boma);
        let fighter_kind = boma.kind();
        if situation_kind == *SITUATION_KIND_AIR
        && ( ([*FIGHTER_KIND_FALCO, *FIGHTER_KIND_FOX, *FIGHTER_KIND_GAMEWATCH, *FIGHTER_KIND_WOLF].contains(&fighter_kind) && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N) )
        {
            kinetic_type_new = *FIGHTER_KINETIC_TYPE_FALL;
        }
    }
    original!()(boma, kinetic_type_new)
}

pub fn install() {
    skyline::install_hooks!(
        change_kinetic_hook,
    );
}
