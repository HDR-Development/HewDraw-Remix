use super::*;

unsafe extern "C" fn richter_axe_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let lr = PostureModule::lr(fighter.module_accessor);
    let axe_owner = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let offset = &Vector2f::new(0.0 * lr, if axe_owner.kind() == *FIGHTER_KIND_KIRBY { -12.2 } else { -4.2 });
    PostureModule::set_rot(boma, &Vector3f::new(0.0, 0.0, if axe_owner.is_situation(*SITUATION_KIND_AIR) { -128.0 } else { -90.0 } * lr), 0);
    PostureModule::set_scale(boma, 1.12, false);
    PostureModule::add_pos_2d(boma, offset);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("axe"), 3.5, 50, 80, 0, 32, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(boma);
    }
}

unsafe extern "C" fn richter_axe_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_greenshell_trace"), Hash40::new("rot"), 0, 1.5, 0, 0, 0, 0, 0.70, true);
        LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.19, 0.0);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_greenshell_trace"), Hash40::new("rot"), 0, 1.5, 0, 0, 0, 0, 0.70, true);
        LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.19, 0.0);
        BURN_COLOR(fighter, 1.0, 1.0, 1.0, 1.0);
        FLASH(fighter, 0.89, 0.706, 0.106, 0.5);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        BURN_COLOR_NORMAL(fighter);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash"), Hash40::new("axe"), 0, -2.3, 0.4, 0, 0, 0, 0.1, false);
        LAST_EFFECT_SET_RATE(fighter, 2.0);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        FLASH(fighter, 1.0, 1.0, 1.0, 1.0);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        FLASH(fighter, 1.0, 1.0, 1.0, 1.0);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        FLASH(fighter, 1.0, 1.0, 1.0, 1.0);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        FLASH(fighter, 1.0, 1.0, 1.0, 0.5);
    }
}

unsafe extern "C" fn richter_axe_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_sword_swing_s"));
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_fly", richter_axe_game);
    agent.acmd("effect_fly", richter_axe_effect);
    agent.acmd("sound_fly", richter_axe_sound);
}