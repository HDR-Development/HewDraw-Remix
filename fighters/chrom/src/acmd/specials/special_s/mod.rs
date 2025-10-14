use super::*;

mod special_s2;
mod special_s3;
mod special_s4;

pub fn install(agent: &mut Agent) {
    // STUBS
    agent.acmd("game_specials1", acmd_stub, Priority::Low);
    agent.acmd("effect_specials1", acmd_stub, Priority::Low);
    agent.acmd("sound_specials1", acmd_stub, Priority::Low);
    agent.acmd("expression_specials1", acmd_stub, Priority::Low);
    agent.acmd("game_specialairs1", acmd_stub, Priority::Low);
    agent.acmd("effect_specialairs1", acmd_stub, Priority::Low);
    agent.acmd("sound_specialairs1", acmd_stub, Priority::Low);
    agent.acmd("expression_specialairs1", acmd_stub, Priority::Low);
    agent.acmd("game_specials2hi", acmd_stub, Priority::Low);
    agent.acmd("effect_specials2hi", acmd_stub, Priority::Low);
    agent.acmd("sound_specials2hi", acmd_stub, Priority::Low);
    agent.acmd("expression_specials2hi", acmd_stub, Priority::Low);
    agent.acmd("game_specialairs2hi", acmd_stub, Priority::Low);
    agent.acmd("effect_specialairs2hi", acmd_stub, Priority::Low);
    agent.acmd("sound_specialairs2hi", acmd_stub, Priority::Low);
    agent.acmd("expression_specialairs2hi", acmd_stub, Priority::Low);
    agent.acmd("game_specials3hi", acmd_stub, Priority::Low);
    agent.acmd("effect_specials3hi", acmd_stub, Priority::Low);
    agent.acmd("sound_specials3hi", acmd_stub, Priority::Low);
    agent.acmd("expression_specialairs3hi", acmd_stub, Priority::Low);
    agent.acmd("game_specialairs3hi", acmd_stub, Priority::Low);
    agent.acmd("effect_specialairs3hi", acmd_stub, Priority::Low);
    agent.acmd("sound_specialairs3hi", acmd_stub, Priority::Low);
    agent.acmd("expression_specialairs3hi", acmd_stub, Priority::Low);

    special_s2::install(agent);
    special_s3::install(agent);
    special_s4::install(agent);
}