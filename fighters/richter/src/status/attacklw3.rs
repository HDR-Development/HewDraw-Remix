use super::*;
use globals::*;


// FIGHTER_SIMON_STATUS_KIND_ATTACK_LW32_LANDING

#[status_script(agent = "richter", status = FIGHTER_SIMON_STATUS_KIND_ATTACK_LW32_LANDING, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe extern "C" fn attack_lw32_landing_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let landing_lag = WorkModule::get_param_int(fighter.module_accessor, hash40("param_private"), hash40("attack_lw32_landing_frame"));
    let anim_length = MotionModule::end_frame_from_hash(fighter.module_accessor, Hash40::new("attack_lw32_landing"));
    let rate: f32 = if landing_lag > 0 {
        anim_length / landing_lag as f32
    } else {
        1.0
    };
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_lw32_landing"), 0.0, rate, false, 0.0, false, false);
    fighter.main_shift(attack_lw32_landing_main_loop)
}

unsafe extern "C" fn attack_lw32_landing_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) && (fighter.sub_wait_ground_check_common(false.into()).get_bool() || fighter.sub_air_check_fall_common().get_bool()) {
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
            fighter.change_status_req(*FIGHTER_STATUS_KIND_WAIT, false);
        }
        else {
            fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL, false);
        }
        return 1.into();
    }
    // <HDR>
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL, false);
        return 1.into();
    }
    // </HDR>
    0.into()
}

pub fn install() {
    install_status_scripts!(
        attack_lw32_landing_main
    );
}