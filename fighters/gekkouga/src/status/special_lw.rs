use super::*;

#[no_mangle]
pub unsafe extern "C" fn gekkouga_get_sub_id(battle_object: *mut BattleObject) -> u32 {
    let sub_const = if VarModule::is_flag(battle_object, vars::gekkouga::instance::SPECIAL_LW_IS_DOLL) {
        0x100000C2
    }
    else {
        0x100000C1
    };
    WorkModule::get_int((*battle_object).module_accessor, sub_const) as u32
}

unsafe extern "C" fn special_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        0,
        0,
        0,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW |
            *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON
        ) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn special_lw_init(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_SAVE_SPEED);
    let doll_id = gekkouga_get_sub_id(fighter.battle_object);
    if !sv_battle_object::is_active(doll_id) {
        VarModule::on_flag(fighter.battle_object, vars::gekkouga::status::SPECIAL_LW_SPAWN_SUB);
    }
    if VarModule::is_flag(fighter.battle_object, vars::gekkouga::instance::SPECIAL_LW_CAN_TELEPORT) {
        VarModule::on_flag(fighter.battle_object, vars::gekkouga::status::SPECIAL_LW_TELEPORT_OK);
    }
    VarModule::set_int(fighter.battle_object, vars::gekkouga::status::SPECIAL_LW_VANISH_TIMER, 15);
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_lw"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );

    if !StopModule::is_stop(fighter.module_accessor) {
        special_lw_substatus(fighter, false.into());
    }
    fighter.global_table[globals::SUB_STATUS].assign(&L2CValue::Ptr(special_lw_substatus as *const () as _));

    fighter.main_shift(special_lw_main_loop)
}

unsafe extern "C" fn special_lw_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if !param_1.get_bool() {
        if VarModule::is_flag(fighter.battle_object, vars::gekkouga::status::SPECIAL_LW_VANISH) {
            VarModule::dec_int(fighter.battle_object, vars::gekkouga::status::SPECIAL_LW_VANISH_TIMER);
        }
    }
    0.into()
}

unsafe extern "C" fn special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_lw_sub_helper(fighter);

    let vanish_timer = VarModule::get_int(fighter.battle_object, vars::gekkouga::status::SPECIAL_LW_VANISH_TIMER);
    if vanish_timer == 1 {
        let eff = EffectModule::req_on_joint(
            fighter.module_accessor,
            Hash40::new("gekkouga_migawari_smoke"),
            Hash40::new("top"),
            &Vector3f{x: 0.0, y: 0.0, z: 0.0},
            &Vector3f{x: 0.0, y: 0.0, z: 0.0},
            1.0,
            &Vector3f{x: 0.0, y: 0.0, z: 0.0},
            &Vector3f{x: 0.0, y: 0.0, z: 0.0},
            false,
            0,
            0,
            0
        ) as u32;
        EffectModule::set_rate(fighter.module_accessor, eff, 2.0);
    }
    else if vanish_timer == 0 {
        fighter.change_status(statuses::gekkouga::SPECIAL_LW_JUMP.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn special_lw_sub_helper(fighter: &mut L2CFighterCommon) {
    if !VarModule::is_flag(fighter.battle_object, vars::gekkouga::status::SPECIAL_LW_VANISH)
    && MotionModule::is_end(fighter.module_accessor) {
        VarModule::on_flag(fighter.battle_object, vars::gekkouga::status::SPECIAL_LW_VANISH);

        // Call smoke effects
        let eff = EffectModule::req_on_joint(
            fighter.module_accessor,
            Hash40::new("gekkouga_migawari_smoke"),
            Hash40::new("top"),
            &Vector3f{x: 0.0, y: 0.0, z: 0.0},
            &Vector3f{x: 0.0, y: 0.0, z: 0.0},
            1.0,
            &Vector3f{x: 0.0, y: 0.0, z: 0.0},
            &Vector3f{x: 0.0, y: 0.0, z: 0.0},
            false,
            0,
            0,
            0
        ) as u32;
        EffectModule::set_rate(fighter.module_accessor, eff, 2.0);

        // Reset all momentum
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_RESET);

        // Turn Greninja invisible and intangible
        VisibilityModule::set_whole(fighter.module_accessor, false);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);

        // Disable Jostling (so he doesn't push anything while invisible)
        JostleModule::set_status(fighter.module_accessor, false);

        // Set situation kind to AIR
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        GroundModule::set_passable_check(fighter.module_accessor, true);

        // Do the following if the sub can be summoned.
        if VarModule::is_flag(fighter.battle_object, vars::gekkouga::status::SPECIAL_LW_SPAWN_SUB) {
            // Is it the doll or log?
            let doll_probability = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("doll_probability")) / 100.0;
            let rng = sv_math::randf(hash40("fighter"), 1.0);
            let (is_doll, notify_hash) = if rng <= doll_probability {
                (true, 0x26b93cec86)
            }
            else {
                (false, 0x25cc93770c)
            };

            // Custom flag for tracking which sub was called
            VarModule::set_flag(fighter.battle_object, vars::gekkouga::instance::SPECIAL_LW_IS_DOLL, is_doll);
            notify_event_msc_cmd!(fighter, Hash40::new_raw(notify_hash));

            // Set cooldown for summoning the sub
            VarModule::set_int(fighter.battle_object, vars::gekkouga::instance::SPECIAL_LW_SUMMON_SUB_COOLDOWN, 600);
        }
    }
}

unsafe extern "C" fn special_lw_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Return if Greninja isn't currently vanishing
    if !VarModule::is_flag(fighter.battle_object, vars::gekkouga::status::SPECIAL_LW_VANISH) {
        return 0.into();
    }

    // If Greninja's searchbox found the sub OR the sub was summoned during this use,
    // Force Greninja's position to be the same as the sub's.
    if VarModule::is_flag(fighter.battle_object, vars::gekkouga::status::SPECIAL_LW_TELEPORT_OK)
    || VarModule::is_flag(fighter.battle_object, vars::gekkouga::status::SPECIAL_LW_SPAWN_SUB) {
        let doll_id = gekkouga_get_sub_id(fighter.battle_object);
        if sv_battle_object::is_active(doll_id) {
            let doll_module_accessor = sv_battle_object::module_accessor(doll_id);
            let doll_pos = PostureModule::pos(doll_module_accessor);
            PostureModule::set_pos(fighter.module_accessor, doll_pos);
        }
    }

    0.into()
}

unsafe extern "C" fn special_lw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    VisibilityModule::set_whole(fighter.module_accessor, true);
    0.into()
}

unsafe extern "C" fn special_lw_exit(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_pre);
    agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_init);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_main);
    agent.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_exec);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_end);
    agent.status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_exit);
}