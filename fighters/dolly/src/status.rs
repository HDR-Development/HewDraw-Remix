use super::*;
use globals::*;
// status script import
 
utils::import_noreturn!(common::shoto_status::{
    fgc_pre_dashback,
    fgc_end_dashback,
    ryu_idkwhatthisis2,
    fgc_init_landing,
    fgc_exec_landing
});

extern "Rust" {
    // from common::shoto_status
    fn ryu_kara_cancel(fighter: &mut L2CFighterCommon) -> L2CValue;
    fn ryu_attack_main_uniq_chk(fighter: &mut L2CFighterCommon) -> L2CValue;
    fn fgc_dashback_main(fighter: &mut L2CFighterCommon) -> L2CValue;
    fn ryu_attack_main_uniq_chk4(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue;
    fn ryu_final_hit_cancel(fighter: &mut L2CFighterCommon, situation: L2CValue) -> L2CValue;
    fn ryu_hit_cancel(fighter: &mut L2CFighterCommon, situation: L2CValue) -> L2CValue;
}

pub fn install() {
    install_status_scripts!(
        pre_turndash,
        pre_dashback,
        main_dashback,
        end_dashback,
        pre_superspecial,
        pre_superspecial2,
        landing_init,
        landing_exec
    );
}

// FIGHTER_STATUS_KIND_TURN_DASH //

#[status_script(agent = "dolly", status = FIGHTER_STATUS_KIND_TURN_DASH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn pre_turndash(fighter: &mut L2CFighterCommon) -> L2CValue {
    app::FighterSpecializer_Dolly::update_opponent_lr_1on1(fighter.module_accessor, *FIGHTER_STATUS_KIND_TURN_DASH);
    let lr = WorkModule::get_float(fighter.module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1);
    if lr != 0.0 {
        if PostureModule::lr(fighter.module_accessor) == lr {
            if fighter.global_table[PREV_STATUS_KIND] != FIGHTER_STATUS_KIND_TURN {
                StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_RYU_STATUS_KIND_DASH_BACK);
                return L2CValue::I32(1);
            }
        }
    }
    StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_STATUS_KIND_TURN);
    return 1.into()
}

// FIGHTER_DOLLY_STATUS_KIND_DASH_BACK //

#[status_script(agent = "dolly", status = FIGHTER_DOLLY_STATUS_KIND_DASH_BACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn pre_dashback(fighter: &mut L2CFighterCommon) -> L2CValue {
    common::shoto_status::fgc_pre_dashback(fighter);
    original!(fighter)
}

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
    let id = VarModule::get_int(fighter.battle_object, vars::common::instance::COSTUME_SLOT_NUMBER) as usize;

    // Only use meter if you didn't cancel directly from a different super
    if  !VarModule::is_flag(boma.object(), vars::dolly::instance::SUPER_CANCEL) {
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
    let id = VarModule::get_int(fighter.battle_object, vars::common::instance::COSTUME_SLOT_NUMBER) as usize;

    // Only use meter if you didn't cancel directly from a different supper
    if  !VarModule::is_flag(boma.object(), vars::dolly::instance::SUPER_CANCEL) {
        MeterModule::drain(boma.object(), 4);
    }
    original!(fighter)
}

// FIGHTER_STATUS_KIND_LANDING //

#[status_script(agent = "dolly", status = FIGHTER_STATUS_KIND_LANDING, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
pub unsafe fn landing_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    app::FighterSpecializer_Dolly::update_opponent_lr_1on1(fighter.module_accessor, *FIGHTER_STATUS_KIND_TURN_DASH);
    common::shoto_status::fgc_init_landing(fighter);
    original!(fighter)
}

#[status_script(agent = "dolly", status = FIGHTER_STATUS_KIND_LANDING, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
pub unsafe fn landing_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    app::FighterSpecializer_Dolly::update_opponent_lr_1on1(fighter.module_accessor, *FIGHTER_STATUS_KIND_TURN_DASH);
    common::shoto_status::fgc_exec_landing(fighter);
    original!(fighter)
}