use super::*;

mod status;

pub fn install() {
    let agent = &mut Agent::new("lucario_auraball");
    status::install(agent);
    agent.install();
}