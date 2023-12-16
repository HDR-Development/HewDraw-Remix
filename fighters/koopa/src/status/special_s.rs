use super::*;
use globals::*;

unsafe extern "C" fn specials_situation_helper(fighter: &mut L2CFighterCommon, is_start: bool) {
    let motion_g;
    let motion_a;
    let throw_input = WorkModule::get_float(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_S_WORK_FLOAT_START_Y);
    let is_Ground = StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND;

    if throw_input.abs() < 0.1 {
        motion_g = Hash40::new("special_s_squat");
        motion_a = Hash40::new("special_air_s_squat");
    }
    else if throw_input > 0.0 {
        motion_g = Hash40::new("special_s_throwf");
        motion_a = Hash40::new("special_air_s_throwf");
    }
    else {
        motion_g = Hash40::new("special_s_throwb");
        motion_a = Hash40::new("special_air_s_throwb");
    }
    fighter.sub_change_motion_by_situation(motion_g.into(), motion_a.into(), (!is_start).into());

    let correct = if is_Ground {*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK} else {*GROUND_CORRECT_KIND_AIR};
    GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(correct));

    fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(),FIGHTER_KINETIC_TYPE_AIR_STOP.into());

    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    if is_start {
        let mut speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let mut speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let speed_x_mul = if throw_input == 0.0 {1.0} else {1.0};
        let speed_y_mul = if throw_input == 0.0 {0.5} else {1.0};
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            ENERGY_STOP_RESET_TYPE_AIR,
            speed_x*speed_x_mul,
            0.0,
            0.0,
            0.0,
            0.0
        );
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            ENERGY_GRAVITY_RESET_TYPE_GRAVITY,
            0.0,
            speed_y*speed_y_mul,
            0.0,
            0.0,
            0.0
        );
    }

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_S_FLAG_CAPTURE) 
    || true {
        sv_kinetic_energy!(
            set_brake,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            0.1,
            0.0
        );
        let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
        let air_accel_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y_stable"), 0);
        sv_kinetic_energy!(
            set_accel,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            -air_accel_y*0.5
        ); 
        sv_kinetic_energy!(
            set_limit_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            air_accel_y_stable*0.01
        );
    }
    
}

unsafe extern "C" fn specials_kinetic_exec(fighter: &mut L2CFighterCommon) {
    let is_Ground = StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND;
    if is_Ground {
        if !KineticModule::is_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) {
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY); 
        }
        if KineticModule::is_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL) {
            sv_kinetic_energy!(
                clear_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL
            );
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL); 
        }
    }
    //Falling
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_S_FLAG_CAPTURE) {
        let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
        let air_accel_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y_stable"), 0);
        let air_accel_x = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_mul"), 0);
        let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);

        let mut speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        if !KineticModule::is_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) {
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            sv_kinetic_energy!(
                reset_energy,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                ENERGY_GRAVITY_RESET_TYPE_GRAVITY,
                0.0,
                speed_y,
                0.0,
                0.0,
                0.0
            );
            sv_kinetic_energy!(
                set_accel,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                -air_accel_y*0.425
            );
            sv_kinetic_energy!(
                set_limit_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                air_accel_y_stable
            );
        }
        if !KineticModule::is_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL) {
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL); 
            sv_kinetic_energy!(
                reset_energy,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST,
                0.0, 0.0, 0.0, 0.0, 0.0
            );
            sv_kinetic_energy!(
                set_accel_x_mul,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                air_accel_x*0.5
            );
            sv_kinetic_energy!(
                set_limit_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                air_speed_x_stable * 0.75,
                0.0
            );
        }
    }
    //Throwing
    else {
        sv_kinetic_energy!(
            clear_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL
        );

        let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let air_accel_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y_stable"), 0);
        if speed_y <= 0.0 {
            sv_kinetic_energy!(
                set_accel,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                0.0
            );
            sv_kinetic_energy!(
                set_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                0.0
            );
        }
    }
    
}

#[status_script(agent = "koopa", status = FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_SQUAT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn specials_squat_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_float(fighter.module_accessor, 0.0,*FIGHTER_KOOPA_STATUS_SPECIAL_S_WORK_FLOAT_START_Y);
    specials_situation_helper(fighter,true);
    fighter.sub_shift_status_main(L2CValue::Ptr(specials_squat_main_loop as *const () as _))
}

unsafe extern "C" fn specials_squat_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_LANDING.into(),false.into());
        return 0.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        specials_situation_helper(fighter,false);
    }

    0.into()
}
#[status_script(agent = "koopa", status = FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_SQUAT, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn specials_squat_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    specials_kinetic_exec(fighter);
    0.into()
}


