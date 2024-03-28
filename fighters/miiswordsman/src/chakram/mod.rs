use super::*;

mod acmd;
mod status;

pub fn install() {
    let agent = &mut Agent::new("miiswordsman_chakram");
    acmd::install(agent);
    status::install(agent);
    agent.install();
}