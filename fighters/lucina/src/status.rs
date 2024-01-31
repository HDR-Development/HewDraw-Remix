use super::*;
mod special_s;
mod special_lw;

// stub out mask removal at the end of taunt
#[status_script(agent = "lucina", status = FIGHTER_STATUS_KIND_APPEAL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn appeal_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    1.into()
}

unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_GROUND) || fighter.is_situation(*SITUATION_KIND_CLIFF)
    || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD]) {
        VarModule::off_flag(fighter.battle_object, vars::lucina::instance::DISABLE_SPECIAL_LW);
    }
    true.into()
}
unsafe extern "C" fn should_use_special_lw_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) && VarModule::is_flag(fighter.battle_object, vars::lucina::instance::DISABLE_SPECIAL_LW) {
        false.into()
    } else {
        true.into()
    }
}

#[smashline::fighter_init]
fn lucina_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        if fighter.kind() != *FIGHTER_KIND_LUCINA {
            return;
        }
        fighter.global_table[globals::USE_SPECIAL_LW_CALLBACK].assign(&L2CValue::Ptr(should_use_special_lw_callback as *const () as _));
        fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));   
    }
}


pub fn install() {
    special_s::install();
    special_lw::install();

    install_status_scripts!(
        appeal_end
    );
    smashline::install_agent_init_callbacks!(lucina_init);
}