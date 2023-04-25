
use super::*;


#[acmd_script( agent = "falco", script = "game_specialnstart" , category = ACMD_GAME , low_priority)]
unsafe fn falco_special_n_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 6.0/(14.0-1.0));
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER, false, 0);
        if ArticleModule::is_exist(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER){
            ArticleModule::change_motion(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER, smash::phx::Hash40::new("open"), false, 0.0);
        }
    }
}

#[acmd_script( agent = "falco", script = "game_specialnloop" , category = ACMD_GAME , low_priority)]
unsafe fn falco_special_n_loop_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        //DamageModule::add_damage(boma, 1.0, 0);
        FT_MOTION_RATE(fighter, 10.0/(4.0-1.0));
        WorkModule::on_flag(boma, *FIGHTER_FALCO_BLASTER_STATUS_WORK_ID_FLAG_LOOP_ACCEPT);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        ArticleModule::generate_article(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER_BULLET, false, 0);
        if ArticleModule::is_exist(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER){
            ArticleModule::change_motion(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER, smash::phx::Hash40::new("loop"), false, 0.0);
        }
    }
    frame(lua_state, 7.0);
    FT_MOTION_RATE(fighter, 0.25);
    frame(lua_state, 17.0);
    if is_excute(fighter) {
    WorkModule::off_flag(boma, *FIGHTER_FALCO_BLASTER_STATUS_WORK_ID_FLAG_LOOP_ACCEPT);
    }
}

#[acmd_script( agent = "falco", script = "game_specialairnstart" , category = ACMD_GAME , low_priority)]
unsafe fn falco_special_air_n_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 4.0/(5.0-1.0));
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER, false, 0);
        if ArticleModule::is_exist(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER){
            ArticleModule::change_motion(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER, smash::phx::Hash40::new("open"), false, 0.0);
        }
    }
}

#[acmd_script( agent = "falco", script = "game_specialairnloop" , category = ACMD_GAME , low_priority)]
unsafe fn falco_special_air_n_loop_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        //DamageModule::add_damage(boma, 1.0, 0);
        WorkModule::on_flag(boma, *FIGHTER_FALCO_BLASTER_STATUS_WORK_ID_FLAG_LOOP_ACCEPT);
        FT_MOTION_RATE(fighter, 5.0/(4.0-1.0));
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        //DamageModule::add_damage(boma, 1.0, 0);
        ArticleModule::generate_article(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER_BULLET, false, 0);
        if ArticleModule::is_exist(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER){
            ArticleModule::change_motion(boma, *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER, smash::phx::Hash40::new("loop"), false, 0.0);
        }
    }
    frame(lua_state, 7.0);
    FT_MOTION_RATE(fighter, 0.5);
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_FALCO_BLASTER_STATUS_WORK_ID_FLAG_LOOP_ACCEPT);
        if WorkModule::is_flag(boma,*FIGHTER_FALCO_BLASTER_STATUS_WORK_ID_FLAG_LOOP){
            //laser loop motion rate(changing this allows for faster or slower sequential lasers from the first)
            FT_MOTION_RATE(fighter,0.10);
        }
    }
}

#[acmd_script( agent = "falco", script = "game_specialairsend" , category = ACMD_GAME , low_priority)]
unsafe fn falco_special_air_s_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) { }
}
 #[acmd_script( agent = "falco", script = "game_specialhiholdair" , category = ACMD_GAME , low_priority)]
 unsafe fn falco_special_hi_hold(fighter: &mut L2CAgentBase) {
        let lua_state = fighter.lua_state_agent;
        let boma = fighter.boma();
    frame(lua_state, 15.0);
    for _ in 0..7 {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 368, 40, 0, 30, 7.0, 0.0, 6.0, 0.0, None, None, None, 0.75, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            let target = smash::phx::Vector2f { x: 0.0, y: 5.0 };
            AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &target, 8, false);
        }
        wait(lua_state, 1.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 1.0);
    }
}

