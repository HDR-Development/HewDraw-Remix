use super::*;

unsafe extern "C" fn game_shot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        AttackModule::set_power_mul_5th(boma, 1.0);
        let owner_module_accessor = boma.get_owner_boma();
        if owner_module_accessor.kind() == *FIGHTER_KIND_PALUTENA {
            let (damage, bkb, kbg, hitlag, size) = if VarModule::is_flag(owner_module_accessor.object(), vars::palutena::status::SPECIAL_N_PRIMARY_POWERED)
                { (7.0, 63, 40, 0.5, 2.4) } else { (4.0, 60, 60, 0.25, 2.1) }; // para cap of 90, stun is kb * hitlag mul + 1
            ATTACK(agent, 0, 0, Hash40::new("top"), damage, 65, kbg, 0, bkb, size, 0.0, 0.0, 0.0, None, None, None, hitlag, 0.6, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        }
        else {
            ATTACK(agent, 0, 0, Hash40::new("top"), 2.5, 36, 53, 0, 61, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        }
        ControlModule::set_rumble(boma, Hash40::new("rbkind_beamss"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn effect_shot(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let owner_module_accessor = boma.get_owner_boma();
    let palutena = owner_module_accessor.kind() == *FIGHTER_KIND_PALUTENA;
    if is_excute(agent) {
        if palutena {
            let eff_name = if VarModule::is_flag(owner_module_accessor.object(), vars::palutena::status::SPECIAL_N_PRIMARY_POWERED) {"palutena_bullet_grey_super"} else {"palutena_bullet_grey"};
            let scale_x = if VarModule::is_flag(owner_module_accessor.object(), vars::palutena::status::SPECIAL_N_PRIMARY_POWERED) {1.15} else {0.9};
            let scale_y = if VarModule::is_flag(owner_module_accessor.object(), vars::palutena::status::SPECIAL_N_PRIMARY_POWERED) {0.95} else {0.65};
            let red = if VarModule::is_flag(owner_module_accessor.object(), vars::palutena::status::SPECIAL_N_PRIMARY_POWERED) {1.25} else {1.05};
            let green = if VarModule::is_flag(owner_module_accessor.object(), vars::palutena::status::SPECIAL_N_PRIMARY_POWERED) {0.55} else {0.7};
            EFFECT_FOLLOW(agent, Hash40::new(eff_name), Hash40::new("top"), 0, 0, 0.0, 0, 0, 0, 1.0, false);
            LAST_EFFECT_SET_COLOR(agent, red, green, 0.0125);
            LAST_EFFECT_SET_SCALE_W(agent, scale_x, 0.85, scale_x);
            EFFECT(agent, Hash40::new("palutena_bullet_shot_grey"), Hash40::new("top"), 0, 0, 0.25, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
            LAST_EFFECT_SET_COLOR(agent, 1.25, 1.00, 0.025);
            LAST_EFFECT_SET_RATE(agent, 1.5);
            LAST_EFFECT_SET_ALPHA(agent, 0.75);
        } else {
            EFFECT(agent, Hash40::new("palutena_bullet_shot"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    if palutena {
        wait(lua_state, 2.0);
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("palutena_elec"), Hash40::new("top"), 0.0, 0.0, -1.0, 0, 0, 0, 0.35, true);
            LAST_EFFECT_SET_SCALE_W(agent, 0.55, 0.4, 0.4);
            LAST_EFFECT_SET_COLOR(agent, 1.25, 0.65, 0.025);
            LAST_EFFECT_SET_ALPHA(agent, 0.3);
            let yellow = if VarModule::is_flag(owner_module_accessor.object(), vars::palutena::status::SPECIAL_N_PRIMARY_POWERED) {0.45} else {0.85};
            EFFECT_FOLLOW(agent, Hash40::new("palutena_elec"), Hash40::new("top"), 0.0, 0.0, -1.0, 0, 0, 0, 0.35, true);
            LAST_EFFECT_SET_SCALE_W(agent, 0.75, 0.45, 0.45);
            LAST_EFFECT_SET_COLOR(agent, 1.25, yellow, 0.025);
            LAST_EFFECT_SET_ALPHA(agent, 0.4);
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_shot", game_shot, Priority::Low);
    agent.acmd("effect_shot", effect_shot, Priority::Low);
}