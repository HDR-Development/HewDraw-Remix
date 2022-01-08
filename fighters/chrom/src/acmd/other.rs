
use super::*;

#[acmd_script( agent = "chrom", script = "game_appeallwl" , category = ACMD_GAME , low_priority)]
unsafe fn chrom_appeallwl_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        let id = hdr::get_player_number(boma);
        if(ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_HI)){
            CHROY_SWORD_TRAIL_EFFECT[id] = true;
        }
    }
    
}

pub fn install() {
    install_acmd_scripts!(
        chrom_catch_game,
        dash_game,
        dash_effect,
        turn_dash_game,
        chrom_appeallwr_game,
        chrom_appeallwl_game,
    );
}

