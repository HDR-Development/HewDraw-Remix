use super::*;

unsafe extern "C" fn game_hit(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    if is_excute(fighter) {
        MeterModule::add(owner_module_accessor.object(), 8.0);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_hit", game_hit);
}