unsafe extern "C" fn specials_ejected(fighter: &mut L2CFighterCommon) {
    let no_change = fighter.global_table[0xB] == FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_FALL;
    WorkModule::set_flag(fighter.module_accessor, no_change,*FIGHTER_KOOPA_INSTANCE_WORK_ID_FLAG_THROW_NO_CHANGE);
    if fighter.global_table[0xB] != FIGHTER_STATUS_KIND_DEAD {
        CatchModule::set_send_cut_event(fighter.module_accessor, true);
        CatchModule::catch_cut(fighter.module_accessor, false,false);
    }
    else {
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_KOOPA_INSTANCE_WORK_ID_FLAG_THROW_NO_CHANGE);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x31dbed6513), Hash40::new_raw(0xbefb89abe),Hash40::new_raw(0x7fb997a80));
    }
    WorkModule::off_flag(fighter.module_accessor,*FIGHTER_KOOPA_STATUS_SPECIAL_S_FLAG_CAPTURE);
}


#[status_script(agent = "koopa", status = FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_SQUAT, condition = LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS)]
unsafe fn specials_squat_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[0xB] != FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_JUMP
    && fighter.global_table[0xB] != FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_LANDING {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_S_FLAG_CAPTURE) {
            specials_ejected(fighter);
        }
    }

    0.into()
}

#[status_script(agent = "koopa", status = FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_LANDING, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn specials_landing_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    //Unable energies

    if LinkModule::is_linked(fighter.module_accessor, *LINK_NO_CAPTURE){
        //notify_event_msc_cmd!(fighter, Hash40::new_raw(0x329eb012b6), Hash40::new_rw(0xbefb89abe),Hash40::new_raw(0xbefb89abe));
    }
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_S_FLAG_HIT);
    0.into()
}

#[status_script(agent = "koopa", status = FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_LANDING, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn specials_landing_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let kinetic = KineticModule::get_kinetic_type(fighter.module_accessor);
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        kinetic,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_DISABLE,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        0,
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S) as u32,
        0
    );
    0.into()
}

#[status_script(agent = "koopa", status = FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_LANDING, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn specials_landing_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let throw_F = PostureModule::lr(fighter.module_accessor).signum() == ControlModule::get_stick_x(fighter.module_accessor).signum()
    || ControlModule::get_stick_x(fighter.module_accessor).abs() < 0.2;
    let throw_Input = if throw_F {1.0} else {-1.0};
    let capture_id = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE);
    if capture_id != 0x50000000 {
        let capture_boma = sv_battle_object::module_accessor(capture_id as u32);

        let motion_share = WorkModule::get_param_int(fighter.module_accessor, 0xad2ee25eu64, 0x7d88ea0u64);
        let throw_motion = if throw_F {36603360558 as u64} else {36554879287 as u64}; //39642420386 lw 41418534085 hi 36603360558 f 36554879287 b
        let throw_rate = if throw_F {1.7} else {1.0};
        
        let mut share_type = 0;
        if motion_share == *FIGHTER_MOTION_SHARE_TYPE_TARO {
            share_type = *BODY_TYPE_MOTION_DX;
        }
        else if motion_share == *FIGHTER_MOTION_SHARE_TYPE_GIRL {
            share_type = *BODY_TYPE_MOTION_GIRL;
        }
        else if motion_share == *FIGHTER_MOTION_SHARE_TYPE_BIG {
            share_type = *BODY_TYPE_MOTION_BIG;
        }
        if share_type > 0 {
            FighterMotionModuleImpl::add_body_type_hash(
                capture_boma,
                Hash40::new_raw(throw_motion),
                *BODY_TYPE_MOTION_DX
            );
        }
        MotionModule::change_motion(capture_boma,Hash40::new_raw(throw_motion),0.0,throw_rate,false,0.0,false,false);
        
    }

    WorkModule::set_float(fighter.module_accessor, throw_Input,*FIGHTER_KOOPA_STATUS_SPECIAL_S_WORK_FLOAT_START_Y);
    specials_situation_helper(fighter,true);

    fighter.sub_shift_status_main(L2CValue::Ptr(specials_landing_main_loop as *const () as _))
}

unsafe extern "C" fn specials_landing_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        let new_status = if fighter.is_situation(*SITUATION_KIND_GROUND) {FIGHTER_STATUS_KIND_WAIT} else {FIGHTER_STATUS_KIND_FALL};
        fighter.change_status(new_status.into(), false.into());
        return 0.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }

    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        specials_situation_helper(fighter,false);
    }

    0.into()
}

#[status_script(agent = "koopa", status = FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_LANDING, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn specials_landing_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    specials_kinetic_exec(fighter);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_S_FLAG_HIT) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_S_FLAG_CAPTURE) {
            let capture_id = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE);
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x31dbed6513), Hash40::new("throw"),Hash40::new_raw(0x7fb997a80));
            if capture_id != 0x50000000 {
                AttackModule::hit_absolute_joint(fighter.module_accessor, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW,capture_id as u32,Hash40::new("throw"), 0, 0);
            }
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_S_FLAG_HIT);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_S_FLAG_CAPTURE);
            
            //unable energies, then do hop energy
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY); 
            specials_kinetic_exec(fighter);
        }
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        specials_squat_main,
        specials_squat_exec,
        specials_squat_exit,
        specials_landing_init,
        specials_landing_pre,
        specials_landing_main,
        specials_landing_exec,
    );
}