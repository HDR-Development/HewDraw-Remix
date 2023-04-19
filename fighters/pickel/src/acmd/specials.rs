
use super::*;

#[acmd_script( agent = "pickel", script = "game_specialsset", category = ACMD_GAME, low_priority )]
unsafe fn pickel_special_s_set_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
        VarModule::on_flag(fighter.battle_object, vars::pickel::instance::DISABLE_SPECIAL_S);
    }
}

pub fn install() {
    install_acmd_scripts!(
        pickel_special_s_set_game,
    );
}

