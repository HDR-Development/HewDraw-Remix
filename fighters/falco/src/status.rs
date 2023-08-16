use super::*;
use globals::*;

mod special_s;
mod special_hi;
mod special_lw;


unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Reset shine stall flag on landing or ledgegrab
    if [*SITUATION_KIND_GROUND, *SITUATION_KIND_CLIFF].contains(&fighter.global_table[SITUATION_KIND].get_i32()) {
        VarModule::off_flag(fighter.battle_object, vars::falco::instance::SPECIAL_LW_DISABLE_STALL);
    }
    true.into()
}

#[smashline::fighter_init]
fn falco_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        if fighter.kind() == *FIGHTER_KIND_FALCO {
            fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));   
        }
    }
}

pub fn install() {
    smashline::install_agent_init_callbacks!(falco_init);
    special_s::install();
    special_hi::install();
    special_lw::install();
}

pub fn add_statuses() {
    special_lw::install_custom();
}