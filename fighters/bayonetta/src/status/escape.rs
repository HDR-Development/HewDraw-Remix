use super::*;
use globals::*;

 
pub fn install() {
    install_status_scripts!(
        escape_f_end,
        escape_b_end
    );
}

// FIGHTER_STATUS_KIND_ESCAPE_F //

#[status_script(agent = "bayonetta", status = FIGHTER_STATUS_KIND_ESCAPE_F, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn escape_f_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND] != FIGHTER_BAYONETTA_STATUS_KIND_BATWITHIN {
        fighter.sub_status_end_EscaleFB();
    }
    0.into()
}

// FIGHTER_STATUS_KIND_ESCAPE_B //

#[status_script(agent = "bayonetta", status = FIGHTER_STATUS_KIND_ESCAPE_B, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn escape_b_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND] != FIGHTER_BAYONETTA_STATUS_KIND_BATWITHIN {
        fighter.sub_status_end_EscaleFB();
    }
    0.into()
}