
use super::*;

#[acmd_script( agent = "demon", script = "expression_throwhi" , category = ACMD_EXPRESSION , low_priority)]
unsafe fn demon_throw_hi_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
	    ControlModule::set_rumble(boma, Hash40::new("rbkind_nohit_attacks"), 0, false, 0 as u32);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        MotionModule::set_rate(boma, 2.0);
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        MotionModule::set_rate(boma, 1.0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_80_beam"), 0, false, 0 as u32);
    }
}

pub fn install() {
    install_acmd_scripts!(
        demon_throw_hi_expression
);
}

