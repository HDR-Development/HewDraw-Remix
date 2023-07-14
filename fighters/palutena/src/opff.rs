// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_3,
        *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_LW_ATTACK,
        *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_LW_REFLECT
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    palutena_teleport_wall_ride(fighter, boma, id, status_kind, situation_kind, cat[0]);
    actionable_teleport_air(fighter, boma, id, status_kind, situation_kind, frame);
    aegis_reflector_timer(fighter, boma, id);
    aegis_reflector_reset(fighter, id, status_kind);
    aegis_reflector_training(fighter, id, status_kind);
    dj_upB_jump_refresh(fighter);
    fastfall_specials(fighter);
}

unsafe fn actionable_teleport_air(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, frame: f32) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI
    && boma.status_frame() == 1 {
        VarModule::off_flag(boma.object(), vars::palutena::instance::GROUNDED_TELEPORT);
        if situation_kind == *SITUATION_KIND_GROUND {
            VarModule::on_flag(boma.object(), vars::palutena::instance::GROUNDED_TELEPORT);
        }
    }
    // Allows Palutena to turnaround based on stick position when reappearing
    if status_kind == *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_2 && MotionModule::is_end(boma) {
        if boma.get_num_used_jumps() < boma.get_jump_count_max() {
            PostureModule::set_stick_lr(boma, 0.0);
            PostureModule::update_rot_y_lr(boma);
        }
    }
    // Actionability when double jump isn't burned
    if status_kind == *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_3 && situation_kind == *SITUATION_KIND_AIR && frame > 7.0 {
        if boma.get_num_used_jumps() < boma.get_jump_count_max() {
            VarModule::on_flag(boma.object(), vars::common::instance::UP_SPECIAL_CANCEL);
            CancelModule::enable_cancel(boma);
            // Consume double jump, except when Teleport is initiated on ground
            if !VarModule::is_flag(boma.object(), vars::palutena::instance::GROUNDED_TELEPORT) {
                WorkModule::inc_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
            }
        }
    }
}

unsafe fn dj_upB_jump_refresh(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_JUMP_AERIAL) {
        // If first 3 frames of dj
        if fighter.status_frame() <= 3 {
            VarModule::on_flag(fighter.battle_object, vars::palutena::instance::UP_SPECIAL_JUMP_REFRESH);
        }
        else {
            VarModule::off_flag(fighter.battle_object, vars::palutena::instance::UP_SPECIAL_JUMP_REFRESH);
        }
    }
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI)
    && fighter.is_prev_status(*FIGHTER_STATUS_KIND_JUMP_AERIAL)
    && VarModule::is_flag(fighter.battle_object, vars::palutena::instance::UP_SPECIAL_JUMP_REFRESH) {
        // Grants 1 extra jump if all jumps used up
        if fighter.get_num_used_jumps() == fighter.get_jump_count_max() {
            WorkModule::dec_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        }
        VarModule::off_flag(fighter.battle_object, vars::palutena::instance::UP_SPECIAL_JUMP_REFRESH);
    }
}

pub unsafe fn palutena_teleport_wall_ride(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, cat1: i32) {
    // Wall Ride momentum fixes
    let touch_right = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_RIGHT_SIDE as u32);
    let touch_left = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_LEFT_SIDE as u32);

    if boma.is_status(*FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_2) {
        let touch_normal_y_left = GroundModule::get_touch_normal_y(fighter.module_accessor, *GROUND_TOUCH_FLAG_LEFT_SIDE as u32);
        let touch_normal_y_right = GroundModule::get_touch_normal_y(fighter.module_accessor, *GROUND_TOUCH_FLAG_RIGHT_SIDE as u32);
        if (touch_right && touch_normal_y_right != 0.0)
        || (touch_left && touch_normal_y_left != 0.0)
        {
            let init_speed_y = VarModule::get_float(boma.object(), vars::common::status::TELEPORT_INITIAL_SPEED_Y);

            if init_speed_y > 0.0 {
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, init_speed_y);
                app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
            }
        }
    }
    else if boma.is_status(*FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_3) {
        if touch_right || touch_left {
            if KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) > 0.0 {
                let wall_ride = Vector3f{x: 0.0, y: 1.0, z: 1.0};
                KineticModule::mul_speed(boma, &wall_ride, *FIGHTER_KINETIC_ENERGY_ID_STOP);
            }
        }
    }
}

extern "Rust" {
    fn gimmick_flash(boma: &mut BattleObjectModuleAccessor);
}


// Aegis Reflector Timer Count
unsafe fn aegis_reflector_timer(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize) {
    let gimmick_timerr = VarModule::get_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER);
    if gimmick_timerr > 0 && gimmick_timerr < 901 {
        if gimmick_timerr > 899 {
            VarModule::set_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER, 0);
            gimmick_flash(boma);
        } else {
            VarModule::set_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER, gimmick_timerr + 1);
        }
    }
}

// Aegis Reflector Timer Death Reset
unsafe fn aegis_reflector_reset(fighter: &mut L2CFighterCommon, id: usize, status_kind: i32) {
    if [*FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_REBIRTH,
        *FIGHTER_STATUS_KIND_WIN,
        *FIGHTER_STATUS_KIND_LOSE,
        *FIGHTER_STATUS_KIND_ENTRY].contains(&status_kind) {
        VarModule::set_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER, 0);
    }
}

// Training Mode Aegis Reflector timer taunt reset
unsafe fn aegis_reflector_training(fighter: &mut L2CFighterCommon, id: usize, status_kind: i32) {
    if is_training_mode() {
        if status_kind == *FIGHTER_STATUS_KIND_APPEAL || !smash::app::sv_information::is_ready_go() {
            VarModule::set_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER, 0);
        }
    }
}

#[utils::macros::opff(FIGHTER_KIND_PALUTENA )]
pub fn palutena_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		palutena_frame(fighter)
    }
}

pub unsafe fn palutena_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

#[smashline::weapon_frame_callback(main)]
pub fn reflection_board_callback(weapon: &mut smash::lua2cpp::L2CFighterBase) {
    unsafe { 
        if weapon.kind() != WEAPON_KIND_PALUTENA_REFLECTIONBOARD {
            return
        }
        if weapon.is_status(*WEAPON_PALUTENA_REFLECTIONBOARD_STATUS_KIND_SHOOT) {
            let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
            let palutena = utils::util::get_battle_object_from_id(owner_id);
            let palutena_boma = &mut *(*palutena).module_accessor;
            if AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_ATTACK){
                StatusModule::change_status_request_from_script(weapon.module_accessor, *WEAPON_PALUTENA_REFLECTIONBOARD_STATUS_KIND_BREAK, false);
            }
        }
    }
}