use utils::{
    *,
    ext::*,
    consts::*,
    consts::globals::*
};
use smash::app::BattleObjectModuleAccessor;
use smash::phx::{Vector2f, Vector3f};
use smash::lib::{lua_const::*, L2CValue, L2CAgent};
use smash::app::{self, lua_bind::*, sv_kinetic_energy, sv_animcmd};
use smash::lua2cpp::L2CFighterCommon;
use smash::hash40;
use smash::phx::Hash40;
use smash_script::{self, *, macros::*};

pub mod groups {
    pub const SMALL: i32 = 0;
    pub const MEDIUM: i32 = 1;
    pub const LARGE: i32 = 2;
    pub const XLARGE: i32 = 3;
    pub const XXLARGE: i32 = 4;
}


/// Shifts fighter's ECB (Environment Collision Box) rhombus up to around their knees when they are in the air for over
/// a certain amount of frames *and* they are in the proper status
unsafe fn ecb_shifts(boma: &mut BattleObjectModuleAccessor) {
    if !smash::app::sv_information::is_ready_go() {
        GroundModule::set_rhombus_offset(boma, &Vector2f::zero());
        VarModule::set_float(boma.object(), vars::common::ECB_Y_OFFSETS, 0.0);
        return;
    }

    if !boma.is_prev_status_one_of(&[
        *FIGHTER_STATUS_KIND_CAPTURE_PULLED,
        *FIGHTER_STATUS_KIND_CAPTURE_WAIT,
        *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE,
        *FIGHTER_STATUS_KIND_CAPTURE_CUT,
        *FIGHTER_STATUS_KIND_THROWN
    ]) && !boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_CAPTURE_PULLED,
        *FIGHTER_STATUS_KIND_CAPTURE_WAIT,
        *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE,
        *FIGHTER_STATUS_KIND_CAPTURE_CUT,
        *FIGHTER_STATUS_KIND_ENTRY,
        *FIGHTER_STATUS_KIND_THROWN,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
        *FIGHTER_STATUS_KIND_DAMAGE_FALL,
        *FIGHTER_STATUS_KIND_TREAD_DAMAGE_AIR,
        *FIGHTER_STATUS_KIND_BURY,
        *FIGHTER_STATUS_KIND_BURY_WAIT
    ]) && !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_AIR)
    && !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_GROUND)
    {
        /*
        let offset = if boma.is_situation(*SITUATION_KIND_AIR) {
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR) < ParamModule::get_int(boma.object(), ParamType::Common, "ecb_shift_air_trans_frame") {
                return;
            }

            let group = ParamModule::get_int(boma.object(), ParamType::Shared, "ecb_group_shift");
            
            let mut sh_amount: f32 = match group {
                groups::SMALL   => ParamModule::get_float(boma.object(), ParamType::Common, "ecb_group_shift_amount.small"),
                groups::MEDIUM  => ParamModule::get_float(boma.object(), ParamType::Common, "ecb_group_shift_amount.medium"),
                groups::LARGE   => ParamModule::get_float(boma.object(), ParamType::Common, "ecb_group_shift_amount.large"),
                groups::XLARGE  => ParamModule::get_float(boma.object(), ParamType::Common, "ecb_group_shift_amount.x_large"),
                groups::XXLARGE => ParamModule::get_float(boma.object(), ParamType::Common, "ecb_group_shift_amount.xx_large"),
                _ => panic!("malformed parammodule file! unknown group number for ecb shift: {}", group.to_string())
            };

            if boma.is_status(*FIGHTER_STATUS_KIND_ESCAPE_AIR) {
                sh_amount += ParamModule::get_float(boma.object(), ParamType::Common, "ecb_shift_for_waveland");
            }

            // this is required for other ecb shift operations to perform correctly.
            VarModule::set_float(boma.object(), vars::common::ECB_Y_OFFSETS, sh_amount);

            sh_amount
        } else if boma.is_situation(*SITUATION_KIND_GROUND) {
            0.0
        } else {
            VarModule::get_float(boma.object(), vars::common::ECB_Y_OFFSETS)
        };
        */
        let mut offset = 0.0;
        if boma.is_situation(*SITUATION_KIND_AIR) {
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR) > ParamModule::get_int(boma.object(), ParamType::Common, "ecb_shift_air_trans_frame") {
                let group = ParamModule::get_int(boma.object(), ParamType::Shared, "ecb_group_shift");
                
                let mut sh_amount = 0.0;
                match group {
                    groups::SMALL   => sh_amount = ParamModule::get_float(boma.object(), ParamType::Common, "ecb_group_shift_amount.small"),
                    groups::MEDIUM  => sh_amount = ParamModule::get_float(boma.object(), ParamType::Common, "ecb_group_shift_amount.medium"),
                    groups::LARGE   => sh_amount = ParamModule::get_float(boma.object(), ParamType::Common, "ecb_group_shift_amount.large"),
                    groups::XLARGE  => sh_amount = ParamModule::get_float(boma.object(), ParamType::Common, "ecb_group_shift_amount.x_large"),
                    groups::XXLARGE => sh_amount = ParamModule::get_float(boma.object(), ParamType::Common, "ecb_group_shift_amount.xx_large"),
                    _ => panic!("malformed parammodule file! unknown group number for ecb shift: {}", group.to_string())
                };
                
                //let mut sh_amount = ParamModule::get_float(boma.object(), ParamType::Common, "ecb_group_shift_amount.xx_large");
                if boma.is_status(*FIGHTER_STATUS_KIND_ESCAPE_AIR) {
                    sh_amount += ParamModule::get_float(boma.object(), ParamType::Common, "ecb_shift_for_waveland");
                }

                // this is required for other ecb shift operations to perform correctly.
                VarModule::set_float(boma.object(), vars::common::ECB_Y_OFFSETS, sh_amount);
                offset = sh_amount;
            }

        } else if boma.is_situation(*SITUATION_KIND_GROUND) {
            offset = 0.0;
        } else {
            offset = VarModule::get_float(boma.object(), vars::common::ECB_Y_OFFSETS);
        }
        GroundModule::set_rhombus_offset(boma, &Vector2f::new(0.0, offset));
    }
    
}

