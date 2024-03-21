use super::*;
use globals::*;
use utils::game_modes::CustomMode;

//=================================================================
//== StatusModule::init_settings
//=================================================================
#[skyline::hook(replace=StatusModule::init_settings)]
unsafe fn init_settings_hook(boma: &mut BattleObjectModuleAccessor, mut situation: smash::app::SituationKind, kinetic_type: i32, arg4: u32,
                             ground_cliff_check_kind: smash::app::GroundCliffCheckKind, jostle: bool,
                             keep_flag: i32, keep_int: i32, keep_float: i32, arg10: i32) -> u64 {
    let id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let fighter_kind = boma.kind();
    let status_kind = StatusModule::status_kind(boma);
    let situation_kind = StatusModule::situation_kind(boma);
    let mut cliff_check_kind = ground_cliff_check_kind;
    let mut kinetic_type = kinetic_type.clone();
                                
    // Call edge_slippoffs init_settings
    let fix = super::ground::init_settings_edges(boma, situation, kinetic_type, arg4, ground_cliff_check_kind, jostle, keep_flag, keep_int, keep_float, arg10);

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
        // if [*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE, *FIGHTER_STATUS_KIND_JUMP_SQUAT].contains(&status_kind) {
        //     ControlModule::reset_flick_y(boma);
        //     // VarModule::off_flag(boma.object(), vars::common::instance::ENABLE_WAVELAND_PLATDROP);
        // }

        if boma.is_prev_status_one_of(&[*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE, *FIGHTER_STATUS_KIND_JUMP_SQUAT])
        && boma.is_status(*FIGHTER_STATUS_KIND_LANDING)
        && GroundModule::is_passable_ground(boma) {
            ControlModule::reset_flick_y(boma);
        }

        // Occupy ledge on ledgegrab
        if boma.is_status_one_of(&[
            *FIGHTER_STATUS_KIND_CLIFF_CATCH,
            *FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE,
            *FIGHTER_STATUS_KIND_CLIFF_WAIT]
        ) {
            let cliff_id = GroundModule::get_cliff_id_uint32(boma);
            VarModule::set_int(boma.object(), vars::common::instance::LEDGE_ID, cliff_id as i32);
        }

        // heavy item pickup should keep momentum and be affected by gravity in the air
        if boma.is_status(*FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP) && boma.is_situation(*SITUATION_KIND_AIR) {
            // change the kinetic type to the regular air type
            kinetic_type = *FIGHTER_KINETIC_TYPE_FALL;
            situation.0 = *SITUATION_KIND_NONE;
        }

        if boma.kind() == *FIGHTER_KIND_DONKEY && boma.is_situation(*SITUATION_KIND_AIR) 
            && boma.is_status_one_of(&[
            *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_START,
            *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_WAIT,
            *FIGHTER_STATUS_KIND_THROW,]) {
                kinetic_type = *FIGHTER_KINETIC_TYPE_FALL;
                situation.0 = *SITUATION_KIND_NONE;
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

        if boma.is_prev_status(*FIGHTER_STATUS_KIND_SWALLOWED_DRINK) {
            VisibilityModule::set_whole(boma, true);
        }
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

#[skyline::hook(replace=StatusModule::change_status_request)]
unsafe fn change_status_request_hook(boma: &mut BattleObjectModuleAccessor, status_kind: i32, arg3: bool) -> u64 {
    let mut next_status = status_kind;

    if boma.is_fighter() {
        // Tether trump logic
        if boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_AIR_LASSO, *FIGHTER_STATUS_KIND_AIR_LASSO_REACH, *FIGHTER_STATUS_KIND_AIR_LASSO_HANG, *FIGHTER_STATUS_KIND_AIR_LASSO_REWIND])
        && [*FIGHTER_STATUS_KIND_CLIFF_CATCH, *FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE, *FIGHTER_STATUS_KIND_CLIFF_WAIT].contains(&next_status) {
            let player_number = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
            let cliff_id = GroundModule::get_cliff_id_uint32(boma);

            for object_id in util::get_all_active_battle_object_ids() {
                let object = ::utils::util::get_battle_object_from_id(object_id);
                if !object.is_null() {
                    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) == WorkModule::get_int(&mut *(*object).module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) {
                        continue;
                    }
    
                    if VarModule::get_int(object, vars::common::instance::LEDGE_ID) == cliff_id as i32 {
                        next_status = *FIGHTER_STATUS_KIND_CLIFF_ROBBED;
                    }
                }
            }
        }
    } else if boma.is_item() {
        // handle barrel item not breaking when it hits someone
        if boma.kind() == *ITEM_KIND_BARREL {
            //println!("Barrel is requesting change into: {:x}", next_status);
            if next_status == *ITEM_STATUS_KIND_BORN || next_status == *ITEM_STATUS_KIND_LOST {
                let bounce_mul = Vector3f { x: -0.25, y: -0.25, z: -0.25 };
                KineticModule::mul_speed(boma, &bounce_mul, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                PostureModule::reverse_lr(boma);
                AttackModule::clear_all(boma);

                // instead change into fall
                next_status = *ITEM_STATUS_KIND_FALL;

                // set hit team to none for now?
                TeamModule::set_hit_team(boma, *TEAM_NONE);
            }
        }
    }
    original!()(boma, next_status, arg3)
}

