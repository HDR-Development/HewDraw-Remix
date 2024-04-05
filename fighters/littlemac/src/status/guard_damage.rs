use super::*;

unsafe extern "C" fn guard_damage_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
        let meter_gain = 20.0; //ParamModule::get_float(fighter.battle_object, ParamType::Agent, "power_meter.parry_meter_gain");
        let meter = WorkModule::get_float(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE);
        let meter_inc = (meter + meter_gain).clamp(0.0, 100.0);
        WorkModule::set_float(fighter.module_accessor, meter_inc, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE);
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        crate::vtable_hook::update_littlemac_ui(entry_id, meter_inc);
    }

    fighter.sub_ftStatusUniqProcessGuardDamage_initStatus()
}

pub fn install(agent: &mut Agent) {
    agent.status(Init, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, guard_damage_init);
}