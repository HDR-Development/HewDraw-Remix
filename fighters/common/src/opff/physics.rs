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
        VarModule::set_float(boma.object(), vars::common::instance::ECB_Y_OFFSETS, 0.0);
        return;
    }

    let mut offset = 0.0;
    if !(*boma).is_status_one_of(&[
        *FIGHTER_STATUS_KIND_ENTRY,
        *FIGHTER_STATUS_KIND_CAPTURE_PULLED,
        *FIGHTER_STATUS_KIND_CAPTURE_WAIT,
        *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE,
        *FIGHTER_STATUS_KIND_THROWN])
    && boma.is_situation(*SITUATION_KIND_AIR)
    {
        if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR) >= ParamModule::get_int(boma.object(), ParamType::Common, "ecb_shift_air_trans_frame") {
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
            offset = sh_amount;
        }
        else {
            offset = 0.0;
        }

    } else {
        offset = 0.0;
    }
    VarModule::set_float(boma.object(), vars::common::instance::ECB_Y_OFFSETS, offset);
    
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
        *FIGHTER_STATUS_KIND_ATTACK_LW4,
        *FIGHTER_STATUS_KIND_CATCH,
        *FIGHTER_STATUS_KIND_CATCH_WAIT,
        *FIGHTER_STATUS_KIND_CATCH_ATTACK,
        *FIGHTER_STATUS_KIND_CATCH_PULL,
        *FIGHTER_STATUS_KIND_ITEM_THROW
    ];

    if boma.is_status_one_of(&double_traction_statuses) {
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
        let motion_accel = smash::app::sv_kinetic_energy::get_accel(fighter.lua_state_agent);

        // reset flag at beginning of any status
        if fighter.global_table[CURRENT_FRAME].get_i32() == 0 {
            VarModule::off_flag(boma.object(), vars::common::instance::IS_MOTION_BASED_ATTACK);
        }
        // if we detect that the current animation is trans-motion-based (shifts your character's position), disable traction for the entire attack 
        if motion_accel.x != 0.0 && !VarModule::is_flag(boma.object(), vars::common::instance::IS_MOTION_BASED_ATTACK) {
            VarModule::on_flag(boma.object(), vars::common::instance::IS_MOTION_BASED_ATTACK);
        }
        if speed_x.abs() > max_walk
        && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND
        && !VarModule::is_flag(boma.object(), vars::common::instance::IS_MOTION_BASED_ATTACK) {
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

pub unsafe fn run(fighter: &mut L2CFighterCommon, lua_state: u64, l2c_agent: &mut L2CAgent, boma: &mut BattleObjectModuleAccessor, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, fighter_kind: i32, stick_x: f32, stick_y: f32, facing: f32) {
    ecb_shifts(boma);
    extra_traction(fighter, boma);
    grab_jump_refresh(boma);

    //WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FLY_REFLECT_D); //Melee style spike knockdown (courtesey of zabimaru), leaving it commented here just to have it saved somewhere
}

