use super::*;
use globals::*;
mod special_hi;
mod special_lw;
 

// FIGHTER_STATUS_KIND_ATTACK_LW4 //

pub unsafe extern "C" fn attack_lw4_main(agent: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK);
    agent.attack_lw4_mtrans();
    WorkModule::enable_transition_term(agent.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_THROW_KIRBY_GROUND);
    if !StopModule::is_stop(agent.module_accessor) {
        agent.status_ThrowKirby_Uniq(L2CValue::Bool(false));
    }
    agent.global_table[SUB_STATUS].assign(&L2CValue::Ptr(smash::lua2cpp::L2CFighterCommon_status_ThrowKirby_Uniq as *const () as _));
    agent.sub_shift_status_main(L2CValue::Ptr(attack_lw4_main_loop as *const () as _))
}

pub unsafe extern "C" fn attack_lw4_main_loop(agent: &mut L2CFighterCommon) -> L2CValue {
    if !CancelModule::is_enable_cancel(agent.module_accessor)
    && !WorkModule::is_enable_transition_term(agent.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_THROW_KIRBY_GROUND)
    && !MotionModule::is_end(agent.module_accessor) {
        agent.sub_status_uniq_process_ThrowKirby_execFixPos();
        return 0.into()
    }
    agent.status_AttackLw4_Main()
}

pub unsafe extern "C" fn attack_lw4_map_correction(agent: &mut L2CFighterCommon) -> L2CValue {
    let frame = MotionModule::frame(agent.module_accessor);
    let prev_frame = MotionModule::prev_frame(agent.module_accessor);
    let start_air_frame = 2.0;
    let fall_start_frame = 19.0;
    let fall_stop_frame = 20.0;
    let landing_frame = 21.0;

    if frame <= fall_start_frame {
        return 0.into()
    }
    if prev_frame < start_air_frame && frame >= start_air_frame {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_THROW_FLAG_START_AIR);
    }
    if agent.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
        if prev_frame < fall_stop_frame && frame >= fall_stop_frame {
            KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            agent.clear_lua_stack();
            lua_args!(agent, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -12.0);
            app::sv_kinetic_energy::set_speed(agent.lua_state_agent);
            agent.clear_lua_stack();
            lua_args!(agent, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0);
            app::sv_kinetic_energy::set_accel_x_mul(agent.lua_state_agent);
            agent.clear_lua_stack();
            lua_args!(agent, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0);
            app::sv_kinetic_energy::set_accel_x_add(agent.lua_state_agent);
            MotionModule::set_frame(agent.module_accessor, fall_stop_frame, true);
            MotionModule::set_rate(agent.module_accessor, 0.0);
        }
    }
    else {
        if frame < landing_frame {
            KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_IGNORE_NORMAL);
            GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            WorkModule::enable_transition_term(agent.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_THROW_KIRBY_GROUND);
            MotionModule::set_frame(agent.module_accessor, landing_frame, true);
            MotionModule::set_rate(agent.module_accessor, 1.0);
        }
    }
    0.into()
}

// handle damage to belly
#[no_mangle]
pub unsafe extern "C" fn krool_belly_damage_hook_impl(damage: f32, fighter: *mut Fighter, unk: bool) {
    let mut battle_object = &mut (*fighter).battle_object;
    let boma = battle_object.module_accessor;
    let mut waist = WorkModule::get_float(boma, 0x4d);  // WAIST_LIFE

    // play belly flash
    WorkModule::on_flag(boma, 0x200000e3);              // *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_WAIST_HIT_FLASH
    WorkModule::set_int(boma, 0x1e, 0x100000c1);        // *FIGHTER_KROOL_INSTANCE_WORK_ID_INT_WAIST_HIT_FLASH_COUNT

    // store incoming damage
    let stored_damage = VarModule::get_float(battle_object, vars::krool::instance::STORED_DAMAGE);
    VarModule::set_float(battle_object, vars::krool::instance::STORED_DAMAGE, f32::min(stored_damage + damage, 45.0));

    if damage > ParamModule::get_float(battle_object, ParamType::Agent, "param_waist.deplete_damage_min") {
        // decrease belly health
        waist -= 1.0;
        WorkModule::set_float(boma, waist, 0x4d);

        // critical zoom if out of health
        if WorkModule::get_float(boma, 0x4d) <= 0.0 {
            MotionAnimcmdModule::call_script_single(boma, *FIGHTER_ANIMCMD_SOUND, Hash40::new("sound_waistbreak"), -1);
        }
        else {
            let krool = utils::util::get_fighter_common_from_accessor(&mut (*boma));
            PLAY_SE(krool, Hash40::new("se_krool_damage_clack"));
        }
    }
    else {
        let krool = utils::util::get_fighter_common_from_accessor(&mut (*boma));
        PLAY_SE(krool, Hash40::new("se_krool_special_n11"));    //s07 l01, l02 l05
    }

    // disable belly for the rest of the move
    WorkModule::on_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_OFF);
}

// handle toggling belly on/off
// #[no_mangle]
// pub unsafe extern "C" fn krool_belly_toggle_hook(ctx: &mut skyline::hooks::InlineCtx) {
//     *ctx.registers[0].x.as_mut() = 0;    // bool for toggle
//     // ...as_mut() &= logic
// }

pub fn install(agent: &mut Agent) {
    special_hi::install(agent);
    special_lw::install(agent);
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_LW4, attack_lw4_main);
    agent.status(MapCorrection, *FIGHTER_STATUS_KIND_ATTACK_LW4, attack_lw4_map_correction);
}