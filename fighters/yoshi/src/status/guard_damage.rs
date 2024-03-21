use super::*;
use globals::*;

// FIGHTER_STATUS_KIND_GUARD_DAMAGE //

pub unsafe extern "C" fn guard_damage_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_ftStatusUniqProcessGuardDamage_initStatus_Inner();
    smashline::original_status(Init, fighter, *FIGHTER_STATUS_KIND_GUARD_DAMAGE)(fighter)
}

unsafe extern "C" fn init_guard_damage_uniq(fighter: &mut L2CFighterCommon) {
    let shield_radius =
        WorkModule::get_param_float(fighter.module_accessor, hash40("shield_radius"), 0);
    let throw_scale: Vector3f = Vector3f {
        x: shield_radius,
        y: shield_radius,
        z: shield_radius,
    };

    ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("throw"), &throw_scale);

    fighter.clear_lua_stack();
    lua_args!(fighter, 0x2dc1210b69i64);
    app::sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
    fighter.pop_lua_stack(0);
}

pub unsafe extern "C" fn guard_damage_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_ftStatusUniqProcessGuardDamage_exitStatus_common();
    smashline::original_status(Exit, fighter, *FIGHTER_STATUS_KIND_GUARD_DAMAGE)(fighter)
}

pub unsafe extern "C" fn guard_damage_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_GuardDamage_common(L2CValue::Bool(false));
    fighter.sub_shift_status_main(L2CValue::Ptr(
        L2CFighterCommon_status_GuardDamage_Main as *const () as _,
    ))
}

pub fn install(agent: &mut Agent) {
    agent.status(Init, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, guard_damage_init);
    agent.status(Exit, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, guard_damage_exit);
    agent.status(Main, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, guard_damage_main);
}
