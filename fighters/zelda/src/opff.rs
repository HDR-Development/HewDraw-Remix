// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;
 
unsafe fn teleport_tech(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, frame: f32) {
    if boma.is_status(*FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_2) {
        if compare_mask(ControlModule::get_pad_flag(boma), *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER) {
            VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
            fighter.change_status(FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_3.into(), true.into());
            return;
        }
    }

    // Wall Ride momentum fixes
    let touch_right = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_RIGHT_SIDE as u32);
    let touch_left = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_LEFT_SIDE as u32);

    if boma.is_status(*FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_2) {
        let touch_normal_y_left = GroundModule::get_touch_normal_y(boma, *GROUND_TOUCH_FLAG_LEFT_SIDE as u32);
        let touch_normal_y_right = GroundModule::get_touch_normal_y(boma, *GROUND_TOUCH_FLAG_RIGHT_SIDE as u32);
        if (touch_right && touch_normal_y_right != 0.0)
        || (touch_left && touch_normal_y_left != 0.0)
        || VarModule::is_flag(boma.object(), vars::common::instance::IS_TELEPORT_WALL_RIDE)
        {
            if !VarModule::is_flag(boma.object(), vars::common::instance::IS_TELEPORT_WALL_RIDE) {
                VarModule::on_flag(boma.object(), vars::common::instance::IS_TELEPORT_WALL_RIDE);
            }
            let init_speed_y = VarModule::get_float(boma.object(), vars::common::status::TELEPORT_INITIAL_SPEED_Y);

            if init_speed_y > 0.0 {
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, init_speed_y);
                app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
            }
        }
    }
    else if boma.is_status(*FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_3) {
        if touch_right || touch_left {
            if KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) > 0.0 {
                let wall_ride = Vector3f{x: 0.0, y: 1.0, z: 1.0};
                KineticModule::mul_speed(boma, &wall_ride, *FIGHTER_KINETIC_ENERGY_ID_STOP);
            }
        }
    }
    else {
        if VarModule::is_flag(boma.object(), vars::common::instance::IS_TELEPORT_WALL_RIDE) {
            VarModule::off_flag(boma.object(), vars::common::instance::IS_TELEPORT_WALL_RIDE);
        }
    }
}

unsafe fn phantom_special_cancel(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) 
    && !fighter.is_in_hitlag()
    && fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_ATTACK,
        *FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
        *FIGHTER_STATUS_KIND_ATTACK_S4,
        *FIGHTER_STATUS_KIND_ATTACK_HI4,
        *FIGHTER_STATUS_KIND_ATTACK_LW4,
        *FIGHTER_STATUS_KIND_ATTACK_DASH,
        *FIGHTER_STATUS_KIND_ATTACK_AIR]) {
        if fighter.is_cat_flag(Cat1::SpecialLw) && !ArticleModule::is_exist(boma, *FIGHTER_ZELDA_GENERATE_ARTICLE_PHANTOM) {
            if !fighter.is_status(*FIGHTER_STATUS_KIND_ATTACK_AIR) { //displacement flag
                VarModule::on_flag(fighter.battle_object, vars::zelda::instance::HIT_CANCEL_PHANTOM);
            }//cancel if phantom is off cd
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, false);
        }
    }
}

unsafe fn nayru_drift_land_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32, stick_y: f32, frame: f32) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
        if situation_kind == *SITUATION_KIND_GROUND {
            if StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR && frame < 55.0 {
                //StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, false);
                EffectModule::kill_kind(boma, Hash40::new("zelda_nayru_l"), true, true);
                EffectModule::kill_kind(boma, Hash40::new("zelda_nayru_r"), true, true);
                WorkModule::on_flag(boma, *FIGHTER_ZELDA_STATUS_SPECIAL_N_FLAG_REFLECTOR_END);
                MotionModule::set_frame_sync_anim_cmd(boma, 56.0, true, true, false);
            }
        }
        else if situation_kind == *SITUATION_KIND_AIR {
            if frame >= 31.0 {
                if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_FALL {
                    KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
                }
            }
        }
    }
}

