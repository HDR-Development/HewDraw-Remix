use super::*;
use globals::*;

#[skyline::hook(replace=request_change_pokemon)]
unsafe fn request_change_pokemon() -> bool {
    return false;
}

#[status_script(agent = "pzenigame", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let parent_id = LinkModule::get_parent_id(fighter.module_accessor, *FIGHTER_POKEMON_LINK_NO_PTRAINER, true) as u32;
    let object = utils::util::get_battle_object_from_id(parent_id);
    VarModule::on_flag(object, vars::ptrainer::instance::IS_SWITCH_BACKWARDS); // we will turn this off in opff
    original!(fighter)
}

pub fn install() {
    install_status_scripts!(
        special_lw_main
    );
}
