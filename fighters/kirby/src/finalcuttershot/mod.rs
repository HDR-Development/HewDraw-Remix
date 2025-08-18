use super::*;

mod status;

pub fn install() {
    let agent = &mut Agent::new("kirby_finalcuttershot");
    status::install(agent);
    agent.install();
}