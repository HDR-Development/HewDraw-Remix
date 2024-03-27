use super::*;

mod opff;

pub fn install() {
    let agent = &mut Agent::new("koopajr_remainclown");
    opff::install(agent);
    agent.install();
}