use utils::{
    *,
    ext::*,
    consts::*
};
use smash::app::BattleObjectModuleAccessor;
use smash::phx::{Vector2f, Vector3f, Vector4f};
use smash::app::lua_bind::*;
use smash::lib::{lua_const::*, L2CValue, L2CAgent};
use smash::hash40;
use smash::phx::Hash40;
use smash::cpp::root::app::SituationKind;
use smash::lua2cpp::L2CFighterCommon;
use smash::app::{self, lua_bind::*, sv_kinetic_energy, sv_animcmd};
use vars::*;

//=================================================================
//== TUMBLE EXIT
//=================================================================
unsafe fn tumble_exit(boma: &mut BattleObjectModuleAccessor, cat1: i32, status_kind: i32, situation_kind: i32) {
    // TODO: Move some of this into ParamModule
    let remaining_hitstun = WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME);
    let total_hitstun = WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME_LAST);
    let hitstun_passed = total_hitstun - remaining_hitstun;
    /*
     * Pick: damage fall OR (damage_fly variant + hitstun + 5 frame)
     */

    if remaining_hitstun > 0.0
    && VarModule::is_flag(boma.object(), common::CAN_ESCAPE_TUMBLE)
    && boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL
    ])
    {
        VarModule::off_flag(boma.object(), common::CAN_ESCAPE_TUMBLE);
    }

    if FighterStopModuleImpl::is_damage_stop(boma) {
        return;
    }

    if !VarModule::is_flag(boma.object(), common::TUMBLE_KB)
    && (boma.is_status(*FIGHTER_STATUS_KIND_DAMAGE_FALL)
        || (boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL])
                && remaining_hitstun > 0.0 && hitstun_passed > 5.0))
    && !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_AIR)
    && !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_GROUND)
    {
        VarModule::on_flag(boma.object(), common::TUMBLE_KB);
    }

    if !boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_DAMAGE_FALL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL
    ])
    {
        VarModule::off_flag(boma.object(), common::TUMBLE_KB);
        VarModule::off_flag(boma.object(), common::CAN_ESCAPE_TUMBLE);
    }

    if VarModule::is_flag(boma.object(), common::TUMBLE_KB) && remaining_hitstun == 0.0 {
        VarModule::on_flag(boma.object(), common::CAN_ESCAPE_TUMBLE);
    }

    if boma.is_situation(*SITUATION_KIND_AIR)
    && VarModule::is_flag(boma.object(), common::CAN_ESCAPE_TUMBLE)
    && boma.is_cat_flag(Cat1::Dash | Cat1::TurnDash)
    {
        boma.change_status_req(*FIGHTER_STATUS_KIND_FALL, false);
    }
}

//=================================================================
//== NON-TUMBLE KNOCKBACK DI
//=================================================================
unsafe fn non_tumble_di(fighter: &mut L2CFighterCommon, lua_state: u64, l2c_agent: &mut L2CAgent, boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    //let prev_status = StatusModule::prev_status_kind(boma, 0);
    let status_kind_prev = StatusModule::prev_status_kind(boma, 0);

    if [*FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_AIR].contains(&status_kind) {

        //let stick_x = ControlModule::get_stick_x(boma);
        //let stick_y = ControlModule::get_stick_y(boma);
        //let di_stick = (stick_y.abs() + 0.00001) / (stick_x.abs() + 0.00001);
        //let di_angle = di_stick.atan();
        if FighterStopModuleImpl::get_damage_stop_frame(boma) as i32 == 1 {

            // println!("last frame of nontumble hitlag");
            /*
            if(stick_x.abs() > 0.1 || stick_y.abs() > 0.1){
                // println!("nontumble di angle input: {}", di_angle);
                // println!("");
                l2c_agent.clear_lua_stack(); //clear the stack from any previous args
                l2c_agent.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_DAMAGE as u64)); //push the first arg, that being a KINETIC_ENERGY_ID const
                l2c_agent.push_lua_stack(&mut L2CValue::new_num(di_angle)); //push the second arg, that being a float of the new speed we want to set
                sv_kinetic_energy::set_angle(lua_state); //call the desired function with the lua state which will grab the args we previously pushed
            }
            */
            smash::lua2cpp::L2CFighterCommon::FighterStatusDamage__correctDamageVector(fighter);

        }
        if [*FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_AIR].contains(&status_kind)
            && [*FIGHTER_STATUS_KIND_THROWN, *FIGHTER_STATUS_KIND_CLUNG_THROWN_DIDDY, *FIGHTER_STATUS_KIND_SHOULDERED_DONKEY_THROWN, *FIGHTER_STATUS_KIND_MIIFIGHTER_COUNTER_THROWN, *FIGHTER_STATUS_KIND_MIIFIGHTER_SUPLEX_THROWN, *FIGHTER_STATUS_KIND_SWING_GAOGAEN_THROWN].contains(&status_kind_prev) {
            let remaining_hitstun = WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME);
            let total_hitstun = WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME_LAST);
            let hitstun_passed = total_hitstun - remaining_hitstun;
            // println!("Nontumble throw");
            // println!("total hitstun: {}", total_hitstun);
            // println!("remaining hitstun: {}", remaining_hitstun);
            // println!("hitstun passed: {}", hitstun_passed);
            if total_hitstun > 0.0 && hitstun_passed == 1.0 {
                // println!(" === Nontumble throw DI frame");
                smash::lua2cpp::L2CFighterCommon::FighterStatusDamage__correctDamageVector(fighter);
            }

        }
    }
    //let remaining_hitstun = WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME);
    //let total_hitstun = WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME_LAST);
    //let hitstun_passed = total_hitstun - remaining_hitstun;
    // println!("Status Kind: {}", status_kind);
    // println!("total hitstun: {}", total_hitstun);
    // println!("remaining hitstun: {}", remaining_hitstun);
    // println!("hitstun passed: {}", hitstun_passed);
}

