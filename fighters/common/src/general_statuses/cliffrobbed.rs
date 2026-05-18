use super::*;
use globals::*;

// This file contains code for ledge trumps

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_CliffRobbed,
            sub_cliff_robbed_uniq
        );
    }
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_CliffRobbed)]
unsafe fn status_CliffRobbed(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_CLIFF_ROBBED);

    let motion = if fighter.global_table[FIGHTER_KIND] == FIGHTER_KIND_KOOPAG {
        Hash40::new("fall")
    } else {
        Hash40::new("damage_air_2")
    };

    let cliff_robbed_no_control_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("cliff_robbed_no_control_frame"));

    let rate = fighter.sub_get_adjust_rate_from_cancel_frame(L2CValue::Hash40(motion), L2CValue::I32(cliff_robbed_no_control_frame)).get_f32();

    MotionModule::change_motion(fighter.module_accessor, motion, 0.0, rate, false, 0.0, false, false);

    // <HDR>

    KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, fighter.module_accessor);

    let cliff_robbed_speed = ParamModule::get_float(fighter.battle_object, ParamType::Common, "cliff_robbed_speed");
    let cliff_robbed_angle = ParamModule::get_float(fighter.battle_object, ParamType::Common, "cliff_robbed_angle");
    let mut cliff_robbed_true_angle = if PostureModule::lr(fighter.module_accessor) == -1.0 {
        (180.0 - cliff_robbed_angle).to_radians()
    } else {
        cliff_robbed_angle.to_radians()
    };
    let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
    let stick_x = fighter.global_table[STICK_X].get_f32();
    let stick_y = fighter.global_table[STICK_Y].get_f32();

    // Derives initial x/y launch speed from base (parameterized) ledge trump values
    let mut cliff_robbed_speed_x = cliff_robbed_true_angle.cos() * cliff_robbed_speed;
    let mut cliff_robbed_speed_y = cliff_robbed_true_angle.sin() * cliff_robbed_speed;

    if stick_x != 0.0
    || stick_y != 0.0 {
        // Performs "DI" on your ledge trump launch angle
        // This logic is copied from normal DI calculations
        let cliff_robbed_correction_max = ParamModule::get_float(fighter.battle_object, ParamType::Common, "cliff_robbed_correction_max");
        let max_di_radians = cliff_robbed_correction_max.to_radians();

        let stick_calc = (cliff_robbed_speed_y * stick_x) - (cliff_robbed_speed_x * stick_y);
        let mut ratio = stick_calc.abs() / cliff_robbed_speed;
        let stick_calc_2 = (cliff_robbed_speed_x * stick_y) - (cliff_robbed_speed_y * stick_x);
        if stick_calc_2 < 0.0 {
            ratio *= -1.0;
        }

        // Determines your new launch angle
        cliff_robbed_true_angle += (max_di_radians * ratio);

        // Derives new initial x/y launch speed from your updated launch angle
        cliff_robbed_speed_x = cliff_robbed_true_angle.cos() * cliff_robbed_speed;
        cliff_robbed_speed_y = cliff_robbed_true_angle.sin() * cliff_robbed_speed;
    }

    // Applies a vertical speed boost to characters with higher gravity
    // This normalizes the launch distance somewhat
    // Note: 0.11 is HDR's median gravity value
    let gravity_modifier = (air_accel_y - 0.11) * 17.5;
    cliff_robbed_speed_y += gravity_modifier;

    KineticModule::clear_speed_all(fighter.module_accessor);

    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_DAMAGE,
        ENERGY_STOP_RESET_TYPE_DAMAGE_AIR,
        cliff_robbed_speed_x,
        cliff_robbed_speed_y,
        0.0,
        0.0,
        0.0
    );

    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);

    // </HDR>
    
    let cliff_release_disable_wall_jump_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("cliff_release_disable_wall_jump_frame"));
    WorkModule::set_int(fighter.module_accessor, cliff_release_disable_wall_jump_frame, *FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_WALL_JUMP_FRAME);

    // ShakeModule::req(
    //     fighter.module_accessor,
    //     Hash40::new("damage_air"),
    //     5,
    //     false,
    //     &Vector2f{x: 0.0, y: 1.0},
    //     1.0,
    //     0.0,
    //     false,
    //     false
    // );

    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_cliff_robbed_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_cliff_robbed_uniq as *const () as _));

    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_CliffRobbed_Main as *const () as _))
}

#[skyline::hook(replace = L2CFighterCommon_sub_cliff_robbed_uniq)]
unsafe extern "C" fn sub_cliff_robbed_uniq(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if !param_1.get_bool() {
        let cliff_robbed_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_CLIFF_ROBBED);
        let cliff_robbed_no_control_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("cliff_robbed_no_control_frame"));
        
        if cliff_robbed_frame == cliff_robbed_no_control_frame {
            // <HDR>

            // Enable drift/fastfall once past your cliff_robbed_no_control_frame
            sv_kinetic_energy!(
                reset_energy,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0
            );

            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);

            // </HDR>
        
            fighter.sub_air_check_fall_common_pre();
        }
    }
    else {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_CLIFF_ROBBED);
    }

    fighter.sub_fall_common_uniq(param_1);

    0.into()
}