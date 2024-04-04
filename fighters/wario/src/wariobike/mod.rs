use super::*;

mod status;

pub fn install() {
    let agent = &mut Agent::new("wario_wariobike");
    status::install(agent);
    agent.install();
}