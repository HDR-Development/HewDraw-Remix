use super::*;

unsafe extern "C" fn special_s_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_S_END 
    && !StatusModule::is_situation_changed(fighter.module_accessor) {
        VarModule::on_flag(fighter.battle_object, vars::zelda::status::DINS_REFRESH);
    } 
    0.into()
}

unsafe extern "C" fn special_s_end_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_LANDING {
        WorkModule::set_float(fighter.module_accessor, 8.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_end);
    agent.status(End, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_S_END, special_s_end_end);
}