use super::*;

unsafe extern "C" fn special_hi_jump_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE,
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

unsafe extern "C" fn special_hi_jump_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_jump"), 0.0, 1.0, false, 0.0, false, false);
    VarModule::on_flag(fighter.battle_object, vars::elight::instance::DISABLE_SPECIAL_HI);
    
    

    // [v] get the current position of the stick to be used for angle calculations
    let stick = Vector2f::new(
        fighter.stick_x(),
        fighter.stick_y()        
    );

    let jump_stick = fighter.get_param_float("param_special_hi", "jump_stick");

    // [v]
    //      - in vanilla, this block is basically moot since the angle limits are both 0
    //          if you put different angle limits in, you'll actually be able to angle the momentum.
    //      - it probably adds the absolute values here so that you can only slightly roll the
    //          analog stick to the left/right while still holding straight up to get a little
    //          angle change instead of a big one
    if (stick.x.abs() + stick.y.abs()) >= jump_stick {
        // [v] this function calculates the absolute value of the angle between the first vector pair and the second vector pair
        //      in this case, it is calculating the angle to move from the vertical to the stick position
        let angle_from_vertical = app::sv_math::vec2_angle(stick.x, stick.y, 0.0, 1.0).to_degrees();

        // [v] this makes the angle to the right going clockwise (negative angle) and the angle to the left counter clockwise (positive angle)
        //      it also gets the back and front limits
        let angle = angle_from_vertical * stick.x.signum() * -1.0;
        let back = 5.0; // fighter.get_param_float("param_special_hi", "jump_angle_limit_back");
        let front = 20.0; // fighter.get_param_float("param_special_hi", "jump_angle_limit_front");

        // [v] reverse the params depending on LR
        let angle = if PostureModule::lr(fighter.module_accessor) < 0.0 {
            angle.clamp(-back, front)
        } else {
            angle.clamp(-front, back)
        };

        // [v] convert the angle to radians and apply the new angle
        let angle = angle.to_radians();

        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, angle);
        app::sv_kinetic_energy::set_angle(fighter.lua_state_agent);
    }

    // [v] change the vertical speed depending on if the move was started on the ground
    let speed_mul = if fighter.is_flag(*FIGHTER_ELIGHT_STATUS_SPECIAL_HI_FLAG_GROUND_START) {
        fighter.get_param_float("param_special_hi", "jump_speed_mul_ground")
    } else {
        fighter.get_param_float("param_special_hi", "jump_speed_mul_air")
    };

    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, speed_mul);
    app::sv_kinetic_energy::set_speed_mul(fighter.lua_state_agent);

    fighter.main_shift(special_hi_jump_main_loop)
}

unsafe extern "C" fn special_hi_jump_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    // [v] check if you grabbed a ledge
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    // [v] exit early if the animation is not over, since the end of this function
    //      forces a status change one way or the other
    // [h] instead of checking MotionModule::is_end we check for if we are beyond the input frame
    if fighter.motion_frame() < 15.0 {
        return 0.into();
    }

    // [v] check if you are pressing the buttons for spreadbullet
    if fighter.is_button_on(Buttons::Special | Buttons::Attack) {
        fighter.on_flag(*FIGHTER_ELIGHT_STATUS_SPECIAL_HI_FLAG_SPREADBULLET);
    }

    if fighter.is_button_on(Buttons::Special) {
        VarModule::set_int(fighter.battle_object, vars::elight::status::SPECIAL_HI_JUMP_RESERVE_ACTION, vars::elight::SPECIAL_HI_JUMP_RESERVE_ACTION_ATTACK1);
    } else if fighter.is_button_on(Buttons::Attack) {
        VarModule::set_int(fighter.battle_object, vars::elight::status::SPECIAL_HI_JUMP_RESERVE_ACTION, vars::elight::SPECIAL_HI_JUMP_RESERVE_ACTION_ATTACK2);
    }

    // [v] if we are using spreadbullet then switch to Attack2 and not Attack1
    // [h] instead of using the spreadbullet flag, we use a custom VarModule int to
    //      determine what kind of action we are going into here. it is initialized
    //      in the SpecialHi script and then changed depending on the inputs
    match VarModule::get_int(fighter.battle_object, vars::elight::status::SPECIAL_HI_JUMP_RESERVE_ACTION) {
        vars::elight::SPECIAL_HI_JUMP_RESERVE_ACTION_ATTACK1 => fighter.change_status(FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_HI_ATTACK1.into(), false.into()),
        vars::elight::SPECIAL_HI_JUMP_RESERVE_ACTION_ATTACK2 => fighter.change_status(FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_HI_ATTACK2.into(), false.into()),
        vars::elight::SPECIAL_HI_JUMP_RESERVE_ACTION_FALL    => fighter.change_status(statuses::elight::SPECIAL_HI_FINISH2.into(), false.into()),
        _ => {} // undefined behavior
    }

    0.into()
}

unsafe extern "C" fn special_hi_jump_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    // [v] the only natural conclusion of this move is that you do spreadbullet or ray of punishment
    //      so if you are doing neither of those, then you were interrupted
    if fighter.global_table[globals::STATUS_KIND] != FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_HI_ATTACK1 
    && fighter.global_table[globals::STATUS_KIND] != FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_HI_ATTACK2 {
        MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EFFECT, Hash40::new("effect_specialhiinterrupt"), -1);
        MotionAnimcmdModule::enable_skip_delay_update(fighter.module_accessor);
    }

    if fighter.global_table[globals::STATUS_KIND].get_i32() != statuses::elight::SPECIAL_HI_FINISH2 {
        VarModule::on_flag(fighter.battle_object, vars::elight::instance::UP_SPECIAL_FREEFALL);
    }
    
    //Disable up special
    VarModule::on_flag(fighter.battle_object, vars::elight::instance::DISABLE_SPECIAL_HI);
    super::Set_Pyra_Up_Special_Cancel(fighter, true);

    0.into()
}

unsafe extern "C" fn special_hi_jump_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    // [v] increment this flag, just like regular special hi for spreadbullet check
    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_ELIGHT_STATUS_SPECIAL_HI_INT_FRAME_FROM_START);
    0.into()
}

pub fn install() {
    smashline::Agent::new("elight")
        .status(Pre, *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_HI_JUMP, special_hi_jump_pre)
        .status(Main, *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_HI_JUMP, special_hi_jump_main)
        .status(End, *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_HI_JUMP, special_hi_jump_end)
        .status(Exec, *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_HI_JUMP, special_hi_jump_exec)
        .install();
}