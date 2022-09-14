use smash::app::sv_battle_object::module_accessor;

use super::*;

/// Prevents down special from being used if a bomb is present
unsafe extern "C" fn should_use_special_lw_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ArticleModule::is_exist(fighter.boma(), *FIGHTER_SAMUS_GENERATE_ARTICLE_BOMB) {// && VarModule::is_flag(fighter.battle_object, vars::samusd::instance::MANUAL_DETONATE_READY) {
        false.into()
    } else {
        true.into()
    }
}

/// Re-enables the ability to use down special once the bomb has been detonated
unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !ArticleModule::is_exist(fighter.boma(), *FIGHTER_SAMUS_GENERATE_ARTICLE_BOMB)
        || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD]) {
        VarModule::off_flag(fighter.battle_object, vars::samusd::instance::DISABLE_SPECIAL_LW);
    }
    true.into()
}

#[smashline::fighter_init]
fn samusd_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        // set the callbacks on fighter init
        if fighter.kind() == *FIGHTER_KIND_SAMUSD {
            fighter.global_table[globals::USE_SPECIAL_LW_CALLBACK].assign(&L2CValue::Ptr(should_use_special_lw_callback as *const () as _));
            fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));   
        }
    }
}


pub fn install() {
    smashline::install_agent_init_callbacks!(samusd_init);
}