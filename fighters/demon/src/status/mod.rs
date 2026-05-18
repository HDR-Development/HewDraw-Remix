use super::*;
use globals::*;
// status script import

pub unsafe extern "C" fn demon_attack_loop_common(fighter: &mut L2CFighterCommon, status: L2CValue) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return true.into();
        }
    }

    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return true.into();
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(status, false.into());
        return true.into();
    }

    false.into()
}

mod wait;
mod squat_wait;
mod walk;
mod dash;
mod dash_back;
mod turn_dash;
mod landing;
mod landing_attack_air;
mod down;

mod attack;
mod attack_combo;

mod attack_s3;
mod attack_hi3;
mod attack_lw3;

mod attack_stand_1;

mod attack_squat_2;
mod attack_squat_4;

// mod attack_step_2s;

mod attack_air;

mod special_hi;

mod cancel_step;

mod catch;

pub fn install(agent: &mut Agent) {
    wait::install(agent);
    squat_wait::install(agent);
    walk::install(agent);
    dash::install(agent);
    dash_back::install(agent);
    turn_dash::install(agent);
    landing::install(agent);
    landing_attack_air::install(agent);
    down::install(agent);

    attack::install(agent);
    attack_combo::install(agent);

    attack_s3::install(agent);
    attack_hi3::install(agent);
    attack_lw3::install(agent);

    attack_stand_1::install(agent);

    attack_squat_2::install(agent);
    attack_squat_4::install(agent);

    // attack_step_2s::install(agent);

    attack_air::install(agent);

    special_hi::install(agent);

    cancel_step::install(agent);

    catch::install(agent);
}