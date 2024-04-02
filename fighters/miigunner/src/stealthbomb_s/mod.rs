use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("miigunner_stealthbomb_s");
    acmd::install(agent);
    agent.install();
}