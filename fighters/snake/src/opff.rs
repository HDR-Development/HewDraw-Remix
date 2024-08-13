// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

/*unsafe fn grab_walk(boma: &mut BattleObjectModuleAccessor, status_kind: i32, cat2: i32) {
    if status_kind == *FIGHTER_STATUS_KIND_CATCH_WAIT {
        let motion_value = 0.65;
        let mut motion_vec = Vector3f{x: 0.0, y: 0.0, z: 0.0};

        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
            motion_vec.x = motion_value;
        } else if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_S_L) {
            motion_vec.x = -motion_value;
        }
        KineticModule::add_speed_outside(boma, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
    }
}*/

// Snake Grenade Counter reset
unsafe fn grenade_counter_reset(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32) {
    if [*FIGHTER_STATUS_KIND_ENTRY,
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_REBIRTH].contains(&status_kind) {
        VarModule::off_flag(boma.object(), vars::snake::instance::TRANQ_NEED_RELEOAD);
    }
}

// handles fsmash transitioning into the second/third hits (reimpl of AParticularUser's snake_frame)
unsafe fn fsmash_combo(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if [*FIGHTER_STATUS_KIND_ATTACK_S4].contains(&status_kind) {
        if !VarModule::is_flag(boma.object(), vars::snake::instance::KNIFE_COMBO_IS_BUFFERED) {
            if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_ATTACK)
            || ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_SMASH) {
                VarModule::on_flag(boma.object(), vars::snake::instance::KNIFE_COMBO_IS_BUFFERED);
            }
        }
        if VarModule::is_flag(boma.object(), vars::snake::instance::KNIFE_COMBO_ENABLE)
        && VarModule::is_flag(boma.object(), vars::snake::instance::KNIFE_COMBO_IS_BUFFERED) {
            VarModule::off_flag(boma.object(), vars::snake::instance::KNIFE_COMBO_ENABLE);
            VarModule::off_flag(boma.object(), vars::snake::instance::KNIFE_COMBO_IS_BUFFERED);
            if VarModule::get_int(boma.object(), vars::snake::instance::KNIFE_COMBO_COUNT) == 0 {
                VarModule::set_int(boma.object(), vars::snake::instance::KNIFE_COMBO_COUNT, 1);
                ControlModule::reset_trigger(boma);
                MotionModule::change_motion(boma, Hash40::new("attack_s4_s2"), 0.0, 1.0, false, 0.0, false, false);
            }
            else {
                MotionModule::change_motion(boma, Hash40::new("attack_s4_s3"), 0.0, 1.0, false, 0.0, false, false);
            }
        }
    }
}

// freeze snake in place when using down taunt on the respawn platform
unsafe fn rebirth_box_handling(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status(*FIGHTER_STATUS_KIND_REBIRTH)
    && boma.is_motion_one_of(&[Hash40::new("appeal_lw_l"), Hash40::new("appeal_lw_r")])
    && MotionModule::frame(boma) as i32 == 37 {
        MotionModule::set_rate(boma, 0.0);
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_AIR,
        *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_JUMP,
        *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_JUMP_AERIAL,
        *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_THROW,
        *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_HI_CUT,
        *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_LW_DROP,
        *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_LW_PRODUCE,
        *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_LW_EXPLODING
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

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    //grab_walk(boma, status_kind, cat[1]);
    fsmash_combo(boma, status_kind);
    grenade_counter_reset(boma, id, status_kind);
    fastfall_specials(fighter);
    rebirth_box_handling(fighter, boma);
}

/*#[weapon_frame( agent = WEAPON_KIND_SNAKE_C4 )]
fn snake_c4_frame(weapon: &mut L2CFighterBase) {
    unsafe {
        if StatusModule::status_kind(weapon.module_accessor) == *WEAPON_SNAKE_C4_STATUS_KIND_ESTABLISH_TARGET{
            if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32)
            || GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
                StatusModule::change_status_request_from_script(weapon.module_accessor, *WEAPON_SNAKE_C4_STATUS_KIND_DROP_FALL, false);
            }
        }
        else if StatusModule::status_kind(weapon.module_accessor) == *WEAPON_SNAKE_C4_STATUS_KIND_DROP_FALL {
            StatusModule::change_status_request_from_script(weapon.module_accessor, *WEAPON_SNAKE_C4_STATUS_KIND_ESTABLISH_TARGET, false);
        }
    }
}*/

pub extern "C" fn snake_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		snake_frame(fighter)
    }
}

pub unsafe fn snake_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, snake_frame_wrapper);
}