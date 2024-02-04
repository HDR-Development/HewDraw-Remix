
use super::*;


unsafe extern "C" fn damageflyhi_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
                app::sv_math::rand(hash40("fighter"), 3)
            } else {
                0
            };
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_dedede_rnd_futtobi01"), Hash40::new("seq_dedede_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_dedede_rnd_futtobi01"), Hash40::new("seq_dedede_rnd_futtobi02"));}
    }
}


unsafe extern "C" fn damageflylw_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
                app::sv_math::rand(hash40("fighter"), 3)
            } else {
                0
            };
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_dedede_rnd_futtobi01"), Hash40::new("seq_dedede_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_dedede_rnd_futtobi01"), Hash40::new("seq_dedede_rnd_futtobi02"));}
    }
}


unsafe extern "C" fn damageflyn_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
                app::sv_math::rand(hash40("fighter"), 3)
            } else {
                0
            };
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_dedede_rnd_futtobi01"), Hash40::new("seq_dedede_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_dedede_rnd_futtobi01"), Hash40::new("seq_dedede_rnd_futtobi02"));}
    }
}


unsafe extern "C" fn damageflyroll_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            PLAY_FLY_VOICE(fighter, Hash40::new("seq_dedede_rnd_futtobi01"), Hash40::new("seq_dedede_rnd_futtobi02"));
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_dedede_rnd_futtobi01"), Hash40::new("seq_dedede_rnd_futtobi02"));
    }
}


unsafe extern "C" fn damageflytop_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if !StopModule::is_stop(fighter.module_accessor) {
            let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
                app::sv_math::rand(hash40("fighter"), 3)
            } else {
                0
            };
            if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_dedede_rnd_futtobi01"), Hash40::new("seq_dedede_rnd_futtobi02"));}
        }
    }
    frame(lua_state, 1.1);
    if is_excute(fighter) {
        let play_vc = if DamageModule::reaction(boma, 0) < 100.0 {
            app::sv_math::rand(hash40("fighter"), 3)
        } else {
            0
        };
        if play_vc == 0 {PLAY_FLY_VOICE(fighter, Hash40::new("seq_dedede_rnd_futtobi01"), Hash40::new("seq_dedede_rnd_futtobi02"));}
    }
}


unsafe extern "C" fn expression_landingheavy(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_landl"), 0, false, 0x50000000 /* default value */);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        if !fighter.is_prev_status(*FIGHTER_STATUS_KIND_ESCAPE_AIR)
        && !fighter.is_status(*FIGHTER_STATUS_KIND_JUMP_SQUAT) {
            QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
        }
    } 
}


unsafe extern "C" fn dedede_catch_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 5.0/(7.0-1.0));
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("top"), 5.5, 0.0, 7.0, 0.0, Some(0.0), Some(7.0), Some(11.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        //FT_MOTION_RATE(fighter, 1.120);
    }
    
}


unsafe extern "C" fn dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.2);
    }
	frame(lua_state, 11.0); // Effectively F13
    if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.0);
		WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}


unsafe extern "C" fn dash_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        let dash_sfx_handle = SoundModule::play_se(fighter.module_accessor, Hash40::new("se_dedede_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
    wait(lua_state, 9.0);
    if is_excute(fighter) {
        PLAY_STEP(fighter, Hash40::new("se_dedede_step_right_l"));
    }
}


unsafe extern "C" fn turn_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}


unsafe extern "C" fn dedede_gordo_special_s_throw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let gordo_speed_x = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);    
    
    if is_excute(fighter){
        if VarModule::is_flag(owner_module_accessor.object(), vars::dedede::instance::IS_DASH_GORDO){
            PostureModule::reverse_rot_y_lr(boma);
            PostureModule::reverse_lr(boma);
        }
        /* Prevents backwards gordos */
        if PostureModule::lr(owner_module_accessor) * gordo_speed_x < 0.0{
            KineticModule::mul_speed(boma, &Vector3f{x: -1.0, y: 1.0,z:  1.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        }
    }
    /* Checks every frame the gordo is active, set equal to the gordo life param */
    for _ in 0..181{
        if is_excute(fighter) {
            if !boma.is_status(*WEAPON_DEDEDE_GORDO_STATUS_KIND_HOP) {
                if VarModule::is_flag(owner_module_accessor.object(), vars::dedede::instance::IS_DASH_GORDO){

                    let bounce_dmg_multiplier = ((WorkModule::get_int(boma, *WEAPON_DEDEDE_GORDO_STATUS_WORK_INT_BOUND_COUNT) as f32 + 2.0) * 0.25);
                    ATTACK(fighter, 0, 0, Hash40::new("hip"), 7.5 * bounce_dmg_multiplier, 120, 110, 60, 0, 6.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
                    ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 0.7);
                    
                    //Reduces the max amount of bounces by 1 per recatch on the same gordo
                    if (WorkModule::get_int(boma, *WEAPON_DEDEDE_GORDO_STATUS_WORK_INT_BOUND_COUNT) - VarModule::get_int(owner_module_accessor.object(), vars::dedede::instance::RECATCH_COUNTER)) < 0{
                        StatusModule::change_status_request(fighter.module_accessor, *WEAPON_DEDEDE_GORDO_STATUS_KIND_DEAD, true);
                    }
                }
                else{
                    /* Reduces damage on every bounce, by 10% of its last damage in this case */
                    let bounce_dmg_multiplier = ((WorkModule::get_int(boma, *WEAPON_DEDEDE_GORDO_STATUS_WORK_INT_BOUND_COUNT) as f32 + 8.0) * 0.1);
                    ATTACK(fighter, 0, 0, Hash40::new("hip"), 7.5 * bounce_dmg_multiplier, 60, 110, 60, 0, 6.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
                    ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 0.7);
                }
            }
        }
        wait(lua_state, 1.0);
    }

}


unsafe extern "C" fn dedede_gordo_special_s_throw_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter){
        //Intentionally blank to kill vanilla effects
    }
}


