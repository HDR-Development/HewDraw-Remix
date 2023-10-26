use super::*;

pub mod doyle;
pub mod special_lw;

pub unsafe fn special_check_summon(fighter: &mut L2CFighterCommon) -> bool {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_RESERVE_SUMMON_DISPATCH)
    && WorkModule::get_int(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_INT_CUSTOMIZE_TO) < 0 {
        return false;
    }

    let interrupt = fighter.global_table[globals::STATUS_KIND_INTERRUPT].get_i32();
    WorkModule::set_int(fighter.module_accessor, interrupt, *FIGHTER_JACK_INSTANCE_WORK_ID_INT_SPECIAL_KIND_CUSTOMIZE);
    StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_JACK_STATUS_KIND_SPECIAL_CUSTOMIZE);
    true
}

pub unsafe fn try_interrupt_with_summon(fighter: &mut L2CFighterCommon) -> bool {
    if summon_arsene(fighter) {
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_JACK_STATUS_KIND_SUMMON);
        true
    } else {
        false
    }
}

pub unsafe fn summon_arsene(fighter: &mut L2CFighterCommon) -> bool {
    let rebel_gauge = fighter.get_float(0x4D);
    fighter.on_flag(0x200000e3);
    VarModule::set_float(fighter.battle_object, vars::jack::REBEL_GAUGE_ON_SUMMON, rebel_gauge);
    let status_kind = app::FighterSpecializer_Jack::check_doyle_summon_dispatch(fighter.module_accessor, true, true) as i32;
    if status_kind == *FIGHTER_JACK_STATUS_KIND_SUMMON {
        true
    } else {
        false
    }
}

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

unsafe extern "C" fn jack_move_customizer(fighter: &mut L2CFighterCommon) -> L2CValue {
    let customize_to = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_WAZA_CUSTOMIZE_TO);
    if customize_to == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_LW_1 {
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(), std::mem::transmute(special_lw::special_lw_pre as *const ()));
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(), std::mem::transmute(special_lw::special_lw_main as *const ()));
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_END.into(), std::mem::transmute(special_lw::special_lw_end as *const ()));
        0.into()
    } else if let Some(original) = get_original_customizer(fighter) {
        original(fighter)
    } else {
        0.into()
    }
}

#[fighter_init]
fn jack_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        if fighter.kind() != *FIGHTER_KIND_JACK {
            return;
        }

        set_move_customizer(fighter, jack_move_customizer);
        jack_move_customizer(fighter);
    }
}

pub fn install() {
    smashline::install_agent_init_callbacks!(jack_init);
    special_lw::install();
    doyle::install();
}