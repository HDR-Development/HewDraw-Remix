use super::*;

unsafe extern "C" fn special_hi_finish2_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_MOTION_AIR,
        *GROUND_CORRECT_KIND_AIR as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_AIR_LASSO) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );

    0.into()
}

unsafe extern "C" fn special_hi_finish2_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // [h] momentum transfer the motion energy into the control energy
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    
    // [h] get the x/y speed
    let speed_x = {
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
        app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent)
    };

    let speed_y = {
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
        app::sv_kinetic_energy::get_speed_y(fighter.lua_state_agent)
    };

    // reset the energy with fall adjust no cap (no horizontal speed limit)
    let kinetic_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    app::lua_bind::KineticEnergy::reset_energy(kinetic_energy as _, *ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST_NO_CAP, &Vector2f { x: speed_x, y: speed_x }, &Vector3f::zero(), fighter.module_accessor);
    
    fighter.main_shift(special_hi_finish2_main_loop)
}

unsafe extern "C" fn special_hi_finish2_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    // [h] when the motion is over disable special hi jump and special s
    if MotionModule::is_end(fighter.module_accessor) {
        VarModule::on_flag(fighter.battle_object, vars::common::instance::UP_SPECIAL_CANCEL);
        VarModule::on_flag(fighter.battle_object, vars::elight::instance::DISABLE_SPECIAL_S);
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }

    0.into()
}

unsafe extern "C" fn special_hi_finish2_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    CustomStatusManager::add_new_agent_status_script(
        Hash40::new("fighter_kind_elight"),
        statuses::elight::SPECIAL_HI_FINISH2,
        StatusInfo::new()
            .with_pre(special_hi_finish2_pre)
            .with_main(special_hi_finish2_main)
            .with_end(special_hi_finish2_end)    
    );
}