//=================================================================
//== EXTRA TRACTION
//=================================================================

/// Sets the extra traction flag depending on current speed and current status in order to prevent
/// the game feeling too slippery
unsafe fn extra_traction(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    let speed_x = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN);
    let max_walk = WorkModule::get_param_float(boma, hash40("walk_speed_max"), 0);
    let ground_brake = WorkModule::get_param_float(boma, hash40("ground_brake"), 0);
    let added_traction: smash::phx::Vector3f = smash::phx::Vector3f {x: -1.0 * PostureModule::lr(boma) * ground_brake * speed_x.signum(), y: 0.0, z: 0.0};
    let double_traction_statuses = [
        *FIGHTER_STATUS_KIND_WAIT,
        *FIGHTER_STATUS_KIND_JUMP_SQUAT,
        *FIGHTER_STATUS_KIND_SQUAT,
        *FIGHTER_STATUS_KIND_SQUAT_RV,
        *FIGHTER_STATUS_KIND_SQUAT_WAIT,
        *FIGHTER_STATUS_KIND_LANDING_LIGHT,
        *FIGHTER_STATUS_KIND_LANDING,
        *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR,
        *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL,
        *FIGHTER_STATUS_KIND_ATTACK,
        *FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
        *FIGHTER_STATUS_KIND_ATTACK_S4_START,
        *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD,
        *FIGHTER_STATUS_KIND_ATTACK_S4,
        *FIGHTER_STATUS_KIND_ATTACK_HI4_START,
        *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD,
        *FIGHTER_STATUS_KIND_ATTACK_HI4,
        *FIGHTER_STATUS_KIND_ATTACK_LW4_START,
        *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD,
        *FIGHTER_STATUS_KIND_ATTACK_LW4
    ];

    if boma.is_status_one_of(&double_traction_statuses) {
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
        let motion_accel = smash::app::sv_kinetic_energy::get_accel(fighter.lua_state_agent);

        // reset flag at beginning of any status
        if fighter.global_table[CURRENT_FRAME].get_i32() == 0 {
            VarModule::off_flag(boma.object(), vars::common::IS_MOTION_BASED_ATTACK);
        }
        // if we detect that the current animation is trans-motion-based (shifts your character's position), disable traction for the entire attack 
        if motion_accel.x != 0.0 && !VarModule::is_flag(boma.object(), vars::common::IS_MOTION_BASED_ATTACK) {
            VarModule::on_flag(boma.object(), vars::common::IS_MOTION_BASED_ATTACK);
        }
        if speed_x.abs() > max_walk
        && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND
        && !VarModule::is_flag(boma.object(), vars::common::IS_MOTION_BASED_ATTACK) {
            if boma.is_prev_status_one_of(&double_traction_statuses) {
                KineticModule::add_speed(boma, &added_traction);
            }
            else if fighter.global_table[CURRENT_FRAME].get_i32() > 0 {
                KineticModule::add_speed(boma, &added_traction);
            }
        }
    }
}

//=================================================================
//== GRAB JUMP REFRESH
//=================================================================

/// Gives fighters an additional jump if they are grabbed
unsafe fn grab_jump_refresh(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_CAPTURE_PULLED,
        *FIGHTER_STATUS_KIND_CAPTURE_WAIT,
        *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE,
        *FIGHTER_STATUS_KIND_CAPTURE_CUT,
        *FIGHTER_STATUS_KIND_CAPTURE_JUMP
    ]) && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) == WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX)
    {
        WorkModule::dec_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    }
}

