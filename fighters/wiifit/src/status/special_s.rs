use super::*;
use globals::*;


// FIGHTER_STATUS_KIND_SPECIAL_S


unsafe extern "C" fn special_s_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        0.0,
        0.0
    );
    sv_kinetic_energy!(
        controller_set_accel_x_add,
        fighter,
        0.0
    );
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    
    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_MOTION,
        0.0,
        0.0
    );
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);

    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        0.0,
        0.0
    );
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);

    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR
    && !VarModule::is_flag(fighter.battle_object, vars::wiifit::instance::SPECIAL_S_DISABLE_STALL) {
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            ENERGY_GRAVITY_RESET_TYPE_GRAVITY,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0
        );

        let heading_init_speed_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("heading_init_speed_y"));

        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            heading_init_speed_y
        );

        let heading_gravity = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("heading_gravity"));

        sv_kinetic_energy!(
            set_accel,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            -heading_gravity
        );

        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }

    0.into()
}


unsafe extern "C" fn special_s_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::status_kind_next(fighter.module_accessor) != *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_S_JUMP {
        VarModule::set_flag(fighter.battle_object, vars::wiifit::instance::SPECIAL_S_DISABLE_STALL, fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR);
    }
    0.into()
}

// FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_S_JUMP


unsafe extern "C" fn special_s_jump_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, vars::wiifit::instance::SPECIAL_S_DISABLE_STALL) {
        return 0.into();
    }
    smashline::original_status(Init, fighter, *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_S_JUMP)(fighter)
}


unsafe extern "C" fn special_s_jump_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::set_flag(fighter.battle_object, vars::wiifit::instance::SPECIAL_S_DISABLE_STALL, fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR);
    0.into()
}


pub fn install() {
    smashline::Agent::new("wiifit")
        .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_init)
        .status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_end)
        .status(
            Init,
            *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_S_JUMP,
            special_s_jump_init,
        )
        .status(
            End,
            *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_S_JUMP,
            special_s_jump_end,
        )
        .install();
}
