use super::*;

// disables art wheel during hitstun, and enables it during jab, tilts and aerials

#[skyline::hook(offset = 0x116a3d0)]
pub unsafe extern "C" fn shulk_check_valid_arts_statuses(fighter: &mut Fighter) -> u64 {
    let module_accessor = (fighter.battle_object).module_accessor;
    let status = StatusModule::status_kind(module_accessor);
    let object = &mut fighter.battle_object;
    if VarModule::is_flag(object, vars::shulk::status::MONADO_BEAT) {
        u64::from([
            *FIGHTER_STATUS_KIND_WAIT,
            *FIGHTER_STATUS_KIND_WALK,
            *FIGHTER_STATUS_KIND_DASH,
            *FIGHTER_STATUS_KIND_RUN,
            *FIGHTER_STATUS_KIND_TURN,
            *FIGHTER_STATUS_KIND_TURN_DASH,
            *FIGHTER_STATUS_KIND_TURN_RUN,
            *FIGHTER_STATUS_KIND_JUMP_SQUAT, // new
            *FIGHTER_STATUS_KIND_JUMP,
            *FIGHTER_STATUS_KIND_JUMP_AERIAL,
            *FIGHTER_STATUS_KIND_FLY,
            *FIGHTER_STATUS_KIND_FALL,
            *FIGHTER_STATUS_KIND_FALL_AERIAL,
            *FIGHTER_STATUS_KIND_LANDING,
            *FIGHTER_STATUS_KIND_LANDING_LIGHT,
            *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR,
            *FIGHTER_STATUS_KIND_LANDING_DAMAGE_LIGHT,
            *FIGHTER_STATUS_KIND_ATTACK, // new
            *FIGHTER_STATUS_KIND_ATTACK_100, // new
            *FIGHTER_STATUS_KIND_ATTACK_S3, // new
            *FIGHTER_STATUS_KIND_ATTACK_HI3, // new
            *FIGHTER_STATUS_KIND_ATTACK_LW3, // new
            *FIGHTER_STATUS_KIND_ATTACK_AIR, // new
            *FIGHTER_STATUS_KIND_DAMAGE_FALL,
            *FIGHTER_STATUS_KIND_OTTOTTO_WAIT,
            *FIGHTER_STATUS_KIND_ITEM_SCREW_JUMP,
            *FIGHTER_STATUS_KIND_ITEM_SCREW_JUMP_AERIAL,
            *FIGHTER_STATUS_KIND_ITEM_SCREW_FALL,
            *FIGHTER_STATUS_KIND_GIMMICK_SPRING_JUMP,
            *FIGHTER_STATUS_KIND_ITEM_ROCKETBELT_HOVER_KEEP,
            *FIGHTER_STATUS_KIND_KILLER_JUMP,
        ].contains(&status))
    }
    else {    
        u64::from([
            *FIGHTER_STATUS_KIND_WAIT,
            *FIGHTER_STATUS_KIND_WALK,
            *FIGHTER_STATUS_KIND_DASH,
            *FIGHTER_STATUS_KIND_RUN,
            *FIGHTER_STATUS_KIND_TURN,
            *FIGHTER_STATUS_KIND_TURN_DASH,
            *FIGHTER_STATUS_KIND_TURN_RUN,
            *FIGHTER_STATUS_KIND_JUMP_SQUAT, // new
            *FIGHTER_STATUS_KIND_JUMP,
            *FIGHTER_STATUS_KIND_JUMP_AERIAL,
            *FIGHTER_STATUS_KIND_FLY,
            *FIGHTER_STATUS_KIND_FALL,
            *FIGHTER_STATUS_KIND_FALL_AERIAL,
            *FIGHTER_STATUS_KIND_LANDING,
            *FIGHTER_STATUS_KIND_LANDING_LIGHT,
            *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR,
            *FIGHTER_STATUS_KIND_LANDING_DAMAGE_LIGHT,
            *FIGHTER_STATUS_KIND_DAMAGE_FALL,
            *FIGHTER_STATUS_KIND_OTTOTTO_WAIT,
            *FIGHTER_STATUS_KIND_ITEM_SCREW_JUMP,
            *FIGHTER_STATUS_KIND_ITEM_SCREW_JUMP_AERIAL,
            *FIGHTER_STATUS_KIND_ITEM_SCREW_FALL,
            *FIGHTER_STATUS_KIND_GIMMICK_SPRING_JUMP,
            *FIGHTER_STATUS_KIND_ITEM_ROCKETBELT_HOVER_KEEP,
            *FIGHTER_STATUS_KIND_KILLER_JUMP,
        ].contains(&status))
    }
}

pub fn install() {
    skyline::install_hooks!(
        shulk_check_valid_arts_statuses,
    );
}