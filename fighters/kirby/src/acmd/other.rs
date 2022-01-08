
use super::*;

#[acmd_script( agent = "kirby", script = "effect_dash" , category = ACMD_EFFECT , low_priority)]
unsafe fn dash_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.63, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_ALPHA(fighter, 0.7);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("footr"), 3, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }    
}

#[acmd_script( agent = "kirby", script = "game_turndash" , category = ACMD_GAME , low_priority)]
unsafe fn kirby_turn_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 3.0);
    if is_excute(fighter) {
		WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

#[acmd_script( agent = "kirby", script = "game_appealsl" , category = ACMD_GAME , low_priority)]
unsafe fn game_appealsl(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    //taunt_starrod(fighter);
}

#[acmd_script( agent = "kirby", script = "game_appealsr" , category = ACMD_GAME , low_priority)]
unsafe fn game_appealsr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    //taunt_starrod(fighter);   
}

unsafe fn taunt_starrod(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 65.0);
    if is_excute(fighter) {
        let id = hdr::get_player_number(boma);
        if !kirby_star_rod[id] {
            if [*CONTROL_PAD_BUTTON_APPEAL_S_L, *CONTROL_PAD_BUTTON_APPEAL_S_R].iter().any(|x| ControlModule::check_button_on(boma, *x)) {
                DamageModule::add_damage(boma, 75.0, 0);
                ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_STARROD), 0, 0, false, false);
                kirby_star_rod[id] = true;
            }
        }
    }
}

pub fn install() {
    install_acmd_scripts!(
        dash_effect,
        kirby_turn_dash_game,
		game_appealsl,
        game_appealsr,
    );
}

