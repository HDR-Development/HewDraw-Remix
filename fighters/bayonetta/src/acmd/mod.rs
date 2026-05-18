use super::*;

mod ground;
mod tilts;
mod smashes;
mod aerials;
mod specials;
mod throws;
mod other;
mod bullets;

pub unsafe fn CHECK_BA(agent: &mut L2CAgentBase, enable: bool) -> bool {
    // only enables bullet arts if button has been held longer than 5f
    let buffer = ControlModule::get_command_life_count_max(agent.module_accessor) as usize;
    if agent.is_button_on(Buttons::Special) {
        if agent.is_button_trigger(Buttons::Special)
        || InputModule::get_trigger_count(agent.battle_object, Buttons::Special) < buffer {
            return false.into()
        }
        if enable{agent.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION); }
        return true.into()
    }
    if agent.is_button_on(Buttons::Attack) {
        if agent.is_button_trigger(Buttons::Attack)
        || InputModule::get_trigger_count(agent.battle_object, Buttons::Attack) < buffer {
            return false.into()
        }
        if enable{agent.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION); }
        return true.into()
    }
    agent.off_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
    false.into()
}

pub fn install(agent: &mut Agent) {
    ground::install(agent);
    tilts::install(agent);
    smashes::install(agent);
    aerials::install(agent);
    specials::install(agent);
    throws::install(agent);
    other::install(agent);
    bullets::install(agent);
}