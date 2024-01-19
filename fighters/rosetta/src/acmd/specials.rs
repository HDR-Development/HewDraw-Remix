
use super::*;

#[acmd_script( agent = "rosetta", script = "expression_speciallw", category = ACMD_EXPRESSION, low_priority )]
unsafe fn rosetta_special_lw_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if !VarModule::is_flag(fighter.battle_object, vars::rosetta::instance::IS_TICO_UNAVAILABLE) && VarModule::get_int(fighter.battle_object, vars::rosetta::instance::COOLDOWN) == 0 {
        if is_excute(fighter) {
            slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
            ItemModule::set_have_item_visibility(boma, false, 0);
        }
        frame(lua_state, 17.0);
        if is_excute(fighter) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_awaken"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(lua_state, 27.0);
        if is_excute(fighter) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(lua_state, 35.0);
        if is_excute(fighter) {
            ItemModule::set_have_item_visibility(boma, true, 0);
        }
    } else {
        if is_excute(fighter) {
            slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
            ItemModule::set_have_item_visibility(boma, false, 0);
        }
        frame(lua_state, 2.0);
        if is_excute(fighter) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(lua_state, 14.0);
        if is_excute(fighter) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(lua_state, 26.0);
        if is_excute(fighter) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        
        frame(lua_state, 35.0);
        if is_excute(fighter) {
            ItemModule::set_have_item_visibility(boma, true, 0);
        }
        frame(lua_state, 38.0);
        if is_excute(fighter) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    }
    
}


pub fn install() {
    install_acmd_scripts!(
        rosetta_special_lw_expression
    );
}

