use super::*;

extern "C" {
    #[link_name = "\u{1}_ZN3app35WeaponShizueFishingrodLinkEventReel13new_l2c_tableEv"]
    fn WeaponShizueFishingrodLinkEventReel__new_l2c_table() -> L2CValue;

    #[link_name = "\u{1}_ZN3app8lua_bind31LinkEvent__store_l2c_table_implEPKNS_9LinkEventE"]
    fn store_event_table(event: *const app::LinkEvent) -> L2CValue;
}

unsafe extern "C" fn special_s_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );

    return 0.into();
}

// FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_REEL

unsafe extern "C" fn special_s_reel_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut motion = "";
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        motion = "special_s_reel";

        fighter.set_int64(hash40("special_s_reel_hi") as i64, *FIGHTER_SHIZUE_STATUS_WORK_ID_SPECIAL_S_INT_MOTION_KIND_HI);
        fighter.set_int64(hash40("special_s_reel_lw") as i64, *FIGHTER_SHIZUE_STATUS_WORK_ID_SPECIAL_S_INT_MOTION_KIND_LW);

        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    } 
    else if fighter.is_situation(*SITUATION_KIND_AIR) {
        motion = "special_air_s_reel";

        fighter.set_int64(hash40("special_air_s_reel_hi") as i64, *FIGHTER_SHIZUE_STATUS_WORK_ID_SPECIAL_S_INT_MOTION_KIND_HI);
        fighter.set_int64(hash40("special_air_s_reel_lw") as i64, *FIGHTER_SHIZUE_STATUS_WORK_ID_SPECIAL_S_INT_MOTION_KIND_LW);
        
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }

    MotionModule::change_motion(fighter.module_accessor, Hash40::new(motion), 0.0, 1.0, false, 0.0, false, false);
    MotionModule::set_rate(fighter.module_accessor, 0.0);

    fighter.set_float(1.0, *FIGHTER_SHIZUE_STATUS_WORK_ID_SPECIAL_S_FLOAT_REEL_MOTION_RATE);
    fighter.set_int64(hash40("invalid") as i64, *FIGHTER_SHIZUE_STATUS_WORK_ID_SPECIAL_S_INT_MOTION_KIND_2ND);
    
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_reel_main_loop as *const () as _))
}

