use super::*;

mod opff;

pub fn install() {
    let agent = &mut Agent::new("edge_flaredummy");
    opff::install(agent);
    agent.install();
}