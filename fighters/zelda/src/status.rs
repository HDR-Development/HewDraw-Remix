use smash::app::sv_battle_object::module_accessor;

use super::*;

// Prevents side special from being used if a Din's Fire is present
unsafe extern "C" fn should_use_special_s_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, vars::zelda::instance::DEIN_ACTIVE) {
        false.into()
    } else {
        true.into()
    }
}

#[smashline::fighter_init]
fn zelda_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        // set the callbacks on fighter init
        if fighter.kind() == *FIGHTER_KIND_ZELDA {
            fighter.global_table[globals::USE_SPECIAL_S_CALLBACK].assign(&L2CValue::Ptr(should_use_special_s_callback as *const () as _));
        }
    }
}

pub fn install() {
    smashline::install_agent_init_callbacks!(zelda_init);
}