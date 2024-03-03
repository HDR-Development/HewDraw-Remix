use super::*;
use globals::*;

pub unsafe extern "C" fn init_attack_air(fighter: &mut L2CFighterCommon) -> L2CValue {
    VisibilityModule::set_int64(fighter.module_accessor, Hash40::new("sword").hash as i64, Hash40::new("sword_normal").hash as i64);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON);
    fighter.sub_attack_air_uniq_process_init();
    0.into()
}

pub fn install() {
    smashline::Agent::new("reflet")
        .status(Init, *FIGHTER_STATUS_KIND_ATTACK_AIR, init_attack_air)
        .install();
}
