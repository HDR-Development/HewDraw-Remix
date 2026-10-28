use super::*;

mod acmd;
mod vtable;

pub fn install() {
    let agent = &mut Agent::new("miigunner_grenadelauncher");
    acmd::install(agent);
    agent.install();

    vtable::install();
}