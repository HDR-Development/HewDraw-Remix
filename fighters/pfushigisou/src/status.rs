use super::*;
use globals::*;
// status script import

// FIGHTER_RYU_STATUS_KIND_DASH_BACK //
#[status_script(agent = "pfushigisou", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn pre_down_special(fighter: &mut L2CFighterCommon) -> L2CValue {
    1.into()
}

#[status_script(agent = "pfushigisou", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn main_down_special(fighter: &mut L2CFighterCommon) -> L2CValue {
    1.into()
}

#[status_script(agent = "pfushigisou", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn end_down_special(fighter: &mut L2CFighterCommon) -> L2CValue {
    1.into()
}


extern "C" {
    #[link_name = "_ZN3app34WeaponSpecializer_PTrainerPTrainer22request_change_pokemonERNS_6WeaponE"]
    fn request_change_pokemon() -> bool;
}

#[skyline::hook(replace=request_change_pokemon)]
unsafe fn request_change_pokemon() -> bool {
    return false;
}


#[skyline::hook(offset = 0xf96310)]
unsafe fn stub_death_switch() {}

pub fn install() {
    install_status_scripts!(
        pre_down_special,
        main_down_special,
        end_down_special
    );
    skyline::install_hooks!(
        stub_death_switch,
        request_change_pokemon,
    );
}