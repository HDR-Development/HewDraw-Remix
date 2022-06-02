// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn hit_cancel_blade_switch(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if (fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
        *FIGHTER_STATUS_KIND_ATTACK_AIR,
        *FIGHTER_STATUS_KIND_ATTACK_DASH,
        *FIGHTER_STATUS_KIND_ATTACK_S4,
        *FIGHTER_STATUS_KIND_ATTACK_HI4,
        *FIGHTER_STATUS_KIND_ATTACK_LW4,
        *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_HI_JUMP
    ]) || fighter.is_motion(Hash40::new("attack_13")))
    && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
    && fighter.is_cat_flag(Cat1::SpecialLw)
    && !fighter.is_in_hitlag() {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_SPECIAL_LW, true);
    }
}
#[utils::macros::opff(FIGHTER_KIND_ELIGHT )]
pub unsafe fn elight_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    common::opff::fighter_common_opff(fighter);
    hit_cancel_blade_switch(fighter);
}
