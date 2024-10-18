use super::*;

#[skyline::hook(replace=request_change_pokemon)]
unsafe fn request_change_pokemon() -> bool {
    return false;
}

unsafe extern "C" fn special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let parent_id = LinkModule::get_parent_id(fighter.module_accessor, *FIGHTER_POKEMON_LINK_NO_PTRAINER, true) as u32;
    let object = utils::util::get_battle_object_from_id(parent_id);
    VarModule::on_flag(object, vars::ptrainer::instance::SPECIAL_LW_BACKWARDS_SWITCH); // we will turn this off in opff
    smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_LW)(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_main);
}
