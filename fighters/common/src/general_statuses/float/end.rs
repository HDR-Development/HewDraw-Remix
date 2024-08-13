use super::*;

#[no_mangle]
unsafe fn float_end_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR {
        let motion = MotionModule::motion_kind(fighter.module_accessor);
        WorkModule::set_int64(fighter.module_accessor, motion as i64, *FIGHTER_STATUS_ATTACK_AIR_WORK_INT_MOTION_KIND);
    }
    AttackModule::set_shield_stiff_mul(fighter.module_accessor, 1.0);
    0.into()
}
