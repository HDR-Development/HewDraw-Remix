use super::*;
use globals::*;

mod special_n;

unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Reset cape stall flag on landing or ledgegrab
    if [*SITUATION_KIND_GROUND, *SITUATION_KIND_CLIFF].contains(&fighter.global_table[SITUATION_KIND].get_i32())
    || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD]) {
        VarModule::off_flag(fighter.battle_object, vars::mario::instance::SPECIAL_S_DISABLE_STALL);
    }
    true.into()
}

#[status_script(agent = "mario", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe extern "C" fn special_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT);
    return 1.into()
}

#[smashline::fighter_init]
fn mario_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        if fighter.kind() == *FIGHTER_KIND_MARIO {
            fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));   
        }
    }
}

#[status_script(agent = "mario", status = FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe extern "C" fn special_lw_shoot_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor,
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );

    return 0.into()
}

pub fn install() {
    smashline::install_agent_init_callbacks!(mario_init);
    special_n::install();
    install_status_scripts!(
        special_lw_pre,
        special_lw_shoot_pre
    );
}