unsafe extern "C" fn dedede_gordo_special_s_shot_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    for _ in 0..181{
        if is_excute(fighter) {
            if !boma.is_status(*WEAPON_DEDEDE_GORDO_STATUS_KIND_HOP) {
                GroundModule::update_force(boma);
                /* Reduces damage on every bounce, by 12.5% of its last damage in this case */
                let bounce_dmg_multiplier = ((WorkModule::get_int(boma, *WEAPON_DEDEDE_GORDO_STATUS_WORK_INT_BOUND_COUNT) as f32 + 5.0) * 0.125);
                ATTACK(fighter, 0, 0, Hash40::new("hip"), 12.8 * bounce_dmg_multiplier, 60, 50, 0, 60, 0.9, 3.8, 3.8, 0.0, Some(-3.8), Some(-3.8), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -8.4, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
                ATTACK(fighter, 1, 0, Hash40::new("hip"), 12.8 * bounce_dmg_multiplier, 60, 50, 0, 60, 0.9, 3.8, -3.8, 0.0, Some(-3.8), Some(3.8), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -8.4, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
                ATTACK(fighter, 2, 0, Hash40::new("hip"), 12.8 * bounce_dmg_multiplier, 60, 50, 0, 60, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -8.4, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
            }
        }
        wait(lua_state, 1.0);
    }
}


unsafe extern "C" fn dedede_gordo_special_s_shot_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

}


unsafe extern "C" fn dedede_gordo_special_s_attack_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);

    //Disables vanilla regrab searchbox, this ALWAYS needs to be on due to new regrab
    WorkModule::on_flag(owner_module_accessor, *FIGHTER_DEDEDE_INSTANCE_WORK_ID_FLAG_PERSONAL); 
    let mut speed_x = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
    let mut speed_y = KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);

    if is_excute(fighter) {
        WorkModule::set_int(boma, 300, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
        /* below grabs the boma of the opponent hitting gordo, the attack data of that hit, and adjusts the speed accordingly */
        let num_players = Fighter::get_fighter_entry_count(); 
        if StopModule::is_hit(boma){ 
            for i in 0..num_players{
                let opponent_boma = sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(i));

                if AttackModule::is_infliction(opponent_boma, *COLLISION_KIND_MASK_HIT){
                    let data = AttackModule::attack_data(opponent_boma, 0, false);
                    let mut angle = (*data).vector as f32;
                    let mut damage = (*data).power;
                    let kbg = (*data).r_eff as f32;
                    let bkb = (*data).r_add as f32;
                    
                    //Covering sakurai angle and other funky angles
                    if angle > 360.0{ 
                        angle = 32.0;
                    }
                    //Damage cap, gordo goes to the moon otherwise
                    if damage > 25.0{
                        damage = 25.0;
                    }

                    let radians = angle.to_radians();
                    let cos = radians.cos();
                    let sin = radians.sin();

                    //formulas for the speed multipliers
                    let x_speed_mul = cos * ((kbg * 0.3718) + bkb / 100.0) * (damage / 8.0) / 70.0;
                    let y_speed_mul =  sin  *  (damage / 2.5) * ((kbg * 0.3718) + bkb / 100.0) / 60.0 / speed_y;

                    KineticModule::mul_speed(boma, &Vector3f{x: x_speed_mul, y: y_speed_mul , z: 1.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL); 
                }
            }
        /* Seeing the speed is still the same. This only occurs if the above did not run, which happens on projectiles or non-direct hits (Bayo smash attacks) */
        if speed_x == KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) &&  speed_y == KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL){
            let damage = DamageModule::damage(boma, 0);
            if damage > 11.0{
                KineticModule::mul_speed(boma, &Vector3f{x: 0.8, y: 1.0, z: 1.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
            }
            else{
                KineticModule::mul_speed(boma, &Vector3f{x: 0.4 + 0.05 * (damage - 5.0), y: 1.0, z: 1.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
            }
        }
        }
    }
    for _ in 0..301{
        if is_excute(fighter) {
            if !boma.is_status(*WEAPON_DEDEDE_GORDO_STATUS_KIND_HOP) {
                /* Reduces damage on every bounce, by 12.5% of its last damage in this case */
                let bounce_dmg_multiplier = ((WorkModule::get_int(boma, *WEAPON_DEDEDE_GORDO_STATUS_WORK_INT_BOUND_COUNT) as f32 + 5.0) * 0.125);
                if !StopModule::is_stop(boma){
                    speed_x = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL).abs();
                    speed_y = KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL).abs();
                }
                let mut speed = speed_x.max(speed_y);
                let mut damage = (7.5 * (speed * 0.6));
                damage = damage.max(7.5);

                ATTACK(fighter, 0, 0, Hash40::new("hip"), damage * bounce_dmg_multiplier, 60, 110, 60, 0, 6.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
            }
        }
        wait(lua_state, 1.0);
    }
}


unsafe extern "C" fn dedede_gordo_special_s_attack_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    //Intentionally blank to kill vanilla effects

}


