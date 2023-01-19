// status imports
use super::*;
use globals::*;

// This file contains code for wavelanding

pub fn install() {
    install_status_scripts!(
        status_pre_Jump,
        status_Jump,
        //status_end_Jump
    );
    install_hooks!(
        status_pre_Jump_Common,
        status_pre_Jump_Common_param,
        status_pre_Jump_sub,
        status_pre_Jump_sub_param,
        //status_Jump_Main,
        status_Jump_sub,
        //status_pre_JumpAerial_sub
    );
    skyline::nro::add_hook(nro_hook);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_Jump_Main
        );
    }
}

#[common_status_script(status = FIGHTER_STATUS_KIND_JUMP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE,
    symbol = "_ZN7lua2cpp16L2CFighterCommon15status_pre_JumpEv")]
unsafe fn status_pre_Jump(fighter: &mut L2CFighterCommon) -> L2CValue {
    let interrupted = fighter.status_pre_Jump_Common_param(L2CValue::Bool(true)).get_bool();
    if !interrupted {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
        fighter.status_pre_Jump_sub();
    }
    L2CValue::Bool(interrupted)
}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon22status_pre_Jump_CommonEv")]
unsafe extern "C" fn status_pre_Jump_Common(fighter: &mut L2CFighterCommon) {
    fighter.status_pre_Jump_Common_param(L2CValue::Bool(true));
}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon28status_pre_Jump_Common_paramEN3lib8L2CValueE")]
unsafe extern "C" fn status_pre_Jump_Common_param(fighter: &mut L2CFighterCommon, arg: L2CValue) -> L2CValue {
    //println!("status_pre_Jump_Common_param");
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_HAMMER) {
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_STATUS_KIND_HAMMER_JUMP);
        L2CValue::Bool(true)
    } else if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SCREW) && arg.get_bool() {
        let screw_jump_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_SCREW_JUMP_COUNT);
        if screw_jump_count < *FIGHTER_STATUS_SCREW_JUMP_COUNT_MAX {
            StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_STATUS_KIND_ITEM_SCREW_JUMP);
        }
        L2CValue::Bool(true)
    } else if ItemModule::get_have_item_kind(fighter.module_accessor, 0) == *ITEM_KIND_GENESISSET {
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_STATUS_KIND_ITEM_SHOOT_JUMP);
        L2CValue::Bool(true)
    } else if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_BOARD) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_BOARD);
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI) {
            StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_STATUS_KIND_GIMMICK_JUMP_BOARD);
        }
        L2CValue::Bool(true)
    } else {
        L2CValue::Bool(false)
    }
}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon19status_pre_Jump_subEv")]
unsafe extern "C" fn status_pre_Jump_sub(fighter: &mut L2CFighterCommon) {
    fighter.status_pre_Jump_sub_param(
        L2CValue::I32(*FIGHTER_STATUS_WORK_KEEP_FLAG_JUMP_FLAG),
        L2CValue::I32(*FIGHTER_STATUS_WORK_KEEP_FLAG_JUMP_INT),
        L2CValue::I32(*FIGHTER_STATUS_WORK_KEEP_FLAG_JUMP_FLOAT),
        L2CValue::I32(*FIGHTER_KINETIC_TYPE_JUMP),
        L2CValue::I32(0)
    );
}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon25status_pre_Jump_sub_paramEN3lib8L2CValueES2_S2_S2_S2_")]
unsafe extern "C" fn status_pre_Jump_sub_param(fighter: &mut L2CFighterCommon, flag_keep: L2CValue, int_keep: L2CValue, float_keep: L2CValue, kinetic_type: L2CValue, arg: L2CValue) {
    //println!("status_pre_Jump_sub_param");
    let flag_keep = flag_keep.get_i32();
    let int_keep = int_keep.get_i32();
    let float_keep = float_keep.get_i32();
    let mut kinetic_type = kinetic_type.get_i32();
    let arg = arg.get_i32();
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let ground_correct_kind = app::FighterUtil::get_ground_correct_kind_air_trans(fighter.module_accessor, status_kind);
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        kinetic_type,
        ground_correct_kind as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
        true,
        flag_keep,
        int_keep,
        float_keep,
        arg
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_ENABLE,
        true,
        false,
        true,
        0,
        (*FIGHTER_STATUS_ATTR_ENABLE_ROCKETBELT_EJECT | *FIGHTER_STATUS_ATTR_INTO_DOOR) as u32,
        0,
        0
    );
}

#[common_status_script(status = FIGHTER_STATUS_KIND_JUMP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN,
    symbol = "_ZN7lua2cpp16L2CFighterCommon11status_JumpEv")]
