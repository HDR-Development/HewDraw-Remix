use super::*;

pub unsafe extern "C" fn special_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut rebel_gauge = WorkModule::get_float(fighter.module_accessor, 0x4D);
    // scales penalty from 15% @ 25% meter to 5% at 100% meter
    rebel_gauge = (1.15 * rebel_gauge - 20.0).clamp(10.0, 100.0);
    WorkModule::set_float(fighter.module_accessor, rebel_gauge, 0x4D);
    WorkModule::on_flag(fighter.module_accessor, 0x200000E3); // FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE_SUMMON
    FighterSpecializer_Jack::check_doyle_summon_dispatch(fighter.module_accessor, true, false);
    StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_JACK_STATUS_KIND_SUMMON);
    1.into()
}

pub unsafe extern "C" fn special_lw2_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, 0x200000E4); // FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE_END
    FighterSpecializer_Jack::check_doyle_summon_dispatch(fighter.module_accessor, true, false);
    StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_JACK_STATUS_KIND_DISPATCH);
    1.into()
}
