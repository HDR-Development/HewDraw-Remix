use super::*;
use globals::*;
// status script import

pub mod helper;
mod walk;
mod attack;
mod attack_s3;
mod attack_s4;

mod attack_air;
mod ladder_attack;

mod special_n;
mod rockbuster;

mod special_s;

mod special_lw;

unsafe extern "C" fn check_special_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, vars::rockman::instance::CHARGE_SHOT_RELEASE)
    && VarModule::is_flag(fighter.battle_object, vars::rockman::instance::CHARGE_SHOT_PLAYED_FX) {
        fighter.global_table[CMD_CAT1].assign(&L2CValue::I32(*FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N));
    }
    false.into()
}

unsafe extern "C" fn special_lw_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    (!WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_LEAFSHIELD)).into()
}

unsafe extern "C" fn check_turn_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    let leafshield = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_LEAFSHIELD);
    WorkModule::set_flag(fighter.module_accessor, leafshield, *FIGHTER_STATUS_TURN_FLAG_NO_TURN_TO_ESCAPE);
    false.into()
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    fighter.global_table[0x27].assign(&L2CValue::Ptr(check_special_uniq as *const () as _));
    fighter.global_table[0x26].assign(&L2CValue::Ptr(check_special_uniq as *const () as _));
    fighter.global_table[DASH_CALLBACK].assign(&false.into());
    fighter.global_table[0x2A].assign(&false.into());
    fighter.global_table[0x2B].assign(&false.into());
    fighter.global_table[0x34].assign(&false.into());
    fighter.global_table[0x35].assign(&L2CValue::Ptr(check_turn_uniq as *const () as _));
    fighter.global_table[0x4E].assign(&false.into());
    fighter.global_table[USE_SPECIAL_LW_CALLBACK].assign(&L2CValue::Ptr(special_lw_uniq as *const () as _));
}

unsafe extern "C" fn rebirth_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mot = MotionModule::motion_kind(fighter.module_accessor);
    if [
        hash40("appeal_lw_l"),
        hash40("appeal_lw_r")
    ].contains(&mot) {
        VisibilityModule::set_whole(fighter.module_accessor, true);
    }
    fighter.status_end_Rebirth();
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);
    agent.status(smashline::End, *FIGHTER_STATUS_KIND_REBIRTH, rebirth_end);

    walk::install(agent);

    attack::install(agent);

    attack_s3::install(agent);

    attack_s4::install(agent);

    attack_s4::install(agent);

    attack_air::install(agent);
    ladder_attack::install(agent);
    
    special_n::install(agent);
    rockbuster::install(agent);

    special_s::install(agent);

    special_lw::install(agent);
}
