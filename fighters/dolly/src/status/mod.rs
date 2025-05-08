use super::*;
use globals::*;
// status script import

mod wait;
mod dash;
mod landing;
mod guard_off;
mod rebirth;
mod escape;

mod special_cmd4;
mod special_s;
mod special_supers;
mod special_hi;
mod special_lw;
mod special_lw_breaking;

utils::import_noreturn!(common::shoto_status::{
    fgc_end_dashback
});

extern "Rust" {
    // from common::shoto_status
    fn fgc_dashback_main(fighter: &mut L2CFighterCommon) -> L2CValue;
}

// Prevents sideB from being used again if it has already been used once in the current airtime
unsafe extern "C" fn should_use_special_s_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) && VarModule::is_flag(fighter.battle_object, vars::dolly::instance::DISABLE_SPECIAL_S) {
        false.into()
    } else {
        true.into()
    }
}

unsafe extern  "C" fn check_autoturn(fighter: &mut L2CFighterCommon) -> L2CValue {
    let next_status = fighter.global_table[globals::STATUS_KIND].get_i32();
    let prev_status = fighter.global_table[globals::STATUS_KIND_INTERRUPT].get_i32();
    let situation_kind = fighter.global_table[globals::SITUATION_KIND].get_i32();

    if fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_WAIT {
        FighterSpecializer_Dolly::update_opponent_lr_1on1(fighter.module_accessor, fighter.global_table[STATUS_KIND].get_i32());
    }
    fighter.off_flag(*FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLAG_AUTO_TURN_END_STATUS);
    let lr = WorkModule::get_float(fighter.module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1);
    if lr == 0.0
    || PostureModule::lr(fighter.module_accessor) == lr
    || StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_GROUND {
        return false.into();
    }

    // all autocancellable statuses, if the status is not in this list we skip all other checks
    if ![
        *FIGHTER_STATUS_KIND_WAIT,
        *FIGHTER_STATUS_KIND_WALK,
        *FIGHTER_STATUS_KIND_SQUAT,
        *FIGHTER_STATUS_KIND_SQUAT_RV,
        *FIGHTER_STATUS_KIND_ATTACK,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
        *FIGHTER_STATUS_KIND_ATTACK_HI4_START,
        *FIGHTER_STATUS_KIND_ATTACK_LW4_START,
        *FIGHTER_STATUS_KIND_CATCH,
        *FIGHTER_STATUS_KIND_ITEM_THROW,
        *FIGHTER_STATUS_KIND_ITEM_SWING,
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_FINAL,
        *FIGHTER_DOLLY_STATUS_KIND_WALK_BACK,
        statuses::dolly::ATTACK_COMMAND_4
    ].contains(&next_status) {
        return false.into();
    }

    if next_status == *FIGHTER_STATUS_KIND_WAIT 
    && [ // these statuses go through a separate TurnAuto status instead of wait
        *FIGHTER_STATUS_KIND_DASH,
        *FIGHTER_DOLLY_STATUS_KIND_DASH_BACK,
        *FIGHTER_STATUS_KIND_RUN_BRAKE,
        *FIGHTER_STATUS_KIND_TURN_RUN_BRAKE,
        *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL,
        *FIGHTER_STATUS_KIND_SQUAT_RV,
        *FIGHTER_STATUS_KIND_TREAD_DAMAGE_RV,
        *FIGHTER_STATUS_KIND_GUARD_OFF,
        *FIGHTER_STATUS_KIND_GUARD_DAMAGE,
        *FIGHTER_STATUS_KIND_ESCAPE,
        *FIGHTER_STATUS_KIND_ESCAPE_F,
        *FIGHTER_STATUS_KIND_ESCAPE_B,
        *FIGHTER_STATUS_KIND_ATTACK_DASH,
        *FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_S4,
        *FIGHTER_STATUS_KIND_ATTACK_HI4,
        *FIGHTER_STATUS_KIND_ATTACK_LW4,
        *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR,
        *FIGHTER_STATUS_KIND_CATCH,
        *FIGHTER_STATUS_KIND_CATCH_DASH,
        *FIGHTER_STATUS_KIND_CATCH_TURN,
        *FIGHTER_STATUS_KIND_CATCH_CUT,
        *FIGHTER_STATUS_KIND_THROW,
        *FIGHTER_STATUS_KIND_CAPTURE_CUT,
        *FIGHTER_STATUS_KIND_DAMAGE,
        *FIGHTER_STATUS_KIND_DOWN_STAND,
        *FIGHTER_STATUS_KIND_DOWN_STAND_FB,
        *FIGHTER_STATUS_KIND_PASSIVE,
        *FIGHTER_STATUS_KIND_PASSIVE_FB,
        *FIGHTER_STATUS_KIND_FURAFURA_END,
        *FIGHTER_STATUS_KIND_DAMAGE_SONG_END,
        *FIGHTER_STATUS_KIND_CLIFF_CLIMB,
        *FIGHTER_STATUS_KIND_CLIFF_ATTACK,
        *FIGHTER_STATUS_KIND_CLIFF_ESCAPE,
        *FIGHTER_STATUS_KIND_SLIP_STAND,
        *FIGHTER_STATUS_KIND_SLIP_STAND_ATTACK,
        *FIGHTER_STATUS_KIND_SLIP_STAND_F,
        *FIGHTER_STATUS_KIND_SLIP_STAND_B,
        *FIGHTER_STATUS_KIND_ITEM_LIGHT_PICKUP,
        *FIGHTER_STATUS_KIND_ITEM_THROW,
        *FIGHTER_STATUS_KIND_ITEM_THROW_DASH,
        *FIGHTER_STATUS_KIND_ITEM_THROW_HEAVY,
        *FIGHTER_STATUS_KIND_ITEM_SWING,
        *FIGHTER_STATUS_KIND_ITEM_SWING_S3,
        *FIGHTER_STATUS_KIND_ITEM_SWING_S4,
        *FIGHTER_STATUS_KIND_ITEM_SWING_DASH,
        *FIGHTER_STATUS_KIND_APPEAL,
        *FIGHTER_STATUS_KIND_SPECIAL_N,
    ].contains(&prev_status) {
        return false.into();
    }

    // don't autoturn in the middle of a jab combo
    if next_status == *FIGHTER_STATUS_KIND_ATTACK 
    && prev_status == *FIGHTER_STATUS_KIND_ATTACK
    && ComboModule::count(fighter.module_accessor) != 0 {
        return false.into();
    }

    if !VarModule::is_flag(fighter.battle_object, vars::common::instance::WAS_PREV_STATUS_CANCELABLE)
    && [ // don't autoturn if using a direct cancel from shield
        *FIGHTER_STATUS_KIND_GUARD_ON,
        *FIGHTER_STATUS_KIND_GUARD,
        *FIGHTER_STATUS_KIND_GUARD_DAMAGE,
        *FIGHTER_STATUS_KIND_GUARD_OFF,
    ].contains(&prev_status) {
        return false.into();
    }

    // item toss directional restruictions
    let cat3 = fighter.global_table[globals::CMD_CAT3].get_i32();
    if next_status == *FIGHTER_STATUS_KIND_ITEM_THROW
    && situation_kind == *SITUATION_KIND_GROUND
    && !(cat3 & *FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_4 != 0 && cat3 & *FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_FB4 == 0)
    && cat3 & *FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_HI == 0
    && cat3 & *FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_LW == 0 {
        return false.into();
    }

    PostureModule::set_lr(fighter.module_accessor, lr);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    return true.into();
}

unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Re-enables the ability to use sideB when connecting to ground or cliff
    if fighter.is_situation(*SITUATION_KIND_GROUND) || fighter.is_situation(*SITUATION_KIND_CLIFF)
    || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_GIMMICK_SPRING_JUMP]) {
        VarModule::off_flag(fighter.battle_object, vars::dolly::instance::DISABLE_SPECIAL_S);
    }

    check_autoturn(fighter);
    return false.into();
}

pub const CHECK_SPECIAL_N_UNIQ:            i32 = 0x38;
pub const CHECK_SPECIAL_S_UNIQ:            i32 = 0x39;
pub const CHECK_SPECIAL_HI_UNIQ:           i32 = 0x3A;
pub const CHECK_SPECIAL_LW_UNIQ:           i32 = 0x3B;

pub unsafe extern "C" fn dolly_check_super_special_command(fighter: &mut L2CFighterCommon) -> L2CValue {
    let cat1 =  fighter.global_table[CMD_CAT1].get_i32();
    let cat4 = fighter.global_table[CMD_CAT4].get_i32();

    fighter.set_int(cat4, *FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_CAT4_SPECIAL_COMMAND);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND
    || !fighter.is_flag(*FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_ENABLE_SUPER_SPECIAL) {
        return false.into();
    }

    if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL2_COMMAND != 0
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2) {
        fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2.into(), true.into());
        return true.into();
    }
    if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL_COMMAND != 0
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL) {
        fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL.into(), true.into());
        return true.into();
    }
    // if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL2_R_COMMAND != 0
    // && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2) {
    //     let opplr = WorkModule::get_float(fighter.module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1);
    //     if opplr != 0.0 {
    //         PostureModule::reverse_lr(fighter.module_accessor);
    //     }
    //     fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2.into(), true.into());
    //     return true.into();
    // }
    // if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL_R_COMMAND != 0
    // && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL) {
    //     let opplr = WorkModule::get_float(fighter.module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1);
    //     if opplr != 0.0 {
    //         PostureModule::reverse_lr(fighter.module_accessor);
    //     }
    //     fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL.into(), true.into());
    //     return true.into();
    // }
    return false.into();
}

