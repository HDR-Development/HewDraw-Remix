use super::*;
use globals::*;

extern "Rust" {
    #[link_name = "attack_air_float_pre"]
    fn attack_air_float_pre(fighter: &mut L2CFighterCommon, float_status: L2CValue) -> L2CValue;
    #[link_name = "attack_air_float_main"]
    fn attack_air_float_main(fighter: &mut L2CFighterCommon, float_status: L2CValue) -> L2CValue;
}

unsafe extern "C" fn reflet_attack_air_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    attack_air_float_pre(fighter, statuses::reflet::FLOAT.into())
}

pub unsafe extern "C" fn init_attack_air(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_STATUS_KIND].get_i32() != statuses::reflet::FLOAT {
        VisibilityModule::set_int64(fighter.module_accessor, Hash40::new("sword").hash as i64, Hash40::new("sword_normal").hash as i64);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON);
    }
    fighter.sub_attack_air_uniq_process_init();
    0.into()
}

unsafe extern "C" fn reflet_attack_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    attack_air_float_main(fighter, statuses::reflet::FLOAT.into())
}

pub fn install() {
    smashline::Agent::new("reflet")
        .status(Pre, *FIGHTER_STATUS_KIND_ATTACK_AIR, reflet_attack_air_pre)
        .status(Init, *FIGHTER_STATUS_KIND_ATTACK_AIR, init_attack_air)
        .status(Main, *FIGHTER_STATUS_KIND_ATTACK_AIR, reflet_attack_air_main)
        .install();
}
