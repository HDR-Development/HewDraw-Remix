use super::*;

// FIGHTER_STATUS_KIND_GUARD_ON

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

pub unsafe extern "C" fn guard_on_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_IGNORE_2ND_MOTION);
    fighter.sub_status_guard_on_common();
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_IGNORE_2ND_MOTION);
    init_guard_damage_uniq(fighter);
    fighter.main_shift(guard_on_main_loop)
}

unsafe extern "C" fn guard_on_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_status_guard_on_main_air_common().get_bool() {
        return 0.into();
    }

    fighter.sub_guard_cont();

    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND
    && MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_GUARD, false);
        return 1.into();
    }
    else {
        let guard_shield = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
        let min_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_MIN_FRAME);

        if guard_shield > 0.0
        && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
        && min_frame <= 0
        && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
            VarModule::off_flag(
                fighter.object(),
                vars::common::instance::IS_PARRY_FOR_GUARD_OFF,
            );
            fighter.change_status_req(*FIGHTER_STATUS_KIND_GUARD_OFF, true);
            return 1.into();
        }
    }

    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_GUARD_ON, guard_on_main);
}
