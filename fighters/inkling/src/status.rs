use super::*;

mod special_s;

#[smashline::status_script(agent = "inkling", status = FIGHTER_STATUS_KIND_GUARD_ON, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn guard_on(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_GuardOn()
}

#[smashline::status_script(agent = "inkling", status = FIGHTER_STATUS_KIND_GUARD, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn guard(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Guard()
}

pub fn install() {
    smashline::install_status_scripts!(guard_on, guard);
    special_s::install();
}
