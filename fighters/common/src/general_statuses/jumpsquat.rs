// status imports
use super::*;
use globals::*;
// This file contains code for wavedashing out of jumpsquat, fullhop buffered aerials/attack canceling

pub fn install() {
    install_status_scripts!(
        //status_pre_JumpSquat,
        status_JumpSquat,
        status_end_JumpSquat,
        status_exec_JumpSquat
    );

    install_hooks!(
        //status_pre_JumpSquat_param,
        status_JumpSquat_Main,
        status_JumpSquat_common,
        uniq_process_JumpSquat_exec_status,
        uniq_process_JumpSquat_exec_status_param,
        sub_jump_squat_uniq_check_sub,
        sub_jump_squat_uniq_check_sub_mini_attack
    );
}
/***
// pre status stuff
#[common_status_script(status = FIGHTER_STATUS_KIND_JUMP_SQUAT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE,
    symbol = "_ZN7lua2cpp16L2CFighterCommon20status_pre_JumpSquatEv")]
unsafe fn status_pre_JumpSquat(fighter: &mut L2CFighterCommon) -> L2CValue {
    status_pre_JumpSquat_param(
        fighter,
        FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG.into(),
        FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT.into(),
        FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT.into(),
        FIGHTER_KINETIC_TYPE_MOTION.into(),
        L2CValue::I32(0)
    )
}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon26status_pre_JumpSquat_paramEN3lib8L2CValueES2_S2_S2_S2_")]
unsafe extern "C" fn status_pre_JumpSquat_param(fighter: &mut L2CFighterCommon, flag_keep: L2CValue, int_keep: L2CValue, float_keep: L2CValue, kinetic: L2CValue, arg: L2CValue) -> L2CValue {
    let flag_keep = flag_keep.into();
    let int_keep = int_keep.into();
    let float_keep = float_keep.into();
    let kinetic = kinetic.into();
    let arg = arg.into();
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_GROUND),
        kinetic,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        flag_keep,
        int_keep,
        float_keep,
        arg
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        true,
        *FIGHTER_TREADED_KIND_ENABLE,
        false,
        false,
        false,
        0,
        *FIGHTER_STATUS_ATTR_INTO_DOOR as u32,
        0,
        0
    );

    0.into()
} ***/

// main status stuff

#[common_status_script(status = FIGHTER_STATUS_KIND_JUMP_SQUAT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN,
    symbol = "_ZN7lua2cpp16L2CFighterCommon16status_JumpSquatEv")]
unsafe fn status_JumpSquat(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_JumpSquat_common(L2CValue::Bool(false));
    fighter.sub_shift_status_main(L2CValue::Ptr(status_JumpSquat_Main as *const () as _))
}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon21status_JumpSquat_MainEv")]
unsafe fn status_JumpSquat_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    //println!("main");
    // Check if a character (like greninja) has a custom subroutine for status checks
    let should_end = if fighter.global_table[CUSTOM_ROUTINE].get_bool() {
        let custom_routine: *const extern "C" fn(&mut L2CFighterCommon) -> L2CValue = fighter.global_table[CUSTOM_ROUTINE].get_ptr() as _;
        if !custom_routine.is_null() {
            let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(custom_routine);
            callable(fighter).get_bool()
        } else {
            false
        }
    } else { false };
    if should_end {
        return L2CValue::I32(1);
    }

    // begin testing for transitions out of jump squat
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if fighter.global_table[FIGHTER_KIND].get_i32() != *FIGHTER_KIND_PICKEL || ![*FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N1_JUMP_SQUAT, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_JUMP_SQUAT].contains(&StatusModule::prev_status_kind(fighter.module_accessor, 0)) {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH) {
            fighter.change_status(
                L2CValue::I32(*FIGHTER_STATUS_KIND_CATCH),
                L2CValue::Bool(true)
            );
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR) {
            fighter.change_status(
                L2CValue::I32(*FIGHTER_STATUS_KIND_ESCAPE_AIR), // We don't want to change to ESCAPE_AIR_SLIDE in case they do a nair dodge
                L2CValue::Bool(true)
            );
            return L2CValue::I32(0);
        }
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_JUMP_START) {
        fighter.change_status(
            L2CValue::I32(*FIGHTER_STATUS_KIND_JUMP),
            L2CValue::Bool(false)
        );
    } else if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL)
            && situation_kind == *SITUATION_KIND_AIR {
        fighter.change_status(
            L2CValue::I32(*FIGHTER_STATUS_KIND_FALL),
            L2CValue::Bool(false)
        );
    } else if !fighter.sub_transition_group_check_ground_item().get_bool() {
        let cat1 = fighter.global_table[CMD_CAT1].get_i32();
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI)
            && cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI != 0
            && situation_kind == *SITUATION_KIND_GROUND {
            fighter.change_status(
                L2CValue::I32(*FIGHTER_STATUS_KIND_SPECIAL_HI),
                L2CValue::Bool(true)
            );
        } else if !fighter.sub_transition_specialflag_hoist().get_bool() {
            let cat2 = fighter.global_table[CMD_CAT2].get_i32();
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START)
                && !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON)
                && cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_ATTACK_DASH_ATTACK_HI4 != 0
                && situation_kind == *SITUATION_KIND_GROUND {
                fighter.change_status(
                    L2CValue::I32(*FIGHTER_STATUS_KIND_ATTACK_HI4_START),
                    L2CValue::Bool(true)
                );
            }
        }
    }
    0.into()
}

