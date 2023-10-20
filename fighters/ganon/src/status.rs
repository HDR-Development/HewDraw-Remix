use super::*;

mod special_n;
mod special_n_float;
mod special_lw;
mod special_s;

/// Prevents side b from being used again in air when it has been disabled by up-b fall
unsafe extern "C" fn should_use_special_n_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        let float_status = CustomStatusModule::get_agent_status_kind(fighter.battle_object, statuses::ganon::SPECIAL_N_FLOAT);
        if VarModule::is_flag(fighter.battle_object, vars::ganon::instance::DISABLE_SPECIAL_N) || fighter.is_status(float_status) {
            false.into()
        }
        else {
            true.into()
        }
    } else {
        true.into()
    }
}

/// Re-enables the ability to use aerial specials when connecting to ground or cliff
unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_GROUND) || fighter.is_situation(*SITUATION_KIND_CLIFF)
    || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LANDING]) {
        VarModule::off_flag(fighter.battle_object, vars::ganon::instance::DISABLE_SPECIAL_N);
    }
    true.into()
}

#[smashline::fighter_init]
fn ganon_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        // set the callbacks on fighter init
        if fighter.kind() == *FIGHTER_KIND_GANON {
            fighter.global_table[globals::USE_SPECIAL_N_CALLBACK].assign(&L2CValue::Ptr(should_use_special_n_callback as *const () as _));
            fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));   
        }
    }
}


pub fn install() {
    smashline::install_agent_init_callbacks!(ganon_init);
    special_n::install();
    special_lw::install();
    special_s::install();
}

pub fn add_statuses() {
    special_n_float::install();
}