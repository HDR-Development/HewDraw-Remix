use super::*;

unsafe extern "C" fn game_attacks4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE(agent, 0.8);
    if is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(boma, Hash40::new("drag1"), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y));
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 35.0);
    FT_MOTION_RATE(agent, 1.2);
}

nsafe extern "C" fn game_specialsattack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(boma, Hash40::new("drag1"), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y));
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *WEAPON_KAMUI_SPEARHAND_INSTANCE_WORK_ID_FLAG_PIERCE_GROUND);
        let tip_pos = &mut Vector3f{ x: 0.0, y: 0.0, z: 0.0 };
        ModelModule::joint_global_position(boma, Hash40::new("drag8"), tip_pos, false);
        let base_pos = &mut Vector3f{ x: 0.0, y: 0.0, z: 0.0 };
        ModelModule::joint_global_position(boma, Hash40::new("drag2"), base_pos, false);
        let facing = PostureModule::lr(boma);
        let ground = &mut Vector2f{ x: 0.0, y: 0.0 };
        if GroundModule::line_segment_check(boma,
            &Vector2f{ x: base_pos.x + (10.0 * facing), y: base_pos.y - 8.0 },
            &Vector2f{ x: tip_pos.x + (35.0 * facing), y: tip_pos.y - 32.0 },
            &Vector2f::zero(),
            ground,
            true).is_null() {
            ATTACK(agent, 1, 0, Hash40::new("drag8"), 17.0, 40, 90, 0, 32, 3.2, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        }
        ATTACK(agent, 0, 0, Hash40::new("drag2"), 7.0, 50, 100, 0, 25, 3.5, 4.0, 0.0, 0.0, Some(17.5), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_pierce"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *WEAPON_KAMUI_SPEARHAND_INSTANCE_WORK_ID_FLAG_PIERCE_GROUND);
    }
}

unsafe extern "C" fn game_specialairsattack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(boma, Hash40::new("drag1"), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y));
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *WEAPON_KAMUI_SPEARHAND_INSTANCE_WORK_ID_FLAG_PIERCE_GROUND);
        let tip_pos = &mut Vector3f{ x: 0.0, y: 0.0, z: 0.0 };
        ModelModule::joint_global_position(boma, Hash40::new("drag8"), tip_pos, false);
        let base_pos = &mut Vector3f{ x: 0.0, y: 0.0, z: 0.0 };
        ModelModule::joint_global_position(boma, Hash40::new("drag2"), base_pos, false);
        let facing = PostureModule::lr(boma);
        let ground = &mut Vector2f{ x: 0.0, y: 0.0 };
        // debug_draw_line(
        //     &Vector2f{ x: base_pos.x + (10.0 * facing), y: base_pos.y - 8.0 },
        //     &Vector2f{ x: tip_pos.x + (38.0 * facing), y: tip_pos.y - 35.0 },
        //     500
        // );
        // debug_draw_circle(&Vector2f{ x: base_pos.x + (10.0 * facing), y: base_pos.y - 8.0 }, 5.0, 100);
        // debug_draw_circle(&Vector2f{ x: tip_pos.x + (38.0 * facing), y: tip_pos.y - 35.0 }, 5.0, 100);
        // debug_set_draw_color(255.0, 0.0, 0.0, 1.0);
        if GroundModule::line_segment_check(boma,
            &Vector2f{ x: base_pos.x + (10.0 * facing), y: base_pos.y - 8.0 },
            &Vector2f{ x: tip_pos.x + (35.0 * facing), y: tip_pos.y - 32.0 },
            &Vector2f::zero(),
            ground,
            true).is_null() {
            ATTACK(agent, 1, 0, Hash40::new("drag8"), 17.0, 40, 90, 0, 32, 3.2, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        }
        ATTACK(agent, 0, 0, Hash40::new("drag2"), 8.0, 50, 100, 0, 25, 3.5, 4.0, 0.0, 0.0, Some(17.5), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_pierce"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *WEAPON_KAMUI_SPEARHAND_INSTANCE_WORK_ID_FLAG_PIERCE_GROUND);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attacks4", game_attacks4);

    agent.acmd("game_specialsattack", game_specialsattack);
    agent.acmd("game_specialairsattack", game_specialairsattack);
}
