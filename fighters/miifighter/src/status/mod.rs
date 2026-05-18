use super::*;
use globals::*;
// status script import

mod special_hi1;
mod special_hi3;
mod special_lw1;
mod special_lw2;
mod special_lw3;
mod special_n1;
mod special_n2;
mod special_n3;
mod special_s1;

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
    if customize_to == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_N_1 {
        fighter.sv_set_status_func(
            FIGHTER_STATUS_KIND_SPECIAL_N.into(),
            LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(),
            std::mem::transmute(special_n1::special_n1_main as *const ())
        );
        fighter.sv_set_status_func(
            FIGHTER_STATUS_KIND_SPECIAL_N.into(),
            LUA_SCRIPT_STATUS_FUNC_STATUS_END.into(),
            std::mem::transmute(special_n1::special_n1_end as *const ())
        );
    }
    else if customize_to == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_N_2 {
        fighter.sv_set_status_func(
            FIGHTER_STATUS_KIND_SPECIAL_N.into(),
            LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(),
            std::mem::transmute(special_n2::special_n2_pre as *const ())
        );
        fighter.sv_set_status_func(
            FIGHTER_STATUS_KIND_SPECIAL_N.into(),
            LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(),
            std::mem::transmute(special_n2::special_n2_main as *const ())
        );
        fighter.sv_set_status_func(
            FIGHTER_STATUS_KIND_SPECIAL_N.into(),
            LUA_SCRIPT_STATUS_FUNC_STATUS_END.into(),
            std::mem::transmute(special_n2::special_n2_end as *const ())
        );
    } else if customize_to == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_N_3 {
        fighter.sv_set_status_func(
            FIGHTER_STATUS_KIND_SPECIAL_N.into(),
            LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(),
            std::mem::transmute(special_n3::special_n3_pre as *const ())
        );
    } else if customize_to == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_S_1 {
        fighter.sv_set_status_func(
            FIGHTER_STATUS_KIND_SPECIAL_S.into(),
            LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(),
            std::mem::transmute(special_s1::special_s1_main as *const ())
        );
    } else if customize_to == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_HI_3 {
        fighter.sv_set_status_func(
            FIGHTER_STATUS_KIND_SPECIAL_HI.into(),
            LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(),
            std::mem::transmute(special_hi3::special_hi3_pre as *const ())
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
        fighter.sv_set_status_func(
            FIGHTER_STATUS_KIND_SPECIAL_LW.into(),
            LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(),
            std::mem::transmute(special_lw3::special_lw3_main as *const ())
        );
        fighter.sv_set_status_func(
            FIGHTER_STATUS_KIND_SPECIAL_LW.into(),
            LUA_SCRIPT_STATUS_FUNC_STATUS_END.into(),
            std::mem::transmute(special_lw3::special_lw3_end as *const ())
        );
    }
    
    return 0.into();
}

unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if [*SITUATION_KIND_GROUND, *SITUATION_KIND_CLIFF].contains(&fighter.global_table[SITUATION_KIND].get_i32())
    || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_GIMMICK_SPRING_JUMP]) {
        VarModule::off_flag(fighter.battle_object, vars::miifighter::instance::SPECIAL_N3_STALL);
        VarModule::off_flag(fighter.battle_object, vars::miifighter::instance::SPECIAL_LW3_STALL);
    }
    true.into()
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    set_move_customizer(fighter, move_customizer);
    move_customizer(fighter);
    fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));

    VarModule::set_int(fighter.battle_object, vars::miifighter::instance::SPECIAL_N2_CHARGE_COUNT, 0);
    VarModule::set_int(fighter.battle_object, vars::miifighter::instance::SPECIAL_LW3_STAGE, 0);
    VarModule::set_int(fighter.battle_object, vars::miifighter::instance::SPECIAL_LW3_TIMER, 0);
    VarModule::set_int(fighter.battle_object, vars::miifighter::instance::SPECIAL_LW3_EFFECT_HANDLE_1, -1);
    //VarModule::set_int(fighter.battle_object, vars::miifighter::instance::SPECIAL_LW3_EFFECT_HANDLE_2, -1);
}