unsafe extern "C" fn dolly_check_special_hi_command(fighter: &mut L2CFighterCommon) -> L2CValue {
    let cat1 =  fighter.global_table[CMD_CAT1].get_i32();
    let cat4 = fighter.global_table[CMD_CAT4].get_i32();

    if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_HI2_COMMAND != 0
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND)
    && fighter.sub_transition_term_id_cont_disguise(fighter.global_table[CHECK_SPECIAL_HI_UNIQ].clone()).get_bool() {
        fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_COMMAND.into(), true.into());
        return true.into();
    }
    return false.into();
}

pub unsafe extern "C" fn dolly_check_other_special_command(fighter: &mut L2CFighterCommon) -> L2CValue {
    let cat1 =  fighter.global_table[CMD_CAT1].get_i32();
    let cat4 = fighter.global_table[CMD_CAT4].get_i32();

    if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_HI_COMMAND != 0
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND)
    && fighter.sub_transition_term_id_cont_disguise(fighter.global_table[CHECK_SPECIAL_LW_UNIQ].clone()).get_bool() {
        fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_COMMAND.into(), true.into());
        return true.into();
    }

    if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_S_COMMAND != 0
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND)
    && fighter.sub_transition_term_id_cont_disguise(fighter.global_table[CHECK_SPECIAL_S_UNIQ].clone()).get_bool() {
        fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_COMMAND.into(), true.into());
        return true.into();
    }
    
    if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N_COMMAND != 0
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND)
    && fighter.sub_transition_term_id_cont_disguise(fighter.global_table[CHECK_SPECIAL_S_UNIQ].clone()).get_bool() {
        fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SPECIAL_S_COMMAND.into(), true.into());
        return true.into();
    }

    if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N2_COMMAND != 0
    && fighter.is_situation(*SITUATION_KIND_GROUND)
    && !fighter.is_in_hitlag()
    && StatusModule::status_kind(fighter.module_accessor) != statuses::dolly::ATTACK_COMMAND_4
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND) {
        fighter.change_status(statuses::dolly::ATTACK_COMMAND_4.into(), true.into());
        return true.into();
    }

    // Uncomment to implement dash cancel FTilt
    // if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_6N6 != 0
    // && StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_S3
    // && MeterModule::level(fighter.battle_object) >= 1 {
    //     fighter.change_status(FIGHTER_STATUS_KIND_DASH.into(), true.into());
    //     VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_ENTER_DASH_CANCEL);
    //     return true.into();
    // }

    return false.into();
}

pub unsafe extern "C" fn dolly_check_normal_command(fighter: &mut L2CFighterCommon) -> L2CValue {
    let cat1 =  fighter.global_table[CMD_CAT1].get_i32();
    if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3 != 0
    && fighter.is_status(*FIGHTER_STATUS_KIND_ATTACK)
    && !fighter.is_in_hitlag()
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND) {
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_HI3.into(), true.into());
        return true.into();
    }
    if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3 != 0
    && fighter.is_status(*FIGHTER_STATUS_KIND_ATTACK)
    && !fighter.is_in_hitlag()
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND) {
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_S3.into(), true.into());
        return true.into();
    }
    return false.into();
}

pub unsafe extern "C" fn dolly_check_special_command(fighter: &mut L2CFighterCommon) -> L2CValue {
    if dolly_check_super_special_command(fighter).get_bool() 
    || dolly_check_special_hi_command(fighter).get_bool() 
    || dolly_check_other_special_command(fighter).get_bool()
    || dolly_check_normal_command(fighter).get_bool() {
        return true.into();
    }
    return false.into();
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    // set the callbacks on fighter init
    fighter.global_table[globals::USE_SPECIAL_S_CALLBACK].assign(&L2CValue::Ptr(should_use_special_s_callback as *const () as _));
    fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));   
    fighter.global_table[globals::CHECK_SPECIAL_COMMAND].assign(&L2CValue::Ptr(dolly_check_special_command as *const () as _));
    fighter.set_command_input_button(0, 2);
    fighter.set_command_input_button(1, 1);
    fighter.set_command_input_button(2, 2);
    fighter.set_command_input_button(3, 2);
    fighter.set_command_input_button(7, 2);
    fighter.set_command_input_button(8, 2);
    fighter.set_command_input_button(9, 2);
    fighter.set_command_input_button(10, 2);
    VarModule::set_int(fighter.battle_object, vars::dolly::instance::ADDED_METER_LEVELS, 0);
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);

    wait::install(agent);
    dash::install(agent);
    landing::install(agent);
    guard_off::install(agent);
    rebirth::install(agent);
    escape::install(agent);

    special_cmd4::install(agent);
    special_s::install(agent);
    special_supers::install(agent);
    special_hi::install(agent);
    special_lw::install(agent);
    special_lw_breaking::install(agent);
}