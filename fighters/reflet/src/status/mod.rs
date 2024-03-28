use super::*;
use globals::*;
// status script import

mod attack_air;

mod special_n;

mod float;

extern "Rust" {
    #[link_name = "float_check_air_jump"]
    fn float_check_air_jump(fighter: &mut L2CFighterCommon, float_status: L2CValue) -> L2CValue;
    #[link_name = "float_check_air_jump_aerial"]
    fn float_check_air_jump_aerial(fighter: &mut L2CFighterCommon, float_status: L2CValue) -> L2CValue;
}

unsafe extern "C" fn reflet_air_jump_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT) <= 0 {
        return false.into();
    }
    float_check_air_jump(fighter, statuses::reflet::FLOAT.into())
}

unsafe extern "C" fn reflet_air_jump_aerial_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT) <= 0 {
        return false.into();
    }
    float_check_air_jump_aerial(fighter, statuses::reflet::FLOAT.into())
}

unsafe extern "C" fn reflet_on_start(fighter: &mut L2CFighterCommon) {
    fighter.global_table[0x32].assign(&L2CValue::Ptr(reflet_air_jump_uniq as *const () as _));
    fighter.global_table[0x33].assign(&L2CValue::Ptr(reflet_air_jump_aerial_uniq as *const () as _));
    VarModule::set_int(fighter.battle_object, vars::common::instance::FLOAT_STATUS_KIND, statuses::reflet::FLOAT);
}

pub fn install(agent: &mut Agent) {
    agent.on_start(reflet_on_start);

    attack_air::install(agent);

    special_n::install(agent);

    float::install(agent);
}
