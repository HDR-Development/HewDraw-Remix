use super::*;
use globals::*;
// status script import

mod attack;
mod dash;
mod wait;
mod guard;
mod landing;

mod finals;
mod special_cmd4;
mod special_lw;
mod special_s;

utils::import_noreturn!(common::shoto_status::{
    fgc_end_dashback,
    ryu_idkwhatthisis2
});

extern "Rust" {
    // from common::shoto_status
    fn ryu_kara_cancel(fighter: &mut L2CFighterCommon) -> L2CValue;
    fn ryu_attack_main_uniq_chk(fighter: &mut L2CFighterCommon) -> L2CValue;
    fn fgc_dashback_main(fighter: &mut L2CFighterCommon) -> L2CValue;
    fn ryu_attack_main_uniq_chk4(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue;
    fn ryu_final_hit_cancel(fighter: &mut L2CFighterCommon, situation: L2CValue) -> L2CValue;
    fn ryu_hit_cancel(fighter: &mut L2CFighterCommon, situation: L2CValue) -> L2CValue;
    fn fgc_landing_main(fighter: &mut L2CFighterCommon) -> L2CValue;
}

extern "C" {
    fn status_pre_EscapeAir(fighter: &mut L2CFighterCommon) -> L2CValue;
}
extern "C" {
    fn status_end_EscapeAir(fighter: &mut L2CFighterCommon) -> L2CValue;
}

// Prevents sideB from being used again if it has already been used once in the current airtime
unsafe extern "C" fn should_use_special_s_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) && VarModule::is_flag(fighter.battle_object, vars::shotos::instance::DISABLE_SPECIAL_S) {
        false.into()
    } else {
        true.into()
    }
}

unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    // resets IS_USE_EX_SPECIAL
    if !fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_JUMP,
        // *FIGHTER_STATUS_KIND_SPECIAL_N,
        // *FIGHTER_RYU_STATUS_KIND_SPECIAL_N_COMMAND,
        // *FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND,
        *FIGHTER_STATUS_KIND_SPECIAL_S, 
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND, 
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_END, 
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP,
        // *FIGHTER_RYU_STATUS_KIND_ATTACK_COMMAND1,
        // *FIGHTER_RYU_STATUS_KIND_ATTACK_COMMAND2,
        // statuses::ryu::ATTACK_COMMAND_4
    ]) {
        VarModule::off_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL);
        VarModule::off_flag(fighter.battle_object, vars::shotos::instance::IS_ENABLE_FADC);
        // VarModule::off_flag(fighter.battle_object, vars::shotos::instance::IS_ENABLE_SPECIAL_LW_INSTALL);
    }
    
    // Re-enables the ability to use sideB when connecting to ground or cliff
    if fighter.is_situation(*SITUATION_KIND_GROUND) || fighter.is_situation(*SITUATION_KIND_CLIFF)
    || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LANDING]) {
        VarModule::off_flag(fighter.battle_object, vars::shotos::instance::DISABLE_SPECIAL_S);
    }
    
    // ORIGINAL
    fighter.off_flag(*FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLAG_AUTO_TURN_END_STATUS);
    let lr = WorkModule::get_float(fighter.module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1);
    if lr == 0.0
    || PostureModule::lr(fighter.module_accessor) == lr
    || StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_GROUND {
        return 0.into();
    }

    unsafe fn update_lr(fighter: &mut L2CFighterCommon, lr: f32) {
        PostureModule::set_lr(fighter.module_accessor, lr);
        PostureModule::update_rot_y_lr(fighter.module_accessor);
    }

    if [
        *FIGHTER_STATUS_KIND_WALK,
        *FIGHTER_STATUS_KIND_SQUAT,
        *FIGHTER_STATUS_KIND_SQUAT_RV,
        *FIGHTER_STATUS_KIND_LANDING,
        *FIGHTER_STATUS_KIND_LANDING_LIGHT,
        *FIGHTER_STATUS_KIND_GUARD_ON,
        *FIGHTER_STATUS_KIND_ESCAPE,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
        *FIGHTER_STATUS_KIND_ATTACK_HI4_START,
        *FIGHTER_STATUS_KIND_ATTACK_LW4_START,
        *FIGHTER_STATUS_KIND_CATCH,
        *FIGHTER_STATUS_KIND_ITEM_SWING,
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_FINAL,
        *FIGHTER_RYU_STATUS_KIND_WALK_BACK,
    ].contains(&fighter.global_table[globals::STATUS_KIND].get_i32())
    {
        update_lr(fighter, lr);
        return 0.into();
    }

    if fighter.global_table[globals::STATUS_KIND] == FIGHTER_STATUS_KIND_WAIT {
        if ![
            *FIGHTER_STATUS_KIND_DASH,
            *FIGHTER_RYU_STATUS_KIND_DASH_BACK,
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
            *FIGHTER_STATUS_KIND_SPECIAL_N
        ].contains(&fighter.global_table[globals::STATUS_KIND_INTERRUPT].get_i32())
        {
            update_lr(fighter, lr);
        }
        return 0.into();
    }

    if fighter.global_table[globals::STATUS_KIND] == FIGHTER_STATUS_KIND_JUMP_SQUAT {
        if fighter.global_table[globals::STATUS_KIND_INTERRUPT] != FIGHTER_STATUS_KIND_TURN_RUN {
            update_lr(fighter, lr);
        }
        return 0.into();
    }

    if fighter.global_table[globals::STATUS_KIND] == FIGHTER_STATUS_KIND_ATTACK {
        if fighter.global_table[globals::STATUS_KIND] != FIGHTER_STATUS_KIND_ATTACK || ComboModule::count(fighter.module_accessor) == 0 {
            update_lr(fighter, lr);
        }
        return 0.into();
    }

    if fighter.global_table[globals::STATUS_KIND] == FIGHTER_STATUS_KIND_ITEM_THROW
    && fighter.global_table[globals::SITUATION_KIND] == SITUATION_KIND_GROUND
    {
        let cat3 = fighter.global_table[globals::CMD_CAT3].get_i32();
        if cat3 & *FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_4 != 0 && cat3 & *FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_FB4 == 0 {
            update_lr(fighter, lr);
        } else if cat3 & *FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_HI != 0 {
            update_lr(fighter, lr);
        } else if cat3 & *FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_LW != 0 {
            update_lr(fighter, lr);
        }
    }

    0.into()
}

