use smash::app::sv_battle_object::module_accessor;

use super::*;

// Prevents down special from being used if a bomb is present
unsafe extern "C" fn should_use_special_lw_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Grab the stored bomb ID
    let bomb_object_id = VarModule::get_int(fighter.battle_object, vars::samusd::instance::BOMB_OBJECT_ID) as u32;
    // Check if the stored object ID is *actually* a Dark Samus Bomb or not.
    if sv_battle_object::is_active(bomb_object_id)
    && sv_battle_object::category(bomb_object_id) == *BATTLE_OBJECT_CATEGORY_WEAPON
    && sv_battle_object::kind(bomb_object_id) == *WEAPON_KIND_SAMUSD_BOMB {
        false.into()
    } else {
        true.into()
    }
}

#[smashline::fighter_init]
fn samusd_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        // set the callbacks on fighter init
        if fighter.kind() == *FIGHTER_KIND_SAMUSD {
            fighter.global_table[globals::USE_SPECIAL_LW_CALLBACK].assign(&L2CValue::Ptr(should_use_special_lw_callback as *const () as _));
        }
    }
}


pub fn install() {
    smashline::install_agent_init_callbacks!(samusd_init);
}