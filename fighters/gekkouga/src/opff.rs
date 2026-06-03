// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn max_water_shuriken_dc(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_N_MAX_SHOT)
    && fighter.status_frame() > 12 {
        fighter.check_dash_cancel();
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
        let doll_pos = *GroundModule::get_rhombus(doll_module_accessor, true).add(1);
        let doll_pos_x = doll_pos.x;
        let doll_pos_y = doll_pos.y;
        let doll_top_pos = *GroundModule::get_rhombus(doll_module_accessor, true);
        let doll_top_pos_x = doll_top_pos.x;
        let doll_top_pos_y = doll_top_pos.y;

        let mut angle = (doll_pos_y - pos_y).atan2(doll_pos_x - pos_x).to_degrees();
        let distance = sv_math::vec2_distance(pos_x, pos_y, doll_pos_x, doll_pos_y);

        let can_teleport = distance <= 80.0;
        VarModule::set_flag(fighter.battle_object, vars::gekkouga::instance::SPECIAL_LW_CAN_TELEPORT, can_teleport);

        let mut eff_handle = VarModule::get_int(fighter.battle_object, vars::gekkouga::instance::SPECIAL_LW_MARKER_EFF_HANDLE) as u32;
        if !EffectModule::is_exist_effect(fighter.module_accessor, eff_handle) {
            eff_handle = EffectModule::req(
                fighter.module_accessor,
                Hash40::new("sys_direction"),
                &Vector3f{x: doll_top_pos_x, y: doll_top_pos_y + 5.0, z: 0.0},
                &Vector3f::new(135.0, 0.0, 0.0),
                0.7,
                0,
                -1,
                false,
                0
            ) as u32;
            VarModule::set_int(fighter.battle_object, vars::gekkouga::instance::SPECIAL_LW_MARKER_EFF_HANDLE, eff_handle as i32);
        }
        else {
            EffectModule::set_pos(fighter.module_accessor, eff_handle, &Vector3f{x: doll_top_pos_x, y: doll_top_pos_y + 5.0, z: 0.0});
        }
        let team_color = FighterUtil::get_team_color(fighter.module_accessor);
        let mut effect_team_color = FighterUtil::get_effect_team_color(EColorKind(team_color as i32), Hash40::new("direction_effect_color"));
        let color_add = if can_teleport { 0.0 } else { 0.22 };
        EffectModule::set_rgb(fighter.module_accessor, eff_handle, effect_team_color.x() + color_add, effect_team_color.y() + color_add, effect_team_color.z() + color_add);
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

pub unsafe fn moveset(fighter: &mut L2CFighterCommon) {
    max_water_shuriken_dc(fighter);
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
        moveset(fighter);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, gekkouga_frame_wrapper);
}
