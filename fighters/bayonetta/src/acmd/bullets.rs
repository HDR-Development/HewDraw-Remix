use super::*;

unsafe extern "C" fn BA_SHOOTING_ON(agent: &mut L2CAgentBase, hand: bool, joint: Hash40, msc_arg_1: f32, msc_arg_2: f32) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let lr = agent.lr();
    // check if this gun is first to fire
    let mut first_fire = false;
    if !VarModule::is_flag(agent.battle_object, vars::bayonetta::status::BULLET_FIRST_FIRE) {
        VarModule::on_flag(agent.battle_object, vars::bayonetta::status::BULLET_FIRST_FIRE);
        first_fire = true;
    }
    // calc firing position and angle
    let mut gun_pos = &mut Vector3f{ x: 0.0, y: 0.0, z: 0.0 };
    let mut gun_pos_2 = &mut Vector3f{ x: 0.0, y: 0.0, z: 0.0 };
    let top_pos = *PostureModule::pos(boma);
    let mut ground_bounce = false; // vanilla ricochet mechanic from dsmash (but automated)
    if hand {
        ModelModule::joint_global_position_with_offset(boma, joint, &Vector3f{x: 1.0, y: 0.0, z: 0.65}, gun_pos, true);
        ModelModule::joint_global_position_with_offset(boma, joint, &Vector3f{x: 7.0, y: 0.0, z: 0.65}, gun_pos_2, true);
    } else {
        ModelModule::joint_global_position_with_offset(boma, joint, &Vector3f{x: -2.5, y: 0.0, z: 0.0}, gun_pos, true);
        ModelModule::joint_global_position_with_offset(boma, joint, &Vector3f{x: 3.5, y: 0.0, z: 0.0}, gun_pos_2, true);
    }
    // calc for ricochet
    let ground_pos_any = &mut Vector2f::zero();
    let ground_pos_stage = &mut Vector2f::zero();
    let is_touch_any = !GroundModule::line_segment_check(boma, &Vector2f{x: gun_pos.x, y: gun_pos.y}, &Vector2f{x: gun_pos_2.x, y: gun_pos_2.y}, &Vector2f::zero(), ground_pos_any, true).is_null();
    let is_touch_stage = !GroundModule::line_segment_check(boma, &Vector2f{x: gun_pos.x, y: gun_pos.y}, &Vector2f{x: gun_pos_2.x, y: gun_pos_2.y}, &Vector2f::zero(), ground_pos_stage, false).is_null();
    let ground_bounce = is_touch_stage || (is_touch_any && ground_pos_any.y < gun_pos.y); // dont bounce on bottom of platform
    // calc angle of hitboxes
    let angle = if ground_bounce { (gun_pos_2.y - gun_pos.y).atan2((gun_pos_2.x - gun_pos.x) * lr) - std::f32::consts::PI} else {(gun_pos_2.y - gun_pos.y).atan2((gun_pos_2.x - gun_pos.x) * lr)}; //?
    let angle_deg = if angle.to_degrees() < 0.0 {angle.to_degrees().round() + 360.0 } else {angle.to_degrees().round() }; // launch angle cant be negative
    let gun_offset = Vector3f{ x: gun_pos.x - top_pos.x, y: gun_pos.y - top_pos.y, z: gun_pos.z - top_pos.z};
    let offset_y = 100.0 * angle.sin();
    let offset_x = 100.0 * angle.cos();
    let push_y = 25.0 * angle.sin();
    let push_x = 25.0 * angle.cos();
    // hitbox properties
    let hitbox_id = if first_fire {*FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_00} else {*FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_02};
    if is_excute(agent) {
        if !agent.is_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_DISABLE_ROOT_ATTACK) {
            ATTACK(agent, hitbox_id as u64, 1, Hash40::new("top"), 0.6, angle_deg as u64, 0, 0, 0, 3.25, 0.0, gun_offset.y, gun_offset.x * lr, Some(0.0), Some(gun_offset.y + push_y), Some(gun_offset.x * lr + push_x), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
        }
        ATTACK(agent, hitbox_id as u64 + 1, 1, Hash40::new("top"), 0.4, angle_deg as u64, 0, 0, 0, 2.75, 0.0, gun_offset.y, gun_offset.x * lr, Some(0.0), Some(gun_offset.y + offset_y), Some(gun_offset.x * lr + offset_x), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_63_bullet"), 4, true, *BATTLE_OBJECT_ID_INVALID as u32);
        if msc_arg_1 > -1.0 && !ground_bounce {
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x36db1a34c9), smash::lib::LuaConst::new(hitbox_id as u32), msc_arg_1, msc_arg_2);
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x36db1a34c9), smash::lib::LuaConst::new(hitbox_id as u32 + 1), msc_arg_1, msc_arg_2);
        } // truncates hitbox length when it goes through a wall
    }
}

unsafe extern "C" fn game_shootingon_generic_arml(agent: &mut L2CAgentBase) {
    BA_SHOOTING_ON(agent, true, Hash40::new("handl"),  6.0, 4.0);
}
unsafe extern "C" fn game_shootingon_generic_armr(agent: &mut L2CAgentBase) {
    BA_SHOOTING_ON(agent, true, Hash40::new("handr"), 6.0, 4.0);
}
unsafe extern "C" fn game_shootingon_generic_legl(agent: &mut L2CAgentBase) {
    BA_SHOOTING_ON(agent, false, Hash40::new("footl2"), 6.0, 4.0);
}
unsafe extern "C" fn game_shootingon_generic_legr(agent: &mut L2CAgentBase) {
    BA_SHOOTING_ON(agent, false, Hash40::new("footr2"), 6.0, 4.0);
}

// floats are for determining hitbox cut off when clipping thru stage

unsafe extern "C" fn game_shootingoff_generic(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    if is_excute(agent) {
        AttackModule::clear(boma, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01, false);
        AttackModule::clear(boma, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_03, false);
        if !agent.is_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_DISABLE_ROOT_ATTACK) {
            AttackModule::clear(boma, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_00, false);
            AttackModule::clear(boma, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_02, false);
        }
        VarModule::off_flag(agent.battle_object, vars::bayonetta::status::BULLET_FIRST_FIRE);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_shootingon_generic_arml", game_shootingon_generic_arml, Priority::Low);
    agent.acmd("game_shootingon_generic_armr", game_shootingon_generic_armr, Priority::Low);
    agent.acmd("game_shootingon_generic_legl", game_shootingon_generic_legl, Priority::Low);
    agent.acmd("game_shootingon_generic_legr", game_shootingon_generic_legr, Priority::Low);

    agent.acmd("game_shootingoff_generic", game_shootingoff_generic, Priority::Low);
}
