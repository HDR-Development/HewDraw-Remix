use smash::app::lua_bind::*;
use smash::app::BattleObjectModuleAccessor;
use smash::app::{self, lua_bind::*, sv_animcmd, sv_kinetic_energy};
use smash::cpp::root::app::SituationKind;
use smash::hash40;
use smash::lib::{lua_const::*, L2CAgent, L2CValue};
use smash::lua2cpp::L2CFighterCommon;
use smash::phx::Hash40;
use smash::phx::{Vector2f, Vector3f, Vector4f};
use utils::{consts::globals::*, consts::*, ext::*, *};
use vars::*;

//=================================================================
//== TUMBLE EXIT
//=================================================================
unsafe fn tumble_exit(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status(*FIGHTER_STATUS_KIND_DAMAGE_FALL) {
        if boma.status_frame() == 0 {
            ControlModule::clear_command_one(
                boma,
                *FIGHTER_PAD_COMMAND_CATEGORY1,
                *FIGHTER_PAD_CMD_CAT1_DASH,
            );
            ControlModule::clear_command_one(
                boma,
                *FIGHTER_PAD_COMMAND_CATEGORY1,
                *FIGHTER_PAD_CMD_CAT1_TURN_DASH,
            );
        }
        if !(WorkModule::is_flag(
            boma,
            *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_AIR,
        ) || WorkModule::is_flag(
            boma,
            *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_GROUND,
        )) && boma.is_cat_flag(Cat1::Dash | Cat1::TurnDash)
        {
            ControlModule::reset_trigger(boma);
            boma.change_status_req(*FIGHTER_STATUS_KIND_FALL, true);
        }
    }
}

// This allows us to generate the blue DI line effect on non-tumble knockback
unsafe fn non_tumble_di_gfx(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_DAMAGE_AIR) {
        fighter.FighterStatusDamage__correctDamageVectorEffect(L2CValue::Bool(false));
    }
}

// plat drop if you input down during a waveland (airdodge landing lag)
unsafe fn waveland_plat_drop(boma: &mut BattleObjectModuleAccessor, cat2: i32, status_kind: i32) {
    let pass_thresh = boma.get_param_float("common", "pass_stick_y");
    if boma.is_status(*FIGHTER_STATUS_KIND_LANDING)
    && GroundModule::is_passable_ground(boma)
    && ControlModule::get_flick_y(boma) < 2
    && boma.left_stick_y() < pass_thresh
    && boma.is_prev_status_one_of(&[
        *FIGHTER_STATUS_KIND_ESCAPE_AIR,
        *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE
    ]) {
        boma.change_status_req(*FIGHTER_STATUS_KIND_PASS, true);
        return;
    }
}

//=================================================================
//== DASH DROP
//=================================================================
unsafe fn dash_drop(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    let flick_y_sens =
        ParamModule::get_float(boma.object(), ParamType::Common, "general_flick_y_sens");
    let flick_y = ControlModule::get_flick_y(boma);
    if GroundModule::is_passable_ground(boma)
        && flick_y != 0xFE
        && boma.stick_y() < flick_y_sens
        && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_RUN, *FIGHTER_STATUS_KIND_RUN_BRAKE])
    {
        boma.change_status_req(*FIGHTER_STATUS_KIND_PASS, true);
    }
}

//=================================================================
//== CROUCH DURING RUN
//=================================================================
unsafe fn run_squat(boma: &mut BattleObjectModuleAccessor, status_kind: i32, stick_y: f32) {
    //let crouch_thresh: f32 = WorkModule::get_param_float(boma, hash40("common"), hash40("pass_stick_y"));
    if boma.is_status(*FIGHTER_STATUS_KIND_RUN_BRAKE)
        && boma.stick_y()
            < WorkModule::get_param_float(boma, hash40("common"), hash40("squat_stick_y"))
    {
        boma.change_status_req(*FIGHTER_STATUS_KIND_SQUAT, false);
    }
}

