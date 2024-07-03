use super::*;

unsafe extern "C" fn demon_down_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    demon_down_common_pre(fighter);
    let down_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("down_frame"));
    WorkModule::set_float(fighter.module_accessor, down_frame, *FIGHTER_STATUS_DOWN_WORK_FLOAT_DOWN_FRAME);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_LANDING_CANCEL);
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_Down_Main as *const () as _))
}

unsafe extern "C" fn demon_down_common_pre(fighter: &mut L2CFighterCommon) {
    let (motion, rate) = if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_lw") {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_DOWN_FLAG_UP);
        let landing_attack_air_frame_lw = WorkModule::get_param_float(fighter.module_accessor, hash40("landing_attack_air_frame_lw"), 0);
        let end_frame = MotionModule::end_frame_from_hash(fighter.module_accessor, Hash40::new("landing_air_lw"));
        (hash40("landing_air_lw"), end_frame / landing_attack_air_frame_lw)
    }
    else {
        (WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_DOWN_WORK_INT_MOTION_KIND), 1.0)
    };
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new_raw(motion),
        0.0,
        rate,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_down_chk_reflect_wall_init();
    physics!(fighter, MA_MSC_CMD_PHYSICS_SWING_FLOOR_HIT_ALL);
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_DOWN, demon_down_main);
}