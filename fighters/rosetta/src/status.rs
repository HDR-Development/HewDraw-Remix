use super::*;
/// Prevents down b being reused
unsafe extern "C" fn should_use_special_lw_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::get_int(fighter.battle_object, vars::rosetta::instance::COOLDOWN) > 0 || VarModule::get_int(fighter.battle_object, vars::rosetta::instance::TICO_RAYCAST) == 1 ||
	(VarModule::get_int(fighter.battle_object, vars::rosetta::instance::TICO_X_DIST) > 130 || VarModule::get_int(fighter.battle_object, vars::rosetta::instance::TICO_X_DIST) < -130)||
	(VarModule::get_int(fighter.battle_object, vars::rosetta::instance::TICO_Y_DIST) > 70 || VarModule::get_int(fighter.battle_object, vars::rosetta::instance::TICO_Y_DIST) < -70){
        false.into()
    } else {
        true.into()
    }
}

#[smashline::fighter_init]
fn rosetta_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        // set the callbacks on fighter init
        if fighter.kind() == *FIGHTER_KIND_ROSETTA {
            fighter.global_table[globals::USE_SPECIAL_LW_CALLBACK].assign(&L2CValue::Ptr(should_use_special_lw_callback as *const () as _));
        }
    }
}


pub fn install() {
    smashline::install_agent_init_callbacks!(rosetta_init);
}