use super::*;
use globals::*;
// status script import

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
    install_status_scripts!(
        pre_turndash,
        main_dashback,
        end_dashback,
        pre_superspecial,
        pre_superspecial2,
        wait_pre,
        //wait_main,
        landing_main,
        guard
    );
}

#[status_script(agent = "dolly", status = FIGHTER_STATUS_KIND_GUARD_OFF, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn guard(fighter: &mut L2CFighterCommon) -> L2CValue {
    let rate = fighter.status_GuardOff_Common().get_f32();
    WorkModule::enable_transition_term(
        fighter.module_accessor,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND,
    );
    if VarModule::is_flag(
        fighter.object(),
        vars::common::instance::IS_PARRY_FOR_GUARD_OFF,
    ) {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("just_shield_off"),
            0.0,
            0.0,
            false,
            0.0,
            false,
            false,
        );
        app::FighterUtil::flash_eye_info(fighter.module_accessor);
        if !WorkModule::is_flag(
            fighter.module_accessor,
            *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL,
        ) {
            ModelModule::enable_gold_eye(fighter.module_accessor);
            WorkModule::on_flag(
                fighter.module_accessor,
                *FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLAG_GOLD_EYE,
            );
        }
        EffectModule::req_on_joint(
            fighter.module_accessor,
            Hash40::new_raw(0xff4f9200f),
            Hash40::new("throw"),
            &Vector3f::zero(),
            &Vector3f::zero(),
            0.5,
            &Vector3f::zero(),
            &Vector3f::zero(),
            false,
            *EFFECT_SUB_ATTRIBUTE_NONE as u32,
            *EFFECT_FLIP_NONE,
            1,
        );
        EffectModule::req_common(fighter.module_accessor, Hash40::new("just_shield"), 0.0);
        // let shield_se = app::FighterUtil::get_just_shield_se(fighter.global_table[0x2].get_i32());
        SoundModule::play_se(
            fighter.module_accessor,
            smash::phx::Hash40::new("se_item_deathscythe_swing_m"),
            true,
            false,
            false,
            false,
            app::enSEType(0),
        );
    } else {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new_raw(0x97ab1c684),
            0.0,
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

#[status_script(agent = "dolly", status = FIGHTER_STATUS_KIND_TURN_DASH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn pre_turndash(fighter: &mut L2CFighterCommon) -> L2CValue {
    app::FighterSpecializer_Dolly::update_opponent_lr_1on1(
        fighter.module_accessor,
        *FIGHTER_STATUS_KIND_TURN_DASH,
    );
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

// FIGHTER_DOLLY_STATUS_KIND_DASH_BACK //

#[status_script(agent = "dolly", status = FIGHTER_DOLLY_STATUS_KIND_DASH_BACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn main_dashback(fighter: &mut L2CFighterCommon) -> L2CValue {
    fgc_dashback_main(fighter)
}

#[status_script(agent = "dolly", status = FIGHTER_DOLLY_STATUS_KIND_DASH_BACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn end_dashback(fighter: &mut L2CFighterCommon) -> L2CValue {
    common::shoto_status::fgc_end_dashback(fighter);
    original!(fighter)
}

// FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL //

#[status_script(agent = "dolly", status = FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn pre_superspecial(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let boma = app::sv_system::battle_object_module_accessor(lua_state);
    let mut agent_base = fighter.fighter_base.agent_base;
    let id = VarModule::get_int(
        fighter.battle_object,
        vars::common::instance::COSTUME_SLOT_NUMBER,
    ) as usize;

    // Only use meter if you didn't cancel directly from a different super
    if !VarModule::is_flag(boma.object(), vars::dolly::instance::SUPER_CANCEL) {
        MeterModule::drain(boma.object(), 4);
    }
    original!(fighter)
}

// FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2 //

#[status_script(agent = "dolly", status = FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn pre_superspecial2(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let boma = app::sv_system::battle_object_module_accessor(lua_state);
    let mut agent_base = fighter.fighter_base.agent_base;
    let id = VarModule::get_int(
        fighter.battle_object,
        vars::common::instance::COSTUME_SLOT_NUMBER,
    ) as usize;

    // Only use meter if you didn't cancel directly from a different supper
    if !VarModule::is_flag(boma.object(), vars::dolly::instance::SUPER_CANCEL) {
        MeterModule::drain(boma.object(), 4);
    }
    original!(fighter)
}

// FIGHTER_STATUS_KIND_WAIT //

#[status_script(agent = "dolly", status = FIGHTER_STATUS_KIND_WAIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn wait_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_Wait()
}

// vanilla script
#[status_script(agent = "dolly", status = FIGHTER_STATUS_KIND_WAIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn wait_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_wait_common();
    fighter.sub_wait_motion_mtrans();
    fighter.sub_shift_status_main(L2CValue::Ptr(fgc_wait_main_loop as *const () as _))
}

pub unsafe extern "C" fn fgc_wait_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.status_Wait_Main().get_bool() {
        return 0.into();
    }
    let lr = WorkModule::get_float(
        fighter.module_accessor,
        *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1,
    );
    if lr != 0.0 && PostureModule::lr(fighter.module_accessor) != lr {
        let stick_x_corrected = fighter.global_table[STICK_X].get_f32()
            * (PostureModule::lr(fighter.module_accessor) * -1.0);
        let stick_y = fighter.global_table[STICK_Y].get_f32();
        let walk_stick_x = WorkModule::get_param_float(
            fighter.module_accessor,
            hash40("common"),
            hash40("walk_stick_x"),
        );
        let squat_stick_y = WorkModule::get_param_float(
            fighter.module_accessor,
            hash40("common"),
            hash40("squat_stick_y"),
        );

        if WorkModule::is_enable_transition_term(
            fighter.module_accessor,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_WALK,
        ) {
            if walk_stick_x <= stick_x_corrected {
                if squat_stick_y < stick_y {
                    fighter.change_status(FIGHTER_RYU_STATUS_KIND_WALK_BACK.into(), true.into());
                    return 0.into();
                }
            }
        }
        fighter.change_status(FIGHTER_RYU_STATUS_KIND_TURN_AUTO.into(), false.into());
        return 0.into();
    }
    0.into()
}

// FIGHTER_STATUS_KIND_LANDING //

#[status_script(agent = "dolly", status = FIGHTER_STATUS_KIND_LANDING, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn landing_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fgc_landing_main(fighter)
}
