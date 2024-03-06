use super::*;
use globals::*;
use smashline::*;

pub unsafe extern "C" fn ryu_attack_command_4_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_MOTION,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_ATTACK_COMMAND2 | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_3 as u32,
        0
    );
    0.into()
}

pub unsafe extern "C" fn ryu_attack_command_4_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_command4"), 0.0, 1.0, false, 0.0, false, false);
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_LOG_ATTACK_KIND_ATTACK_COMMAND2, *FIGHTER_RYU_STATUS_ATTACK_INT_LOG_KIND);
    // fighter.clear_lua_stack();
    // fighter.push_lua_stack(&mut L2CValue::I32(*FIGHTER_KINETIC_ENERGY_ID_MOTION));
    // let mut lr = PostureModule::lr(fighter.module_accessor);
    // fighter.push_lua_stack(&mut L2CValue::F32(lr));
    // app::sv_kinetic_energy::set_chara_dir(fighter.lua_state_agent);
    fighter.off_flag(*FIGHTER_RYU_STATUS_ATTACK_FLAG_RELEASE_BUTTON);
    fighter.set_int(*FIGHTER_RYU_STRENGTH_S, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH);
    fighter.sub_shift_status_main(L2CValue::Ptr(ryu_attack_command_4_main_loop as *const () as _))
}

pub unsafe extern "C" fn ryu_attack_command_4_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return 0.into();
    }
    let motion_frame = fighter.motion_frame();
    if !fighter.is_flag(*FIGHTER_RYU_STATUS_ATTACK_FLAG_RELEASE_BUTTON) && !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
        fighter.on_flag(*FIGHTER_RYU_STATUS_ATTACK_FLAG_RELEASE_BUTTON);
        let button_strength = if motion_frame <= 2.0 {
            *FIGHTER_RYU_STRENGTH_W
        } else if motion_frame <= 4.0 {
            *FIGHTER_RYU_STRENGTH_M
        } else {
            *FIGHTER_RYU_STRENGTH_S
        };
        fighter.set_int(button_strength, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH);
    }
    if motion_frame < 5.0 && fighter.is_button_on(Buttons::SpecialAll | Buttons::Catch | Buttons::AppealAll)
    && (MeterModule::level(fighter.battle_object) >= 2 || VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_MAGIC_SERIES_CANCEL)) {
        fighter.on_flag(*FIGHTER_RYU_STATUS_ATTACK_FLAG_RELEASE_BUTTON);
        fighter.set_int(*FIGHTER_RYU_STRENGTH_S, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH);
        VarModule::on_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL)
    }

    0.into()
}

pub fn install() {
    smashline::Agent::new("ryu")
        .status(Pre, statuses::ryu::ATTACK_COMMAND_4, ryu_attack_command_4_pre)
        .status(Main, statuses::ryu::ATTACK_COMMAND_4, ryu_attack_command_4_main)
        .install();
}
