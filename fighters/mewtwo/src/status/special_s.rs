use super::*;

// FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_S_THROW

unsafe extern "C" fn special_s_throw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_S_THROW)(fighter);
    // kill ref
    shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_MEWTWO_REFLECTOR_KIND_REFLECTOR, *FIGHTER_REFLECTOR_GROUP_EXTEND);
    // fx
    let effect_2 = EffectModule::req_follow(fighter.module_accessor, Hash40::new("mewtwo_final_aura"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 1.1, true, 0, 0, 0, 0, 0, false, false);
    VarModule::set_int64(fighter.battle_object, vars::mewtwo::status::EFFECT_HANDLER_2, effect_2);
    VarModule::set_vec2(fighter.battle_object, vars::mewtwo::status::SPECIAL_S_THROW_POS, Vector2f{x: 14.3 * fighter.lr(), y: 9.3});
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(move_to_center as *const () as _));
    ret
}

unsafe extern "C" fn move_to_center(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lr = fighter.lr();
    let captured_id = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE) as u32;
    let captured_boma = sv_battle_object::module_accessor(captured_id);
    let move_end_frame = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_s.move_end_frame");
    if fighter.motion_frame() < move_end_frame { // stop moving ~6f? b4 the finisher starts
        let add_per_frame = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_s.add_per_frame");
        let last_pos = VarModule::get_vec2(fighter.battle_object, vars::mewtwo::status::SPECIAL_S_THROW_POS);
        let mut add_stick = Vector2f{x: last_pos.x + add_per_frame*fighter.left_stick_x(), y: last_pos.y + add_per_frame*fighter.left_stick_y()};
        // prevent them from being inside mewtwo? if weird can just add reverse hit check
        if add_stick.x < 11.5 && add_stick.x > -11.5 {add_stick.x = last_pos.x};
        VarModule::set_vec2(fighter.battle_object, vars::mewtwo::status::SPECIAL_S_THROW_POS, Vector2f{x: add_stick.x, y: add_stick.y});
    }
    if StatusModule::status_kind(captured_boma) == *FIGHTER_STATUS_KIND_MEWTWO_THROWN {
        let offset_from_mewtwo = VarModule::get_vec2(fighter.battle_object, vars::mewtwo::status::SPECIAL_S_THROW_POS);
        LinkModule::remove_model_constraint(captured_boma, true);
        let mewtwo_pos = *PostureModule::pos(fighter.module_accessor);
        let captured_pos = *PostureModule::pos(captured_boma);
        let mut hip_offset = Vector3f::zero();
        ModelModule::joint_global_offset_from_top(captured_boma, Hash40::new("hip"), &mut hip_offset);
        let target_hip_loc = &Vector2f{x: mewtwo_pos.x - hip_offset.x + (offset_from_mewtwo.x), y: mewtwo_pos.y - hip_offset.y + offset_from_mewtwo.y};
        let diff = &Vector2f{x: (captured_pos.x - target_hip_loc.x), y: captured_pos.y - target_hip_loc.y};
        PostureModule::add_pos_2d(captured_boma, &Vector2f{x: -diff.x / 6.0, y: -diff.y / 6.0});
        // eff
        let captured_hip_pos = &mut Vector3f{ x: 0.0, y: 0.0, z: 0.0 };
        ModelModule::joint_global_position(captured_boma, Hash40::new("hip"), captured_hip_pos, false);
        let target_effect_loc = &Vector2f{x: (captured_hip_pos.x - mewtwo_pos.x) * lr, y: captured_hip_pos.y - mewtwo_pos.y};
        let effect = VarModule::get_int64(fighter.battle_object, vars::mewtwo::status::EFFECT_HANDLER);
        let effect_2 = VarModule::get_int64(fighter.battle_object, vars::mewtwo::status::EFFECT_HANDLER_2);
        EffectModule::set_pos(fighter.module_accessor, effect as u32, &Vector3f{x: 0.0, y: target_effect_loc.y, z: target_effect_loc.x});
        EffectModule::set_pos(fighter.module_accessor, effect_2 as u32, &Vector3f{x: 0.0, y: target_effect_loc.y, z: target_effect_loc.x});
        VarModule::set_vec2(fighter.battle_object, vars::mewtwo::status::SPECIAL_S_THROW_CAPTURED_POS_OFFSET, Vector2f{x: target_effect_loc.x, y: target_effect_loc.y});
    }
    0.into()
}

unsafe extern "C" fn special_s_throw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("mewtwo_final_aura"), false, true);
    EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_nenriki"), false, true);
    smashline::original_status(End, fighter, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_S_THROW)(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_S_THROW, special_s_throw_main);
    agent.status(End, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_S_THROW, special_s_throw_end);
}