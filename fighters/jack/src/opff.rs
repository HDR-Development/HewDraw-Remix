// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;


/// Allows Joker to cancel Wings of Rebellion by pressing Shield during the move
unsafe fn wings_cancel(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_JACK_STATUS_KIND_SPECIAL_HI2_RUSH)
    && fighter.is_button_on(Buttons::Guard) {
        fighter.change_status_req(*FIGHTER_JACK_STATUS_KIND_SPECIAL_HI2_END, true);
    }
}

/// Changes Joker's aerial grappling hook to also have the pull down hitbox that grounded grappling hook does
/// TODO: Change this to be in ACMD instead, if possible
unsafe fn aerial_grappling_hook(fighter: &mut L2CFighterCommon) {
    if fighter.is_motion(Hash40::new("special_hi_start")) {
        MotionModule::change_motion_kind(fighter.module_accessor, Hash40::new("special_hi"));
    }
}

/// Disables gravity during the first 36 frames of Joker's grapple throw if he is in air.
/// This prevents Joker from dying due to gravity
unsafe fn aerial_grappling_hook_stall(fighter: &mut L2CFighterCommon) {
    if !fighter.is_motion(Hash40::new("special_air_hi_throw")) {
        return;
    }

    if fighter.motion_frame() < 36.0 {
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    } else {
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
}

/// Gets the last damage dealt and adds it to rebel's guage
unsafe fn damage_to_meter(fighter: &mut L2CFighterCommon) {
    const MULTIPLIER: f32 = 2.0;

    // Exit if the last dealt damage was 0.0 or if we currently have Arsene out
    let last_damage = VarModule::get_float(fighter.battle_object, vars::common::LAST_ATTACK_DAMAGE_DEALT);
    if last_damage == 0.0 || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        return;
    }

    // TODO: Add this as a real lua_const for FIGHTER_JACK_INSTANCE_WORK_ID_FLOAT_REBEL_GAUGE
    WorkModule::add_float(fighter.module_accessor, last_damage * MULTIPLIER, 0x4D);

    // Set the const to 0.0 since we don't have a different way to detect when we hit someone
    // (need to implement something beter for this, probably in MeterModule refactor)
    VarModule::set_float(fighter.battle_object, vars::common::LAST_ATTACK_DAMAGE_DEALT, 0.0);
}

#[utils::macros::opff(FIGHTER_KIND_JACK )]
pub unsafe fn jack_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    common::opff::fighter_common_opff(fighter);
    wings_cancel(fighter);
    aerial_grappling_hook(fighter);
    aerial_grappling_hook_stall(fighter);
    damage_to_meter(fighter);
    // Sets Joker's knife to be a little bit longer
    ModelModule::set_joint_scale(
        fighter.module_accessor,
        Hash40::new("knife"),
        &Vector3f::new(1.01, 1.1, 1.01)
    );
}