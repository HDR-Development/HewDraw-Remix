use super::*;

// FIGHTER_STATUS_KIND_SPECIAL_S

pub unsafe extern "C" fn special_s(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Once per airtime
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        VarModule::on_flag(fighter.battle_object, vars::koopajr::instance::DISABLE_SPECIAL_S);
    }
    smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_S)(fighter)
}

// FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_S_DASH

pub unsafe extern "C" fn special_s_dash_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    smashline::original_status(Main, fighter, *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_S_DASH)(fighter);

    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_dash_main_loop as *const () as _))    
}

unsafe extern "C" fn special_s_dash_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KOOPAJR_STATUS_SPECIAL_S_FLAG_CLIFF_JUMP)
    && (fighter.is_cat_flag(Cat2::CommonGuard)
        || fighter.is_cat_flag(Cat1::AttackN)
        || fighter.is_cat_flag(Cat1::SpecialAny)) {
        fighter.change_status(FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_S_SPIN_TURN.into(), true.into());
        return 1.into();
    }

    if let Some(target) = smashline::api::get_target_function("lua2cpp_koopajr.nrs", 0x14500) {
        let og_special_s_dash_main_loop: fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(target);
        return og_special_s_dash_main_loop(fighter);
    }

    0.into()
}

// FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_S_JUMP

pub unsafe extern "C" fn special_s_jump_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Burn double jump when jumping out of Clown Kart Dash
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR
    && fighter.get_num_used_jumps() < fighter.get_jump_count_max() {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s);
    agent.status(Main, *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_S_DASH, special_s_dash_main);
    agent.status(Init, *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_S_JUMP, special_s_jump_init);
}