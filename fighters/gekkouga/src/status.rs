use super::*;

mod special_lw;
mod special_lw_jump;

unsafe extern "C" fn gekkouga_special_lw_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    (!WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_SAVE_SPEED)).into()
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    fighter.global_table[globals::USE_SPECIAL_LW_CALLBACK].assign(&L2CValue::Ptr(gekkouga_special_lw_uniq as *const () as _));
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);

    special_lw::install(agent);
    special_lw_jump::install(agent);
}