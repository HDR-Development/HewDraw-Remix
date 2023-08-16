use super::*;

// Prevents side special from being used if a missile is present
unsafe extern "C" fn should_use_special_s_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Grab the stored missile ID
    let missile_object_id = VarModule::get_int(fighter.battle_object, vars::miigunner::instance::MISSILE_OBJECT_ID) as u32;
    // Check if the stored object ID is *actually* a Gunner missile or not.
    if sv_battle_object::is_active(missile_object_id)
    && sv_battle_object::category(missile_object_id) == *BATTLE_OBJECT_CATEGORY_WEAPON
    && sv_battle_object::kind(missile_object_id) == *WEAPON_KIND_MIIGUNNER_SUPERMISSILE {
        false.into()
    } else {
        true.into()
    }
}

#[smashline::fighter_init]
fn miigunner_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        // set the callbacks on fighter init
        if fighter.kind() == *FIGHTER_KIND_MIIGUNNER {
            fighter.global_table[globals::USE_SPECIAL_S_CALLBACK].assign(&L2CValue::Ptr(should_use_special_s_callback as *const () as _));
        }
    }
}

pub fn install() {
    smashline::install_agent_init_callbacks!(miigunner_init);
}