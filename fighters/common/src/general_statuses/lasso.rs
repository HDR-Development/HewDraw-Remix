// status imports
use super::*;
use globals::*;

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_air_lasso_hang_uniq
        );
    }
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_air_lasso_hang_uniq)]
pub unsafe fn sub_air_lasso_hang_uniq(fighter: &mut L2CFighterCommon, arg1: L2CValue) -> L2CValue {
    let ret = original!()(fighter, arg1);
    if fighter.status_frame() < ParamModule::get_int(fighter.battle_object, ParamType::Common, "lasso_hang_fall_disable_frame") {
        fighter.off_flag(*FIGHTER_STATUS_AIR_LASSO_HANG_FLAG_FALL_ENABLE);
    }
    ret
}