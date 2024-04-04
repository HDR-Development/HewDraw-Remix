use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("miigunner_attackairf_bullet");
    acmd::install(agent);
    agent.install();
}