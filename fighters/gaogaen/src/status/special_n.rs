use super::*;
use globals::*;




// FIGHTER_STATUS_KIND_SPECIAL_N


pub unsafe extern "C" fn gaogaen_special_n_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_pre_SpecialNCommon();
    let mask_flag = if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64
    } else {
        0 as u64
    };
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        mask_flag,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
        0
    );

    0.into()
    
}


pub unsafe extern "C" fn exec_special_n(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR && StatusModule::prev_situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }

    return 0.into()
}

    // SVar5 = lib::L2CValue::as_integer(SITUATION_KIND_NONE);
    // iVar6 = lib::L2CValue::as_integer(FIGHTER_KINETIC_TYPE_UNIQUE);
    // uVar7 = lib::L2CValue::as_integer(GROUND_CORRECT_KIND_KEEP);
    // GVar8 = lib::L2CValue::as_integer(GROUND_CLIFF_CHECK_KIND_NONE);
    // bVar1 = lib::L2CValue::as_bool(true);
    // iVar9 = lib::L2CValue::as_integer(FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG);
    // iVar10 = lib::L2CValue::as_integer(FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT);
    // iVar11 = lib::L2CValue::as_integer(0);

    // init_settings_impl(boma, SITUATION_KIND_NONE, FIHGTER_KINETIC_TYPE_UNIQUE,
    // GROUND_CORRECT_KIND_KEEP, GROUND_CLIFF_CHECK_KIND_NONE, true,
    // FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
    // 0, 0)

    // L2CValue(aLStack200, FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N |
    // FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) //array?

    // bVar1 = lib::L2CValue::as_bool(false);
    // iVar6 = lib::L2CValue::as_integer(FIHGTER_TREADED_KIND_NO_REAC);
    // bVar2 = lib::L2CValue::as_bool(false);
    // bVar3 = lib::L2CValue::as_bool(false);
    // bVar4 = lib::L2CValue::as_bool(false);
    // uVar13 = lib::L2CValue::as_integer(array);
    // uVar7 = lib::L2CValue::as_integer(FIHGTER_STATUS_ATTR_START_TURN);
    // uVar12 = lib::L2CValue::as_integer(FIHGTER_POWER_UP_ATTACK_BIT_SPECIAL_N);

    // set_fighter_status_data_impl(boma, false, FIGHTER_TREADED_KIND_NO_REAC, false, false,
    // false, array, FIGHTER_STATUS_ATTR_START_TURN, FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N)



    // if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR && StatusModule::prev_situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
    //     KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    //     GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_KEEP));
    //     GroundModule::set_cliff_check(fighter.module_accessor, app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES));
    //     WorkModule::on_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_STATUS_SPECIAL_N_FLAG_GRAVITY_DEFAULT);
    // }
    // if GroundModule::can_entry_cliff(fighter.module_accessor) == 1 && KineticModule::get_kinetic_type(fighter.module_accessor) == *FIGHTER_KINETIC_TYPE_FALL && fighter.global_table[STICK_Y].get_f32() > -0.66 {
    //     fighter.change_status(
    //         L2CValue::I32(*FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE),
    //         L2CValue::Bool(false)
    //     );
    //     return 1.into()
    // }
    // return 0.into()

pub fn install() {
    smashline::Agent::new("gaogaen")
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_N, gaogaen_special_n_pre)
        .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_N, exec_special_n)
        .install();
}
