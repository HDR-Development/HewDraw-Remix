use super::*;

mod special_hi;

/// Prevents down b being reused
unsafe extern "C" fn should_use_special_lw_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::get_int(fighter.battle_object, vars::rosetta::instance::COOLDOWN) > 0 {
        false.into()
    } else {
        true.into()
    }
}

extern "C" fn rosetta_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        // set the callbacks on fighter init
        if fighter.kind() == *FIGHTER_KIND_ROSETTA {
            fighter.global_table[globals::USE_SPECIAL_LW_CALLBACK].assign(&L2CValue::Ptr(should_use_special_lw_callback as *const () as _));
        }
    }
}

pub fn install() {
    smashline::Agent::new("rosetta")
        .on_start(rosetta_init)
        .install();
}
