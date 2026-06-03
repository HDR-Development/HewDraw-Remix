// status imports
use super::*;
use globals::*;

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_air_lasso_main,
            sub_air_lasso_hang_uniq
        );
    }

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_air_lasso_main)]
pub unsafe fn status_air_lasso_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::on_flag(fighter.battle_object, vars::common::instance::DISABLE_AIR_LASSO);

    original!()(fighter)
}
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_air_lasso_hang_uniq)]
pub unsafe fn sub_air_lasso_hang_uniq(fighter: &mut L2CFighterCommon, arg: L2CValue) -> L2CValue {
    if arg.get_bool() {
        WorkModule::dec_int(fighter.module_accessor, *FIGHTER_STATUS_AIR_LASSO_HANG_WORK_INT_FRAME);
        WorkModule::count_down_int(fighter.module_accessor, *FIGHTER_STATUS_AIR_LASSO_HANG_WORK_INT_BODY_ANGLE_INTP, 0);
    }
    else {
        // Disable tether canceling for non-tether-upB characters
        if fighter.global_table[FIGHTER_KIND] == FIGHTER_KIND_LUCAS
        || fighter.global_table[FIGHTER_KIND] == FIGHTER_KIND_RICHTER
        || fighter.global_table[FIGHTER_KIND] == FIGHTER_KIND_SAMUS
        || fighter.global_table[FIGHTER_KIND] == FIGHTER_KIND_SAMUSD
        || fighter.global_table[FIGHTER_KIND] == FIGHTER_KIND_SHIZUE
        || fighter.global_table[FIGHTER_KIND] == FIGHTER_KIND_SIMON
        || fighter.global_table[FIGHTER_KIND] == FIGHTER_KIND_SZEROSUIT
        || fighter.global_table[FIGHTER_KIND] == FIGHTER_KIND_TOONLINK
        || fighter.global_table[FIGHTER_KIND] == FIGHTER_KIND_YOUNGLINK {
            return 0.into();
        }

        if fighter.global_table[CURRENT_FRAME].get_i32() < ParamModule::get_int(fighter.battle_object, ParamType::Common, "lasso_hang_fall_disable_frame") {
            return 0.into();
        }

        let prev_stick_y = ControlModule::get_stick_prev_y(fighter.module_accessor);
        let squat_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("squat_stick_y"));
        if prev_stick_y <= squat_stick_y {
            return 0.into();
        }

        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_AIR_LASSO_HANG_FLAG_FALL_ENABLE);
    }

    0.into()
}