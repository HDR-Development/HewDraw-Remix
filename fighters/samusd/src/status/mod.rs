use super::*;
use globals::*;
// status script import

mod attack_air;
mod attack_lw3;
mod float;
mod special_hi;
mod special_n;
mod special_s;

extern "Rust" {
    #[link_name = "float_check_air_jump"]
    fn float_check_air_jump(fighter: &mut L2CFighterCommon, float_status: L2CValue) -> L2CValue;
    #[link_name = "float_check_air_jump_aerial"]
    fn float_check_air_jump_aerial(fighter: &mut L2CFighterCommon, float_status: L2CValue) -> L2CValue;
}

unsafe extern "C" fn air_jump_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    float_check_air_jump(fighter, statuses::samusd::FLOAT.into())
}

unsafe extern "C" fn air_jump_aerial_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    float_check_air_jump_aerial(fighter, statuses::samusd::FLOAT.into())
}

unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_GROUND) || fighter.is_situation(*SITUATION_KIND_CLIFF)
    || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_GIMMICK_SPRING_JUMP]) {
        VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_FLOATING);
        if !fighter.is_situation(*SITUATION_KIND_CLIFF) {
            VarModule::on_flag(fighter.battle_object, vars::common::instance::ENABLE_FLOAT);
        }
    }
    true.into()
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    fighter.global_table[globals::CHECK_AIR_JUMP_UNIQ].assign(&L2CValue::Ptr(air_jump_uniq as *const () as _));
    fighter.global_table[globals::CHECK_AIR_JUMP_AERIAL_UNIQ].assign(&L2CValue::Ptr(air_jump_aerial_uniq as *const () as _));
    fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));   
    VarModule::on_flag(fighter.battle_object, vars::common::instance::OMNI_FLOAT);
    VarModule::set_int(fighter.battle_object, vars::common::instance::FLOAT_STATUS_KIND, statuses::samusd::FLOAT);
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);

    attack_air::install(agent);
    attack_lw3::install(agent);
    float::install(agent);
    special_hi::install(agent);
    special_n::install(agent);
    special_s::install(agent);
}