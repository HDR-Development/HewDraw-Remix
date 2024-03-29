use super::*;
use globals::*;

mod jump_aerial;
mod attack_air;
mod float;
mod fall;
mod special_n;

extern "Rust" {
    #[link_name = "float_check_air_jump"]
    fn float_check_air_jump(fighter: &mut L2CFighterCommon, float_status: L2CValue) -> L2CValue;
    #[link_name = "float_check_air_jump_aerial"]
    fn float_check_air_jump_aerial(fighter: &mut L2CFighterCommon, float_status: L2CValue) -> L2CValue;
}

unsafe extern "C" fn air_jump_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    float_check_air_jump(fighter, statuses::mewtwo::FLOAT.into())
}

unsafe extern "C" fn air_jump_aerial_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    float_check_air_jump_aerial(fighter, statuses::mewtwo::FLOAT.into())
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    fighter.global_table[0x32].assign(&L2CValue::Ptr(air_jump_uniq as *const () as _));
    fighter.global_table[0x33].assign(&L2CValue::Ptr(air_jump_aerial_uniq as *const () as _));
    VarModule::set_int(fighter.battle_object, vars::common::instance::FLOAT_DURATION, 60);
    VarModule::on_flag(fighter.battle_object, vars::common::instance::OMNI_FLOAT);
    VarModule::set_int(fighter.battle_object, vars::common::instance::FLOAT_STATUS_KIND, statuses::mewtwo::FLOAT);
}

pub fn install() {
    jump_aerial::install();
    attack_air::install();
    float::install();
    fall::install();
    special_n::install();
    smashline::Agent::new("mewtwo")
        .on_start(on_start)
        .install();
}