use super::*;

// unsafe extern "C" fn special_s3_init(fighter: &mut L2CFighterCommon) -> L2CValue {
//     let customize_to = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_WAZA_CUSTOMIZE_TO);
//     if customize_to != *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_S_3 {
//         return original_status(Init, fighter, *FIGHTER_STATUS_KIND_SPECIAL_S)(fighter);
//     }
//     let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
//     let sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
//     KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
//     KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
//     KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
//     KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);

//     sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_AIR_BRAKE, 0.0, 0.0, 0.0, 0.0);
//     fighter.clear_lua_stack();
//     lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
//     let brake_y = sv_kinetic_energy::get_brake_x(fighter.lua_state_agent);
//     let accel_x = fighter.get_param_float("param_special_s", "s3_acc_x");
//     sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, accel_x, brake_y);
//     KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);

//     sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0);
//     KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);

//     return 0.into();
// }

// unsafe extern "C" fn special_s3_1_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
//     if fighter.is_situation(*SITUATION_KIND_GROUND) {
//         fighter.change_status(FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_2_GROUND.into(), true.into());
//     }
//     else {
//         fighter.change_status(FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_2_AIR.into(), true.into());
//         KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
//     }

//     return 1.into();
// }

unsafe extern "C" fn special_s3_2_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON |
            *FIGHTER_LOG_MASK_FLAG_SHOOT | *FIGHTER_LOG_MASK_FLAG_HAJIKI) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );

    return 0.into();
}

// unsafe extern "C" fn special_s3_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
//     PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
//     PostureModule::update_rot_y_lr(fighter.module_accessor);
//     if fighter.is_flag(*FIGHTER_MIIGUNNER_STATUS_MIIMISSILE_FLAG_MOT_RESTART) {
//         fighter.off_flag(*FIGHTER_MIIGUNNER_STATUS_MIIMISSILE_FLAG_MOT_RESTART);
//         MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s3_2"), 0.0, 1.0, false, 0.0, false, false);
//     }
//     else {
//         MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s3_2"), -1.0, 1.0, 0.0, false, false);
//     }
//     GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
//     notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, (*FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_06) + -1);
//     notify_event_msc_cmd!(fighter, Hash40::new_raw(0x3a40337e2c), (*FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_06) + -1);

//     fighter.main_shift(special_s3_air_main_loop)
// }

// unsafe extern "C" fn special_s3_air_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
//     if CancelModule::is_enable_cancel(fighter.module_accessor) {
//         if fighter.sub_wait_ground_check_common(false.into()).get_bool()
//         || fighter.sub_air_check_fall_common().get_bool() {
//             return 1.into();
//         }
//     }
//     fighter.sub_air_check_dive();
//     if StatusModule::is_situation_changed(fighter.module_accessor) {
//         if fighter.is_situation(*SITUATION_KIND_GROUND) {
//             if fighter.status_frame() > 23 {
//                 // land cancel
//                 MotionModule::set_frame(fighter.module_accessor, 40.0, false);
//             }
//             else {
//                 MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s3_2"), -1.0, 1.0, 0.0, false, false);
//             }
//             KineticModule::clear_speed_all(fighter.module_accessor);
//             GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
//             KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
//             sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, ENERGY_MOTION_RESET_TYPE_GROUND_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
//             KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
//         }
//         else {
//             MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s3_2"), -1.0, 1.0, 0.0, false, false);
//             KineticModule::clear_speed_all(fighter.module_accessor);
//             GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
//             KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
//             sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, ENERGY_MOTION_RESET_TYPE_AIR_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
//             KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
//         }
//         special_s3_check_missile(fighter);
//     }
//     if MotionModule::is_end(fighter.module_accessor) {
//         fighter.change_status_by_situation(*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_FALL, false);
//         return 1.into();
//     }

//     return 0.into();
// }

// unsafe extern "C" fn special_s3_check_missile(fighter: &mut L2CFighterCommon) {
//     if fighter.global_table[IS_STOPPING].get_bool()
//     || StatusModule::status_kind_interrupt(fighter.module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_S as u64 {
//         return;
//     }
//     if fighter.is_flag(*FIGHTER_MIIGUNNER_STATUS_MIIMISSILE_FLAG_WEAPON) {
//         if !fighter.is_flag(*FIGHTER_MIIGUNNER_STATUS_MIIMISSILE_FLAG_WEAPON_GENERATED)
//         && (!StatusModule::status_kind_interrupt(fighter.module_accessor) == *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_2_GROUND as u64
//         || StatusModule::status_kind_interrupt(fighter.module_accessor) == *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_2_AIR as u64) {
//             ArticleModule::generate_article_enable(fighter.module_accessor, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_SUPERMISSILE, false, -1);
//             ArticleModule::shoot_exist(fighter.module_accessor, *FIGHTER_MIIGUNNER_GENERATE_ARTICLE_SUPERMISSILE, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
//             fighter.on_flag(*FIGHTER_MIIGUNNER_STATUS_MIIMISSILE_FLAG_WEAPON_GENERATED);
//         }
//     }

//     return;
// }

pub fn install(agent: &mut Agent) {
    //agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s3_init);

    //agent.status(Pre, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_1_GROUND, special_s3_1_pre);
    //agent.status(Init, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_1_GROUND, special_s3_1_air_init);

    //agent.status(Pre, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_1_AIR, special_s3_1_pre);
    //agent.status(Init, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_1_AIR, special_s3_1_air_init);

    agent.status(Pre, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_2_GROUND, special_s3_2_pre);
    
    // agent.status(Init, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_2_AIR, special_s3_1_air_init);
    // agent.status(Pre, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_2_AIR, special_s3_2_pre);
    // agent.status(Main, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_2_AIR, special_s3_air_main);
}