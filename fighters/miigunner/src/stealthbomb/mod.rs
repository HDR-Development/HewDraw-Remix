use super::*;

mod acmd;
mod status;

pub fn install() {
    let agent = &mut Agent::new("miigunner_stealthbomb");
    acmd::install(agent);
    status::install(agent);
    agent.install();
}