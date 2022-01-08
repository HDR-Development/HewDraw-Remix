
use super::*;

#[acmd_script( agent = "miiswordsman_chakram", script = "effect_stick" , category = ACMD_EFFECT , low_priority)]
unsafe fn miiswordsman_chakram_stick_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    if VarModule::is_flag(owner_module_accessor, miiswordsman::CHAKRAM_STICK_ATTACK) {
        frame(lua_state, 1.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_magicball_aura"), Hash40::new("top"), 0, 2.0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
        }
        wait(lua_state, 3.0);
        for _ in 0..7 {
            if is_excute(fighter) {
                EFFECT(fighter, Hash40::new("sys_magicball_aura"), Hash40::new("top"), 2.0, 2.0, -3.0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
            }
            wait(lua_state, 3.0);
            if is_excute(fighter) {
                EFFECT(fighter, Hash40::new("sys_magicball_aura"), Hash40::new("top"), 2.0, 4.0, 2.0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
            }
            wait(lua_state, 3.0);
            if is_excute(fighter) {
                EFFECT(fighter, Hash40::new("sys_magicball_aura"), Hash40::new("top"), -2.0, 5.0, -4.0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
            }
            wait(lua_state, 3.0);
            if is_excute(fighter) {
                EFFECT(fighter, Hash40::new("sys_magicball_aura"), Hash40::new("top"), 0, 0.0, 1.0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
            }
            wait(lua_state, 3.0);
            if is_excute(fighter) {
                EFFECT(fighter, Hash40::new("sys_magicball_aura"), Hash40::new("top"), 2.0, 1.0, 0.0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
            }
            wait(lua_state, 3.0);
        }
    }
    frame(lua_state, 142.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 0, 1, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
        
}

pub fn install() {
    install_acmd_scripts!(
        miiswordsman_catch_game,
        miiswordsman_catch_dash_game,
        miiswordsman_catch_turn_game,
        dash_game,
        dash_effect,
        turn_dash_game,
        miiswordsman_tornadoshot_fly_game,
        miiswordsman_wave_fly_game,
        miiswordsman_chakram_fly_game,
        miiswordsman_chakram_fly_normal_sub_game,
        miiswordsman_chakram_fly_flick_sub_game,
        //miiswordsman_chakram_hop_game,
        miiswordsman_chakram_stick_game,
        miiswordsman_chakram_stick_effect,
    );
}

