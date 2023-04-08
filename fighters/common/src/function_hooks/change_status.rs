use super::*;
use globals::*;

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
    }
    original!()(boma, next_status, arg3)
}

#[skyline::hook(replace=StatusModule::change_status_request_from_script)]
unsafe fn change_status_request_from_script_hook(boma: &mut BattleObjectModuleAccessor, status_kind: i32, arg3: bool) -> u64 {
    let mut next_status = status_kind;
    let mut clear_buffer = arg3;

    if boma.is_fighter() {
        // Allow buffered wavedashes when Shield is pressed at any time within Jump input's buffer window
        if next_status == *FIGHTER_STATUS_KIND_JUMP_SQUAT {
            if boma.is_cat_flag(Cat1::AirEscape) && !boma.is_cat_flag(Cat1::AttackN) {
                VarModule::on_flag(boma.object(), vars::common::instance::ENABLE_AIR_ESCAPE_JUMPSQUAT);
            }
        }
        // Clears buffer when sliding off an edge in a damaged state, to prevent accidental buffered aerials/airdodges (common on missed techs)
        if [*FIGHTER_STATUS_KIND_DOWN,
            *FIGHTER_STATUS_KIND_DOWN_WAIT,
            *FIGHTER_STATUS_KIND_SLIP_WAIT,
            *FIGHTER_STATUS_KIND_DAMAGE].contains(&StatusModule::status_kind(boma))
        && next_status == *FIGHTER_STATUS_KIND_FALL {
            clear_buffer = true;
        }
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
        if boma.is_status(*FIGHTER_STATUS_KIND_DAMAGE_AIR)
        && next_status == *FIGHTER_STATUS_KIND_LANDING
        && boma.motion_frame() < 1.0 {
            VarModule::on_flag(boma.object(), vars::common::instance::IS_CC_NON_TUMBLE);
        }

        if boma.kind() == *FIGHTER_KIND_TRAIL
        && StatusModule::status_kind(boma) == *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_SEARCH
        && next_status == *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_TURN
        && ((!VarModule::is_flag(boma.object(), vars::trail::status::IS_SIDE_SPECIAL_INPUT)
        && !(ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)))
            || VarModule::is_flag(boma.object(), vars::trail::status::STOP_SIDE_SPECIAL)) { 
            next_status = *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_END;
        }
        if boma.kind() == *FIGHTER_KIND_KOOPAJR
        && StatusModule::status_kind(boma) == *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_S_DASH
        && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND
        && next_status == *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_S_JUMP {
            next_status = *FIGHTER_STATUS_KIND_JUMP_SQUAT;
        }
        if boma.kind() == *FIGHTER_KIND_REFLET
        && StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_SPECIAL_HI
        && next_status == *FIGHTER_STATUS_KIND_FALL_SPECIAL {
            next_status = *FIGHTER_STATUS_KIND_FALL;
        }
        if boma.kind() == *FIGHTER_KIND_MEWTWO 
        && StatusModule::status_kind(boma) == *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_3
        && next_status == *FIGHTER_STATUS_KIND_FALL_SPECIAL
        && VarModule::is_flag(boma.object(), vars::common::instance::UP_SPECIAL_CANCEL) {
            next_status = *FIGHTER_STATUS_KIND_FALL;
        }
        if boma.kind() == *FIGHTER_KIND_PALUTENA 
        && StatusModule::status_kind(boma) == *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_3
        && next_status == *FIGHTER_STATUS_KIND_FALL_SPECIAL
        && VarModule::is_flag(boma.object(), vars::common::instance::UP_SPECIAL_CANCEL) {
            next_status = *FIGHTER_STATUS_KIND_FALL;
        }
        if boma.kind() == *FIGHTER_KIND_WARIO
        && StatusModule::status_kind(boma) == *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_ESCAPE_START
        && next_status == *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_ESCAPE
        && boma.get_num_used_jumps() >= boma.get_jump_count_max() {
            next_status = *FIGHTER_STATUS_KIND_FALL;
            clear_buffer = true;
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