use super::*;
use globals::*;

pub mod helper;
mod walk;
mod attack;
mod attack_s3;
mod attack_s4;

mod attack_air;
mod ladder_attack;

mod special_n;
mod rockbuster;
mod chargeshot;

mod special_s;

mod special_lw;

unsafe extern "C" fn rockman_check_special_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, vars::rockman::instance::CHARGE_SHOT_RELEASE)
    && VarModule::is_flag(fighter.battle_object, vars::rockman::instance::CHARGE_SHOT_PLAYED_FX) {
        fighter.global_table[CMD_CAT1].assign(&L2CValue::I32(*FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N));
    }
    false.into()
}

unsafe extern "C" fn rockman_special_lw_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    (!WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_LEAFSHIELD)).into()
}

#[fighter_reset]
fn agent_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind != *FIGHTER_KIND_ROCKMAN {
            return;
        }
        fighter.global_table[0x27].assign(&L2CValue::Ptr(rockman_check_special_uniq as *const () as _));
        fighter.global_table[0x26].assign(&L2CValue::Ptr(rockman_check_special_uniq as *const () as _));
        fighter.global_table[DASH_CALLBACK].assign(&false.into());
        fighter.global_table[0x2A].assign(&false.into());
        fighter.global_table[0x2B].assign(&false.into());
        fighter.global_table[0x34].assign(&false.into());
        fighter.global_table[0x35].assign(&false.into());
        fighter.global_table[0x4E].assign(&false.into());
        fighter.global_table[USE_SPECIAL_LW_CALLBACK].assign(&L2CValue::Ptr(rockman_special_lw_uniq as *const () as _));
    }
}

pub fn install() {
    install_agent_resets!(
        agent_reset
    );

    walk::install();

    attack::install();

    attack_s3::install();

    attack_s4::install();

    attack_s4::install();

    attack_air::install();
    ladder_attack::install();
    chargeshot::install();

    special_n::install();
    rockbuster::install();

    special_s::install();

    special_lw::install();
}