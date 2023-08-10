use super::*;
use globals::*;
mod special_s;


unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Reset sideB stall flag on landing or ledgegrab
    if [*SITUATION_KIND_GROUND, *SITUATION_KIND_CLIFF].contains(&fighter.global_table[SITUATION_KIND].get_i32()) {
        VarModule::off_flag(fighter.battle_object, vars::wiifit::instance::SPECIAL_S_DISABLE_STALL);
    }
    true.into()
}

#[smashline::fighter_init]
fn wiifit_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        if fighter.kind() == *FIGHTER_KIND_WIIFIT {
            fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));   
        }
    }
}

pub fn install() {
    smashline::install_agent_init_callbacks!(wiifit_init);
    special_s::install();
}