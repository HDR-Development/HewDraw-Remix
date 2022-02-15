
use super::*;

#[acmd_script(agent = "demon", script = "game_attackairb", category = ACMD_GAME, low_priority)]
unsafe fn demon_attackairb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;

    frame(lua_state, 5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }

    frame(lua_state, 6.0);
    if is_excute(fighter) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
            PostureModule::reverse_lr(boma);
            PostureModule::update_rot_y_lr(boma);
        }
    }

    frame(lua_state, 11.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0,Hash40::new("toer"), 16.0, 40, 74, 0, 40, 3.0, -0.3, -1.0, 1.2, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 0, 0,Hash40::new("top"), 16.0, 40, 74, 0, 40, 5.0, 0.0, 11.0, -9.0, Some(0.0), Some(11.0), Some(-3.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 0, 0,Hash40::new("top"), 14.0, 40, 74, 0, 40, 4.0, 0.0, 11.0, 11.0, Some(0.0), Some(11.0), Some(-3.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
    }

    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear(boma, 2, false);
        ATTACK(fighter, 0, 0, Hash40::new("toer"), 10.0, 40, 74, 0, 40, 3.0, -0.3, -1.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 40, 74, 0, 40, 4.0, 0.0, 11.0, -11.0, Some(0.0), Some(11.0), Some(-3.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
    }

    wait(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }

    frame(lua_state, 35.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

pub fn install() {
    install_acmd_scripts!(
        demon_attackairb
    );
}

