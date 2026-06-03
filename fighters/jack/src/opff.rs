// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn wings_of_rebellion_cancel(fighter: &mut L2CFighterCommon) {
    if fighter.is_status_one_of(&[
        *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI2_RUSH,
        *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI2_END
    ])
    && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
    && !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD | *COLLISION_KIND_MASK_PARRY)
    && !fighter.is_in_hitlag() {
        if fighter.check_aerial_cancel()
        || fighter.check_airdodge_cancel() {
            VarModule::on_flag(fighter.battle_object, vars::common::instance::UP_SPECIAL_CANCEL);
            KineticModule::mul_speed(fighter.module_accessor, &Vector3f{x: 0.7, y: 0.7, z: 0.7}, *FIGHTER_KINETIC_ENERGY_ID_STOP);
            return;
        }
    }
}

/// Gets the last damage dealt and adds it to rebel's guage
unsafe fn damage_to_meter(fighter: &mut L2CFighterCommon) {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        VarModule::set_float(fighter.battle_object, vars::common::instance::LAST_ATTACK_DAMAGE_DEALT, 0.0);
        return;
    }

    // Exit if the last dealt damage was 0.0 or if we currently have Arsene out
    let last_damage = VarModule::get_float(fighter.battle_object, vars::common::instance::LAST_ATTACK_DAMAGE_DEALT);
    if last_damage == 0.0 {
        return;
    }

    app::FighterSpecializer_Jack::add_rebel_gauge(fighter.module_accessor, app::FighterEntryID(fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID)), last_damage);

    // Set the const to 0.0 since we don't have a different way to detect when we hit someone
    // (need to implement something beter for this, probably in MeterModule refactor)
    VarModule::set_float(fighter.battle_object, vars::common::instance::LAST_ATTACK_DAMAGE_DEALT, 0.0);
}

unsafe fn arsene_dtilt_motion_change(fighter: &mut L2CFighterCommon) {
    if fighter.is_motion(Hash40::new("attack_lw3")) && !fighter.is_flag(*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_lw3_ex"), 1.0, 1.0, false, 0.0, false, false);
    }
}

unsafe fn training_mode_full_meter(fighter: &mut L2CFighterCommon) {
    if app::smashball::is_training_mode()
    && fighter.is_status(*FIGHTER_STATUS_KIND_APPEAL)
    && fighter.is_button_on(Buttons::Guard) {
        app::FighterSpecializer_Jack::add_rebel_gauge(fighter.module_accessor, app::FighterEntryID(fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID)), 100.0);
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI2_END,
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    wings_of_rebellion_cancel(fighter);
    fastfall_specials(fighter);
    damage_to_meter(fighter);
    arsene_dtilt_motion_change(fighter);
    training_mode_full_meter(fighter);

    // Lengthen knife
	ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("knife"), &Vector3f::new(1.01, 1.1, 1.01));
}

pub extern "C" fn jack_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		jack_frame(fighter)
    }
}

pub unsafe fn jack_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, jack_frame_wrapper);
}