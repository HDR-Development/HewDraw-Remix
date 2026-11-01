use super::*;

use globals::*;

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_attack_air_inherit_jump_aerial_motion_uniq_process_init_impl,
            sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec_impl
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}

// TAGS: DJC, Double Jump Cancel, FIGHTER_STATUS_KIND_ATTACK_AIR
/// Regular attack air status script except uses the animation's movement by default.
/// To be used by fighters who have double jump cancel
#[utils::export(common::djc)]
pub unsafe extern "C" fn attack_air_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air_common(L2CValue::Bool(false));
    MotionModule::set_trans_move_speed_no_scale(fighter.module_accessor, true);
    fighter.sub_shift_status_main(L2CValue::Ptr(attack_air_main_status_loop as *const () as _))
}

// TAGS: DJC, Double Jump Cancel, FIGHTER_STATUS_KIND_ATTACK_AIR
/// Performs the leniency check for double jump canceling
#[utils::export(common::djc)]
pub unsafe extern "C" fn attack_air_main_status_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.status_AttackAir_Main_common().get_bool() {
        fighter.sub_air_check_superleaf_fall_slowly();
        if !fighter.global_table[IS_STOPPING].get_bool() {
            fighter.sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec_fix_pos();
        }
        0.into()
    }
    else {
        1.into()
    }
}

// TAGS: DJC, Double Jump Cancel, FIGHTER_STATUS_KIND_ATTACK_AIR
/// Inherits the double jump animation movement when doing an aerial (init)
#[skyline::hook(replace = L2CFighterCommon_sub_attack_air_inherit_jump_aerial_motion_uniq_process_init)]
pub unsafe extern "C" fn sub_attack_air_inherit_jump_aerial_motion_uniq_process_init_impl(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[FIGHTER_KIND] == FIGHTER_KIND_DEMON {
        call_original!(fighter)
    } else {
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
        let frame = MotionModule::frame(fighter.module_accessor);

        fighter.sub_attack_air_kind();
        if motion_kind == smash::hash40("jump_aerial_f") || motion_kind == smash::hash40("jump_aerial_b") {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_IGNORE_2ND_MOTION) {
                let rate = if fighter.global_table[FIGHTER_KIND] == FIGHTER_KIND_NESS {
                    ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_jump_aerial.start_motion_rate")
                } else {
                    1.0
                };
                MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new_raw(motion_kind), frame, rate, false, 1.0);
                MotionModule::set_weight(fighter.module_accessor, 1.0, true);
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) || frame < 2.0 {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION_2ND);
                } else {
                    dj_cancel(fighter);
                }
            } else {
                fighter.on_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
            }
        } else {
            fighter.on_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
        }
        let _ = fighter.sub_attack_air_uniq_process_init();
        0.into()
    }
}

#[utils::export(common::djc)]
fn sub_attack_air_inherit_jump_aerial_motion_uniq_process_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    unsafe {
        sub_attack_air_inherit_jump_aerial_motion_uniq_process_init_impl(fighter) 
    }
}

// TAGS: DJC, Double Jump Cancel, FIGHTER_STATUS_KIND_ATTACK_AIR
/// Inherits the double jump animation movement when doing an aerial (exec)
#[skyline::hook(replace = L2CFighterCommon_sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec)]
pub unsafe extern "C" fn sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec_impl(fighter: &mut L2CFighterCommon) -> L2CValue {
    if KineticModule::get_kinetic_type(fighter.module_accessor) == *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION_2ND 
    && fighter.global_table[FIGHTER_KIND] != FIGHTER_KIND_DEMON
    && MotionModule::frame_2nd(fighter.module_accessor) >= 2.0
    && fighter.global_table[CURRENT_FRAME].get_i32() <= ParamModule::get_int(fighter.battle_object, ParamType::Common, "djc_leniency_frame")
    && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
        dj_cancel(fighter);
    }

    if fighter.global_table[FIGHTER_KIND] == FIGHTER_KIND_NESS
    && KineticModule::get_kinetic_type(fighter.module_accessor) == *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION_2ND {
        let frame = MotionModule::frame_2nd(fighter.module_accessor);

        if frame >= 15.0 {
            MotionModule::set_rate_2nd(fighter.module_accessor, 1.0);
        }
        if frame >= 29.0 {
            dj_cancel(fighter);
        }
    }

    call_original!(fighter)
}

unsafe fn dj_cancel(fighter: &mut L2CFighterCommon) {
    fighter.on_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
    let lr = fighter.lr();
    let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
    let djc_limit_speed = ParamModule::get_float(fighter.battle_object, ParamType::Common, "djc_limit_speed");
    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, ENERGY_MOTION_RESET_TYPE_AIR_TRANS_2ND, 0.0, 0.0, 0.0, 0.0, 0.0);
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, speed_x, 0.0);
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed_y.clamp(-djc_limit_speed, djc_limit_speed));
}

#[utils::export(common::djc)]
fn sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    unsafe {
        sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec_impl(fighter)
    }
}