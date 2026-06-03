use super::*;

// FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_HI_JUMP

unsafe extern "C" fn special_hi_jump_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_jump"), 0.0, 1.0, false, 0.0, false, false);

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

    special_hi::special_hi_common_check_spreadbullet(fighter);

    if MotionModule::is_end(fighter.module_accessor) {
        let status = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ELIGHT_STATUS_SPECIAL_HI_FLAG_SPREADBULLET) {
            VarModule::on_flag(fighter.battle_object, vars::elight::instance::SPECIAL_HI_ENABLE_FREEFALL);
            FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_HI_ATTACK2
        }
        else {
            FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_HI_ATTACK1
        };
        fighter.change_status(status.into(), false.into());
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

    //Disable up special
    VarModule::on_flag(fighter.battle_object, vars::elight::instance::DISABLE_SPECIAL_HI);
    if VarModule::is_flag(fighter.battle_object, vars::elight::instance::SPECIAL_HI_ENABLE_FREEFALL) {
        VarModule::on_flag(fighter.battle_object, vars::elight::status::SPECIAL_HI_FREEFALL);
    }
    VarModule::on_flag(fighter.battle_object, vars::elight::instance::SPECIAL_HI_ENABLE_FREEFALL);
    VarModule::on_flag(fighter.battle_object, vars::common::instance::UP_SPECIAL_LAG);
    super::Set_Pyra_Up_Special_Cancel(fighter, true);

    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_HI_JUMP, special_hi_jump_main);
    agent.status(End, *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_HI_JUMP, special_hi_jump_end);
}