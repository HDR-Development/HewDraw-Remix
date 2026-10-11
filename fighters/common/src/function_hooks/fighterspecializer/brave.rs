use super::*;

#[skyline::hook(replace = FighterSpecializer_Brave::get_special_lw_command_sp_cost)]
pub unsafe fn get_special_lw_command_sp_cost(boma: *mut BattleObjectModuleAccessor, command: i32, param_3: bool) -> f32 {
    let ret = original!()(boma, command, param_3);
    let object = (&mut *(boma)).object();
    if VarModule::is_flag(object, vars::brave::instance::SPECIAL_MENU) {
        return 0.0;
    }

    return ret;
}

pub fn install() {
    skyline::install_hooks!(
        get_special_lw_command_sp_cost
    );
}