unsafe fn status_Jump(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_jump_item_rocketbelt();
    fighter.status_Jump_sub(L2CValue::Hash40s("invalid"), L2CValue::F32(0.0));
    fighter.sub_shift_status_main(L2CValue::Ptr(status_Jump_Main as *const () as _))
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_Jump_Main)]
unsafe extern "C" fn status_Jump_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // println!("jump main");
    VarModule::set_float(fighter.battle_object, vars::common::instance::CURRENT_MOMENTUM, KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN));
    VarModule::set_float(fighter.battle_object, vars::common::instance::CURRENT_MOMENTUM_SPECIALS, KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN));
    let ret = if fighter.sub_transition_group_check_air_cliff().get_bool() {
        1.into()
    }
    else if fighter.sub_air_check_fall_common().get_bool() {
        1.into()
    }
    else if fighter.sub_air_check_stop_ceil().get_bool() {
        1.into()
    } else if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(
            L2CValue::I32(*FIGHTER_STATUS_KIND_FALL),
            L2CValue::Bool(false)
        );
        1.into()
    } else {
        fighter.sub_air_check_superleaf_fall_slowly();
        0.into()
    };
    
    // CAT1, FLAG_ATTACK_N
    InputModule::clear_persist_one(fighter.battle_object, 0, 0);
    ret
}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon15status_Jump_subEN3lib8L2CValueES2_")]
unsafe extern "C" fn status_Jump_sub(fighter: &mut L2CFighterCommon, arg1: L2CValue, arg2: L2CValue) -> L2CValue {
    ControlModule::reset_flick_y(fighter.module_accessor);
    ControlModule::reset_flick_sub_y(fighter.module_accessor);
    fighter.global_table[FLICK_Y].assign(&0xFE.into());
    ControlModule::reset_trigger(fighter.module_accessor);
    fighter.sub_air_check_fall_common_pre();
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_STOP_CEIL);
    let mut motion = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_POWBLOCK_QUAKE_JUMP) {
        Hash40::new_raw(0xb38c9ab48)
    } else {
        let stick_x = fighter.global_table[STICK_X].get_f32();
        let lr = -PostureModule::lr(fighter.module_accessor);
        let stick_x = stick_x * lr;
        let neutral_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("jump_neutral_x"));
        let _motion = if neutral_x <= stick_x {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI) {
                Hash40::new("jump_b_mini")
            }
            else {
                Hash40::new("jump_b")
            }
        } else {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI) {
                Hash40::new("jump_f_mini")
            }
            else {
                Hash40::new("jump_f")
            }
        };
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_RABBIT_CAP) {
            SoundModule::play_se(fighter.module_accessor, Hash40::new_raw(0x18ed402875), true, false, false, false, app::enSEType(0));
        }
        _motion
    };
    if arg1.get_hash().hash != hash40("invalid") {
        motion = arg1.get_hash();
    }
    MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.0, false, arg2.get_f32(), false, false);
    let subroutine = fighter.global_table[0x4e].get_ptr();
    if !subroutine.is_null() {
        let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(subroutine);
        callable(fighter);
    }
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_fall_common_uniq(L2CValue::Bool(false));
    }
    // not 100% sure why this line causes a bug but it does
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(bind_call_sub_fall_common_uniq as *const () as _));
    L2CValue::Hash40(motion)
}

unsafe extern "C" fn bind_call_sub_fall_common_uniq(fighter: &mut L2CFighterCommon, arg: L2CValue) -> L2CValue {
    fighter.sub_fall_common_uniq(arg)
}

#[common_status_script(status = FIGHTER_STATUS_KIND_JUMP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END,
    symbol = "_ZN7lua2cpp16L2CFighterCommon15status_end_JumpEv")]
unsafe fn status_end_Jump(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND] != FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE || fighter.global_table[STATUS_KIND] != FIGHTER_STATUS_KIND_ESCAPE_AIR {
        // super::sub_air_check_escape_air_snap(fighter, true);
    }
    let control_module = *(fighter.module_accessor as *const u64).offset(0x48 / 8) as *const u64;
    let vtable = *control_module;
    let vtable = vtable - skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;
    // println!("{:#x}", vtable);
    L2CValue::I32(0)
}


// #[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon25status_pre_JumpAerial_subEv")]
// unsafe extern "C" fn status_pre_JumpAerial_sub(fighter: &mut L2CFighterCommon) -> L2CValue {
//     VarModule::off_flag(fighter.battle_object, vars::common::UP_SPECIAL_JUMP_REFRESH_WINDOW);
//     call_original!(fighter)
// }