unsafe extern "C" fn entry_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    reset_boiling_punt(fighter);
    VarModule::set_int(fighter.battle_object, vars::miifighter::instance::SPECIAL_N2_CHARGE_COUNT, 0);
    smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_ENTRY)(fighter)
}


unsafe extern "C" fn dead_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    reset_boiling_punt(fighter);
    VarModule::set_int(fighter.battle_object, vars::miifighter::instance::SPECIAL_N2_CHARGE_COUNT, 0);
    EffectModule::remove_common(fighter.module_accessor, Hash40::new("charge_max"));
    smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_DEAD)(fighter)
}

unsafe extern "C" fn rebirth_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    reset_boiling_punt(fighter);
    VarModule::set_int(fighter.battle_object, vars::miifighter::instance::SPECIAL_N2_CHARGE_COUNT, 0);
    EffectModule::remove_common(fighter.module_accessor, Hash40::new("charge_max"));
    smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_REBIRTH)(fighter)
}

unsafe extern "C" fn win_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    EffectModule::remove_common(fighter.module_accessor, Hash40::new("charge_max"));
    smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_WIN)(fighter)
}

unsafe extern "C" fn damage_fly_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    reset_boiling_punt(fighter);
    smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_DAMAGE_FLY)(fighter)
}

unsafe extern "C" fn damage_fly_roll_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    reset_boiling_punt(fighter);
    smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL)(fighter)
}

unsafe extern "C" fn damage_fly_meteor_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    reset_boiling_punt(fighter);
    smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR)(fighter)
}

unsafe fn reset_boiling_punt(fighter: &mut L2CFighterCommon) {
    if VarModule::get_int(fighter.battle_object, vars::miifighter::instance::SPECIAL_LW3_STAGE) > 0 {
        VarModule::set_int(fighter.battle_object, vars::miifighter::instance::SPECIAL_LW3_STAGE, 0);
        //EFFECT_FOLLOW(fighter, Hash40::new("sys_flash"), Hash40::new("top"), -3, 13, -2.5, 0, 0, 0, 0.38, false);
        let handle = VarModule::get_int(fighter.battle_object, vars::miifighter::instance::SPECIAL_LW3_EFFECT_HANDLE_1) as u32;
        EffectModule::kill(fighter.module_accessor, handle, false, false);
        //let handle2 = VarModule::get_int(fighter.battle_object, vars::miifighter::instance::SPECIAL_LW3_EFFECT_HANDLE_2) as u32;
        //EffectModule::kill(fighter.module_accessor, handle2, false, false);
        VarModule::set_int(fighter.battle_object, vars::miifighter::instance::SPECIAL_LW3_EFFECT_HANDLE_1, -1);
        //VarModule::set_int(fighter.battle_object, vars::miifighter::instance::SPECIAL_LW3_EFFECT_HANDLE_2, -1);
        ColorBlendModule::cancel_main_color(fighter.module_accessor, 0);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);
    special_hi1::install(agent);
    special_lw1::install(agent);
    special_lw2::install(agent);
    special_n2::install(agent);
    special_n3::install(agent);
    special_s1::install(agent);

    agent.status(Main, *FIGHTER_STATUS_KIND_ENTRY, entry_main);
    agent.status(Main, *FIGHTER_STATUS_KIND_DEAD, dead_main);
    agent.status(Main, *FIGHTER_STATUS_KIND_REBIRTH, rebirth_main);
    agent.status(Main, *FIGHTER_STATUS_KIND_WIN, win_main);
    agent.status(Main, *FIGHTER_STATUS_KIND_DAMAGE_FLY, damage_fly_main);
    agent.status(Main, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, damage_fly_roll_main);
    agent.status(Main, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, damage_fly_meteor_main);
}