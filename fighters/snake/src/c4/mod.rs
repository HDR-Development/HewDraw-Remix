use super::*;

mod acmd;
mod opff;

pub fn install() {
    let agent = &mut Agent::new("snake_c4");
    acmd::install(agent);
    opff::install(agent);
    agent.install();
}