use super::*;
use globals::*;
use smashline::*;


pub fn install() {
    install_status_scripts!(
        pre_special_s, main_special_s
    );
}

#[status_script(agent = "sonic", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn pre_special_s(fighter: &mut L2CFighterCommon) -> L2CValue {
	StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S |
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
            *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON
        ) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}

#[status_script(agent = "sonic", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn main_special_s(fighter: &mut L2CFighterCommon) -> L2CValue {
    // fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_s"), L2CValue::Hash40s("special_air_s"), false.into());
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("jump_squat"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    if fighter.global_table[globals::SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    fighter.main_shift(special_s_main_loop)
}

unsafe extern "C" fn special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Lets you grabl edge once it's enabled
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 0.into();
    }

    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }

    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && fighter.global_table[CURRENT_FRAME].get_f32() >= 5.0
    && !StatusModule::is_changing(fighter.module_accessor) {
        fighter.check_jump_cancel(false);
    }
    // temporary
    // if MotionModule::motion_kind(fighter.module_accessor) == hash40("dash") {
    //     if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
    //         fighter.clear_lua_stack();
    //         lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
    //         let speed_y = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
    //         if speed_y < 0.2 {
    //             sv_kinetic_energy!(
    //                 set_brake,
    //                 fighter,
    //                 FIGHTER_KINETIC_ENERGY_ID_STOP,
    //                 0.0,
    //                 0.0
    //             );
    //         }
    //     }
    //     else {
    //         sv_kinetic_energy!(
    //             set_brake,
    //             fighter,
    //             FIGHTER_KINETIC_ENERGY_ID_STOP,
    //             0.08,
    //             0.0
    //         );
    //     }
    // }
    if MotionModule::is_end(fighter.module_accessor) {
        // temporary
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("jump_squat") {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new("dash"),
                0.0,
                0.75,
                false,
                0.0,
                false,
                false
            );
            let lr = PostureModule::lr(fighter.module_accessor);
            let brake;
            let y_speed = if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
                brake = 0.05;
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                    1.1
                }
                else {
                    -0.1
                }
            }
            else {
                brake = 0.08;
                0.0
            };
            sv_kinetic_energy!(
                set_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_STOP,
                4.0 * lr,
                0.0
            );
            sv_kinetic_energy!(
                set_brake,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_STOP,
                brake,
                0.0
            );
            sv_kinetic_energy!(
                set_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                y_speed
            );
            // let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
            // sv_kinetic_energy!(
            //     set_accel,
            //     fighter,
            //     FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            //     -air_accel_y * 0.5
            // );
            let air_speed_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
            sv_kinetic_energy!(
                set_stable_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                air_speed_y_stable * 0.7
            );
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
        else {
            if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            }
        }
    }

    0.into()
}
