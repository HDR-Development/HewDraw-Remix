use super::*;
use globals::*;
// status script import
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

pub fn install() {
    finals::install();
    special_cmd4::install();
    special_lw::install();
    special_s::install();
    install_status_scripts!(
        pre_turndash,
        main_dashback,
        end_dashback,
        main_attack,
        wait_pre,
        landing_main,
        guard,
    );
    smashline::install_agent_init_callbacks!(ken_init);
}

#[smashline::fighter_init]
fn ken_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        if smash::app::utility::get_kind(&mut *fighter.module_accessor) != *FIGHTER_KIND_KEN {
            return;
        }
        fighter.global_table[globals::USE_SPECIAL_S_CALLBACK].assign(&L2CValue::Ptr(should_use_special_s_callback as *const () as _));
        fighter.global_table[globals::USE_SPECIAL_LW_CALLBACK].assign(&L2CValue::Ptr(should_use_special_lw_callback as *const () as _));
        fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));
        fighter.global_table[globals::CHECK_SPECIAL_COMMAND].assign(&L2CValue::Ptr(ken_check_special_command as *const () as _));
    }
}

// Prevents sideB from being used again if it has already been used once in the current airtime
unsafe extern "C" fn should_use_special_s_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) && VarModule::is_flag(fighter.battle_object, vars::shotos::instance::DISABLE_SPECIAL_S) {
        false.into()
    } else {
        true.into()
    }
}

// Prevents DSpecial from being used again if it has already been used once in the current airtime (resets on hit or landing aerial)
unsafe extern "C" fn should_use_special_lw_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) && VarModule::is_flag(fighter.battle_object, vars::shotos::instance::DISABLE_SPECIAL_LW) {
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
        *FIGHTER_STATUS_KIND_SPECIAL_S, 
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND, 
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_END, 
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP
    ]) {
        VarModule::off_flag(fighter.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL);
    }

    // Re-enables the ability to use sideB when connecting to ground or cliff
    if fighter.is_situation(*SITUATION_KIND_GROUND) || fighter.is_situation(*SITUATION_KIND_CLIFF)
    || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LANDING]) {
        VarModule::off_flag(fighter.battle_object, vars::shotos::instance::DISABLE_SPECIAL_S);
    }

    // re-enable DSpecial when landing or hit
    if fighter.is_situation(*SITUATION_KIND_GROUND) || fighter.is_situation(*SITUATION_KIND_CLIFF)
    || fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_REBIRTH,
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_DAMAGE,
        *FIGHTER_STATUS_KIND_DAMAGE_AIR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
        *FIGHTER_STATUS_KIND_DAMAGE_FALL])
    {
        VarModule::off_flag(fighter.battle_object, vars::shotos::instance::DISABLE_SPECIAL_LW);
    }

    // ORIGINAL
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLAG_AUTO_TURN_END_STATUS);
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
pub unsafe extern "C" fn ken_check_special_command(fighter: &mut L2CFighterCommon) -> L2CValue {
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

    // the tatsu super
    if is_special
    && cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL2_COMMAND != 0
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND)
    && MeterModule::level(fighter.object()) >= 6 {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_IS_DISCRETION_FINAL_USED);
        fighter.change_status(FIGHTER_STATUS_KIND_FINAL.into(), true.into());
        return true.into();
    }

    // the shinryuken
    if is_special
    && cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL_COMMAND != 0
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND)
    && MeterModule::level(fighter.object()) >= 10 {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_IS_DISCRETION_FINAL_USED);
        fighter.change_status(FIGHTER_RYU_STATUS_KIND_FINAL2.into(), true.into());
        return true.into();
    }

    // shoryu
    if is_special
    && cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_HI_COMMAND != 0
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND)
    && fighter.sub_transition_term_id_cont_disguise(fighter.global_table[USE_SPECIAL_HI_CALLBACK].clone()).get_bool() {
        fighter.change_status(FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND.into(), true.into());
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

    // tatsu
    if is_special
    && cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_S_COMMAND != 0
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND)
    && fighter.sub_transition_term_id_cont_disguise(fighter.global_table[USE_SPECIAL_S_CALLBACK].clone()).get_bool()
    && FighterSpecializer_Ryu::check_special_air_s_command(fighter.module_accessor) {
        fighter.change_status(FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND.into(), true.into());
        return true.into();
    }

    // roundhouse
    if is_normal
    && cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N2_COMMAND != 0
    && fighter.is_situation(*SITUATION_KIND_GROUND)
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND) {
        fighter.change_status(FIGHTER_RYU_STATUS_KIND_ATTACK_COMMAND2.into(), true.into());
        return true.into();
    }

    // kamabaraigeri
    if is_normal
    && cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N_COMMAND != 0
    && fighter.is_situation(*SITUATION_KIND_GROUND)
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND) {
        let attack_command_4_status_kind = CustomStatusModule::get_agent_status_kind(fighter.battle_object, statuses::ryu::AIR_DASH);
        fighter.change_status(attack_command_4_status_kind.into(), true.into());
        return true.into();
    }

    // crescent kick
    if is_normal
    && cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_ATTACK_COMMAND1 != 0
    && fighter.is_situation(*SITUATION_KIND_GROUND)
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_COMMAND1) {
        fighter.change_status(FIGHTER_RYU_STATUS_KIND_ATTACK_COMMAND1.into(), true.into());
        return true.into();
    }

    false.into()
}

