use super::*;

unsafe extern "C" fn special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ArticleModule::is_generatable(fighter.module_accessor, *FIGHTER_SHEIK_GENERATE_ARTICLE_EXPLOSIONBOMB) {
        ArticleModule::generate_article_have_item(fighter.module_accessor, *FIGHTER_SHEIK_GENERATE_ARTICLE_EXPLOSIONBOMB, *FIGHTER_HAVE_ITEM_WORK_EXTRA, Hash40::new("havel"));
    }
    else {
        fighter.on_flag(*FIGHTER_SHEIK_STATUS_SPECIAL_S_FLAG_FAIL);
    }

    fighter.main_shift(special_s_main_loop)
}

unsafe extern "C" fn special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_flag(*FIGHTER_SHEIK_STATUS_SPECIAL_S_FLAG_THROW) {
        if !fighter.is_flag(*FIGHTER_SHEIK_STATUS_SPECIAL_S_FLAG_FAIL) {
            let mut throw_angle = fighter.get_param_float("param_special_s", "throw_angle");
            let angle_stick_y_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_s.throw_angle_stick_y_mul");
            throw_angle += ControlModule::get_stick_y(fighter.module_accessor) * angle_stick_y_mul;
            let mut throw_speed = fighter.get_param_float("param_special_s", "throw_speed");
            let speed_stick_y_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_special_s.throw_speed_stick_y_mul");
            throw_speed -= ControlModule::get_stick_y(fighter.module_accessor) * speed_stick_y_mul;
            fighter.clear_lua_stack();
            fighter.push_lua_stack(&mut L2CValue::I32(*MA_MSC_CMD_ITEM_THROW_ITEM));
            fighter.push_lua_stack(&mut L2CValue::F32(throw_angle));
            fighter.push_lua_stack(&mut L2CValue::F32(throw_speed));
            fighter.push_lua_stack(&mut L2CValue::F32(1.0));
            fighter.push_lua_stack(&mut L2CValue::I32(*FIGHTER_HAVE_ITEM_WORK_EXTRA));
            fighter.push_lua_stack(&mut L2CValue::Bool(true));
            sv_module_access::item(fighter.lua_state_agent);
            fighter.pop_lua_stack(1);
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_SHOOT_NUM);
        }
        fighter.off_flag(*FIGHTER_SHEIK_STATUS_SPECIAL_S_FLAG_THROW);
    }
    if StatusModule::is_changing(fighter.module_accessor) {
        special_s_change_motion(fighter, false);
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        special_s_change_motion(fighter, true);
    }
    if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        fighter.on_flag(*FIGHTER_SHEIK_STATUS_SPECIAL_S_FLAG_SKIP_HOLD);
    }
    if fighter.is_flag(*FIGHTER_SHEIK_STATUS_SPECIAL_S_FLAG_FAIL) {
        let mut throw_frame = 16.0;
        if MotionModule::is_flag_start_1_frame(fighter.module_accessor) {
            throw_frame -= 1.0;
        }
        if fighter.motion_frame() >= throw_frame {
            fighter.change_status(FIGHTER_SHEIK_STATUS_KIND_SPECIAL_S_END.into(), false.into());
            return 1.into();
        }
    }
    else {
        if MotionModule::is_end(fighter.module_accessor) {
            let mut next_status;
            if fighter.is_flag(*FIGHTER_SHEIK_STATUS_SPECIAL_S_FLAG_SKIP_HOLD) {
                next_status = FIGHTER_SHEIK_STATUS_KIND_SPECIAL_S_FIRE;
            }
            else {
                next_status = FIGHTER_SHEIK_STATUS_KIND_SPECIAL_S_HOLD;
            }
            fighter.change_status(next_status.into(), false.into());
            return 1.into();
        }
    }
    if !fighter.is_flag(*FIGHTER_SHEIK_STATUS_SPECIAL_S_FLAG_FAIL) {
        let effect_handle = fighter.get_int(*FIGHTER_SHEIK_STATUS_SPECIAL_S_WORK_INT_EFFECT_HANDLE);
        if effect_handle == 0 {
            let effect = EffectModule::req(fighter.module_accessor, Hash40::new("sheik_sakuretu_line_t"), &Vector3f::zero(), &Vector3f::zero(), 1.0, (*EFFECT_SUB_ATTRIBUTE_SYNC_INIT_POS | *EFFECT_SUB_ATTRIBUTE_SYNC_STOP) as u32, 0, false, 0);
            fighter.set_int(effect as i32, *FIGHTER_SHEIK_STATUS_SPECIAL_S_WORK_INT_EFFECT_HANDLE);
        }
        fighter.clear_lua_stack();
        fighter.push_lua_stack(&mut L2CValue::new_int(0x270a162471));
        fighter.push_lua_stack(&mut L2CValue::new_int(0x5e008fd84));
        sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
        fighter.pop_lua_stack(1);
    }

    return 0.into();
}

unsafe fn special_s_change_motion(fighter: &mut L2CFighterCommon, inherit: bool) {
    fighter.ground_correct_by_situation(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP, *GROUND_CORRECT_KIND_AIR);
    fighter.change_kinetic_by_situation(*FIGHTER_KINETIC_TYPE_SHEIK_SPECIAL_S, *FIGHTER_KINETIC_TYPE_SHEIK_SPECIAL_AIR_S);
    if inherit {
        fighter.change_motion_inherit_frame_by_situation("special_s", "special_air_s", -1.0, 1.0, 0.0, false, false);
    }
    else {
        fighter.change_motion_by_situation("special_s", "special_air_s", 0.0, 1.0, false, 0.0, false, false);
    }
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_main);
}