// end status stuff
// no symbol since you can't call `fighter.status_end_JumpSquat()`, and replacing `bind_call_...` makes no sense here
#[common_status_script(status = FIGHTER_STATUS_KIND_JUMP_SQUAT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END,
    symbol = "_ZN7lua2cpp16L2CFighterCommon20status_end_JumpSquatEv" )]
unsafe fn status_end_JumpSquat(fighter: &mut L2CFighterCommon) -> L2CValue {
    //println!("end");
    BufferModule::disable_persist(fighter.battle_object);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_MINI_ATTACK);
    0.into()
}

// exec status stuff
#[common_status_script(status = FIGHTER_STATUS_KIND_JUMP_SQUAT, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn status_exec_JumpSquat(fighter: &mut L2CFighterCommon) -> L2CValue {
    uniq_process_JumpSquat_exec_status_param(fighter, L2CValue::Ptr(0 as _));
    0.into()
}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon34uniq_process_JumpSquat_exec_statusEv")]
unsafe fn uniq_process_JumpSquat_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    uniq_process_JumpSquat_exec_status_param(fighter, L2CValue::Ptr(0 as _));
    0.into()
}

// common jumpsquat subroutine -- to be called by each fighter before transitioning to a custom main status
#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon23status_JumpSquat_commonEN3lib8L2CValueE")]
unsafe fn status_JumpSquat_common(fighter: &mut L2CFighterCommon, lr_update: L2CValue) {
    let is_button_jump = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_STICK_JUMP_COMMAND_LIFE) == 0
                                || fighter.global_table[FLICK_Y_DIR].get_i32() <= 0;
    BufferModule::set_persist_lifetime(fighter.battle_object, 10);
    BufferModule::enable_persist(fighter.battle_object);
    if is_button_jump {
        //println!("button jump");
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_BUTTON);
        // check if we are doing double button shorthop input
        if ControlModule::is_jump_mini_button(fighter.module_accessor) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
        }
    }
    // I think this int might be referring to how many frames we check for tap jump?
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_STICK_JUMP_COMMAND_LIFE);
    ControlModule::reset_flick_y(fighter.module_accessor);
    ControlModule::reset_flick_sub_y(fighter.module_accessor);
    fighter.global_table[FLICK_Y].assign(&0xFE.into());

    // not a conditional enable, so it's not in potential_enables
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
    let potential_enables = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW
    ];
    for x in potential_enables.iter() {
        WorkModule::enable_transition_term(fighter.module_accessor, *x);
    }
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_JUMP_START);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ABNORMAL_MINIJUMP_SLOWWALK) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
    }
    // if we are doing a buffered aerial we want to disable all of the other transitions
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_MINI_ATTACK) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_RESERVE_ATTACK_BUTTON_ON);
        for x in potential_enables.iter() {
            WorkModule::unable_transition_term(fighter.module_accessor, *x);
        }
        MotionAnimcmdModule::enable_skip_delay_update(fighter.module_accessor);
    }
    // same as above, but without the skip stuff
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_RESERVE_JUMP_MINI_ATTACK) {
        for x in potential_enables.iter() {
            WorkModule::unable_transition_term(fighter.module_accessor, *x);
        }
    }

    let magnet_frame = ParamModule::get_int(fighter.battle_object, ParamType::Common, "air_escape_snap_frame");
    VarModule::set_int(fighter.battle_object, vars::common::AIR_ESCAPE_MAGNET_FRAME, magnet_frame);
    // println!("magnet_frame {}", magnet_frame);
    VarModule::off_flag(fighter.battle_object, vars::common::ENABLE_AIR_ESCAPE_MAGNET);
    
    VarModule::off_flag(fighter.battle_object, vars::common::ENABLE_AIR_ESCAPE_JUMPSQUAT);
    VarModule::off_flag(fighter.battle_object, vars::common::CSTICK_OVERRIDE);
    VarModule::off_flag(fighter.battle_object, vars::common::CSTICK_OVERRIDE_SECOND);
    VarModule::set_int(fighter.battle_object, vars::common::JUMP_SQUAT_FRAME, 0);
    VarModule::off_flag(fighter.battle_object, vars::common::IS_TAP_JUMP);
}