#[status_script(agent = "ken", status = FIGHTER_STATUS_KIND_GUARD_OFF, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn guard(fighter: &mut L2CFighterCommon) -> L2CValue {
    let rate = fighter.status_GuardOff_Common().get_f32();
    if VarModule::is_flag(
        fighter.object(),
        vars::common::instance::IS_PARRY_FOR_GUARD_OFF,
    ) {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("guard_damage"),
            2.0,
            0.0,
            false,
            0.0,
            false,
            false,
        );
        // app::FighterUtil::flash_eye_info(fighter.module_accessor);
        // if !WorkModule::is_flag(
        //     fighter.module_accessor,
        //     *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL,
        // ) {
        //     ModelModule::enable_gold_eye(fighter.module_accessor);
        //     WorkModule::on_flag(
        //         fighter.module_accessor,
        //         *FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLAG_GOLD_EYE,
        //     );
        // }
        let shield_radius = WorkModule::get_param_float(fighter.module_accessor, hash40("shield_radius"), 0);
        let lr = PostureModule::lr(fighter.module_accessor);
        EffectModule::req_follow(
            fighter.module_accessor,
            Hash40::new("sys_genesis_end"),
            Hash40::new("throw"),
            &Vector3f::zero(),
            &Vector3f::zero(),
            shield_radius * 0.06,
            true,
            *EFFECT_SUB_ATTRIBUTE_NONE as u32,
            0,
            0,
            *EFFECT_FLIP_NONE,
            0,
            false,
            false,
        );
        EffectModule::set_rate_last(fighter.module_accessor, 1.2);
        // EffectModule::set_alpha_last(fighter.module_accessor, 0.4);
        EffectModule::req_common(fighter.module_accessor, Hash40::new("just_shield"), 0.0);
        // let shield_se = app::FighterUtil::get_just_shield_se(fighter.global_table[0x2].get_i32());
        let sfx_handle = SoundModule::play_se(
            fighter.module_accessor,
            smash::phx::Hash40::new("se_item_backshield_guard01"),
            true,
            false,
            false,
            false,
            app::enSEType(0),
        );
        SoundModule::set_se_vol(fighter.module_accessor, sfx_handle as i32, 0.9, 0);
        SoundModule::stop_se(fighter.module_accessor, Hash40::new("se_common_guardon"), 0);
    } else {
        let guard_off_motion_start_frame = ParamModule::get_float(fighter.battle_object, ParamType::Common, "guard_off_motion_start_frame");
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("guard_off"),
            guard_off_motion_start_frame,
            rate,
            false,
            0.0,
            false,
            false,
        );
    }
    fighter.main_shift(guard_main)
}

unsafe extern "C" fn guard_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_GuardOff_Main()
}
// FIGHTER_STATUS_KIND_TURN_DASH //

