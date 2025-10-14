use super::*;

unsafe extern "C" fn super_special_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MeterModule::drain(fighter.battle_object, 2);

    WorkModule::set_int(fighter.module_accessor, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_DOLLY_STATUS_FINAL_WORK_INT_BURST_OBJECT_ID);

    fighter.sub_set_special_start_common_kinetic_setting(hash40("param_super_special").into());
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("super_special"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    dolly_super_special_set_kinetic(fighter, false.into(), 0.into(), 0.into());

    let func = smashline::api::get_target_function("lua2cpp_dolly.nrs", 0x12d50).unwrap();
    let dolly_map_col : fn(&mut L2CFighterCommon, L2CValue) = std::mem::transmute(func);
    dolly_map_col(fighter, hash40("param_super_special").into());

    if !StopModule::is_stop(fighter.module_accessor) {
        dolly_super_special_substatus(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(dolly_super_special_substatus as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(dolly_super_special_main_loop as *const () as _))
}

unsafe extern "C" fn dolly_super_special_set_kinetic(fighter: &mut L2CFighterCommon, param_1: L2CValue, param_2: L2CValue, param_3: L2CValue) {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        if param_1.get_bool() {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        }
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        if param_2.get_bool() {
            let func : fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(param_2.get_ptr());
            func(fighter);
        }
    }
    else {
        if param_1.get_bool() {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        }
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if param_3.get_bool() {
            let func : fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(param_3.get_ptr());
            func(fighter);
        }
    }
}

unsafe extern "C" fn dolly_super_special_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if !param_1.get_bool() {
        if fighter.motion_frame() < 5.0
        && fighter.is_button_on(Buttons::AttackAll | Buttons::Catch | Buttons::AppealAll)
        && fighter.is_button_on(Buttons::SpecialAll)
        && MeterModule::level(fighter.battle_object) >= 2 {
            VarModule::on_flag(fighter.battle_object, vars::dolly::status::SUPER_SPECIAL_TRIPLE);
        }
    }

    let func = smashline::api::get_target_function("lua2cpp_dolly.nrs", 0x15440).unwrap();
    let dolly_super_special_substatus_inner : fn(&mut L2CValue, &mut L2CFighterCommon, L2CValue) -> L2CValue = std::mem::transmute(func);
    let ret = &mut L2CValue::I32(0);
    dolly_super_special_substatus_inner(ret, fighter, param_1);
    ret.clone()
}

unsafe extern "C" fn dolly_super_special_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return true.into();
        }
    }

    if VarModule::is_flag(fighter.battle_object, vars::dolly::status::SUPER_SPECIAL_TRIPLE)
    && MotionModule::motion_kind(fighter.module_accessor) != hash40("super_special_triple") {
        MotionModule::change_motion_inherit_frame(
            fighter.module_accessor,
            Hash40::new("super_special_triple"),
            -1.0,
            1.0,
            0.0,
            false,
            false
        );
    }

    if MotionModule::is_end(fighter.module_accessor) {
        let status = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            FIGHTER_STATUS_KIND_WAIT
        }
        else {
            FIGHTER_STATUS_KIND_FALL
        };
        fighter.change_status(status.into(), false.into());
        return false.into();
    }

    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        dolly_super_special_set_kinetic(fighter, true.into(), 0.into(), 0.into());
    }

    return false.into();
}

pub unsafe extern "C" fn dolly_super_special_end_helper(fighter: &mut L2CFighterCommon, hash: L2CValue) {
    let param = hash.get_u64();
    let map_coll_joint = WorkModule::get_param_int64(fighter.module_accessor, param, hash40("map_coll_joint"));
    let offx = WorkModule::get_float(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SUPER_SPECIAL_WORK_FLOAT_MAP_COLL_OFFSET_X);
    let offy = WorkModule::get_float(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SUPER_SPECIAL_WORK_FLOAT_MAP_COLL_OFFSET_Y);
    let offz = WorkModule::get_float(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SUPER_SPECIAL_WORK_FLOAT_MAP_COLL_OFFSET_Z);
    GroundModule::set_shape_data_rhombus_modify_node_offset(fighter.module_accessor, Hash40::new_raw(map_coll_joint), &Vector3f{x: offx, y: offy, z: offz});
}

unsafe extern "C" fn super_special_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    dolly_super_special_end_helper(fighter, hash40("param_super_special").into());
    let eff_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SUPER_SPECIAL_WORK_INT_SCREEN_EFFECT_COUNT);
    if eff_count > 0 {
        MotionAnimcmdModule::call_script_single(
            fighter.module_accessor,
            *FIGHTER_ANIMCMD_EFFECT,
            Hash40::new("effect_super_specialcancelfillscreen"),
            -1
        );
    }
    return false.into();
}

pub unsafe extern "C" fn super_special2_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MeterModule::drain(fighter.battle_object, 2);
    smashline::original_status(Main, fighter, *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2)(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL, super_special_main);
    agent.status(End, *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL, super_special_end);

    agent.status(Main, *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2, super_special2_main);
}