unsafe extern "C" fn dedede_gordo_special_s_wall_stop_game(fighter: &mut L2CAgentBase){
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        
    }
}


unsafe extern "C" fn dedede_gordo_special_s_start_game(fighter: &mut L2CAgentBase){
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    if is_excute(fighter){
        if VarModule::is_flag(owner_module_accessor.object(), vars::dedede::instance::IS_DASH_GORDO){
            WorkModule::on_flag(boma, *WEAPON_DEDEDE_GORDO_STATUS_WORK_FLAG_VISIBILITY_ON);
        }
    }
    frame(lua_state, 7.0);
    if is_excute(fighter){
        WorkModule::on_flag(boma, *WEAPON_DEDEDE_GORDO_STATUS_WORK_FLAG_VISIBILITY_ON);
    }
}


unsafe extern "C" fn dedede_gordo_special_air_s_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);

    if is_excute(fighter){
        if VarModule::is_flag(owner_module_accessor.object(), vars::dedede::instance::IS_DASH_GORDO){
            WorkModule::on_flag(boma, *WEAPON_DEDEDE_GORDO_STATUS_WORK_FLAG_VISIBILITY_ON);
        }
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *WEAPON_DEDEDE_GORDO_STATUS_WORK_FLAG_VISIBILITY_ON);
    }
}


unsafe extern "C" fn escape_air_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let escape_air_cancel_frame = WorkModule::get_param_float(boma, hash40("param_motion"), hash40("escape_air_cancel_frame"));
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        VarModule::set_flag(fighter.battle_object, vars::dedede::instance::CAN_WADDLE_DASH_FLAG, false);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
    }
    frame(lua_state, escape_air_cancel_frame);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}


unsafe extern "C" fn escape_air_slide_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        VarModule::set_flag(fighter.battle_object, vars::dedede::instance::CAN_WADDLE_DASH_FLAG, false);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL);
    }
    frame(lua_state, 39.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}


unsafe extern "C" fn fly_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter){
        ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 70, 65, 0, 8, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
    }
}


unsafe extern "C" fn landing_fall_special_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_dedede_landing03"));
    }
}




pub fn install() {
    smashline::Agent::new("dedede_gordo")
        .acmd("game_specialsthrow", dedede_gordo_special_s_throw_game)
        .acmd("effect_specialsthrow", dedede_gordo_special_s_throw_effect)
        .acmd("game_specialsshot", dedede_gordo_special_s_shot_game)
        .acmd("effect_specialsshot", dedede_gordo_special_s_shot_effect)
        .acmd("game_specialsattack", dedede_gordo_special_s_attack_game)
        .acmd("effect_specialsattack", dedede_gordo_special_s_attack_effect)
        .acmd("game_specialswallstop", dedede_gordo_special_s_wall_stop_game)
        .acmd("game_specialsstart", dedede_gordo_special_s_start_game)
        .acmd("game_specialairsstart", dedede_gordo_special_air_s_start_game)
        .install();
    smashline::Agent::new("dedede_star")
        .acmd("game_fly", fly_game)
        .install();
    smashline::Agent::new("dedede")
        .acmd("sound_damageflyhi", damageflyhi_sound)
        .acmd("sound_damageflylw", damageflylw_sound)
        .acmd("sound_damageflyn", damageflyn_sound)
        .acmd("sound_damageflyroll", damageflyroll_sound)
        .acmd("sound_damageflytop", damageflytop_sound)
        .acmd("expression_landingheavy", expression_landingheavy)
        .acmd("game_catch", dedede_catch_game)
        .acmd("game_dash", dash_game)
        .acmd("sound_dash", dash_sound)
        .acmd("game_turndash", turn_dash_game)
        .acmd("game_escapeair", escape_air_game)
        .acmd("game_escapeairslide", escape_air_slide_game)
        .acmd("sound_landingfallspecial", landing_fall_special_sound)
        .install();
}