#[status_script(agent = "ken", status = FIGHTER_STATUS_KIND_TURN_DASH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn pre_turndash(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lr = WorkModule::get_float(
        fighter.module_accessor,
        *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1,
    );
    if lr != 0.0 {
        if PostureModule::lr(fighter.module_accessor) == lr {
            if fighter.global_table[PREV_STATUS_KIND] != FIGHTER_STATUS_KIND_TURN {
                StatusModule::set_status_kind_interrupt(
                    fighter.module_accessor,
                    *FIGHTER_RYU_STATUS_KIND_DASH_BACK,
                );
                return L2CValue::I32(1);
            }
        }
    }
    VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_SMASH_TURN);
    StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_STATUS_KIND_TURN);
    return 1.into();
}

// FIGHTER_RYU_STATUS_KIND_DASH_BACK //

#[status_script(agent = "ken", status = FIGHTER_RYU_STATUS_KIND_DASH_BACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn main_dashback(fighter: &mut L2CFighterCommon) -> L2CValue {
    fgc_dashback_main(fighter)
}

#[status_script(agent = "ken", status = FIGHTER_RYU_STATUS_KIND_DASH_BACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn end_dashback(fighter: &mut L2CFighterCommon) -> L2CValue {
    common::shoto_status::fgc_end_dashback(fighter);
    original!(fighter)
}

// FIGHTER_STATUS_KIND_ATTACK //