// The main exec block, for some reason it's not found in the exec status
#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon40uniq_process_JumpSquat_exec_status_paramEN3lib8L2CValueE")]
unsafe fn uniq_process_JumpSquat_exec_status_param(fighter: &mut L2CFighterCommon, arg: L2CValue) {
    let should_check = if fighter.global_table[CUSTOM_ROUTINE].get_bool() {
        let custom_routine: *const extern "C" fn(&mut L2CFighterCommon) -> L2CValue = fighter.global_table[CUSTOM_ROUTINE].get_ptr() as _;
        if !custom_routine.is_null() {
            let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(custom_routine);
            callable(fighter);
            true
        } else {
            true
        }
    } else { true };
    if should_check {
        fighter.sub_jump_squat_uniq_check_sub(L2CValue::I32(*FIGHTER_STATUS_JUMP_FLAG_BUTTON));
        fighter.sub_jump_squat_uniq_check_sub_mini_attack();
    }

    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);
    let update_rate = MotionModule::update_rate(fighter.module_accessor);
    let cat1 = fighter.global_table[CMD_CAT1].get_i32();
    if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE != 0 || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD_HOLD) {
        VarModule::on_flag(fighter.battle_object, vars::common::ENABLE_AIR_ESCAPE_JUMPSQUAT);
    }
    if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0
        && !WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH) {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH);
    }
    else {
        let end_frame = MotionModule::end_frame_from_hash(fighter.module_accessor, Hash40::new_raw(motion_kind)) as u32 as f32;
        if end_frame <= (frame + update_rate) {
            StatusModule::set_situation_kind(fighter.module_accessor, app::SituationKind(*SITUATION_KIND_AIR), false);
            let situation_kind = fighter.global_table[SITUATION_KIND].clone();
            fighter.global_table[PREV_SITUATION_KIND].assign(&situation_kind);
            if VarModule::is_flag(fighter.battle_object, vars::common::ENABLE_AIR_ESCAPE_JUMPSQUAT) {
                // check if we are doing directional airdodge
                let stick = app::sv_math::vec2_length(fighter.global_table[STICK_X].get_f32(), fighter.global_table[STICK_Y].get_f32());
                if stick >= WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("escape_air_slide_stick")) {
                    VarModule::on_flag(fighter.battle_object, vars::common::PERFECT_WAVEDASH);
                    // change kinetic/ground properties for wavedash
                    GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_NONE));
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_ESCAPE);
                    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);
                } else {
                    VarModule::off_flag(fighter.battle_object, vars::common::PERFECT_WAVEDASH);
                    // change kinetic properties for rising nairdodge
                    GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_JUMP);
                }
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);
            } else {
                // change kinetic/ground properties for jump
                VarModule::off_flag(fighter.battle_object, vars::common::PERFECT_WAVEDASH);
                GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FROM_SQUAT, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_JUMP_FROM);
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_JUMP);
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_JUMP_START);
            }
        }
        else {
            //println!("js_vel: {}", KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN));
            VarModule::set_float(fighter.battle_object, vars::common::JUMPSQUAT_VELOCITY, KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN));
            VarModule::set_float(fighter.battle_object, vars::common::CURRENT_MOMENTUM, KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN));
            VarModule::set_float(fighter.battle_object, vars::common::CURRENT_MOMENTUM_SPECIALS, KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN));
        }
    }
}

