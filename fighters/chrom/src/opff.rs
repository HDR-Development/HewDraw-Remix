use super::*;

utils::import_noreturn!(common::opff::fighter_common_opff);

// Side Special Cancels
unsafe fn side_special_cancels(fighter: &mut L2CFighterCommon) {
    if !fighter.is_status_one_of(&[*FIGHTER_ROY_STATUS_KIND_SPECIAL_S3, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S4])
    || !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
    || fighter.is_in_hitlag()
    {
        return;
    }

    let transition_air = match MotionModule::motion_kind(fighter.module_accessor) {
        utils::hash40!("special_s3_hi") | utils::hash40!("special_air_s3_hi") if fighter.is_cat_flag(Cat1::AttackHi3) => {
            if fighter.is_situation(*SITUATION_KIND_GROUND) {
                fighter.change_status_req(*FIGHTER_STATUS_KIND_ATTACK_HI3, false);
                return;
            }
            fighter.is_situation(*SITUATION_KIND_AIR)
        },

        utils::hash40!("special_s3_hi") | utils::hash40!("special_air_s3_hi") if fighter.is_cat_flag(Cat1::AttackHi4) => {
            if fighter.is_situation(*SITUATION_KIND_GROUND) {
                fighter.change_status_req(*FIGHTER_STATUS_KIND_ATTACK_HI4_START, false);
                return;
            }
            fighter.is_situation(*SITUATION_KIND_AIR)
        },

        utils::hash40!("special_s3_s") | utils::hash40!("special_air_s3_s") if fighter.is_cat_flag(Cat1::AttackS3) => {
            if fighter.is_situation(*SITUATION_KIND_GROUND) {
                fighter.change_status_req(*FIGHTER_STATUS_KIND_ATTACK_S3, false);
                return;
            }
            fighter.is_situation(*SITUATION_KIND_AIR)
        },

        utils::hash40!("special_s3_s") | utils::hash40!("special_air_s3_s") if fighter.is_cat_flag(Cat1::AttackS4) => {
            if fighter.is_situation(*SITUATION_KIND_GROUND) {
                fighter.change_status_req(*FIGHTER_STATUS_KIND_ATTACK_S4_START, false);
                return;
            }
            fighter.is_situation(*SITUATION_KIND_AIR)
        },

        utils::hash40!("special_s3_lw") | utils::hash40!("special_air_s3_lw") if fighter.is_cat_flag(Cat1::AttackLw3) => {
            if fighter.is_situation(*SITUATION_KIND_GROUND) {
                fighter.change_status_req(*FIGHTER_STATUS_KIND_ATTACK_LW3, false);
                return;
            }
            fighter.is_situation(*SITUATION_KIND_AIR)
        },

        utils::hash40!("special_s3_lw") | utils::hash40!("special_air_s3_lw") if fighter.is_cat_flag(Cat1::AttackLw4) => {
            if fighter.is_situation(*SITUATION_KIND_GROUND) {
                fighter.change_status_req(*FIGHTER_STATUS_KIND_ATTACK_LW4_START, false);
                return;
            }
            fighter.is_situation(*SITUATION_KIND_AIR)
        },

        utils::hash40!("special_s4_hi") | utils::hash40!("special_air_s4_hi") if !fighter.is_in_hitlag() => {
            false
        }
        _ => false
    };

    if transition_air {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_ATTACK_AIR, false);
    }
}

pub unsafe fn double_edge_dance_during_hitlag(fighter: &mut L2CFighterCommon) {
    if !fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S2, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S3]) {
        return;
    }
    if fighter.global_table[globals::SUB_STATUS].get_bool() {
        // disables the original substatus - I'd rather not run it twice.
        fighter.global_table[globals::SUB_STATUS].assign(&L2CValue::Void());
    }
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_S_FLAG_INPUT_FAILURE) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_S_FLAG_INPUT_SUCCESS) {
            return;
        }
        if !ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            return;
        }
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_S_FLAG_INPUT_CHECK) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_S_FLAG_INPUT_FAILURE);
        }
        else {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_S_FLAG_INPUT_SUCCESS);
            let enable_hi_lw = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("enable_input_hi_lw"));
            if enable_hi_lw == 0 {
                return;
            }
            let stick_y = fighter.global_table[globals::STICK_Y].get_f32();
            let squat_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("squat_stick_y"));
            if stick_y > -squat_stick_y {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_S_FLAG_INPUT_HI);
            }
            else if stick_y < squat_stick_y {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_S_FLAG_INPUT_LW);
            }
        }
    }
}

pub unsafe fn double_edge_dance_vertical_momentum(fighter: &mut L2CFighterCommon){
    let fighter_gravity = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) as *mut app::FighterKineticEnergyGravity;
    if fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S2]) && fighter.is_situation(*SITUATION_KIND_AIR) {
        lua_bind::FighterKineticEnergyGravity::set_accel(fighter_gravity, -0.072);
        lua_bind::FighterKineticEnergyGravity::set_stable_speed(fighter_gravity, -2.0);
    }

    if fighter.is_situation(*SITUATION_KIND_GROUND) && VarModule::is_flag(fighter.battle_object, vars::common::instance::SPECIAL_STALL_USED) {
        VarModule::off_flag(fighter.battle_object, vars::common::instance::SPECIAL_STALL_USED);
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END,
        *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END2,
        *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3,
        *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_LOOP,
        *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_TURN,
        *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END_MAX,
        *FIGHTER_ROY_STATUS_KIND_SPECIAL_LW_HIT
        ])
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
        if fighter.is_flag(*FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
            if [*FIGHTER_KINETIC_TYPE_MOTION_AIR, *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE].contains(&KineticModule::get_kinetic_type(fighter.module_accessor)) {
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
                let speed_y = app::sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);

                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y, 0.0, 0.0, 0.0);
                app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);

                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                app::sv_kinetic_energy::enable(fighter.lua_state_agent);

                KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
            }
        }
    }
}

unsafe fn sword_length(boma: &mut BattleObjectModuleAccessor) {
    let long_sword_scale = Vector3f{x: 1.015, y: 1.065, z: 1.045};
    ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("havel"), &long_sword_scale);
    ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("haver"), &long_sword_scale);
}

pub unsafe extern "C" fn chrom_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    common::opff::fighter_common_opff(fighter);
    side_special_cancels(fighter);
    double_edge_dance_vertical_momentum(fighter);
    fastfall_specials(fighter);
    sword_length(&mut *(fighter.module_accessor));
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, chrom_frame_wrapper); 
}