#[status_script(agent = "ken", status = FIGHTER_STATUS_KIND_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn main_attack(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_AttackCommon();
    if WorkModule::is_flag(
        fighter.module_accessor,
        *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_NEAR_OPPONENT,
    ) {
        WorkModule::set_int64(
            fighter.module_accessor,
            0x10556e6036,
            *FIGHTER_STATUS_ATTACK_WORK_INT_ATTACK11_MOTION,
        );
        WorkModule::set_int(
            fighter.module_accessor,
            *FIGHTER_LOG_ATTACK_KIND_ATTACK_NEAR,
            *FIGHTER_RYU_STATUS_ATTACK_INT_LOG_KIND,
        );
    } else {
        WorkModule::set_int64(
            fighter.module_accessor,
            0xb4f4e6f8f,
            *FIGHTER_STATUS_ATTACK_WORK_INT_ATTACK11_MOTION,
        );
        WorkModule::set_int(
            fighter.module_accessor,
            *FIGHTER_LOG_ATTACK_KIND_ATTACK11,
            *FIGHTER_RYU_STATUS_ATTACK_INT_LOG_KIND,
        );
    }
    if !StopModule::is_stop(fighter.module_accessor) {
        ryu_attack_main_uniq_chk(fighter);
    }
    fighter.global_table[SUB_STATUS3]
        .assign(&L2CValue::Ptr(ryu_attack_main_uniq_chk as *const () as _));
    if !StopModule::is_stop(fighter.module_accessor) {
        ryu_attack_main_uniq_chk4(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS]
        .assign(&L2CValue::Ptr(ryu_attack_main_uniq_chk4 as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(ken_attack_main_loop as *const () as _))
}

unsafe extern "C" fn ken_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !CancelModule::is_enable_cancel(fighter.module_accessor) {
        if WorkModule::is_flag(
            fighter.module_accessor,
            *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL,
        ) {
            if AttackModule::is_infliction_status(
                fighter.module_accessor,
                *COLLISION_KIND_MASK_SHIELD | *COLLISION_KIND_MASK_HIT,
            ) {
                if ryu_final_hit_cancel(fighter, SITUATION_KIND_GROUND.into()).get_bool() {
                    return 1.into();
                }
            }
        }
        if WorkModule::is_flag(
            fighter.module_accessor,
            *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL,
        ) {
            if AttackModule::is_infliction_status(
                fighter.module_accessor,
                *COLLISION_KIND_MASK_SHIELD | *COLLISION_KIND_MASK_HIT,
            ) {
                if ryu_hit_cancel(fighter, SITUATION_KIND_GROUND.into()).get_bool() {
                    return 1.into();
                }
            }
        }
    }
    if ComboModule::count(fighter.module_accessor) == 1 {
        if !CancelModule::is_enable_cancel(fighter.module_accessor) {
            let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
            let attack_start_cancel_frame = WorkModule::get_param_float(
                fighter.module_accessor,
                hash40("param_private"),
                hash40("attack_start_cancel_frame"),
            );
            if current_frame < attack_start_cancel_frame {
                if ryu_kara_cancel(fighter).get_bool() {
                    return 1.into();
                }
            }
        }
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter
            .sub_wait_ground_check_common(false.into())
            .get_bool()
        {
            return 1.into();
        }
    }
    let mot = MotionModule::motion_kind(fighter.module_accessor);
    if [
        hash40("attack_11_w"),
        hash40("attack_11_s"),
        hash40("attack_11_near_s"),
    ]
    .contains(&mot)
    {
        if WorkModule::is_flag(
            fighter.module_accessor,
            *FIGHTER_RYU_STATUS_ATTACK_FLAG_WEAK_CANCEL,
        ) {
            if WorkModule::is_flag(
                fighter.module_accessor,
                *FIGHTER_RYU_STATUS_ATTACK_FLAG_BUTTON_TRIGGER,
            ) {
                if ControlModule::check_button_off(
                    fighter.module_accessor,
                    *CONTROL_PAD_BUTTON_ATTACK,
                ) {
                    let stick_y = fighter.global_table[STICK_Y].get_f32();
                    let attack_hi3_stick_y = WorkModule::get_param_float(
                        fighter.module_accessor,
                        hash40("common"),
                        hash40("attack_hi3_stick_y"),
                    );
                    let cont;
                    if !(stick_y < attack_hi3_stick_y) {
                        cont = false;
                    } else {
                        let attack_lw3_stick_y = WorkModule::get_param_float(
                            fighter.module_accessor,
                            hash40("common"),
                            hash40("attack_lw3_stick_y"),
                        );
                        if !(attack_lw3_stick_y < stick_y) {
                            cont = false;
                        } else {
                            let stick_x = fighter.global_table[STICK_X].get_f32();
                            let attack_s3_stick_x = WorkModule::get_param_float(
                                fighter.module_accessor,
                                hash40("common"),
                                hash40("attack_s3_stick_x"),
                            );
                            cont = stick_x < attack_s3_stick_x;
                        }
                    }
                    if cont {
                        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK.into(), false.into());
                        return 1.into();
                    }
                }
            }
        }
        if WorkModule::is_flag(
            fighter.module_accessor,
            *FIGHTER_RYU_STATUS_ATTACK_FLAG_SAME_ATTACK_CANCEL,
        ) {
            let button_on_frame = WorkModule::get_int(
                fighter.module_accessor,
                *FIGHTER_RYU_STATUS_ATTACK_INT_BUTTON_ON_FRAME,
            );
            let attack_11_s_button_on_frame = WorkModule::get_param_float(
                fighter.module_accessor,
                hash40("param_private"),
                hash40("attack_11_s_button_on_frame"),
            );
            if attack_11_s_button_on_frame <= button_on_frame as f32 {
                fighter.change_status(FIGHTER_STATUS_KIND_ATTACK.into(), false.into());
                return 1.into();
            }
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }
    // if 0 < WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME) {
    //     if !StopModule::is_stop(fighter.module_accessor) {
    //         if fighter.sub_check_button_jump().get_bool() {

    //         }
    //     }
    // }
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_RESTART) {
        if !MotionModule::is_end(fighter.module_accessor) {
            common::shoto_status::ryu_idkwhatthisis2(fighter);
        } else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    } else {
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK.into(), false.into());
    }
    0.into()
}

// FIGHTER_STATUS_KIND_WAIT //

#[status_script(agent = "ken", status = FIGHTER_STATUS_KIND_WAIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn wait_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_Wait()
}

// FIGHTER_STATUS_KIND_LANDING //

#[status_script(agent = "ken", status = FIGHTER_STATUS_KIND_LANDING, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn landing_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fgc_landing_main(fighter)
}
