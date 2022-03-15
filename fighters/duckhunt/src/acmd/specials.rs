
use super::*;

#[acmd_script( agent = "duckhunt", script = "game_specialhi" , category = ACMD_GAME , low_priority)]
unsafe fn duckhunt_special_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_DUCKHUNT_INSTANCE_WORK_ID_FLAG_REQUEST_SPECIAL_HI_CANCEL);
    }
    
}

#[acmd_script( agent = "duckhunt" , scripts = ["game_specialairlw", "game_speciallw"], category = ACMD_GAME , low_priority)]
unsafe fn duckhunt_special_airlw_game(fighter: &mut L2CAgentBase) {
    let lua_state: u64 = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_DUCKHUNT_STATUS_SPECIAL_LW_FLAG_CALL_TRIGGER);
    }
}

#[acmd_script( agent = "duckhunt_gunman" , scripts = ["sound_readyr", "sound_readyl"] , category = ACMD_SOUND , low_priority)]
unsafe fn gunman_downb_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_duckhunt_special_l02"));
    }
    frame(lua_state, 280.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_duckhunt_special_l09"));
    } 
}

#[acmd_script( agent = "duckhunt_gunman" , scripts = ["effect_readyr" , "effect_readyl"] , category = ACMD_EFFECT , low_priority)]
unsafe fn gunman_downb_effect_stub(fighter: &mut L2CAgentBase) {
    
}

pub fn install() {
    install_acmd_scripts!(
        duckhunt_special_hi_game,
        duckhunt_special_airlw_game,
        gunman_downb_sound,
        //gunman_downb_effect_stub,
    );
}