unsafe extern "C" fn special_s_reel_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let rod = ArticleModule::get_article(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_FISHINGROD);
    let rod_id = smash::app::lua_bind::Article::get_battle_object_id(rod) as u32;
    let rod_boma = sv_battle_object::module_accessor(rod_id);

    let line_length = WorkModule::get_float(rod_boma, *WEAPON_SHIZUE_FISHINGROD_INSTANCE_WORK_ID_FLOAT_LINE_LENGTH);

    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        let mut motion = "";
        let mut motion_2nd = fighter.get_int64(*FIGHTER_SHIZUE_STATUS_WORK_ID_SPECIAL_S_INT_MOTION_KIND_2ND) as i64;

        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            // ground handling
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);

            fighter.set_int64(hash40("special_s_reel_hi") as i64, *FIGHTER_SHIZUE_STATUS_WORK_ID_SPECIAL_S_INT_MOTION_KIND_HI);
            fighter.set_int64(hash40("special_s_reel_lw") as i64, *FIGHTER_SHIZUE_STATUS_WORK_ID_SPECIAL_S_INT_MOTION_KIND_LW);

            motion = "special_air_s_reel";

            if motion_2nd == hash40("special_air_s_reel_hi") as i64 {
                motion_2nd = hash40("special_s_reel_hi") as i64;
            } 
            else if motion_2nd == hash40("special_air_s_reel_lw") as i64 {
                motion_2nd = hash40("special_s_reel_lw") as i64;
            }
        } else {
            // air handling
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);

            fighter.set_int64(hash40("special_air_s_reel_hi") as i64, *FIGHTER_SHIZUE_STATUS_WORK_ID_SPECIAL_S_INT_MOTION_KIND_HI);
            fighter.set_int64(hash40("special_air_s_reel_lw") as i64, *FIGHTER_SHIZUE_STATUS_WORK_ID_SPECIAL_S_INT_MOTION_KIND_LW);
            
            motion = "special_air_s_reel";

            if motion_2nd == hash40("special_s_reel_hi") as i64 {
                motion_2nd = hash40("special_air_s_reel_hi") as i64;
            } 
            else if motion_2nd == hash40("special_s_reel_lw") as i64 {
                motion_2nd = hash40("special_air_s_reel_lw") as i64;
            }
        }

        // weight for the rod's motion
        let mut weight = 0.0;
        if motion_2nd != hash40("invalid") as i64 {
            weight += MotionModule::weight(fighter.module_accessor);
        }

        // smoothly transition to the motion for the new situation
        MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new(motion), -1.0, 1.0, 0.0);
        // transition the rod's motion as well, if applicable
        if motion_2nd != hash40("invalid") as i64 {
            MotionModule::add_motion_2nd(
                fighter.module_accessor,
                Hash40::new_raw(motion_2nd as u64),
                MotionModule::frame(fighter.module_accessor),
                MotionModule::rate(fighter.module_accessor),
                true,
                weight
            );
            fighter.set_int64(motion_2nd, *FIGHTER_SHIZUE_STATUS_WORK_ID_SPECIAL_S_INT_MOTION_KIND_2ND);
        }
    }

    // triggers END if the line reaches it's max length
    if !fighter.is_flag(*FIGHTER_SHIZUE_STATUS_WORK_ID_SPECIAL_S_FLAG_HIT) {
        let reel_length_end = fighter.get_param_float("param_special_s", "reel_length_end") * 10.0;
        if line_length == reel_length_end {
            //println!("end"); 
            fighter.change_status(FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_END.into(), false.into());

            return 0.into();
        }
    }

    // changes status to HOOK if any of the listed buttons are pressed
    if !fighter.is_flag(*FIGHTER_SHIZUE_STATUS_WORK_ID_SPECIAL_S_FLAG_HIT) 
    && fighter.global_table[CMD_CAT1].get_i32() & 
        ( *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N 
        | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N 
        | *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S 
        | *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI 
        | *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW ) != 0
    {
        //println!("hook");
        fighter.change_status(FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_HOOK.into(), true.into());
        
        return 1.into();
    }

    if MotionModule::rate(fighter.module_accessor).abs() < 0.00001 {
        let reel_delay_frame = fighter.get_param_int("param_special_s", "reel_delay_frame");
        if fighter.global_table[CURRENT_FRAME].get_i32() == reel_delay_frame {
            MotionModule::set_rate(fighter.module_accessor, 1.0);

            // link event for automatically reeling the line back in
            let mut event = WeaponShizueFishingrodLinkEventReel__new_l2c_table();
            let speed = fighter.get_param_float("param_special_s", "reel_speed");
            event["reel_speed_"].assign(&L2CValue::F32(speed));
            event["pull_speed_"].assign(&L2CValue::F32(speed));
            event["status_kind_"].assign(&L2CValue::I32(*WEAPON_SHIZUE_FISHINGROD_STATUS_KIND_REEL));
            event["is_hook_"].assign(&L2CValue::Bool(false));
            
            let callable: extern "C" fn() -> *mut app::LinkEvent = std::mem::transmute(event["new_instance_lua_"].get_ptr());
            let link_event = callable();
            lua_bind::LinkEvent::load_from_l2c_table(link_event, &event);
            LinkModule::send_event_nodes_struct(fighter.module_accessor, *WEAPON_LINK_NO_CONSTRAINT, link_event, 0);
            event = store_event_table(link_event);
            let deleter: extern "C" fn(*mut app::LinkEvent) = std::mem::transmute(*((*(link_event as *const u64) + 0x8) as *const u64));
            deleter(link_event);

            let motion_2nd = fighter.get_int64(*FIGHTER_SHIZUE_STATUS_WORK_ID_SPECIAL_S_INT_MOTION_KIND_2ND) as i64;
            if motion_2nd != hash40("invalid") as i64 {
                MotionModule::set_rate_2nd(fighter.module_accessor, MotionModule::rate(fighter.module_accessor));
            }
        }

        return 0.into();
    }

    let reel_stop_threshold = fighter.get_param_float("param_special_s", "reel_stop_threshold");
    let stop_frame = fighter.get_int(*FIGHTER_SHIZUE_STATUS_WORK_ID_SPECIAL_S_INT_REEL_STOP_FRAME);
    if line_length <= reel_stop_threshold {
        if stop_frame >= 0 {
            let reel_rate = fighter.get_float(*FIGHTER_SHIZUE_STATUS_WORK_ID_SPECIAL_S_FLOAT_REEL_MOTION_RATE);
            MotionModule::set_rate(fighter.module_accessor, reel_rate);

            fighter.set_int(-1, *FIGHTER_SHIZUE_STATUS_WORK_ID_SPECIAL_S_INT_REEL_STOP_FRAME);
        }
    } else {
        if stop_frame < 0 {
            let reel_stop_motion_rate = fighter.get_param_float("param_special_s", "reel_stop_motion_rate");
            MotionModule::set_rate(fighter.module_accessor, reel_stop_motion_rate);
            
            let reel_stop_frame_limit = fighter.get_param_int("param_special_s", "reel_stop_frame_limit");
            fighter.set_int(reel_stop_frame_limit, *FIGHTER_SHIZUE_STATUS_WORK_ID_SPECIAL_S_INT_REEL_STOP_FRAME);
        }
    }

    let motion_2nd = fighter.get_int64(*FIGHTER_SHIZUE_STATUS_WORK_ID_SPECIAL_S_INT_MOTION_KIND_2ND);
    if motion_2nd != hash40("invalid") {
        MotionModule::set_rate_2nd(fighter.module_accessor, MotionModule::rate(fighter.module_accessor));
    }

    // conditions for changing to the CUT status
    if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
    || (SlowModule::is_skip(fighter.module_accessor)
    && WorkModule::count_down_int(fighter.module_accessor, *FIGHTER_SHIZUE_STATUS_WORK_ID_SPECIAL_S_INT_REEL_STOP_FRAME, 0)) 
    {
        //println!("cut");
        fighter.change_status(FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_CUT.into(), false.into());
      
        return 0.into();
    }
    
    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_pre);
    agent.status(Main, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_REEL, special_s_reel_main);
}