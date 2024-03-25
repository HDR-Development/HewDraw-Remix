use super::*;

pub mod doyle;
pub mod special_lw;
pub mod summon;
pub mod dispatch;

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
        fighter.sv_set_status_func(
            FIGHTER_STATUS_KIND_SPECIAL_LW.into(),
            LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(),
            std::mem::transmute(special_lw::jack_special_lw_pre as *const ())
        );
        0.into()
    } else if customize_to == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_LW_2 {
        fighter.sv_set_status_func(
            FIGHTER_STATUS_KIND_SPECIAL_LW.into(),
            LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(),
            std::mem::transmute(special_lw::jack_special_lw2_pre as *const ())
        );
        0.into()
    } else if let Some(original) = get_original_customizer(fighter) {
        original(fighter)
    } else {
        0.into()
    }
}

unsafe extern "C" fn jack_special_lw_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    let rebel_gauge = WorkModule::get_float(fighter.module_accessor, 0x4D);
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE_EXIST)
    && rebel_gauge < 10.0 { // FIGHTER_JACK_INSTANCE_WORK_ID_FLOAT_REBEL_GAUGE
        return 0.into();
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE_EXIST)
    && rebel_gauge < 1.0 { // FIGHTER_JACK_INSTANCE_WORK_ID_FLOAT_REBEL_GAUGE
        return 0.into();
    }
    1.into()
}

extern "C" fn jack_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        if fighter.kind() != *FIGHTER_KIND_JACK {
            return;
        }

        set_move_customizer(fighter, jack_move_customizer);
        jack_move_customizer(fighter);
        fighter.global_table[globals::USE_SPECIAL_LW_CALLBACK].assign(&L2CValue::Ptr(jack_special_lw_uniq as *const () as _));
    }
}

pub fn install(agent: &mut Agent) {
    dispatch::install(agent);
    doyle::install(agent);
    summon::install(agent);
}
