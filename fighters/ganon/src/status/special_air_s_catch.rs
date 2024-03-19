use super::*;
use globals::*;

unsafe extern "C" fn special_air_s_catch_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::set_rate(fighter.module_accessor, 0.0);
    fighter.main_shift(special_air_s_catch_main_loop)
}

unsafe extern "C" fn special_air_s_catch_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[globals::CURRENT_FRAME].get_f32() == 1.0 {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_air_s_catch"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        ganon_set_air(fighter);
        let speed_x = WorkModule::get_float(fighter.module_accessor, *FIGHTER_GANON_STATUS_WORK_ID_FLOAT_EXPLOSION_AIR_SPEED_X);
        let speed_y = WorkModule::get_float(fighter.module_accessor, *FIGHTER_GANON_STATUS_WORK_ID_FLOAT_EXPLOSION_AIR_SPEED_Y);
        KineticModule::clear_speed_attr(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            speed_y
        );
        let lr = PostureModule::lr(fighter.module_accessor);
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            speed_x * lr,
            0.0
        );
        KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
    if fighter.global_table[globals::CURRENT_FRAME].get_f32() > 1.0 {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_FALL.into(), false.into());
            return 1.into();
        }
        if fighter.global_table[globals::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END.into(), false.into());
            return 1.into();
        }
    }
    0.into()
}

pub fn install() {
    smashline::Agent::new("ganon")
        .status(
            Main,
            *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_CATCH,
            special_air_s_catch_main,
        )
        .install();
}
