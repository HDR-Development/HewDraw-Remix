use super::*;

pub fn install(agent: &mut Agent) {
        agent.acmd("game_appeallw", ike_sword_appeal_lw_game);
        agent.acmd("effect_appeallw", ike_sword_appeal_lw_effect);
}