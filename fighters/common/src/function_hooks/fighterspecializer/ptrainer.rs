use super::*;

#[skyline::hook(offset = 0x34ceab0)]
pub unsafe extern "C" fn request_change_pokemon(weapon: &mut smash::app::Weapon) -> u64 {
    let boma = weapon.battle_object.module_accessor;
    let object = utils::util::get_battle_object_from_accessor(boma);
    if VarModule::get_int(object, vars::ptrainer::instance::SPECIAL_LW_SWAP_TIMER) > 0 {
        return 0;
    }

    original!()(weapon)
}

pub fn install() {
    skyline::install_hooks!(
        request_change_pokemon
    );
}