#[acmd_script( agent = "falco", script = "game_specialhi" , category = ACMD_GAME , low_priority)]
unsafe fn falco_special_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("hip"), 0.5, 367, 60, 0, 60, 5.0, 3.2, -3.1, -1.5, None, None, None, 0.75, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("hip"), 0.5, 80, 100, 80, 0, 5.0, 3.2, -3.1, -1.5, None, None, None, 0.75, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    for _ in 0..7 {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("hip"), 1.0, 367, 60, 0, 60, 5.0, 3.2, -3.1, -1.5, None, None, None, 0.75, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
            ATTACK(fighter, 1, 0, Hash40::new("hip"), 1.0, 65, 100, 80, 0, 5.0, 3.2, -3.1, -1.5, None, None, None, 0.75, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
       }
        wait(lua_state, 1.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 1.0);
    }
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("hip"), 2.0, 75, 220, 0, 75, 6.0, -4.0, 3.5, -1.5, None, None, None, 1.5, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "falco", script = "game_specialairlw" , category = ACMD_GAME , low_priority)]
unsafe fn falco_special_air_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        // This allows falco jump stalling. DO NOT REMOVE UNLESS YOU FEEL YOU ARE GOING TO STATUS SCRIPT IT.
        KineticModule::unable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        let shine_vec = Vector3f { x: 0.25, y: 0.0, z: 1.0 };
        KineticModule::mul_speed(boma, &shine_vec, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let shineGravity = Vector3f { x: 1.0, y: 0.1588, z: 1.0 };
        KineticModule::mul_accel(boma, &shineGravity, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 84, 48, 0, 92, 6.5, 0.0, 7.0, 0.5, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 0.7);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {  }frame(lua_state, 51.0);
    if is_excute(fighter) {
        KineticModule::resume_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
    }
    
}

#[acmd_script( agent = "falco", script = "game_speciallw" , category = ACMD_GAME , low_priority)]
unsafe fn falco_special_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
        KineticModule::unable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        let shine_vec = Vector3f { x: 0.25, y: 0.0, z: 1.0 };
        KineticModule::mul_speed(boma, &shine_vec, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let shineGravity = Vector3f { x: 1.0, y: 0.1588, z: 1.0 };
        KineticModule::mul_accel(boma, &shineGravity, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 84, 48, 0, 92, 6.5, 0.0, 7.0, -0.4, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 0.7);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {  }frame(lua_state, 51.0);
    if is_excute(fighter) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, false);
    }
    
}

#[acmd_script( agent = "falco", script = "sound_speciallw" , category = ACMD_SOUND , low_priority)]
unsafe fn falco_special_lw_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_falco_special_l01"));
    }

}

#[acmd_script( agent = "falco", script = "sound_specialairlw" , category = ACMD_SOUND , low_priority)]
unsafe fn falco_special_air_lw_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_falco_special_l01"));
    }

}

#[acmd_script( agent = "falco", script = "sound_specialnstart" , category = ACMD_SOUND , low_priority)]
unsafe fn falco_special_n_start_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_falco_special_n02"));
    }

}

#[acmd_script( agent = "falco", script = "sound_specialairnstart" , category = ACMD_SOUND , low_priority)]
unsafe fn falco_special_air_n_start_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_falco_special_n02"));
    }

}

// #[acmd_script( agent = "falco", script = "sound_specialairhi" , category = ACMD_SOUND , low_priority)]
// unsafe fn falco_special_air_hi_sound(fighter: &mut L2CAgentBase) {
//     let lua_state = fighter.lua_state_agent;
//     let boma = sv_system::battle_object_module_accessor(lua_state);
//     frame(lua_state, 1.0);
//     if is_excute(fighter) {
//         PLAY_SEQUENCE(fighter, Hash40::new("seq_falco_rnd_firebird"));
//         PLAY_SE(fighter, Hash40::new("se_falco_special_h02"));
//     }

// }

// #[acmd_script( agent = "falco", script = "sound_specialhi" , category = ACMD_SOUND , low_priority)]
// unsafe fn falco_special_hi_sound(fighter: &mut L2CAgentBase) {
//     let lua_state = fighter.lua_state_agent;
//     let boma = sv_system::battle_object_module_accessor(lua_state);
//     frame(lua_state, 1.0);
//     if is_excute(fighter) {
//         PLAY_SEQUENCE(fighter, Hash40::new("seq_falco_rnd_firebird"));
//         PLAY_SE(fighter, Hash40::new("se_falco_special_h02"));
//     }

// }

pub fn install() {
    install_acmd_scripts!(
        falco_special_n_start_game,
        falco_special_n_loop_game,
        falco_special_air_n_start_game,
        falco_special_air_n_loop_game,
        falco_special_air_s_end_game,
        falco_special_hi_hold,
        falco_special_hi_game,
        falco_special_air_lw_game,
        falco_special_lw_game,
        falco_special_air_lw_sound,
        falco_special_n_start_sound,
        falco_special_air_n_start_sound,
        // falco_special_air_hi_sound,
        // falco_special_hi_sound,
    );
}

