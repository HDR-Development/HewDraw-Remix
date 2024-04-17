use super::*;

unsafe extern "C" fn game_hit(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    if is_excute(agent) {
        MeterModule::add(owner_module_accessor.object(), 8.0);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_hit", game_hit);
}