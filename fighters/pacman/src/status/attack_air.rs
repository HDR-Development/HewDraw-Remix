use super::*;

pub unsafe extern "C" fn attack_air_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.object(), vars::pacman::instance::TRAMPOLINE_AERIAL_USED) {
        if [
            *FIGHTER_STATUS_KIND_LANDING,
            *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR,
        ].contains(&fighter.global_table[STATUS_KIND].get_i32()) {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
        }
        VarModule::off_flag(fighter.object(), vars::pacman::instance::TRAMPOLINE_AERIAL_USED);
    }

    smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_ATTACK_AIR)(fighter)
}

pub unsafe extern "C" fn attack_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air();
    fighter.sub_shift_status_main(L2CValue::Ptr(attack_air_main_loop as *const () as _))
}

unsafe extern "C" fn attack_air_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.object(), vars::pacman::instance::TRAMPOLINE_AERIAL_USED)
    && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) {
        VarModule::off_flag(fighter.object(), vars::pacman::instance::TRAMPOLINE_AERIAL_USED)
    }
    if !status_AttackAir_Main(fighter).get_bool() {
        // something from common impl (thanks Suddy)
        fighter.sub_air_check_superleaf_fall_slowly();
        if !fighter.global_table[IS_STOPPING].get_bool() {
            app::FighterUtil::check_cloud_through_out(fighter.module_accessor);
        }
    }

    false.into()
}

unsafe extern "C" fn status_AttackAir_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.attack_air_common_strans().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && !VarModule::is_flag(fighter.object(), vars::pacman::instance::TRAMPOLINE_AERIAL_USED) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if VarModule::is_flag(fighter.object(), vars::pacman::instance::TRAMPOLINE_AERIAL_USED) {
            EffectModule::req_on_joint(fighter.module_accessor, Hash40::new("pacman_change_start"), Hash40::new("pizzapacman"), &Vector3f::zero(), &Vector3f::zero(), 1.0, &Vector3f::zero(), &Vector3f::zero(), true, 0, 0, 0);
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        return 1.into();
    }

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(End, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_end);
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_main);
}