unsafe fn dash_energy(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_DASH) {
        let run_accel_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("run_accel_add"), 0);
        let run_accel_add = WorkModule::get_param_float(fighter.module_accessor, hash40("run_accel_mul"), 0);
        let ground_brake = WorkModule::get_param_float(fighter.module_accessor, hash40("ground_brake"), 0);
        let dash_speed: f32 = WorkModule::get_param_float(fighter.module_accessor, hash40("dash_speed"), 0);
        let run_speed_max = WorkModule::get_param_float(fighter.module_accessor, hash40("run_speed_max"), 0);
        let dash_stick_x: f32 = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("dash_stick_x"));
        let stick_x = fighter.global_table[STICK_X].get_f32();

        let bidou_buttons = &[
        Buttons::Attack,
        Buttons::AttackRaw,
        Buttons::Special,
        Buttons::SpecialRaw,
        Buttons::Smash
        ];

        let mut enable_bidou = false;
        for button in bidou_buttons.iter() {
            if fighter.boma().was_prev_button_on(*button) {
                enable_bidou = true;
            }
        }

        if ControlModule::check_button_release(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) {
            // prevent game from thinking you are inputting a dashback on the frame the cstick stops overriding left stick (0.625 -> -1.0)
            VarModule::on_flag(fighter.battle_object, vars::common::DISABLE_BACKDASH);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH);
        }
        if stick_x == 0.0 {
            // if you return stick to neutral after a cstick dash, allow dashbacks again
            VarModule::off_flag(fighter.battle_object, vars::common::DISABLE_BACKDASH);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH);
        }

        // initial dash energy
        if fighter.global_table[CURRENT_FRAME].get_i32() == 0 && stick_x.abs() >= dash_stick_x {
            // apply speed on f1 of dash (takes effect on f2 ingame)
            let prev_speed = VarModule::get_float(fighter.battle_object, vars::common::CURR_DASH_SPEED);
            let applied_speed = (dash_speed * PostureModule::lr(fighter.module_accessor)) + (stick_x.signum() * ((run_accel_mul + (run_accel_add * stick_x.abs())))) + prev_speed;  // initial dash speed + 1f of run acceleration + previous status' last speed
            //println!("Changing current dash speed: {}", applied_speed);
            let applied_speed_clamped = applied_speed.clamp(-run_speed_max, run_speed_max);
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, applied_speed_clamped);
            app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
        }

        // dash -> redash/backdash energy
        let is_dash_input: bool = (fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH != 0) || (enable_bidou && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) && stick_x * PostureModule::lr(fighter.module_accessor) > 0.6);  // we register a dash input by 1. Using game's command cat dash check, or 2. Checking if cstick has been input and is > 0.6 (max cstick x value is 0.625)
        let is_backdash_input: bool = (fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH != 0) || (enable_bidou && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) && stick_x * PostureModule::lr(fighter.module_accessor) < -0.6);

        if (WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH)
        && (!ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON))
        && is_backdash_input)  // if valid backdash input
        || (WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("re_dash_frame")) as f32 <= MotionModule::frame(fighter.module_accessor)  // if current frame is after redash frame
        && is_dash_input) {  // OR valid re-dash input
            let mut initial_speed = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN);

            let mut applied_speed = (initial_speed * 0.25) - (ground_brake * PostureModule::lr(fighter.module_accessor));  // Only retain a fraction of your momentum into a re-dash or backdash; makes for snappy dash dancing (Melee functionality)
            if (is_dash_input && VarModule::is_flag(fighter.battle_object, vars::common::IS_MOONWALK) && FighterMotionModuleImpl::is_valid_cancel_frame(fighter.module_accessor, -1, true)) || fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON != 0 {  // if the jump button is held, retain full momentum into next status
                //println!("full momentum");
                applied_speed = initial_speed - (ground_brake * PostureModule::lr(fighter.module_accessor));
            }
            let applied_speed_clamped = applied_speed.clamp(-run_speed_max, run_speed_max);

            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP_NO_STOP);
            app::sv_kinetic_energy::unable(fighter.lua_state_agent);
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            app::sv_kinetic_energy::enable(fighter.lua_state_agent);
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, applied_speed_clamped);
            app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);

            let end_speed = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN);
            VarModule::set_float(fighter.battle_object, vars::common::CURR_DASH_SPEED, end_speed);
        }
    }
}

pub unsafe fn run(fighter: &mut L2CFighterCommon, lua_state: u64, l2c_agent: &mut L2CAgent, boma: &mut BattleObjectModuleAccessor, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, fighter_kind: i32, stick_x: f32, stick_y: f32, facing: f32) {
    ecb_shifts(boma);
    extra_traction(fighter, boma);
    grab_jump_refresh(boma);
    dash_energy(fighter);

    //WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FLY_REFLECT_D); //Melee style spike knockdown (courtesey of zabimaru), leaving it commented here just to have it saved somewhere
}

