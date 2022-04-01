use super::*;

#[status_script(agent = "elight", status = FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_HI_JUMP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn special_hi_jump_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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

#[status_script(agent = "elight", status = FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_HI_JUMP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_hi_jump_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_jump"), 0.0, 1.0, false, 0.0, false, false);
    let jump_stick = fighter.get_param_float("param_special_hi", "jump_stick");
    let stick = Vector2f::new(
        fighter.stick_x(),
        fighter.stick_y()        
    );

    if (stick.x.abs() + stick.y.abs()) >= jump_stick {
        let angle_from_vertical = app::sv_math::vec2_angle(stick.x, stick.y, 0.0, 1.0).to_degrees();
        let angle = angle_from_vertical * stick.x.signum() * -1.0;
        let back = fighter.get_param_float("param_special_hi", "jump_angle_limit_back");
        let front = fighter.get_param_float("param_special_hi", "jump_angle_limit_front");
        let angle = if PostureModule::lr(fighter.module_accessor) < 0.0 {
            angle.clamp(-front, back)
        } else {
            angle.clamp(-back, front)
        };

        let angle = angle.to_radians();

        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, angle);
        app::sv_kinetic_energy::set_angle(fighter.lua_state_agent);
    }

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
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    if !MotionModule::is_end(fighter.module_accessor) {
        return 0.into();
    }

    if fighter.is_button_on(Buttons::Special | Buttons::Attack) {
        fighter.on_flag(*FIGHTER_ELIGHT_STATUS_SPECIAL_HI_FLAG_SPREADBULLET);
    }

    if fighter.is_flag(*FIGHTER_ELIGHT_STATUS_SPECIAL_HI_FLAG_SPREADBULLET) {
        fighter.change_status(FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_HI_ATTACK2.into(), false.into());
    } else {
        fighter.change_status(FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_HI_ATTACK1.into(), false.into());
    }

    0.into()
}


#[status_script(agent = "elight", status = FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_HI_JUMP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn special_hi_jump_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[globals::STATUS_KIND] != FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_HI_ATTACK1 
    && fighter.global_table[globals::STATUS_KIND] != FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_HI_ATTACK2 {
        MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EFFECT, Hash40::new("effect_specialhiinterrupt"), -1);
        MotionAnimcmdModule::enable_skip_delay_update(fighter.module_accessor);
    }

    0.into()
}

#[status_script(agent = "elight", status = FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_HI_JUMP, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn special_hi_jump_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_ELIGHT_STATUS_SPECIAL_HI_INT_FRAME_FROM_START);
    0.into()
}

pub fn install() {
    smashline::install_status_scripts!(
        special_hi_jump_pre,
        special_hi_jump_main,
        special_hi_jump_end,
        special_hi_jump_exec,
    );
}