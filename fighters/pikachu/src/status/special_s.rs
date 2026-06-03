use super::*;
use globals::*;
// status script import

unsafe extern "C" fn special_s_hold_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_PIKACHU_STATUS_WORK_ID_INT_SKULL_BASH_HOLD_COUNT);

    return smashline::original_status(Init, fighter, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_HOLD)(fighter);
}

unsafe extern "C" fn special_s_attack(fighter: &mut L2CFighterCommon) -> L2CValue {

    if fighter.is_situation(*SITUATION_KIND_AIR) {
        VarModule::on_flag(fighter.battle_object, vars::pikachu::instance::DISABLE_SPECIAL_S);
    }

    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_PIKACHU_STATUS_WORK_ID_FLAG_SKULL_BASH_HIT);

    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    StatusModule::set_situation_kind(fighter.module_accessor, app::SituationKind(*SITUATION_KIND_AIR), false);

    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s"), 0.0, 1.0, false, 0.0, false, false);
    GroundModule::select_cliff_hangdata(fighter.module_accessor, *FIGHTER_PICHU_CLIFF_HANG_DATA_SPECIAL_S as u32);

    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_attack_main_loop as *const () as _))

    //return smashline::original_status(Main, fighter, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_ATTACK)(fighter);
}

unsafe extern "C" fn special_s_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return true.into();
    }

    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PIKACHU_STATUS_WORK_ID_FLAG_SKULL_BASH_ATTACK_POWER_MODIFY) {
        /*let min_power = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("special_s_power_min_"));
        let max_power = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("special_s_power_tame_"));
        let max_charge = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("special_s_tame_time_"));

        let charge_frames = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKACHU_STATUS_WORK_ID_INT_SKULL_BASH_HOLD_COUNT);
        let range = max_power - min_power;
        let power_per_frame = range / max_charge;
        let real_power = min_power + (power_per_frame * charge_frames.to_f32());
        AttackModule::set_power(fighter.module_accessor, 0, real_power, false);*/

        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PIKACHU_STATUS_WORK_ID_FLAG_SKULL_BASH_ATTACK_POWER_MODIFY);
    }

    let mut touch_wall = false;
    let bounce = Vector3f::new(2.5, 1.2, 0.0);
    let gravity = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("special_lw_air_yaccel_"));
    if PostureModule::lr(fighter.module_accessor) > 0.0 {
        touch_wall = GroundModule::is_wall_touch_line(fighter.module_accessor, *GROUND_TOUCH_FLAG_RIGHT as u32);
    } else {
        touch_wall = GroundModule::is_wall_touch_line(fighter.module_accessor, *GROUND_TOUCH_FLAG_LEFT as u32);
    }

    if touch_wall {
        /*KineticModule::clear_speed_all(fighter.module_accessor);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, gravity);
        KineticModule::add_speed(fighter.module_accessor, &bounce);*/

        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PIKACHU_STATUS_WORK_ID_FLAG_SKULL_BASH_HIT);
        
        EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("head"), 1, 0, 0, 0, 0, 90, 0.7, 0, 0, 0, 0, 0, 0, false);

        SoundModule::play_se(fighter.module_accessor, Hash40::new("se_common_down_m_01"), true, false, false, false, enSEType(0));
        fighter.change_status(FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_END.into(), false.into());
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_END.into(), false.into());
    }

    let x_speed = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let direction = PostureModule::lr(fighter.module_accessor);
    let mut stop = false;
    
    if (direction > 0.0) {
        if x_speed < 0.0 {
            stop = true;
        }
    } else {
        if x_speed > 0.0 {
            stop = true;
        }
    }

    if (stop == true) {
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);

        lua_args!(fighter,FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        app::sv_kinetic_energy::set_accel(fighter.lua_state_agent);
    }

    return false.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Init, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_HOLD, special_s_hold_init);
    agent.status(Main, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_ATTACK, special_s_attack);
}