/// determines the command inputs
/// I have divided these inputs into command normals (A) and command specials (B)
/// NOTE: order is important! early order has higher priority
pub unsafe extern "C" fn ryu_check_special_command(fighter: &mut L2CFighterCommon) -> L2CValue {

    let cat1 =  fighter.global_table[CMD_CAT1].get_i32();
    let cat4 = fighter.global_table[CMD_CAT4].get_i32();
    let is_special = fighter.is_cat_flag(Cat1::SpecialAny);
    let is_normal = fighter.is_cat_flag(
        Cat1::AttackN | 
        Cat1::AttackHi3 | 
        Cat1::AttackS3 | 
        Cat1::AttackLw3 | 
        Cat1::AttackHi4 | 
        Cat1::AttackS4 | 
        Cat1::AttackLw4
    );

    // shoryu
    if is_special
    && cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_HI_COMMAND != 0
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND)
    && fighter.sub_transition_term_id_cont_disguise(fighter.global_table[USE_SPECIAL_HI_CALLBACK].clone()).get_bool() {
        fighter.change_status(FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND.into(), true.into());
        return true.into();
    }

    // shaku
    if is_special
    && cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N2_COMMAND != 0
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND)
    && fighter.sub_transition_term_id_cont_disguise(fighter.global_table[USE_SPECIAL_N_CALLBACK].clone()).get_bool() {
        fighter.change_status(FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND.into(), true.into());
        return true.into();
    }

    // hado
    if is_special
    && cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N_COMMAND != 0
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND)
    && fighter.sub_transition_term_id_cont_disguise(fighter.global_table[USE_SPECIAL_N_CALLBACK].clone()).get_bool() {
        fighter.change_status(FIGHTER_RYU_STATUS_KIND_SPECIAL_N_COMMAND.into(), true.into());
        return true.into();
    }

    // the supers
    if is_special
    && cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL_COMMAND != 0
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND) {
        if VarModule::is_flag(fighter.battle_object, vars::shotos::instance::IS_MAGIC_SERIES_CANCEL) {
            AttackModule::clear_all(fighter.module_accessor);
            fighter.on_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL);
            fighter.on_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_IS_DISCRETION_FINAL_USED);
            fighter.change_status(FIGHTER_STATUS_KIND_FINAL.into(), true.into());
        } else if MeterModule::level(fighter.battle_object) >= MeterModule::meter_cap(fighter.battle_object) {
            AttackModule::clear_all(fighter.module_accessor);
            fighter.on_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL);
            fighter.on_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_IS_DISCRETION_FINAL_USED);
            fighter.change_status(FIGHTER_RYU_STATUS_KIND_FINAL2.into(), true.into());
        }
        return true.into();
    }

    // tatsu
    if is_special
    && cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_S_COMMAND != 0
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND)
    && fighter.sub_transition_term_id_cont_disguise(fighter.global_table[USE_SPECIAL_S_CALLBACK].clone()).get_bool()
    && FighterSpecializer_Ryu::check_special_air_s_command(fighter.module_accessor) {
        fighter.change_status(FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND.into(), true.into());
        return true.into();
    }

    // donkey kick
    if is_normal
    && cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N2_COMMAND != 0
    && fighter.is_situation(*SITUATION_KIND_GROUND)
    && !fighter.is_in_hitlag()
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND) {
        fighter.change_status(statuses::ryu::ATTACK_COMMAND_4.into(), true.into());
        return true.into();
    }

    false.into()
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    // set the callbacks on fighter init
    fighter.global_table[globals::USE_SPECIAL_S_CALLBACK].assign(&L2CValue::Ptr(should_use_special_s_callback as *const () as _));
    fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));   
    fighter.global_table[globals::CHECK_SPECIAL_COMMAND].assign(&L2CValue::Ptr(ryu_check_special_command as *const () as _));
    VarModule::set_int(fighter.battle_object, vars::shotos::instance::SPECIAL_N_EX_NUM, 0);
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);

    attack::install(agent);
    dash::install(agent);
    wait::install(agent);
    guard::install(agent);
    landing::install(agent);

    finals::install(agent);
    special_cmd4::install(agent);
    special_lw::install(agent);
    special_s::install(agent);
}