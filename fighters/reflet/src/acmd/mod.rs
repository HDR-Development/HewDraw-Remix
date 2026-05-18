use super::*;

mod ground;
mod tilts;
mod smashes;
mod aerials;
mod specials;
mod throws;
mod other;

pub unsafe fn LEVIN_OFF(boma: &mut BattleObjectModuleAccessor) {
    if boma.get_int(*FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) <= 0 
    && boma.is_flag(*FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
        VarModule::off_flag(utils::util::get_battle_object_from_accessor(boma), vars::reflet::instance::DISCARD_SKIP_STATUS);
        VisibilityModule::set_int64(boma, Hash40::new("sword").hash as i64, Hash40::new("sword_normal").hash as i64);
        boma.off_flag( *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON);
    }
}

pub fn install(agent: &mut Agent) {
    ground::install(agent);
    tilts::install(agent);
    smashes::install(agent);
    aerials::install(agent);
    specials::install(agent);
    throws::install(agent);
    other::install(agent);
}