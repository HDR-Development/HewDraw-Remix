use super::*;

pub unsafe fn update_special_lw_by_situation(
    fighter: &mut L2CFighterCommon,
    allow_change_motion: bool,
    grounded_motion: Hash40,
    air_motion: Hash40,
    in_air_callback: unsafe fn(&mut L2CFighterCommon) -> L2CValue
) 
{
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        if allow_change_motion {
            MotionModule::change_motion(fighter.module_accessor, grounded_motion, 0.0, 1.0, false, 0.0, false, false);
        }    
    } else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        let _ = in_air_callback(fighter); // We don't care about the result of this according to the vanilla status script
        if allow_change_motion {
            MotionModule::change_motion(fighter.module_accessor, air_motion, 0.0, 1.0, false, 0.0, false, false);
        } else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, grounded_motion, -1.0, 1.0, 0.0, false, false);
        }
    }
}

pub unsafe fn update_special_lw_in_air(fighter: &mut L2CFighterCommon) -> L2CValue {
    let gravity = -fighter.get_param_float("param_special_lw", "air_accel_y");
    
    // Set the acceleration, TODO: Make this a macro since we use sv_kinetic_energy a lot
    sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, gravity);

    let stable_speed = fighter.get_param_float("param_special_lw", "air_speed_max_y");
    
    // Set the stable fall speed
    sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, stable_speed);

    0.into()
}

#[status_script(agent = "jack", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn special_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if super::special_check_summon(fighter) {
        return 1.into();
    }

    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
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
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );

    0.into()
}

unsafe extern "C" fn status_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_JACK_STATUS_KIND_SPECIAL_LW_HOLD.into(), false.into());
        return 0.into();
    }

    if fighter.global_table[globals::CURRENT_FRAME].get_i32() >= 1
    && fighter.is_button_trigger(Buttons::Special) 
    && super::summon_arsene(fighter) {
        fighter.change_status(FIGHTER_JACK_STATUS_KIND_SUMMON.into(), false.into());
        return 0.into();
    }
    
    if fighter.is_flag(*FIGHTER_JACK_STATUS_SPECIAL_LW_FLAG_GUARD_START) {
        fighter.off_flag(*FIGHTER_JACK_STATUS_SPECIAL_LW_FLAG_GUARD_START);
        fighter.on_flag(*FIGHTER_JACK_STATUS_SPECIAL_LW_FLAG_GUARD_START);
        let hit_stop_mul = fighter.get_param_float("param_special_lw", "hit_stop_mul");
        HitModule::set_hit_stop_mul(fighter.module_accessor, hit_stop_mul, app::HitStopMulTarget { _address: *HIT_STOP_MUL_TARGET_ALL as u8 }, 0.0);
        DamageModule::set_no_reaction_mode_status(fighter.module_accessor, app::DamageNoReactionMode { _address: *DAMAGE_NO_REACTION_MODE_ALWAYS as u8 }, -1.0, -1.0, -1);
    }

    if StatusModule::is_changing(fighter.module_accessor) {
        return 0.into();
    }

    if StatusModule::is_situation_changed(fighter.module_accessor) {
        update_special_lw_by_situation(
            fighter,
            false,
            Hash40::new("special_lw_start"),
            Hash40::new("special_air_lw_start"),
            update_special_lw_in_air
        );
    }
    0.into()
}

#[status_script(agent = "jack", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    update_special_lw_by_situation(
        fighter,
        true,
        Hash40::new("special_lw_start"),
        Hash40::new("special_air_lw_start"),
        update_special_lw_in_air
    );

    let speed_mul = fighter.get_param_float("param_special_lw", "start_speed_mul_x");
    let speed_x = fighter.get_speed_x(*FIGHTER_KINETIC_ENERGY_ID_STOP) * speed_mul;

    fighter.set_speed(Vector2f::new(speed_x, 0.0), *FIGHTER_KINETIC_ENERGY_ID_STOP);

    if fighter.is_situation(*SITUATION_KIND_AIR) {
        let mul = fighter.get_param_float("param_special_lw", "air_start_speed_mul_y");
        let speed_y = fighter.get_speed_y(*FIGHTER_KINETIC_ENERGY_ID_GRAVITY) * mul;

        fighter.set_speed(Vector2f::new(speed_x, speed_y), *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }

    let hold_frame = fighter.get_param_int("param_special_lw", "hold_frame");
    fighter.set_int(hold_frame, *FIGHTER_JACK_STATUS_SPECIAL_LW_INT_HOLD_FRAME);
    
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_13 - 1);

    fighter.main_shift(status_lw_main_loop)
}

#[status_script(agent = "jack", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn special_lw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[globals::STATUS_KIND] != FIGHTER_JACK_STATUS_KIND_SPECIAL_LW_HOLD 
    && fighter.global_table[globals::STATUS_KIND] != FIGHTER_JACK_STATUS_KIND_SPECIAL_LW_ENDURE
    {
        DamageModule::set_damage_mul_2nd(fighter.module_accessor, 1.0);
    }

    0.into()
}

pub fn install() {
    smashline::install_status_scripts!(
        special_lw_pre,
        special_lw_main,
        special_lw_end
    );
}