// plat drop if you input down during a waveland (airdodge landing lag)
unsafe fn waveland_plat_drop(boma: &mut BattleObjectModuleAccessor, cat2: i32, status_kind: i32) {
    let flick_y_sens = ParamModule::get_float(boma.object(), ParamType::Common, "general_flick_y_sens");
    if boma.is_status(*FIGHTER_STATUS_KIND_LANDING)
    && VarModule::is_flag(boma.object(), vars::common::ENABLE_WAVELAND_PLATDROP)
    && GroundModule::is_passable_ground(boma)
    && boma.is_flick_y(flick_y_sens)
    && boma.is_prev_status_one_of(&[
        *FIGHTER_STATUS_KIND_ESCAPE_AIR,
        *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE
    ])
    {
        boma.change_status_req(*FIGHTER_STATUS_KIND_PASS, true);
        return;
    }

    if boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_ESCAPE_AIR,
        *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE,
        *FIGHTER_STATUS_KIND_LANDING
    ])
    && boma.stick_y() > ParamModule::get_float(boma.object(), ParamType::Common, "waveland_pass_neutral_sens")
    {
        VarModule::on_flag(boma.object(), vars::common::ENABLE_WAVELAND_PLATDROP);
    }
}


//=================================================================
//== DASH DROP
//=================================================================
unsafe fn dash_drop(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    let flick_y_sens = ParamModule::get_float(boma.object(), ParamType::Common, "general_flick_y_sens");
    if GroundModule::is_passable_ground(boma)
    && boma.is_flick_y(flick_y_sens)
    && boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_RUN,
        *FIGHTER_STATUS_KIND_RUN_BRAKE,
        *FIGHTER_STATUS_KIND_DASH,
        *FIGHTER_STATUS_KIND_TURN_DASH
    ])
    {
        boma.change_status_req(*FIGHTER_STATUS_KIND_PASS, true);
    }
}

//=================================================================
//== CROUCH DURING RUN
//=================================================================
unsafe fn run_squat(boma: &mut BattleObjectModuleAccessor, status_kind: i32, stick_y: f32) {
    //let crouch_thresh: f32 = WorkModule::get_param_float(boma, hash40("common"), hash40("pass_stick_y"));
    if boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_RUN, *FIGHTER_STATUS_KIND_RUN_BRAKE])
    && boma.stick_y() < WorkModule::get_param_float(boma, hash40("common"), hash40("squat_stick_y"))
    {
        boma.change_status_req(*FIGHTER_STATUS_KIND_WAIT, false);
    }
}

//=================================================================
//== GLIDE TOSS
//=================================================================
unsafe fn glide_toss(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, facing: f32) {
    if boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_ESCAPE_F, *FIGHTER_STATUS_KIND_ESCAPE_B])
    {
        let max_ditcit_frame = ParamModule::get_float(boma.object(), ParamType::Common, "glide_toss_cancel_frame");
        VarModule::set_flag(boma.object(), vars::common::CAN_GLIDE_TOSS, MotionModule::frame(boma) <= max_ditcit_frame);
        return;
    }

    if boma.is_status(*FIGHTER_STATUS_KIND_ITEM_THROW)
    && VarModule::is_flag(boma.object(), vars::common::CAN_GLIDE_TOSS)
    {
        let multiplier = 2.8 * (MotionModule::end_frame(boma) - MotionModule::frame(boma)) / MotionModule::end_frame(boma);
        let speed_x = if boma.is_prev_status(*FIGHTER_STATUS_KIND_ESCAPE_F) {
            multiplier * VarModule::get_float(boma.object(), vars::common::ROLL_DIR)
        } else if boma.is_prev_status(*FIGHTER_STATUS_KIND_ESCAPE_B) {
            multiplier * VarModule::get_float(boma.object(), vars::common::ROLL_DIR) * -1.0
        } else {
            return;
        };

        KineticModule::add_speed_outside(boma, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &Vector3f::new(speed_x, 0.0, 0.0));
    }
}

