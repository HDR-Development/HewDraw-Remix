use super::*;
use globals::*;

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_AttackS3Common,
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}

#[skyline::hook(replace = L2CFighterCommon_status_AttackS3Common)]
unsafe fn status_AttackS3Common(fighter: &mut L2CFighterCommon) {
    ControlModule::reset_trigger(fighter.module_accessor);

    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_attack3_uniq_check(false.into());
    }

    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_attack3_uniq_check as *const () as _));
    
    let attack_s3_cstick_x = VarModule::get_float(fighter.battle_object, vars::common::instance::ATTACK_S3_CSTICK_X);
    if attack_s3_cstick_x != 0.0 {
        PostureModule::set_lr(fighter.module_accessor, attack_s3_cstick_x.signum());
    }
    else {
        PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
    }

    PostureModule::update_rot_y_lr(fighter.module_accessor);

    fighter.clear_lua_stack();
    fighter.push_lua_stack(&mut L2CValue::I32(*FIGHTER_KINETIC_ENERGY_ID_MOTION));
    let mut lr = PostureModule::lr(fighter.module_accessor);
    fighter.push_lua_stack(&mut L2CValue::F32(lr));
    app::sv_kinetic_energy::set_chara_dir(fighter.lua_state_agent);
}