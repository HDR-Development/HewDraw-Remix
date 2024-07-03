// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn max_water_shuriken_dc(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if status_kind == *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_N_MAX_SHOT {
        if frame > 12.0 {
            boma.check_dash_cancel();
        }
    }
}

// Greninja Shadow Sneak Smash Attack Cancel
unsafe fn shadow_sneak_smash_attack_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if status_kind == *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_S_ATTACK {
        if boma.status_frame() < 6 {
            if situation_kind == *SITUATION_KIND_GROUND {
                if boma.is_cat_flag(Cat1::AttackS4) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S4_START, false);
                }
                if boma.is_cat_flag(Cat1::AttackHi4) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, false);
                }
                if boma.is_cat_flag(Cat1::AttackLw4) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, false);
                }
            }
        }
    }
}

// Dair Jump Cancel
unsafe fn dair_jc(boma: &mut BattleObjectModuleAccessor, situation_kind: i32, cat1: i32, motion_kind: u64, frame: f32) {
    if motion_kind == hash40("attack_air_lw") {
        if !boma.is_in_hitlag() {
            if frame > 31.0 {
                if situation_kind == *SITUATION_KIND_AIR {
                    boma.check_jump_cancel(false, false);
                }
            }
        }
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_N_HOLD,
        *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_N_SHOT,
        *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_N_MAX_START,
        *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_N_MAX_SHOT,
        *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_S_ATTACK,
        *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_S_END,
        *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_HI_WALL_DAMAGE,
        *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_HI_END,
        *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_LW_END,
        *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_LW_ATTACK,
        *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_LW_HIT,
        *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_LW_BOUND
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

unsafe extern "C" fn teleport_guide_pos(fighter: &mut L2CFighterCommon, angle: L2CValue) -> Vector2f {
    let pos = PostureModule::pos(fighter.module_accessor);
    let rad = angle.get_f32().to_radians();
    let scale = PostureModule::scale(fighter.module_accessor);
    let dist = 10.0;
    let dist_scaled = dist * scale;
    let x_pos = rad.cos() * dist_scaled + (*pos).x;
    let y_pos = rad.sin() * dist_scaled + (*pos).y;
    let y_offset = 6.0;
    let y_pos = y_offset * scale + y_pos;
    Vector2f{x: x_pos, y: y_pos}
}

pub unsafe fn substitute_teleport_check(fighter: &mut L2CFighterCommon) {
    let doll_id = super::status::special_lw::gekkouga_get_sub_id(fighter.battle_object);
    if sv_battle_object::is_active(doll_id) {
        let doll_module_accessor = sv_battle_object::module_accessor(doll_id);

        let pos_x = PostureModule::pos_x(fighter.module_accessor);
        let pos_y = PostureModule::pos_y(fighter.module_accessor);
        let doll_pos_x = PostureModule::pos_x(doll_module_accessor);
        let doll_pos_y = PostureModule::pos_y(doll_module_accessor);
        // println!("Greninja Pos: {}, {}", pos_x, pos_y);
        // println!("Doll Pos: {}, {}", doll_pos_x, doll_pos_y);

        let mut angle = (doll_pos_y - pos_y).atan2(doll_pos_x - pos_x).to_degrees();
        // println!("angle: {}", angle);
        let distance = sv_math::vec2_distance(pos_x, pos_y, doll_pos_x, doll_pos_y);
        // println!("distance: {}", distance);

        let can_teleport = distance <= 80.0;
        VarModule::set_flag(fighter.battle_object, vars::gekkouga::instance::SPECIAL_LW_CAN_TELEPORT, can_teleport);

        if distance >= 20.0 {
            let mut eff_handle = VarModule::get_int(fighter.battle_object, vars::gekkouga::instance::SPECIAL_LW_MARKER_EFF_HANDLE) as u32;
            let guide_pos = teleport_guide_pos(fighter, angle.into());
            if !EffectModule::is_exist_effect(fighter.module_accessor, eff_handle) {
                eff_handle = EffectModule::req(
                    fighter.module_accessor,
                    Hash40::new("sys_direction2"),
                    &Vector3f{x: guide_pos.x, y: guide_pos.y, z: 0.0},
                    &Vector3f{x: 0.0, y: 0.0, z: 0.0},
                    1.0,
                    0,
                    -1,
                    false,
                    0
                ) as u32;
                VarModule::set_int(fighter.battle_object, vars::gekkouga::instance::SPECIAL_LW_MARKER_EFF_HANDLE, eff_handle as i32);
            }
            else {
                EffectModule::set_pos(fighter.module_accessor, eff_handle, &Vector3f{x: guide_pos.x, y: guide_pos.y, z: 0.0});
            }
            EffectModule::set_rot(fighter.module_accessor, eff_handle, &Vector3f{x: 0.0, y: 0.0, z: angle - 90.0});
    
            if can_teleport {
                let team_color = FighterUtil::get_team_color(fighter.module_accessor);
                let mut effect_team_color = FighterUtil::get_effect_team_color(EColorKind(team_color as i32), Hash40::new("direction_effect_color"));
                if !WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW)
                || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_SAVE_SPEED) {
                    effect_team_color.x += 0.22;
                    effect_team_color.y += 0.22;
                    effect_team_color.z += 0.22;
                }
                EffectModule::set_rgb(fighter.module_accessor, eff_handle, effect_team_color.x, effect_team_color.y, effect_team_color.z);
            }
            else {
                EffectModule::set_rgb(fighter.module_accessor, eff_handle, 0.7, 0.7, 0.7);
            }
        }
        else {
            let eff_handle = VarModule::get_int(fighter.battle_object, vars::gekkouga::instance::SPECIAL_LW_MARKER_EFF_HANDLE) as u32;
            if EffectModule::is_exist_effect(fighter.module_accessor, eff_handle) {
                EffectModule::kill(fighter.module_accessor, eff_handle, true, true);
                VarModule::set_int(fighter.battle_object, vars::gekkouga::instance::SPECIAL_LW_MARKER_EFF_HANDLE, 0);
            }
        }
    }
    else {
        let eff_handle = VarModule::get_int(fighter.battle_object, vars::gekkouga::instance::SPECIAL_LW_MARKER_EFF_HANDLE) as u32;
        if EffectModule::is_exist_effect(fighter.module_accessor, eff_handle) {
            EffectModule::kill(fighter.module_accessor, eff_handle, true, true);
            VarModule::set_int(fighter.battle_object, vars::gekkouga::instance::SPECIAL_LW_MARKER_EFF_HANDLE, 0);
        }
        VarModule::off_flag(fighter.battle_object, vars::gekkouga::instance::SPECIAL_LW_CAN_TELEPORT);
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    max_water_shuriken_dc(boma, status_kind, situation_kind, cat[0], frame);
    shadow_sneak_smash_attack_cancel(boma, status_kind, situation_kind, cat[0], frame);
    //dair_jc(boma, situation_kind, cat[0], motion_kind, frame);
    fastfall_specials(fighter);
    substitute_teleport_check(fighter);
}

pub extern "C" fn gekkouga_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		gekkouga_frame(fighter)
    }
}

pub unsafe fn gekkouga_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, gekkouga_frame_wrapper);
}