/// Handles land canceling when airborne
unsafe fn dins_fire_cancels(boma: &mut BattleObjectModuleAccessor){
    if boma.is_status(*FIGHTER_ZELDA_STATUS_KIND_SPECIAL_S_END) {
        if boma.is_situation(*SITUATION_KIND_GROUND) {
            if StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR {
                boma.change_status_req(*FIGHTER_STATUS_KIND_LANDING, false);
            }
        }
    }
}

/// Reset use of Din's Fire on stock loss or match end
unsafe fn dins_flag_reset(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_WIN,
        *FIGHTER_STATUS_KIND_LOSE,
        *FIGHTER_STATUS_KIND_ENTRY,
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_REBIRTH]) || !sv_information::is_ready_go() {
        VarModule::off_flag(boma.object(), vars::zelda::instance::DEIN_ACTIVE);
    }
}

pub unsafe fn phantom_platdrop_effect(fighter:&mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_LW) && fighter.status_frame() > 9 {
        if ControlModule::get_stick_y(boma) < -0.66 && GroundModule::is_passable_ground(boma) {
            GroundModule::pass_floor(boma);
        }//platdrop
    }
    if VarModule::get_int(fighter.battle_object, vars::zelda::instance::EFF_COOLDOWN_HANDLER) == -1 {
        if ArticleModule::is_exist(boma, *FIGHTER_ZELDA_GENERATE_ARTICLE_PHANTOM) {
            let handle = EffectModule::req_follow(boma, Hash40::new("zelda_phantom_aura"), Hash40::new("havel"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &Vector3f::zero(), 1.05, true, 0, 0, 0, 0, 0, true, true) as u32;
            VarModule::set_int(fighter.battle_object, vars::zelda::instance::EFF_COOLDOWN_HANDLER, handle as i32);
        }
    } else if VarModule::get_int(fighter.battle_object, vars::zelda::instance::EFF_COOLDOWN_HANDLER) == -2 {
        EFFECT_FOLLOW(fighter, Hash40::new("zelda_atk_flash"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.8, true);
        app::FighterUtil::flash_eye_info(fighter.module_accessor);
        EFFECT_OFF_KIND(fighter, Hash40::new("zelda_phantom_aura"), true, true);
        VarModule::set_int(fighter.battle_object, vars::zelda::instance::EFF_COOLDOWN_HANDLER, -1);
    } else {
        let handle = VarModule::get_int(fighter.battle_object, vars::zelda::instance::EFF_COOLDOWN_HANDLER) as u32;
        if !ArticleModule::is_exist(boma, *FIGHTER_ZELDA_GENERATE_ARTICLE_PHANTOM) {
            VarModule::set_int(fighter.battle_object, vars::zelda::instance::EFF_COOLDOWN_HANDLER, -2);//remove
            VarModule::off_flag(fighter.battle_object, vars::zelda::instance::HIT_CANCEL_PHANTOM);
            VarModule::off_flag(fighter.battle_object, vars::zelda::instance::PHANTOM_HIT);
        } else if !EffectModule::is_exist_effect(boma, handle as u32) {
            VarModule::set_int(fighter.battle_object, vars::zelda::instance::EFF_COOLDOWN_HANDLER, -1);
        } //reapply
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_S_LOOP,
        *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_S_END,
        *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_3,
        *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_LW_CHARGE,
        *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_LW_END
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
        if fighter.is_flag(*FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
            if [*FIGHTER_KINETIC_TYPE_MOTION_AIR, *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE].contains(&KineticModule::get_kinetic_type(fighter.module_accessor)) {
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
                let speed_y = app::sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);

                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y, 0.0, 0.0, 0.0);
                app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
                
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                app::sv_kinetic_energy::enable(fighter.lua_state_agent);

                KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
            }
        }
    }
}

pub unsafe fn moveset(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    teleport_tech(fighter, boma, frame);
    dins_fire_cancels(boma);
    dins_flag_reset(boma);
    nayru_drift_land_cancel(boma, status_kind, situation_kind, cat[2], stick_y, frame);
    phantom_special_cancel(fighter, boma);
    phantom_platdrop_effect(fighter, boma);
    fastfall_specials(fighter);
}

pub extern "C" fn zelda_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		zelda_frame(fighter)
    }
}

pub unsafe fn zelda_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, zelda_frame_wrapper);
}
