use super::*;

unsafe extern "C" fn reflet_attack_air_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        VarModule::set_int(fighter.battle_object, vars::reflet::instance::LEVIN_AERIAL_LENIENCY, 5);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.75);
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 8.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            ATTACK(fighter, 0, 0, Hash40::new("sword"), 13.0, 361, 60, 0, 61, 4.3, 0.0, 7.5, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("sword"), 13.0, 361, 60, 0, 61, 4.0, 0.0, 1.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 2, 0, Hash40::new("armr"), 13.0, 361, 60, 0, 61, 3.5, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
        else {
            ATTACK(fighter, 0, 0, Hash40::new("sword"), 9.0, 55, 68, 0, 36, 3.5, 0.0, 6.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("sword"), 9.0, 55, 68, 0, 36, 3.5, 0.0, 0.5, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 2, 0, Hash40::new("armr"), 9.0, 55, 68, 0, 36, 3.5, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            ATTACK(fighter, 0, 0, Hash40::new("sword"), 13.0, 41, 60, 0, 61, 4.3, 0.0, 7.5, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("sword"), 13.0, 41, 60, 0, 61, 4.0, 0.0, 1.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 2, 0, Hash40::new("armr"), 13.0, 41, 60, 0, 61, 3.5, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
        else {
            ATTACK(fighter, 0, 0, Hash40::new("sword"), 9.0, 45, 73, 0, 36, 3.5, 0.0, 6.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("sword"), 9.0, 45, 73, 0, 36, 3.5, 0.0, 0.5, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 2, 0, Hash40::new("armr"), 9.0, 45, 73, 0, 36, 3.5, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            ATTACK(fighter, 0, 0, Hash40::new("sword"), 5.0, 65, 80, 0, 45, 3.0, 0.0, 9.0, 0.0, Some(0.0), Some(2.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            AttackModule::clear(boma, 1, false);
        }
        else {
            AttackModule::clear_all(boma);
        }
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 34.0);
    if is_excute(fighter) {
        if WorkModule::get_int(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) == 0 {
            if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
                VisibilityModule::set_int64(boma, Hash40::new("sword").hash as i64, Hash40::new("sword_normal").hash as i64);
                WorkModule::off_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON);
            }
        }
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }

}

unsafe extern "C" fn reflet_attack_air_n_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_reflet_sword4"), Hash40::new("tex_reflet_sword2"), 5, Hash40::new("sword"), 0.0, 0.0, 0.0, Hash40::new("sword"), 0.0, 10.75, 0.0, true, Hash40::new("reflet_thunderswoed_flare"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
            EFFECT_FOLLOW(fighter, Hash40::new("reflet_thunderswoed"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        else {
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_reflet_sword1"), Hash40::new("tex_reflet_sword5"), 5, Hash40::new("sword"), 0.0, 0.4, 0.0, Hash40::new("sword"), 0.0, 8.5, 0.0, true, Hash40::new("reflet_swoed_flare"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 3);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("reflet_thunderswoed"), false, false);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_reflet_sword4"), Hash40::new("tex_reflet_sword2"), 5, Hash40::new("sword"), 0.0, 0.0, 0.0, Hash40::new("sword"), 0.0, 10.75, 0.0, true, Hash40::new("reflet_thunderswoed_flare"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
            EFFECT_FOLLOW(fighter, Hash40::new("reflet_thunderswoed"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        else {
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_reflet_sword1"), Hash40::new("tex_reflet_sword5"), 6, Hash40::new("sword"), 0.0, 0.4, 0.0, Hash40::new("sword"), 0.0, 8.5, 0.0, true, Hash40::new("reflet_swoed_flare"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 3);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("reflet_thunderswoed"), false, false);
    }
}

unsafe extern "C" fn reflet_attack_air_f_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        VarModule::set_int(fighter.battle_object, vars::reflet::instance::LEVIN_AERIAL_LENIENCY, 5);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 9.0/(12.0));
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 11.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {   
        if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            ATTACK(fighter, 0, 0, Hash40::new("sword"), 13.0, 67, 90, 0, 51, 4.3, 0.0, 7.5, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("sword"), 13.0, 67, 90, 0, 51, 4.0, 0.0, 1.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 2, 0, Hash40::new("armr"), 13.0, 67, 90, 0, 51, 3.5, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
        else {
            ATTACK(fighter, 0, 0, Hash40::new("sword"), 8.5, 67, 86, 0, 51, 3.5, 0.0, 6.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("sword"), 8.5, 67, 86, 0, 51, 3.5, 0.0, 0.5, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 2, 0, Hash40::new("armr"), 8.5, 67, 86, 0, 51, 3.5, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            ATTACK(fighter, 0, 0, Hash40::new("sword"), 5.0, 72, 80, 0, 45, 3.0, 0.0, 9.0, 0.0, Some(0.0), Some(2.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            AttackModule::clear(boma, 1, false);
            AttackModule::clear(boma, 2, false);
        }
        else {
            AttackModule::clear_all(boma);
        }
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        if WorkModule::get_int(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) == 0 {
            if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
                VisibilityModule::set_int64(boma, Hash40::new("sword").hash as i64, Hash40::new("sword_normal").hash as i64);
                WorkModule::off_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON);
            }
        }
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }

}

unsafe extern "C" fn reflet_attack_air_f_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if WorkModule::is_flag(boma,  *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
        if is_excute(fighter) {
            AttackModule::set_attack_reference_joint_id(boma, Hash40::new("sword"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
        }
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 11.0);
    app::sv_animcmd::execute(lua_state, 11.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma,  *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            RUMBLE_HIT(fighter, Hash40::new("rbkind_slashl"), 0);
        } else {
            RUMBLE_HIT(fighter, Hash40::new("rbkind_slashm"), 0);
        }
    }
}

unsafe extern "C" fn reflet_attack_air_f_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_reflet_sword4"), Hash40::new("tex_reflet_sword2"), 7, Hash40::new("sword"), 0.0, 0.0, 0.0, Hash40::new("sword"), 0.0, 10.75, 0.0, true, Hash40::new("null"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
            EFFECT_FOLLOW(fighter, Hash40::new("reflet_thunderswoed"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(fighter, Hash40::new("reflet_thunderswoed_flare"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        else{
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_reflet_sword1"), Hash40::new("tex_reflet_sword5"), 6, Hash40::new("sword"), 0.0, 0.4, 0.0, Hash40::new("sword"), 0.0, 8.5, 0.0, true, Hash40::new("reflet_swoed_flare"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 3);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            EFFECT_OFF_KIND(fighter, Hash40::new("reflet_thunderswoed"), false, false);
            EFFECT_OFF_KIND(fighter, Hash40::new("reflet_thunderswoed_flare"), false, false);
        }
    }
}

unsafe extern "C" fn reflet_attack_air_b_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        VarModule::set_int(fighter.battle_object, vars::reflet::instance::LEVIN_AERIAL_LENIENCY, 5);
    }
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            ATTACK(fighter, 0, 0, Hash40::new("sword"), 15.0, 361, 98, 0, 36, 4.3, 0.0, 7.5, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("sword"), 15.0, 361, 98, 0, 36, 4.0, 0.0, 1.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 2, 0, Hash40::new("armr"), 15.0, 361, 98, 0, 36, 3.5, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
        else {
            ATTACK(fighter, 0, 0, Hash40::new("sword"), 11.5, 361, 79, 0, 36, 3.5, 0.0, 6.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("sword"), 11.5, 361, 79, 0, 36, 3.5, 0.0, 0.5, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 2, 0, Hash40::new("armr"), 11.5, 361, 79, 0, 36, 3.5, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            ATTACK(fighter, 0, 0, Hash40::new("sword"), 5.0, 65, 80, 0, 45, 3.0, 0.0, 9.0, 0.0, Some(0.0), Some(2.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            AttackModule::clear(boma, 1, false);
            AttackModule::clear(boma, 2, false);
        }
        else {
            AttackModule::clear_all(boma);
        }
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        if WorkModule::get_int(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) == 0 {
            if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
                VisibilityModule::set_int64(boma, Hash40::new("sword").hash as i64, Hash40::new("sword_normal").hash as i64);
                WorkModule::off_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON);
            }
        }
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }

}

unsafe extern "C" fn reflet_attack_air_b_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_reflet_sword4"), Hash40::new("tex_reflet_sword2"), 6, Hash40::new("sword"), 0.0, 0.0, 0.0, Hash40::new("sword"), 0.0, 10.75, 0.0, true, Hash40::new("null"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
            EFFECT_FOLLOW(fighter, Hash40::new("reflet_thunderswoed"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1.2, true);
            EFFECT_FOLLOW(fighter, Hash40::new("reflet_thunderswoed_flare"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        else{
            EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash"), Hash40::new("sword"), 0, 7.0, 0, 0, 0, 0, 0.5, true);
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_reflet_sword1"), Hash40::new("tex_reflet_sword5"), 4, Hash40::new("sword"), 0.0, 0.4, 0.0, Hash40::new("sword"), 0.0, 8.5, 0.0, true, Hash40::new("reflet_swoed_flare"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 4);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            EFFECT_OFF_KIND(fighter, Hash40::new("reflet_thunderswoed"), false, false);
            EFFECT_OFF_KIND(fighter, Hash40::new("reflet_thunderswoed_flare"), false, false);
        }
    }

}

unsafe extern "C" fn reflet_attack_air_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        VarModule::set_int(fighter.battle_object, vars::reflet::instance::LEVIN_AERIAL_LENIENCY, 5);
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.5);
    frame(lua_state, 5.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            ATTACK(fighter, 0, 0, Hash40::new("sword"), 14.0, 80, 91, 0, 50, 4.3, 0.0, 7.5, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("sword"), 14.0, 80, 91, 0, 50, 4.0, 0.0, 1.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 2, 0, Hash40::new("armr"), 14.0, 80, 91, 0, 50, 3.5, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
        else {
            ATTACK(fighter, 0, 0, Hash40::new("sword"), 10.0, 80, 75, 0, 51, 3.5, 0.0, 6.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("sword"), 10.0, 80, 75, 0, 51, 3.5, 0.0, 0.5, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 2, 0, Hash40::new("armr"), 10.0, 80, 75, 0, 51, 3.5, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma,  *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            ATTACK(fighter, 0, 0, Hash40::new("sword"), 5.0, 65, 80, 0, 45, 3.0, 0.0, 9.0, 0.0, Some(0.0), Some(2.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            AttackModule::clear(boma, 1, false);
            AttackModule::clear(boma, 2, false);
        }
        else {
            AttackModule::clear_all(boma);
        }
    }
    wait(lua_state, 7.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            AttackModule::clear_all(boma);
        }
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    if is_excute(fighter) {
        if WorkModule::get_int(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) == 0 {
            if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
                VisibilityModule::set_int64(boma, Hash40::new("sword").hash as i64, Hash40::new("sword_normal").hash as i64);
                WorkModule::off_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON);
            }
        }
    }
}

unsafe extern "C" fn reflet_attack_air_hi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_reflet_sword4"), Hash40::new("tex_reflet_sword2"), 6, Hash40::new("sword"), 0.0, 0.0, 0.0, Hash40::new("sword"), 0.0, 10.75, 0.0, true, Hash40::new("null"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
            EFFECT_FOLLOW(fighter, Hash40::new("reflet_thunderswoed"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(fighter, Hash40::new("reflet_thunderswoed_flare"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        else {
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_reflet_sword1"), Hash40::new("tex_reflet_sword5"), 4, Hash40::new("sword"), 0.0, 0.4, 0.0, Hash40::new("sword"), 0.0, 8.5, 0.0, true, Hash40::new("reflet_swoed_flare"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 4);
    }
    frame(lua_state, 24.0);
    if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
        EFFECT_OFF_KIND(fighter, Hash40::new("reflet_thunderswoed"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("reflet_thunderswoed_flare"), false, false);
    }
}

unsafe extern "C" fn reflet_attack_air_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        VarModule::set_int(fighter.battle_object, vars::reflet::instance::LEVIN_AERIAL_LENIENCY, 5);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            //Air-Only Hitboxes
            ATTACK(fighter, 0, 0, Hash40::new("sword"), 14.0, 270, 54, 0, 22, 4.3, 0.0, 7.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("sword"), 14.0, 270, 54, 0, 22, 4.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 2, 0, Hash40::new("armr"), 14.0, 270, 54, 0, 22, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            //Ground-Only Hitboxes
            ATTACK(fighter, 3, 0, Hash40::new("sword"), 14.0, 70, 81, 0, 24, 4.3, 0.0, 7.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 4, 0, Hash40::new("sword"), 14.0, 70, 81, 0, 24, 4.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 5, 0, Hash40::new("armr"), 14.0, 70, 81, 0, 24, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
        else {
            ATTACK(fighter, 0, 0, Hash40::new("sword"), 10.2, 361, 61, 0, 58, 3.5, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("sword"), 10.2, 361, 61, 0, 58, 3.5, 0.0, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 2, 0, Hash40::new("armr"), 10.2, 361, 61, 0, 58, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            ATTACK(fighter, 0, 0, Hash40::new("sword"), 15.0, 40, 66, 0, 57, 4.3, 0.0, 7.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("sword"), 15.0, 40, 66, 0, 57, 4.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 2, 0, Hash40::new("armr"), 15.0, 40, 66, 0, 57, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            AttackModule::clear(boma, 3, false);
            AttackModule::clear(boma, 4, false);
            AttackModule::clear(boma, 5, false);
        }
        else {
            ATTACK(fighter, 0, 0, Hash40::new("sword"), 9.0, 361, 58, 0, 49, 3.5, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 1, 0, Hash40::new("sword"), 9.0, 361, 58, 0, 49, 3.5, 0.0, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(fighter, 2, 0, Hash40::new("armr"), 9.0, 361, 58, 0, 49, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            ATTACK(fighter, 0, 0, Hash40::new("sword"), 5.0, 65, 80, 0, 45, 3.0, 0.0, 9.0, 0.0, Some(0.0), Some(2.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            AttackModule::clear(boma, 1, false);
            AttackModule::clear(boma, 2, false);
        }
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        if WorkModule::get_int(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) == 0 {
            if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
                VisibilityModule::set_int64(boma, Hash40::new("sword").hash as i64, Hash40::new("sword_normal").hash as i64);
                WorkModule::off_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON);
            }
        }
    }
    frame(lua_state, 48.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn reflet_attack_air_lw_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("sword"), 0, 5, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_reflet_sword4"), Hash40::new("tex_reflet_sword2"), 6, Hash40::new("sword"), 0.0, 0.0, 0.0, Hash40::new("sword"), 0.0, 10.75, 0.0, true, Hash40::new("null"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
            EFFECT_FOLLOW(fighter, Hash40::new("reflet_thunderswoed"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(fighter, Hash40::new("reflet_thunderswoed_flare"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            EFFECT_FOLLOW(fighter, Hash40::new("reflet_attack_flash"), Hash40::new("sword"), 0, 8.5, 0, 0, 0, 0, 1.8, true);
        }
        else {
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_reflet_sword1"), Hash40::new("tex_reflet_sword5"), 4, Hash40::new("sword"), 0.0, 0.4, 0.0, Hash40::new("sword"), 0.0, 8.5, 0.0, true, Hash40::new("reflet_swoed_flare"), Hash40::new("sword"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
            EFFECT_FOLLOW(fighter, Hash40::new("reflet_attack_flash"), Hash40::new("sword"), 0, 7, 0, 0, 0, 0, 1.3, true);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("reflet_swoed_flare2"), Hash40::new("sword"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 2);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
            EFFECT_OFF_KIND(fighter, Hash40::new("reflet_thunderswoed"), false, false);
        }
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("reflet_thunderswoed_flare"), false, false);
    }

}

pub fn install() {
    smashline::Agent::new("reflet")
        .acmd("game_attackairn", reflet_attack_air_n_game)
        .acmd("effect_attackairn", reflet_attack_air_n_effect)
        .acmd("game_attackairf", reflet_attack_air_f_game)
        .acmd("expression_attackairf", reflet_attack_air_f_expression)
        .acmd("effect_attackairf", reflet_attack_air_f_effect)
        .acmd("game_attackairb", reflet_attack_air_b_game)
        .acmd("effect_attackairb", reflet_attack_air_b_effect)
        .acmd("game_attackairhi", reflet_attack_air_hi_game)
        .acmd("effect_attackairhi", reflet_attack_air_hi_effect)
        .acmd("game_attackairlw", reflet_attack_air_lw_game)
        .acmd("effect_attackairlw", reflet_attack_air_lw_effect)
        .install();
}
