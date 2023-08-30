use super::*;
use globals::*;

mod special_s;
mod special_n;
mod special_hi;


pub fn install() {
    install_status_scripts!(
        end_jump_squat
    );
    smashline::install_agent_init_callbacks!(diddy_init);
    special_s::install();
    special_n::install();
    special_hi::install();
}

pub fn add_statuses() {
    special_n::install_custom();
}

// FIGHTER_STATUS_KIND_JUMP_SQUAT

#[status_script(agent = "diddy", status = FIGHTER_STATUS_KIND_JUMP_SQUAT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn end_jump_squat(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_JumpSquat();
    0.into()
}

// Prevents sideB from being used again if it has already been used once in the current airtime
unsafe extern "C" fn should_use_special_s_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) && VarModule::is_flag(fighter.battle_object, vars::diddy::instance::DISABLE_SPECIAL_S) {
        false.into()
    } else {
        true.into()
    }
}

// Re-enables the ability to use sideB when connecting to ground or cliff
unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_GROUND) || fighter.is_situation(*SITUATION_KIND_CLIFF)
    || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LANDING]) {
        VarModule::off_flag(fighter.battle_object, vars::diddy::instance::DISABLE_SPECIAL_S);
    }
    true.into()
}

#[smashline::fighter_init]
fn diddy_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        // set the callbacks on fighter init
        if fighter.kind() == *FIGHTER_KIND_DIDDY {
            fighter.global_table[globals::USE_SPECIAL_S_CALLBACK].assign(&L2CValue::Ptr(should_use_special_s_callback as *const () as _));
            fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));   
        }
    }
}