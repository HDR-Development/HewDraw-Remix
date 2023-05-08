use super::*;
use globals::*;

#[status_script(agent = "ganon", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS)]
unsafe fn special_s_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND] == FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_CATCH {
        return 0.into();
    }
    if fighter.global_table[STATUS_KIND] == FIGHTER_GANON_STATUS_KIND_SPECIAL_S_CATCH {
        sv_kinetic_energy!(
            clear_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_MOTION
        );
    }
    else {
        let landing_lag = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("explosion_landing_frame_faile"));
        WorkModule::set_float(fighter.module_accessor, landing_lag, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    }
    0.into()
}

pub fn install() {
    smashline::install_status_scripts!(
        special_s_exit,
    );
}