unsafe fn double_shield_button_airdodge(
    boma: &mut BattleObjectModuleAccessor,
    status_kind: i32,
    situation_kind: i32,
    cat1: i32,
) {
    // airdodge with second shield button while holding another shield button
    if boma.is_situation(*SITUATION_KIND_AIR)
        && boma.is_button_trigger(Buttons::GuardHold)
        && !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR)
        && boma.is_status_one_of(&[
            *FIGHTER_STATUS_KIND_JUMP,
            *FIGHTER_STATUS_KIND_JUMP_AERIAL,
            *FIGHTER_STATUS_KIND_FALL,
            *FIGHTER_STATUS_KIND_PASS,
            *FIGHTER_STATUS_KIND_FALL_AERIAL,
            *FIGHTER_STATUS_KIND_CLIFF_JUMP1,
            *FIGHTER_STATUS_KIND_CLIFF_JUMP2,
            *FIGHTER_STATUS_KIND_CLIFF_JUMP3,
        ])
    {
        boma.change_status_req(*FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
        return;
    }
}

unsafe fn drift_di(
    fighter: &mut L2CFighterCommon,
    boma: &mut BattleObjectModuleAccessor,
    status_kind: i32,
    situation_kind: i32,
) {
    if boma.is_situation(*SITUATION_KIND_AIR)
        && !StopModule::is_stop(boma)
        && boma.is_status_one_of(&[
            *FIGHTER_STATUS_KIND_DAMAGE_FLY,
            *FIGHTER_STATUS_KIND_DAMAGE_AIR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
        ])
    {
        let damage_speed_x = fighter.get_speed_x(*FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
        let damage_speed_y = fighter.get_speed_y(*FIGHTER_KINETIC_ENERGY_ID_DAMAGE);

        let mut initial_speed_x = VarModule::get_float(
            fighter.battle_object,
            vars::common::status::INITIAL_KNOCKBACK_VEL_X,
        );
        let mut initial_speed_y = VarModule::get_float(
            fighter.battle_object,
            vars::common::status::INITIAL_KNOCKBACK_VEL_Y,
        );

        // if these floats are both exactly zero, its because
        // status change reset them to zero. Thus, we should set them.
        if initial_speed_x == 0.0 && initial_speed_y == 0.0 {
            VarModule::set_float(
                fighter.battle_object,
                vars::common::status::INITIAL_KNOCKBACK_VEL_X,
                damage_speed_x,
            );
            VarModule::set_float(
                fighter.battle_object,
                vars::common::status::INITIAL_KNOCKBACK_VEL_Y,
                damage_speed_y,
            );

            initial_speed_x = VarModule::get_float(
                fighter.battle_object,
                vars::common::status::INITIAL_KNOCKBACK_VEL_X,
            );
            initial_speed_y = VarModule::get_float(
                fighter.battle_object,
                vars::common::status::INITIAL_KNOCKBACK_VEL_Y,
            );
        }

        let mut speed_mul = ParamModule::get_float(
            fighter.battle_object,
            ParamType::Common,
            "drift_di.speed_mul_base",
        );
        let speed_mul_add_max = ParamModule::get_float(
            fighter.battle_object,
            ParamType::Common,
            "drift_di.speed_mul_add_max",
        );

        let lerp_max_speed = ParamModule::get_float(
            fighter.battle_object,
            ParamType::Common,
            "drift_di.speed_lerp_max",
        );

        let ratio = 1.0 - (initial_speed_x.abs() / lerp_max_speed).clamp(0.0, 1.0);
        speed_mul = (speed_mul + speed_mul_add_max) * ratio;

        let drift_value = boma.left_stick_x() * speed_mul;

        fighter.set_speed(
            Vector2f::new(damage_speed_x + drift_value, damage_speed_y),
            *FIGHTER_KINETIC_ENERGY_ID_DAMAGE,
        );
    }
}

extern "C" {
    #[link_name = "\u{1}_ZN3app14sv_information8stage_idEv"]
    pub fn stage_id() -> i32;
}

pub unsafe fn respawn_taunt(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if !boma.is_status(*FIGHTER_STATUS_KIND_REBIRTH) {
        return;
    }

    match MotionModule::motion_kind(boma) {
        utils::hash40!("appeal_hi_r") => return,
        utils::hash40!("appeal_hi_l") => return,
        utils::hash40!("appeal_lw_r") => return,
        utils::hash40!("appeal_lw_l") => return,
        utils::hash40!("appeal_s_l") => return,
        utils::hash40!("appeal_s_r") => return,
        _ => {}
    }

    let motion = if boma.is_button_trigger(Buttons::AppealHi) {
        if PostureModule::lr(boma) == 1.0 {
            Hash40::new("appeal_hi_r")
        } else {
            Hash40::new("appeal_hi_l")
        }
    } else if boma.is_button_trigger(Buttons::AppealSL) {
        Hash40::new("appeal_s_l")
    } else if boma.is_button_trigger(Buttons::AppealSR) {
        Hash40::new("appeal_s_r")
    } else if boma.is_button_trigger(Buttons::AppealLw) {
        if PostureModule::lr(boma) == 1.0 {
            Hash40::new("appeal_lw_r")
        } else {
            Hash40::new("appeal_lw_l")
        }
    } else {
        return;
    };

    MotionModule::change_motion(boma, motion, 0.0, 1.0, false, 0.0, false, false);
}

// Teeter cancelling
pub unsafe fn teeter_cancel(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    if (boma.is_situation(*SITUATION_KIND_GROUND)
        && boma.is_status_one_of(&[
            *FIGHTER_STATUS_KIND_WAIT,
            *FIGHTER_STATUS_KIND_DASH,
            *FIGHTER_STATUS_KIND_RUN_BRAKE,
            *FIGHTER_STATUS_KIND_APPEAL,
            *FIGHTER_STATUS_KIND_LANDING,
            *FIGHTER_STATUS_KIND_LANDING_LIGHT,
            *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR,
            *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL,
            *FIGHTER_STATUS_KIND_LANDING_DAMAGE_LIGHT,
        ])
        && ((KineticModule::get_sum_speed_x(
            fighter.module_accessor,
            *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL,
        ) - KineticModule::get_sum_speed_x(
            fighter.module_accessor,
            *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND,
        ) - KineticModule::get_sum_speed_x(
            fighter.module_accessor,
            *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN,
        )) * PostureModule::lr(boma))
            > 0.0)
    {
        // Conditions for transitioning to teeter animation in sub_ground_check_ottotto
        if (GroundModule::is_ottotto(boma, 1.72) // Original value: 0.86
        && fighter.left_stick_x().abs() < 0.75)
        {
            fighter.change_status(FIGHTER_STATUS_KIND_OTTOTTO.into(), true.into());
        }
    }
}

pub unsafe fn run(
    fighter: &mut L2CFighterCommon,
    lua_state: u64,
    l2c_agent: &mut L2CAgent,
    boma: &mut BattleObjectModuleAccessor,
    cat: [i32; 4],
    status_kind: i32,
    situation_kind: i32,
    fighter_kind: i32,
    stick_x: f32,
    stick_y: f32,
    facing: f32,
    curr_frame: f32,
) {
    tumble_exit(boma);
    non_tumble_di_gfx(fighter);
    dash_drop(boma, status_kind);
    run_squat(boma, status_kind, stick_y); // Must be done after dash_drop()
    double_shield_button_airdodge(boma, status_kind, situation_kind, cat[0]);
    //drift_di(fighter, boma, status_kind, situation_kind);
    waveland_plat_drop(boma, cat[1], status_kind);
    respawn_taunt(boma, status_kind);
    teeter_cancel(fighter, boma);
}
