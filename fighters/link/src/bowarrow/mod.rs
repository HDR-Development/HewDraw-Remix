use super::*;

mod opff;
mod status;

pub fn install() {
    let agent = &mut Agent::new("link_bowarrow");
    opff::install(agent);
    status::install(agent);
    agent.install();
}
