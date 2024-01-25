use super::*;
use globals::*;
use utils::game_modes::CustomMode;


#[skyline::hook(replace=StatusModule::change_status_request)]
unsafe fn change_status_request_hook(boma: &mut BattleObjectModuleAccessor, status_kind: i32, arg3: bool) -> u64 {
    let mut next_status = status_kind;

    if boma.is_fighter() {
        // Tether trump logic
        if boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_AIR_LASSO, *FIGHTER_STATUS_KIND_AIR_LASSO_REACH, *FIGHTER_STATUS_KIND_AIR_LASSO_HANG, *FIGHTER_STATUS_KIND_AIR_LASSO_REWIND])
        && [*FIGHTER_STATUS_KIND_CLIFF_CATCH, *FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE, *FIGHTER_STATUS_KIND_CLIFF_WAIT].contains(&next_status) {
            let player_number = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
            let pos = GroundModule::hang_cliff_pos_3f(boma);

            for object_id in util::get_all_active_battle_object_ids() {
                let object = ::utils::util::get_battle_object_from_id(object_id);
                if !object.is_null() {
                    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) == WorkModule::get_int(&mut *(*object).module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID)
                    || VarModule::get_float(object, vars::common::instance::LEDGE_POS_X) == 0.0 {
                        continue;
                    }
    
                    if pos.x == VarModule::get_float(object, vars::common::instance::LEDGE_POS_X) && pos.y == VarModule::get_float(object, vars::common::instance::LEDGE_POS_Y) {
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
            let pos = GroundModule::hang_cliff_pos_3f(boma);

            for object_id in util::get_all_active_battle_object_ids() {
                let object = ::utils::util::get_battle_object_from_id(object_id);
                if !object.is_null() {
                    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) == WorkModule::get_int(&mut *(*object).module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID)
                    || VarModule::get_float(object, vars::common::instance::LEDGE_POS_X) == 0.0 {
                        continue;
                    }
    
                    if pos.x == VarModule::get_float(object, vars::common::instance::LEDGE_POS_X) && pos.y == VarModule::get_float(object, vars::common::instance::LEDGE_POS_Y) {
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

        else if boma.kind() == *FIGHTER_KIND_TRAIL
        && StatusModule::status_kind(boma) == *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_SEARCH
        && next_status == *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_TURN
        && ((!VarModule::is_flag(boma.object(), vars::trail::status::IS_SIDE_SPECIAL_INPUT)
        && !(ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)))
            || VarModule::is_flag(boma.object(), vars::trail::status::STOP_SIDE_SPECIAL)) { 
            next_status = *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_END;
        }
        else if boma.kind() == *FIGHTER_KIND_BAYONETTA
        && StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_SPECIAL_S
        && next_status == *FIGHTER_STATUS_KIND_DAMAGE_FALL {
            next_status = *FIGHTER_STATUS_KIND_FALL;
            clear_buffer = true;
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
        change_status_request_hook,
        change_status_request_from_script_hook
    );
}