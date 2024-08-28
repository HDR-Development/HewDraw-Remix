use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("pikachu_kaminari");
    acmd::install(agent);
    agent.install();
}