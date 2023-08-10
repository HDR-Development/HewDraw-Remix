// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn normal_side_special(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if WorkModule::is_flag(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_S) {
        if [*FIGHTER_STATUS_KIND_DAMAGE,
            *FIGHTER_STATUS_KIND_DAMAGE_AIR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
            *FIGHTER_STATUS_KIND_DAMAGE_FALL].contains(&status_kind) {
            WorkModule::off_flag(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_S);
        }
    }
}

// Tech Roll distance help
unsafe fn tech_roll_help(boma: &mut BattleObjectModuleAccessor, motion_kind: u64, facing: f32, frame: f32) {
    if frame < 6.0 {
        let mut motion_vec = Vector3f{x: 1.75, y: 0.0, z: 0.0};
        if motion_kind == hash40("passive_stand_f") {
            motion_vec.x *= facing;
        } else if motion_kind == hash40("passive_stand_b") {
            motion_vec.x *= -facing;
        } else {
            return; // Break out if not passive_stand_x
        }
        KineticModule::add_speed_outside(boma, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
    }
}

unsafe fn nspecial_cancels(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if status_kind == *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N_START {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            if fighter.is_cat_flag(Cat2::StickEscape) {
                VarModule::set_int(fighter.battle_object, vars::littlemac::status::SPECIAL_N_CANCEL_TYPE, vars::littlemac::SPECIAL_N_CANCEL_TYPE_ESCAPE);
                fighter.change_to_custom_status(statuses::littlemac::SPECIAL_N_CANCEL, true, false);
            }
            else if fighter.is_cat_flag(Cat2::StickEscapeF) {
                VarModule::set_int(fighter.battle_object, vars::littlemac::status::SPECIAL_N_CANCEL_TYPE, vars::littlemac::SPECIAL_N_CANCEL_TYPE_ESCAPE_F);
                fighter.change_to_custom_status(statuses::littlemac::SPECIAL_N_CANCEL, true, false);
            }
            else if fighter.is_cat_flag(Cat2::StickEscapeB) {
                VarModule::set_int(fighter.battle_object, vars::littlemac::status::SPECIAL_N_CANCEL_TYPE, vars::littlemac::SPECIAL_N_CANCEL_TYPE_ESCAPE_B);
                fighter.change_to_custom_status(statuses::littlemac::SPECIAL_N_CANCEL, true, false);
            }
            else if (fighter.is_cat_flag(Cat1::JumpButton) || (ControlModule::is_enable_flick_jump(fighter.module_accessor) && fighter.is_cat_flag(Cat1::Jump) && fighter.sub_check_button_frick().get_bool())) {
                VarModule::set_int(fighter.battle_object, vars::littlemac::status::SPECIAL_N_CANCEL_TYPE, vars::littlemac::SPECIAL_N_CANCEL_TYPE_GROUND_JUMP);
                fighter.change_to_custom_status(statuses::littlemac::SPECIAL_N_CANCEL, true, false);
            }
            if fighter.sub_check_command_guard().get_bool() {
                VarModule::set_int(fighter.battle_object, vars::littlemac::status::SPECIAL_N_CANCEL_TYPE, vars::littlemac::SPECIAL_N_CANCEL_TYPE_GUARD);
                fighter.change_to_custom_status(statuses::littlemac::SPECIAL_N_CANCEL, true, false);
                WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON);
            }
        }
        else {
            if fighter.is_cat_flag(Cat1::AirEscape)  {
                VarModule::set_int(fighter.battle_object, vars::littlemac::status::SPECIAL_N_CANCEL_TYPE, vars::littlemac::SPECIAL_N_CANCEL_TYPE_ESCAPE_AIR);
                fighter.change_to_custom_status(statuses::littlemac::SPECIAL_N_CANCEL, true, false);
                WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);
            }
            else if (fighter.is_cat_flag(Cat1::JumpButton) || (ControlModule::is_enable_flick_jump(fighter.module_accessor) && fighter.is_cat_flag(Cat1::Jump)))
            && fighter.get_num_used_jumps() < fighter.get_jump_count_max()
            {
                VarModule::set_int(fighter.battle_object, vars::littlemac::status::SPECIAL_N_CANCEL_TYPE, vars::littlemac::SPECIAL_N_CANCEL_TYPE_JUMP_AERIAL);
                fighter.change_to_custom_status(statuses::littlemac::SPECIAL_N_CANCEL_JUMP, true, false);
            }
        }
    }
}

// Fixes weird vanilla behavior where touching ground during upB puts you into special fall for 1f before landing
unsafe fn up_special_proper_landing(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_HI_JUMP)
    && fighter.is_situation(*SITUATION_KIND_GROUND)
    && MotionModule::frame(fighter.module_accessor) >= 28.0 {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, false);
        KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N2,
        *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_S_BLOW_END,
        *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_S_JUMP_END,
        *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_HI_JUMP,
        *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_LW_HIT
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
        if fighter.is_flag(*FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
            if [*FIGHTER_KINETIC_TYPE_MOTION_AIR, *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE].contains(&KineticModule::get_kinetic_type(fighter.module_accessor)) {
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
                let speed_y = app::sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);

                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y, 0.0, 0.0, 0.0);
                app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
                
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                app::sv_kinetic_energy::enable(fighter.lua_state_agent);

                KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
            }
        }
    }
}

pub unsafe fn moveset(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    normal_side_special(boma, status_kind);
    tech_roll_help(boma, motion_kind, facing, frame);
    nspecial_cancels(fighter, boma, status_kind, situation_kind, cat[0], frame);
    up_special_proper_landing(fighter);
    fastfall_specials(fighter);
}

#[utils::macros::opff(FIGHTER_KIND_LITTLEMAC )]
pub fn littlemac_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		littlemac_frame(fighter)
    }
}

pub unsafe fn littlemac_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}