#[skyline::hook(replace=StatusModule::change_status_request_from_script)]
unsafe fn change_status_request_from_script_hook(boma: &mut BattleObjectModuleAccessor, status_kind: i32, arg3: bool) -> u64 {
    let mut next_status = status_kind;
    let mut clear_buffer = arg3;

    if boma.is_fighter() {
        match utils::game_modes::get_custom_mode() {
            Some(modes) => {
                if modes.contains(&CustomMode::Smash64Mode) {
                    if [*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_PASSIVE_CEIL, *FIGHTER_STATUS_KIND_PASSIVE_WALL, *FIGHTER_STATUS_KIND_PASSIVE_WALL_JUMP].contains(&next_status) {
                        return 0;
                    }
                }
            },
            _ => {}
        }
        // Allow buffered wavedashes when Shield is pressed at any time within Jump input's buffer window
        if next_status == *FIGHTER_STATUS_KIND_JUMP_SQUAT {
            if boma.is_cat_flag(Cat1::AirEscape) && !boma.is_cat_flag(Cat1::AttackN) {
                match utils::game_modes::get_custom_mode() {
                    Some(modes) => {
                        if !modes.contains(&CustomMode::Smash64Mode) {
                            VarModule::on_flag(boma.object(), vars::common::instance::ENABLE_AIR_ESCAPE_JUMPSQUAT);
                        }
                    },
                    _ => { VarModule::on_flag(boma.object(), vars::common::instance::ENABLE_AIR_ESCAPE_JUMPSQUAT); }
                }
            }
        }
        // Clears buffer when sliding off an edge in a damaged state, to prevent accidental buffered aerials/airdodges (common on missed techs)
        else if [*FIGHTER_STATUS_KIND_DOWN,
            *FIGHTER_STATUS_KIND_DOWN_WAIT,
            *FIGHTER_STATUS_KIND_SLIP_WAIT,
            *FIGHTER_STATUS_KIND_DAMAGE].contains(&StatusModule::status_kind(boma))
        && next_status == *FIGHTER_STATUS_KIND_FALL {
            clear_buffer = true;
        }
        // Tether trump logic
        else if boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_AIR_LASSO, *FIGHTER_STATUS_KIND_AIR_LASSO_REACH, *FIGHTER_STATUS_KIND_AIR_LASSO_HANG, *FIGHTER_STATUS_KIND_AIR_LASSO_REWIND])
        && [*FIGHTER_STATUS_KIND_CLIFF_CATCH, *FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE, *FIGHTER_STATUS_KIND_CLIFF_WAIT].contains(&next_status) {
            let player_number = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
            let cliff_id = GroundModule::get_cliff_id_uint32(boma);

            for object_id in util::get_all_active_battle_object_ids() {
                let object = ::utils::util::get_battle_object_from_id(object_id);
                if !object.is_null() {
                    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) == WorkModule::get_int(&mut *(*object).module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) {
                        continue;
                    }
    
                    if VarModule::get_int(object, vars::common::instance::LEDGE_ID) == cliff_id as i32 {
                        next_status = *FIGHTER_STATUS_KIND_CLIFF_ROBBED;
                    }
                }
            }
        }
        else if boma.is_status(*FIGHTER_STATUS_KIND_DAMAGE_AIR)
        && next_status == *FIGHTER_STATUS_KIND_LANDING
        && boma.motion_frame() < 1.0 {
            VarModule::on_flag(boma.object(), vars::common::instance::IS_CC_NON_TUMBLE);
        }

        else if boma.kind() == *FIGHTER_KIND_TRAIL {
            if StatusModule::status_kind(boma) == *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_SEARCH
            && next_status == *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_TURN
            && ((!VarModule::is_flag(boma.object(), vars::trail::status::IS_SIDE_SPECIAL_INPUT)
            && !(ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)))
                || VarModule::is_flag(boma.object(), vars::trail::status::STOP_SIDE_SPECIAL)) { 
                next_status = *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_END;
            }
            // prevent sora from immediately acting out of the down smash bounce 
            if boma.is_status(*FIGHTER_STATUS_KIND_CLIFF_JUMP2)
            && !boma.is_prev_status(*FIGHTER_STATUS_KIND_CLIFF_JUMP1)
            && boma.status_frame() < 16 {
                return 0;
            }
        }
        else if boma.kind() == *FIGHTER_KIND_KOOPAJR
        && StatusModule::status_kind(boma) == *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_S_DASH
        && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND
        && next_status == *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_S_JUMP {
            next_status = *FIGHTER_STATUS_KIND_JUMP_SQUAT;
        }
        else if boma.kind() == *FIGHTER_KIND_REFLET
        && StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_SPECIAL_HI
        && next_status == *FIGHTER_STATUS_KIND_FALL_SPECIAL
        && !VarModule::is_flag(boma.object(), vars::reflet::instance::UP_SPECIAL_FREEFALL) {
            next_status = *FIGHTER_STATUS_KIND_FALL;
        }
        else if boma.kind() == *FIGHTER_KIND_RICHTER
        && StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_SPECIAL_HI
        && next_status == *FIGHTER_STATUS_KIND_FALL_SPECIAL
        && !VarModule::is_flag(boma.object(), vars::richter::instance::UP_SPECIAL_FREEFALL) {
            next_status = *FIGHTER_STATUS_KIND_FALL;
        }
        else if boma.kind() == *FIGHTER_KIND_MEWTWO 
        && StatusModule::status_kind(boma) == *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_3
        && next_status == *FIGHTER_STATUS_KIND_FALL_SPECIAL
        && VarModule::is_flag(boma.object(), vars::common::instance::UP_SPECIAL_CANCEL)
        && !VarModule::is_flag(boma.object(), vars::mewtwo::instance::UP_SPECIAL_FREEFALL) {
            next_status = *FIGHTER_STATUS_KIND_FALL;
        }
        else if boma.kind() == *FIGHTER_KIND_PALUTENA 
        && StatusModule::status_kind(boma) == *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_3
        && next_status == *FIGHTER_STATUS_KIND_FALL_SPECIAL
        && VarModule::is_flag(boma.object(), vars::common::instance::UP_SPECIAL_CANCEL)
        && !VarModule::is_flag(boma.object(), vars::palutena::instance::UP_SPECIAL_FREEFALL) {
            next_status = *FIGHTER_STATUS_KIND_FALL;
        }
        // Transition into regular fall when attempting to jump off of Wario bike when out of jumps
        else if boma.kind() == *FIGHTER_KIND_WARIO
        && StatusModule::status_kind(boma) == *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_ESCAPE_START
        && next_status == *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_ESCAPE
        && boma.get_num_used_jumps() >= boma.get_jump_count_max() {
            next_status = *FIGHTER_STATUS_KIND_DAMAGE_FALL;
            clear_buffer = true;
        }
        else if boma.kind() == *FIGHTER_KIND_KOOPAJR {
            // Prevent jumping out of Clown Kart Dash when out of jumps
            if boma.is_status_one_of(&[*FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_S_DASH, *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_S_SPIN_TURN])
            && next_status == *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_S_JUMP
            && boma.get_num_used_jumps() >= boma.get_jump_count_max() {
                return 0;
            }
            // Prevent airdodging out of upB for first 10 frames
            if boma.is_status(*FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_SHOOT)
            && next_status == *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_ESCAPE
            && boma.status_frame() < 10 {
                return 0;
            }
        }
        // Prevent jumping out of Splat Roller when out of jumps
        else if boma.kind() == *FIGHTER_KIND_INKLING
        && boma.is_status_one_of(&[*FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_RUN, *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_WALK])
        && next_status == *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_JUMP_END
        && boma.get_num_used_jumps() >= boma.get_jump_count_max() {
            WorkModule::off_flag(boma, *FIGHTER_INKLING_STATUS_SPECIAL_S_FLAG_JUMP_END);
            return 0;
        }
        else if boma.kind() == *FIGHTER_KIND_DAISY {
            // Prevents Daisy from floating out of upB
            if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_SPECIAL_HI
            && next_status == *FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_FALL {
                next_status = *FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_AIR_END;
            }
            // Prevents Daisy from being able to use both aerial jumps immediately after one another
            else if boma.is_status(*FIGHTER_STATUS_KIND_JUMP_AERIAL)
            && next_status == *FIGHTER_STATUS_KIND_JUMP_AERIAL
            && {
                let triple_jump_lockout_frame = ParamModule::get_int(boma.object(), ParamType::Agent, "triple_jump_lockout_frame");
                boma.status_frame() < triple_jump_lockout_frame
            } {
                return 0;
            }
        }
        // Prevent jumping out of Minecart when out of jumps
        else if boma.kind() == *FIGHTER_KIND_PICKEL
        && next_status == *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_S_JUMP
        && boma.get_num_used_jumps() >= boma.get_jump_count_max() {
            return 0;
        }
        // Stubs vanilla Popgun cancel behavior
        else if boma.kind() == *FIGHTER_KIND_DIDDY
        && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_N_CHARGE])
        && [*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_FALL].contains(&next_status) {
            return 0;
        }
        else if boma.kind() == *FIGHTER_KIND_KIRBY
        && (WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == FIGHTER_KIND_DIDDY)
        && boma.is_status_one_of(&[*FIGHTER_KIRBY_STATUS_KIND_DIDDY_SPECIAL_N, *FIGHTER_KIRBY_STATUS_KIND_DIDDY_SPECIAL_N_CHARGE])
        && [*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_FALL].contains(&next_status) {
            return 0;
        }
        // Allows Clay Pigeon smash input to work properly
        else if boma.kind() == *FIGHTER_KIND_DUCKHUNT
        && next_status == *FIGHTER_STATUS_KIND_SPECIAL_S {
            clear_buffer = false;
        }
    }
    original!()(boma, next_status, clear_buffer)
}

pub fn install() {
    skyline::install_hooks!(
        init_settings_hook,
        change_status_request_hook,
        change_status_request_from_script_hook
    );
}