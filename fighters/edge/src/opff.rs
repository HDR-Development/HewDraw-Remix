// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn sword_length(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status(*FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_CHARGED_RUSH) {
        let sword_scale = Vector3f{x: 0.7, y: 1.0, z: 1.0};
        ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("swordl1"), &sword_scale);
        ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("swordr1"), &sword_scale);
    }
    else {
        let long_sword_scale = Vector3f{x: 0.95, y: 1.0, z: 1.0};
        ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("swordl1"), &long_sword_scale);
        ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("swordr1"), &long_sword_scale);
        //println!("Sephiroth Success! Sephiroth's Fighter Kind Number: {}", *FIGHTER_KIND_EDGE);
    }
}

// Limit Blade Rush jump cancel on hit
unsafe fn limit_blade_rush_jc(boma: &mut BattleObjectModuleAccessor, cat1: i32, status_kind: i32, situation_kind: i32) {
    //println!("Sephiroth status kind: {}", status_kind);
    if status_kind == *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH && WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED) && situation_kind == *SITUATION_KIND_GROUND {
        //println!("Limit Blade Rush");
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !boma.is_in_hitlag() {
            //println!("========== Limit Blade Rush hit!");
            boma.check_jump_cancel(false);
        }
    }
}

unsafe fn nspecial_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32) {
    //PM-like neutral-b canceling
    if status_kind == *FIGHTER_EDGE_STATUS_KIND_SPECIAL_N_CANCEL {
        if situation_kind == *SITUATION_KIND_AIR {
            if WorkModule::get_int(boma, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS) == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
                WorkModule::set_int(boma, *STATUS_KIND_NONE, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS);
                ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
            }
        }
    }
}

// Change jab combo to be a single hit like Ganon, using jab 3
unsafe fn jab3_as_jab1(boma: &mut BattleObjectModuleAccessor, motion_kind: u64) {
    if motion_kind == hash40("attack_11") {
        MotionModule::change_motion(boma, Hash40::new("attack_13"), 0.0, 1.0, false, 0.0, false, false);
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_EDGE_STATUS_KIND_SPECIAL_N_SHOOT,
        *FIGHTER_EDGE_STATUS_KIND_SPECIAL_N_CANCEL,
        *FIGHTER_EDGE_STATUS_KIND_SPECIAL_S_CHARGE,
        *FIGHTER_EDGE_STATUS_KIND_SPECIAL_S_SHOOT,
        *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_END,
        *FIGHTER_EDGE_STATUS_KIND_SPECIAL_LW_END
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
    sword_length(boma);
    limit_blade_rush_jc(boma, cat[0], status_kind, situation_kind);
    nspecial_cancels(boma, status_kind, situation_kind, cat[1]);
    //jab3_as_jab1(boma, motion_kind);
    fastfall_specials(fighter);
}

#[utils::macros::opff(FIGHTER_KIND_EDGE )]
pub fn edge_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		edge_frame(fighter)
    }
}

pub unsafe fn edge_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}
