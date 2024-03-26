
use super::*;

unsafe extern "C" fn rosetta_special_lw_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if !VarModule::is_flag(agent.battle_object, vars::rosetta::instance::IS_TICO_UNAVAILABLE) && VarModule::get_int(agent.battle_object, vars::rosetta::instance::COOLDOWN) == 0 {
        if is_excute(agent) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
            ItemModule::set_have_item_visibility(boma, false, 0);
        }
        frame(lua_state, 17.0);
        if is_excute(agent) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_awaken"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(lua_state, 27.0);
        if is_excute(agent) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(lua_state, 35.0);
        if is_excute(agent) {
            ItemModule::set_have_item_visibility(boma, true, 0);
        }
    } else {
        if is_excute(agent) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
            ItemModule::set_have_item_visibility(boma, false, 0);
        }
        frame(lua_state, 2.0);
        if is_excute(agent) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(lua_state, 14.0);
        if is_excute(agent) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(lua_state, 26.0);
        if is_excute(agent) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        
        frame(lua_state, 35.0);
        if is_excute(agent) {
            ItemModule::set_have_item_visibility(boma, true, 0);
        }
        frame(lua_state, 38.0);
        if is_excute(agent) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    }
    
}

pub fn install() {
    smashline::Agent::new("rosetta")
        .acmd("expression_speciallw", rosetta_special_lw_expression)
        .install();
}