unsafe fn shield_lock_tech(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32) {
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
        *FIGHTER_STATUS_KIND_CLIFF_JUMP3
    ])
    {
        boma.change_status_req(*FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
        return;
    }

    if boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_STATUS_KIND_GUARD])
    && boma.is_cat_flag(Cat1::JumpButton)
    && ((boma.is_button_on(Buttons::SpecialAll) && !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_GUARD_HOLD_SPECIAL_BUTTON))
        || boma.is_button_on(Buttons::GuardHold))
    {
        boma.change_status_req(*FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
    }
}
 
unsafe fn drift_di(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    if boma.is_situation(*SITUATION_KIND_AIR)
    && !StopModule::is_stop(boma)
    && boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_AIR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U
    ])
    {
        let speed_x = fighter.get_speed_x(*FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
        let speed_y = fighter.get_speed_y(*FIGHTER_KINETIC_ENERGY_ID_DAMAGE);

        let mut speed_mul = ParamModule::get_float(fighter.battle_object, ParamType::Common, "drift_di.speed_mul_base");
        let speed_mul_add_max = ParamModule::get_float(fighter.battle_object, ParamType::Common, "drift_di.speed_mul_add_max");

        let lerp_min_speed = ParamModule::get_float(fighter.battle_object, ParamType::Common, "drift_di.speed_lerp_min");
        let lerp_max_speed = ParamModule::get_float(fighter.battle_object, ParamType::Common, "drift_di.speed_lerp_max");

        if speed_x.abs() < lerp_min_speed {
            speed_mul += speed_mul_add_max;
        } else if speed_x.abs() < lerp_max_speed {
            let ratio = 1.0 - ((speed_x.abs() - lerp_min_speed) / (lerp_max_speed - lerp_min_speed));
            speed_mul += ratio * speed_mul_add_max;
        }

        let drift_value = boma.stick_x() * speed_mul;
        fighter.set_speed(Vector2f::new(speed_x + drift_value, speed_y), *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
    }
}


extern "C" {
    #[link_name = "\u{1}_ZN3app14sv_information8stage_idEv"]
    pub fn stage_id() -> i32;
}

pub unsafe fn freeze_stages(boma: &mut BattleObjectModuleAccessor) {

    // determine the current stage id
    //println!("stage id: {}", stage_id());

    // warioware
    if (stage_id() == 104) {
        smash::app::FighterUtil::set_stage_pause_for_final(true, boma);
    }
}

pub unsafe fn hitfall(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, fighter_kind: i32, cat: [i32 ; 4]) {
    if boma.kind() == *FIGHTER_KIND_GAOGAEN
    && boma.is_situation(*SITUATION_KIND_AIR)
    && boma.is_status(*FIGHTER_STATUS_KIND_ATTACK_AIR)
    {
        /* this is written this way because stick_y_flick wont update during
            hitlag, which means we need a flag to allow you to hitfall 1 frame
            after the end of hitlag as well, and we need to check previous 
            stick y directly to detect hitfall. That way, with the 5 frame buffer,
            if you input a fastfall during hitlag, it will get registered after
            the hitlag is over. Without the HITFALL_BUFFER flag, you have to
            input the fastfall BEFORE you hit the move, only.
        */
        if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD)
        || AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD)
        {
            VarModule::set_int(boma.object(), vars::common::HITFALL_BUFFER, 0);
        }

        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) {
            VarModule::inc_int(boma.object(), vars::common::HITFALL_BUFFER);
        }

        let buffer = VarModule::get_int(boma.object(), vars::common::HITFALL_BUFFER);

        if boma.is_cat_flag(Cat2::FallJump)
        && 0 < buffer && buffer <= 5 
        {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
        }
    }
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

pub unsafe fn run(fighter: &mut L2CFighterCommon, lua_state: u64, l2c_agent: &mut L2CAgent, boma: &mut BattleObjectModuleAccessor, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, fighter_kind: i32, stick_x: f32, stick_y: f32, facing: f32, curr_frame: f32) {
    tumble_exit(boma, cat[0], status_kind, situation_kind);
    non_tumble_di(fighter, lua_state, l2c_agent, boma, status_kind);
    dash_drop(boma, status_kind);
    run_squat(boma, status_kind, stick_y); // Must be done after dash_drop()
    glide_toss(fighter, boma, status_kind, facing);
    shield_lock_tech(boma, status_kind, situation_kind, cat[0]);
    drift_di(fighter, boma, status_kind, situation_kind);
    waveland_plat_drop(boma, cat[1], status_kind);
    hitfall(boma, status_kind, situation_kind, fighter_kind, cat);
    respawn_taunt(boma, status_kind);

    freeze_stages(boma);
}
    
