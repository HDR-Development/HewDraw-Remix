use super::*;

pub unsafe fn special_hi_common_check_spreadbullet(fighter: &mut L2CFighterCommon) {
    // [v] this is likely a leniency check so that if you accidentally double tap
    //      the special button you aren't forced into spreadbullet. it should be
    //      noted how here it requires the `trigger` instead of holding it, so if you
    //      hold the button it also doesn't trigger until right before the move would come out
    let frame = fighter.get_int(*FIGHTER_ELIGHT_STATUS_SPECIAL_HI_INT_FRAME_FROM_START);

    if frame <= 0 {
        return;
    }

    if fighter.get_param_int("param_special_hi", "attack_input_frame") <= frame {
        return;
    }

    // [h] instead of checking for either and setting a flag, we will reserve different actions
    //      depending on the button
    if fighter.is_button_trigger(Buttons::Special) {
        VarModule::set_int(fighter.battle_object, vars::elight::status::SPECIAL_HI_JUMP_RESERVE_ACTION, vars::elight::SPECIAL_HI_JUMP_RESERVE_ACTION_ATTACK1);
    } else if fighter.is_button_trigger(Buttons::Attack) {
        VarModule::set_int(fighter.battle_object, vars::elight::status::SPECIAL_HI_JUMP_RESERVE_ACTION, vars::elight::SPECIAL_HI_JUMP_RESERVE_ACTION_ATTACK2);
    }
}

#[status_script(agent = "elight", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn special_hi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_MOTION_CLIFF_MOVE,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_AIR_LASSO | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );

    0.into()
}

#[status_script(agent = "elight", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // [v] fill out the kinetic energy from parameter data and also change the motion kind
    //      depending on whether you are grounded or not.
    fighter.sub_set_special_start_common_kinetic_setting(L2CValue::Hash40s("param_special_hi"));
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_hi_start"), L2CValue::Hash40s("special_air_hi_start"), false.into());

    // [v] set the flag that says you are grounded, which impacts other parts of the move later
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ELIGHT_STATUS_SPECIAL_HI_FLAG_GROUND_START);
    }

    // [h] Initialize our reserved action to the default (fall)
    VarModule::set_int(fighter.battle_object, vars::elight::status::SPECIAL_HI_JUMP_RESERVE_ACTION, vars::elight::SPECIAL_HI_JUMP_RESERVE_ACTION_FALL);

    fighter.main_shift(special_hi_main_loop)
}

unsafe extern "C" fn special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    // [v] update the kinetic energy from parameter data and once change the motion kind
    //      if your situation changes
    fighter.sub_exec_special_start_common_kinetic_setting(L2CValue::Hash40s("param_special_hi"));
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_hi_start"), L2CValue::Hash40s("special_air_hi_start"), true.into());

    // [v] check if you are doing the input for spreadbullet, and if the animation is over (this is the initial swipe)
    //      transition into the jump
    special_hi_common_check_spreadbullet(fighter);
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_HI_JUMP.into(), false.into());
    }

    0.into()
}

#[status_script(agent = "elight", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn special_hi_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    // [v] if you aren't going into jump then this is an interruption
    if fighter.global_table[globals::STATUS_KIND] != FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_HI_JUMP {
        MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EFFECT, Hash40::new("effect_specialhiinterrupt"), -1);
        MotionAnimcmdModule::enable_skip_delay_update(fighter.module_accessor);
    }
    0.into()
}

#[status_script(agent = "elight", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn special_hi_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    // [v] increment the frame counter so that the input leniency check works properly
    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_ELIGHT_STATUS_SPECIAL_HI_INT_FRAME_FROM_START);
    0.into()
}

pub fn install() {
    smashline::install_status_scripts!(
        special_hi_pre,
        special_hi_main,
        special_hi_end,
        special_hi_exec
    );
}
