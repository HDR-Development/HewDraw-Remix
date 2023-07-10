use super::*;

mod attack;
mod batwithin;
mod escape;

/// ABK allow one dabk 2 abks
unsafe extern "C" fn should_use_special_s_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) 
    && ((fighter.get_int(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_AIR_S_USED_COUNT) == 2 && VarModule::get_int(fighter.battle_object, vars::bayonetta::instance::DABK_COUNT) == 0 )
    || (fighter.get_int(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_AIR_S_USED_COUNT) >= 1 && !VarModule::is_flag(fighter.battle_object, vars::bayonetta::instance::SPECIAL_HIT))
    || fighter.get_int(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_AIR_S_USED_COUNT) > 2
    || VarModule::get_int(fighter.battle_object, vars::bayonetta::instance::DABK_COUNT) > 1)
    {
        false.into()
    } else {
        true.into()
    }
}

#[smashline::fighter_init]
fn bayonetta_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        // set the callbacks on fighter init
        if fighter.kind() == *FIGHTER_KIND_BAYONETTA {
            fighter.global_table[globals::USE_SPECIAL_S_CALLBACK].assign(&L2CValue::Ptr(should_use_special_s_callback as *const () as _));
        }
    }
}

pub fn install() {
    smashline::install_agent_init_callbacks!(bayonetta_init);
    attack::install();
    batwithin::install();
    escape::install();
}