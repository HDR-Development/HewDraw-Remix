use super::*;
use globals::*;

#[skyline::hook(replace=StatusModule::change_status_request)]
unsafe fn change_status_request_hook(boma: &mut BattleObjectModuleAccessor, status_kind: i32, arg3: bool) -> u64 {
    let mut next_status = status_kind;

    if boma.is_fighter() {
        // Tether trump logic
        if boma.is_status(*FIGHTER_STATUS_KIND_AIR_LASSO_REWIND)
        && [*FIGHTER_STATUS_KIND_CLIFF_CATCH, *FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE, *FIGHTER_STATUS_KIND_CLIFF_WAIT].contains(&next_status) {
            let player_number = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
            let pos = GroundModule::hang_cliff_pos_3f(boma);

            for i in 0..8 {
                if let Some(object_id) = ::utils::util::get_active_battle_object_id_from_entry_id(i) {
                    let object = ::utils::util::get_battle_object_from_id(object_id);
                    if !object.is_null() {
                        if i == player_number || VarModule::get_float(object, vars::common::LEDGE_POS_X) == 0.0 {
                            continue;
                        }
    
                        if pos.x == VarModule::get_float(object, vars::common::LEDGE_POS_X) && pos.y == VarModule::get_float(object, vars::common::LEDGE_POS_Y) {
                            next_status = *FIGHTER_STATUS_KIND_CLIFF_ROBBED;
                        }
    
                        let module_accessor = &mut *(*object).module_accessor;
                        if module_accessor.kind() == *FIGHTER_KIND_POPO {
                            let nana_object_id = WorkModule::get_int(module_accessor, *FIGHTER_POPO_INSTANCE_WORK_ID_INT_PARTNER_OBJECT_ID) as u32;
                            let object = ::utils::util::get_battle_object_from_id(nana_object_id);
                            if !object.is_null() {
                                if pos.x == VarModule::get_float(object, vars::common::LEDGE_POS_X) && pos.y == VarModule::get_float(object, vars::common::LEDGE_POS_Y) {
                                    next_status = *FIGHTER_STATUS_KIND_CLIFF_ROBBED;
                                }
                            }
                        }
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

    if boma.is_fighter() {
        // Unoccupy ledge on ledge release/end of ledge option
        if boma.is_status_one_of(&[
            *FIGHTER_STATUS_KIND_CLIFF_CATCH,
            *FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE,
            *FIGHTER_STATUS_KIND_CLIFF_WAIT,
            *FIGHTER_STATUS_KIND_CLIFF_ATTACK,
            *FIGHTER_STATUS_KIND_CLIFF_CLIMB,
            *FIGHTER_STATUS_KIND_CLIFF_ESCAPE,
            *FIGHTER_STATUS_KIND_CLIFF_JUMP1,
            *FIGHTER_STATUS_KIND_CLIFF_JUMP2,
            *FIGHTER_STATUS_KIND_CLIFF_JUMP3]) 
        && ![*FIGHTER_STATUS_KIND_CLIFF_ATTACK,
            *FIGHTER_STATUS_KIND_CLIFF_CLIMB,
            *FIGHTER_STATUS_KIND_CLIFF_ESCAPE,
            *FIGHTER_STATUS_KIND_CLIFF_JUMP1,
            *FIGHTER_STATUS_KIND_CLIFF_JUMP2,
            *FIGHTER_STATUS_KIND_CLIFF_JUMP3].contains(&next_status) {
            VarModule::set_vec3(boma.object(), vars::common::LEDGE_POS, Vector3f {x: 0.0, y: 0.0, z: 0.0});
        }

        if boma.kind() == *FIGHTER_KIND_TRAIL
        && StatusModule::status_kind(boma) == *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_SEARCH
        && next_status == *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_TURN
        && ((!VarModule::is_flag(boma.object(), vars::trail::IS_SIDE_SPECIAL_INPUT)
        && !(ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)))
            || VarModule::is_flag(boma.object(), vars::trail::STOP_SIDE_SPECIAL)) { 
            next_status = *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_END;
        }
        if boma.kind() == *FIGHTER_KIND_KOOPAJR
        && StatusModule::status_kind(boma) == *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_S_DASH
        && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND
        && next_status == *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_S_JUMP {
            next_status = *FIGHTER_STATUS_KIND_JUMP_SQUAT;
        }
    }
    original!()(boma, next_status, arg3)
}

pub fn install() {
    skyline::install_hooks!(
        change_status_request_hook,
        change_status_request_from_script_hook
    );
}