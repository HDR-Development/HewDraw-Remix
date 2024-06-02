use super::*;

unsafe extern "C" fn game_move(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();        
    //let h_speed = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) as f32;
    //let v_speed = KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) as f32;
        //let angle_calc = v_speed / h_speed as f32;
        //println!("Angle: {}", angle_calc);
        
        //let mut launch_angle = 180 + {(v_speed / h_speed.abs()).atan().to_degrees() as u64};
    frame(lua_state, 3.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 0, 55, 0, 40, 4.4, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 48, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.object(), vars::ness::status::THUNDER_LOOSE) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 0, 55, 0, 40, 4.4, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 48, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
        }
    }
}

unsafe extern "C" fn game_movechild(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_move", game_move, Priority::Low);
    agent.acmd("game_movechild", game_movechild, Priority::Low);
}
