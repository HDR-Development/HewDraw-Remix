use super::*;

mod acmd;

mod vtable;

pub fn install() {
    let agent = &mut smashline::Agent::new("dolly_burst");
    acmd::install(agent);
    agent.install();

    vtable::install();
}