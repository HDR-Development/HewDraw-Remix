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
        /*   --------------  SPACIE LASER MOMENTUM  -----------------  */
        match crate::function_hooks::momentum_transfer::change_kinetic_momentum_related(boma, kinetic_type_new) {
            Some(x) => kinetic_type_new = x,
            None => ()
        }
        /*   --------------------------------------------------------  */    
    }
    original!()(boma, kinetic_type_new)
}

pub fn install() {
    skyline::install_hooks!(
        change_kinetic_hook,
    );
}
