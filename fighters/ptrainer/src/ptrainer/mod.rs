use super::*;

mod opff;
mod status;

pub fn install() {
    let agent = &mut Agent::new("ptrainer_ptrainer");
    opff::install(agent);
    status::install(agent);
    agent.install();
}