use super::*;

#[skyline::hook(replace = L2CFighterCommon_status_pre_ShieldBreakFly)]
unsafe fn status_pre_ShieldBreakFly(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.kind() == *FIGHTER_KIND_PURIN {
        return call_original!(fighter);
    }
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_MOTION_BIND,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_ENABLE,
        false,
        false,
        false,
        0,
        (*FIGHTER_STATUS_ATTR_DISABLE_JUMP_BOARD_EFFECT | *FIGHTER_STATUS_ATTR_DAMAGE | *FIGHTER_STATUS_ATTR_DISABLE_SHIELD_RECOVERY) as u32,
        0,
        0
    );
    return false.into();
}

extern "C" {
    #[link_name = "\u{1}_ZN7lua2cpp16L2CFighterCommon34sub_status_shield_break_fly_commonEN3lib8L2CValueE"]
    pub fn L2CFighterCommon_sub_status_shield_break_fly_common(
        this: *mut L2CFighterCommon,
        arg1: L2CValue
    );
}

#[skyline::hook(replace = L2CFighterCommon_sub_status_shield_break_fly_common)]
unsafe fn sub_status_shield_break_fly_common(fighter: &mut L2CFighterCommon, arg1: L2CValue) {
    if fighter.kind() == *FIGHTER_KIND_PURIN {
        return call_original!(fighter, arg1);
    }

    let shield_reset = fighter.get_param_float("common", "shield_reset");
    fighter.set_float(shield_reset, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
    
    // fighter.on_flag(*FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_CHECK_DEAD_AREA_FORCE);
    HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);

    let shield_break_motion = Hash40::new("shield_break_fly");
    let start_frame = 3.0;
    let mut end_frame = FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor, shield_break_motion, true);
    if end_frame == 0.0 {
        end_frame = MotionModule::end_frame_from_hash(fighter.module_accessor, shield_break_motion);
    }
    if end_frame > start_frame {
        end_frame -= start_frame;
    }
    let desired_end_frame = 30.0;
    let rate = end_frame / desired_end_frame;
    MotionModule::change_motion(fighter.module_accessor, shield_break_motion, start_frame, rate, false, 0.0, false, false);

    if arg1.get_bool(){
        SoundModule::play_se(fighter.module_accessor, Hash40::new("se_common_guardbreak"), true, false, false, false, enSEType(0));
        SlowModule::set_whole(fighter.module_accessor, 8, 25);
        EffectModule::req_screen(fighter.module_accessor, Hash40::new("bg_criticalhit"), false, true, true);
    }

    let shield_break_xlu_frame = fighter.get_param_int("common", "shield_break_xlu_frame");
    fighter.set_int(shield_break_xlu_frame, 0x11000017); // *FIGHTER_STATUS_FURAFURA_STAND_WORK_INT_TERMINATE_XLU_COUNT

    fighter.clear_lua_stack();
    lua_args!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_SHIELD_BREAK_FLY_NUM);
    app::sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
    fighter.pop_lua_stack(1);
}

#[skyline::hook(replace = L2CFighterCommon_status_ShieldBreakFly)]
unsafe fn status_ShieldBreakFly(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.kind() == *FIGHTER_KIND_PURIN {
        return call_original!(fighter);
    }
    L2CFighterCommon_sub_status_shield_break_fly_common(fighter, true.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(status_ShieldBreakFly_Main as *const () as _))
}

#[skyline::hook(replace = L2CFighterCommon_status_ShieldBreakFly_Main)]
unsafe fn status_ShieldBreakFly_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.kind() == *FIGHTER_KIND_PURIN {
        return call_original!(fighter);
    }
    if !fighter.is_situation(*SITUATION_KIND_GROUND) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        ControlModule::clear_command(fighter.module_accessor, true);
        return true.into();
    }

    if fighter.status_frame() == 8 {
        EffectModule::remove_screen(fighter.module_accessor, Hash40::new("bg_criticalhit"), 0);
    }

    if MotionModule::is_end(fighter.module_accessor) {
        WorkModule::set_flag(
            fighter.module_accessor,
            MotionModule::is_anim_resource(fighter.module_accessor, Hash40::new("down_spot_u")),
            *FIGHTER_STATUS_DOWN_FLAG_UP
        );
        fighter.change_status(FIGHTER_STATUS_KIND_FURAFURA_STAND.into(), true.into());
        return true.into();
    }

    return false.into();
}

#[skyline::hook(replace = L2CFighterCommon_status_end_ShieldBreakFly)]
unsafe fn status_end_ShieldBreakFly(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.kind() == *FIGHTER_KIND_PURIN {
        return call_original!(fighter);
    }
    return false.into();
}

pub fn install() {
    skyline::install_hooks!(
        status_pre_ShieldBreakFly,
        sub_status_shield_break_fly_common,
        status_ShieldBreakFly,
        status_ShieldBreakFly_Main,
        status_end_ShieldBreakFly
    );
}