// subroutine for checking for aerial macro
#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon29sub_jump_squat_uniq_check_subEN3lib8L2CValueE")]
unsafe fn sub_jump_squat_uniq_check_sub(fighter: &mut L2CFighterCommon, flag: L2CValue) {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let js_frame = VarModule::get_int(fighter.battle_object, vars::common::JUMP_SQUAT_FRAME);
    VarModule::set_int(fighter.battle_object, vars::common::JUMP_SQUAT_FRAME, js_frame + 1);

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_MINI_JUMP) { return; }
    // flag is basically always going to be the jump button flag
    // checks if we are pressing a jump **button**
    if WorkModule::is_flag(fighter.module_accessor, flag.get_i32()){
        let frame = VarModule::get_int(fighter.battle_object, vars::common::JUMP_SQUAT_FRAME);
        let end_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("jump_squat_frame"), 0);
        //println!("button jump 2");
        // checks if we have released or if we are pressing two jump buttons, if so, reserve shorthop
        if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP)
            || ControlModule::is_jump_mini_button(fighter.module_accessor) {
                //println!("button shorthop");
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
        }
        // prevents cstick drift if inputting cstick on last 2 frames of jumpsquat
        if frame >= (end_frame - 1) && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) {
            if VarModule::is_flag(fighter.battle_object, vars::common::CSTICK_OVERRIDE) {
                //println!("2nd frame override");
                VarModule::on_flag(fighter.battle_object, vars::common::CSTICK_OVERRIDE_SECOND);
                ControlModule::reset_main_stick_x(fighter.module_accessor);
            }
            if frame >= (end_frame - 1) && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) {
                //println!("override");
                VarModule::on_flag(fighter.battle_object, vars::common::CSTICK_OVERRIDE);
                ControlModule::reset_main_stick_x(fighter.module_accessor);
            }
        }
    } else {
        // if we are here, it means that we are using tap jump
        VarModule::on_flag(fighter.battle_object, vars::common::IS_TAP_JUMP);
        /*if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) {
            println!("CSTICK TRIGGER");
        }*/
        let stick_y_first = ControlModule::get_stick_y(fighter.module_accessor); 
        if stick_y_first == 0.0 {
            ControlModule::set_main_stick_y(fighter.module_accessor, 0.0001);
        }
        let frame = VarModule::get_int(fighter.battle_object, vars::common::JUMP_SQUAT_FRAME);
        let end_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("jump_squat_frame"), 0);

        // detect when left stick position is overridden by cstick (fuck you ult devs)
        if (ControlModule::get_attack_air_stick_y(fighter.module_accessor) - stick_y_first).abs() < 0.03 && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) {
            if VarModule::is_flag(fighter.battle_object, vars::common::CSTICK_OVERRIDE) {
                //println!("2nd frame override");
                VarModule::on_flag(fighter.battle_object, vars::common::CSTICK_OVERRIDE_SECOND);
                ControlModule::reset_main_stick_x(fighter.module_accessor);
                ControlModule::set_main_stick_y(fighter.module_accessor, 0.0);
            }
            if frame >= (end_frame - 1) && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) {
                //println!("override");
                VarModule::on_flag(fighter.battle_object, vars::common::CSTICK_OVERRIDE);
                ControlModule::reset_main_stick_x(fighter.module_accessor);
                ControlModule::set_main_stick_y(fighter.module_accessor, 0.0);
            }
        }

        //println!("get_stick_prev_y: {}", ControlModule::get_stick_prev_y(fighter.module_accessor));
        //println!("get_attack_air_stick_y: {}", ControlModule::get_attack_air_stick_y(fighter.module_accessor));
        //println!("get_stick_y: {}", ControlModule::get_stick_y(fighter.module_accessor));
        //println!(".");

        // compare the value of the left stick with the threshold for stick jumping
        let prev_stick_y = ControlModule::get_stick_prev_y(fighter.module_accessor);
        let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
        if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) {
            if stick_y != 0.0 && stick_y < WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("jump_neutral_y")) {
                if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
                }
            }
            else {
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
            }
        }
        else {
            if prev_stick_y != 0.0 && prev_stick_y < WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("jump_neutral_y")) {
                // used to buffer specials and make sure that we aren't detecting when c stick is off
                if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
                    if frame == end_frame && !VarModule::is_flag(fighter.battle_object, vars::common::CSTICK_OVERRIDE) {
                        if stick_y > WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("jump_neutral_y")) {
                            //println!("edge case fullhop");
                            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
                        }
                    }
                }
            }
            else {
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
            }
        }
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON)
            && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
            && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            // uncomment for short hop aerial only
            // WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
        }
    }
}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon41sub_jump_squat_uniq_check_sub_mini_attackEv")]
unsafe fn sub_jump_squat_uniq_check_sub_mini_attack(fighter: &mut L2CFighterCommon) {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_MINI_JUMP) { return; }
    let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_MINI_ATTACK) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_RESERVE_ATTACK_BUTTON_ON)
            && current_frame > 0.0 {
            FighterControlModuleImpl::reserve_on_attack_button(fighter.module_accessor);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_RESERVE_ATTACK_BUTTON_ON);
        }
    } else {
        if !ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_RESERVE_JUMP_MINI_ATTACK) {
                return;
            }
        } else if !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_RESERVE_JUMP_MINI_ATTACK) {
            return;
        }
        let unables = [
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW
        ];
        for x in unables.iter() {
            WorkModule::unable_transition_term(fighter.module_accessor, *x);
        }
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_MINI_ATTACK);
    }
}