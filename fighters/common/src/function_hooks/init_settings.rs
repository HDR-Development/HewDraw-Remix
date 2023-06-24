use super::*;
use globals::*;

//=================================================================
//== StatusModule::init_settings
//=================================================================
#[skyline::hook(replace=StatusModule::init_settings)]
unsafe fn init_settings_hook(boma: &mut BattleObjectModuleAccessor, situation: smash::app::SituationKind, kinetic_type: i32, arg4: u32,
                             ground_cliff_check_kind: smash::app::GroundCliffCheckKind, jostle: bool,
                             keep_flag: i32, keep_int: i32, keep_float: i32, arg10: i32) -> u64 {
    let id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let fighter_kind = boma.kind();
    let status_kind = StatusModule::status_kind(boma);
    let situation_kind = StatusModule::situation_kind(boma);
    let mut cliff_check_kind = ground_cliff_check_kind;
    let mut kinetic_type = kinetic_type.clone();
                                
    // Call edge_slippoffs init_settings
    let fix = super::edge_slipoffs::init_settings_edges(boma, situation, kinetic_type, arg4, ground_cliff_check_kind, jostle, keep_flag, keep_int, keep_float, arg10);

    if boma.is_fighter() {
        
        if boma.is_prev_situation(*SITUATION_KIND_AIR)
        && ( situation.0 == *SITUATION_KIND_GROUND
            || (boma.is_situation(*SITUATION_KIND_GROUND) && situation.0 == *SITUATION_KIND_NONE) )
        {
            if kinetic_type == *FIGHTER_KINETIC_TYPE_MOTION {
                kinetic_type = *FIGHTER_KINETIC_TYPE_MOTION_IGNORE_NORMAL;
            }
        }
        // Resets your airtime counter when leaving the below statuses
        // Prevents ECB from shifting on f1 after an "ignored" status (those defined below)
        if boma.is_prev_status_one_of(&[
            *FIGHTER_STATUS_KIND_DEMO,
            *FIGHTER_STATUS_KIND_ENTRY,
            *FIGHTER_STATUS_KIND_CAPTURE_PULLED,
            *FIGHTER_STATUS_KIND_CAPTURE_WAIT,
            *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE,
            *FIGHTER_STATUS_KIND_THROWN,
            *FIGHTER_STATUS_KIND_CATCHED_GANON,
            *FIGHTER_STATUS_KIND_CATCHED_AIR_GANON,
            *FIGHTER_STATUS_KIND_CATCHED_REFLET,
            *FIGHTER_STATUS_KIND_CATCHED_RIDLEY,
            *FIGHTER_STATUS_KIND_CAPTURE_JACK_WIRE,
            *FIGHTER_STATUS_KIND_CAPTURE_MASTER_SWORD])
        && situation.0 == *SITUATION_KIND_AIR
        {
            WorkModule::set_int(boma, 0, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR);
        }

        // Walk through other fighters
        JostleModule::set_team(boma, 0);

        // clear platform drop input when entering airdodge (to avoid buffering waveland platdrop with the same down input as the actual waveland)
        if [*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE, *FIGHTER_STATUS_KIND_JUMP_SQUAT].contains(&status_kind) {
            VarModule::off_flag(boma.object(), vars::common::instance::ENABLE_WAVELAND_PLATDROP);
        }

        // Occupy ledge on ledgegrab
        if boma.is_status_one_of(&[
            *FIGHTER_STATUS_KIND_CLIFF_CATCH,
            *FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE,
            *FIGHTER_STATUS_KIND_CLIFF_WAIT]) {
            VarModule::set_vec3(boma.object(), vars::common::instance::LEDGE_POS, GroundModule::hang_cliff_pos_3f(boma));
        }

        // Repeated tilt scaling; UNUSED
        /*
        if [*FIGHTER_KIND_RYU, *FIGHTER_KIND_KEN, *FIGHTER_KIND_DOLLY].contains(&fighter_kind) {
            VarModule::off_flag(boma.object(), vars::common::status::REPEAT_INCREMENTED);
            if status_kind != *FIGHTER_STATUS_KIND_ATTACK_HI3 {
                if VarModule::get_int(boma.object(), vars::common::REPEAT_NUM_HI) > 0 {
                    VarModule::set_int(boma.object(), vars::common::REPEAT_NUM_HI, 0);
                }
            }
            if ![*FIGHTER_STATUS_KIND_ATTACK_LW3,
                *FIGHTER_STATUS_KIND_SQUAT,
                *FIGHTER_STATUS_KIND_SQUAT_WAIT].contains(&status_kind) {
                    if VarModule::get_int(boma.object(), vars::common::REPEAT_NUM_LW) > 0 {
                        VarModule::set_int(boma.object(), vars::common::REPEAT_NUM_LW, 0);
                    }
            }
        }
        */

        //Sword trails
        if (boma.kind() == *FIGHTER_KIND_ROY || boma.kind() == *FIGHTER_KIND_CHROM) 
        && VarModule::is_flag(boma.object(), vars::roy::instance::TRAIL_EFFECT) {
            EffectModule::kill_joint_id(boma, Hash40::new("sword1"), false, false);
            if fighter_kind == *FIGHTER_KIND_ROY {
                EffectModule::req_follow(boma, Hash40::new("roy_fire_small"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 1.0, false, 0, 0, 0, 0, 0, false, false);
            }
            else if fighter_kind == *FIGHTER_KIND_CHROM {
                EffectModule::req_follow(boma, Hash40::new("chrom_sword"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 1.0, false, 0, 0, 0, 0, 0, false, false);
            }

            if [*FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_ENTRY].contains(&status_kind) {
                VarModule::off_flag(boma.object(), vars::roy::instance::TRAIL_EFFECT);
                EffectModule::kill_joint_id(boma, Hash40::new("sword1"), false, false);
            }
        }

        // Set GroundCliffCheckKind here to pass into init_settings
        
        if ((boma.kind() == *FIGHTER_KIND_RYU || boma.kind() == *FIGHTER_KIND_KEN)
            && boma.is_status_one_of(&[
                *FIGHTER_STATUS_KIND_SPECIAL_S,
                *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND,
                *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP,
                *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_END]))
        || (boma.kind() == *FIGHTER_KIND_FALCO
            && boma.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI))
        || (boma.kind() == *FIGHTER_KIND_REFLET
            && boma.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI))
        || (boma.kind() == *FIGHTER_KIND_WOLF
            && boma.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI))
        {
            cliff_check_kind = app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
        }

        VarModule::off_flag(boma.object(), vars::common::instance::IS_MOTION_BASED_ATTACK);

        if boma.is_prev_status(*FIGHTER_STATUS_KIND_SWALLOWED_DRINK) {
            VisibilityModule::set_whole(boma, true);
        }

        VarModule::off_flag(boma.object(), vars::common::instance::EDGE_SLIPPABLE_STATUS);
    }

    // VarModule Status Variable reset checks
    // This makes the assumption that if the KEEP_FLAG is not NONE, you want to clear the
    // status variable array for that data type. Because Smash shares its space between
    // INT and INT64, I have included both of them under a single check.
    let object = boma.object();
    if VarModule::has_var_module(object) {
        let mut mask = 0;
        if keep_flag != *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG {
            mask += VarModule::RESET_STATUS_FLAG;
        }
        if keep_int != *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT {
            mask += VarModule::RESET_STATUS_INT;
            mask += VarModule::RESET_STATUS_INT64;
        }
        if keep_float != *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT {
            mask += VarModule::RESET_STATUS_FLOAT;
        }
        VarModule::reset(object, mask);
    }

    original!()(boma, situation, kinetic_type, fix, cliff_check_kind, jostle, keep_flag, keep_int, keep_float, arg10)
}

pub fn install() {
    skyline::install_hooks!(
        init_settings_hook,
    );
}
