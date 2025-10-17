use super::*;
use globals::*;
// status script import

mod special_lw1;
mod special_lw2;
mod special_lw3;
mod special_n2;
mod special_s1;
mod special_hi3;

unsafe fn set_move_customizer(fighter: &mut L2CFighterCommon, customizer: unsafe extern "C" fn(&mut L2CFighterCommon) -> L2CValue) {
    if fighter.global_table["move_customizer_set"].get_bool() {
        return;
    }

    let clone = fighter.global_table[globals::WAZA_CUSTOMIZE_CONTROL].clone();
    fighter.global_table["move_customizer_set"].assign(&L2CValue::Bool(true));
    fighter.global_table["move_customizer_original"].assign(&clone);
    fighter.global_table[globals::WAZA_CUSTOMIZE_CONTROL].assign(&L2CValue::Ptr(customizer as *const () as _));
}

unsafe fn get_original_customizer(fighter: &mut L2CFighterCommon) -> Option<unsafe extern "C" fn(&mut L2CFighterCommon) -> L2CValue> {
    let ptr = fighter.global_table["move_customizer_original"].get_ptr();
    if !ptr.is_null() {
        Some(std::mem::transmute(ptr))
    } else {
        None
    }
}

unsafe extern "C" fn move_customizer(fighter: &mut L2CFighterCommon) -> L2CValue {
    let customize_to = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_WAZA_CUSTOMIZE_TO);
    if let Some(original) = get_original_customizer(fighter) {
        original(fighter);
    }
    if customize_to == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_N_2 {
        fighter.sv_set_status_func(
            FIGHTER_STATUS_KIND_SPECIAL_N.into(),
            LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(),
            std::mem::transmute(special_n2::special_n2_pre as *const ())
        );
    } else if customize_to == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_S_1 {
        fighter.sv_set_status_func(
            FIGHTER_STATUS_KIND_SPECIAL_S.into(),
            LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(),
            std::mem::transmute(special_s1::special_s1_main as *const ())
        );
    } else if customize_to == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_LW_2 {
        // This is technically bad behavior because Waza Customize does NOT
        // handle exec statuses for the other specials. However, this
        // doesn't matter in this case because 1) you can't swap Mii Brawler's
        // specials mid-match, and 2) even if you could, none of the other
        // specials have search boxes anyway.
        fighter.sv_set_status_func(
            FIGHTER_STATUS_KIND_SPECIAL_LW.into(),
            LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS.into(),
            std::mem::transmute(special_lw2::special_lw2_exec as *const ())
        );
        // This is *also* bad behavior, but for some reason Smashline isn't
        // installing the exec status for this, so...
        fighter.sv_set_status_func(
            FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW2_START.into(),
            LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS.into(),
            std::mem::transmute(special_lw2::special_lw2_exec as *const ())
        );
    } else if customize_to == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_LW_3 {
        fighter.sv_set_status_func(
            FIGHTER_STATUS_KIND_SPECIAL_LW.into(),
            LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(),
            std::mem::transmute(special_lw3::special_lw3_pre as *const ())
        );
    } else if customize_to == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_HI_3 {
        fighter.sv_set_status_func(
            FIGHTER_STATUS_KIND_SPECIAL_HI.into(),
            LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(),
            std::mem::transmute(special_hi3::special_hi3_pre as *const ())
        );
    }
    
    return 0.into();
}

unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if [*SITUATION_KIND_GROUND, *SITUATION_KIND_CLIFF].contains(&fighter.global_table[SITUATION_KIND].get_i32())
    || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_GIMMICK_SPRING_JUMP]) {
        VarModule::off_flag(fighter.battle_object, vars::miifighter::instance::SPECIAL_LW3_STALL);
    }
    true.into()
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    set_move_customizer(fighter, move_customizer);
    move_customizer(fighter);
    fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);
    special_lw1::install(agent);
    special_lw2::install(agent);
    special_lw3::install(agent);
    special_s1::install(agent);
}