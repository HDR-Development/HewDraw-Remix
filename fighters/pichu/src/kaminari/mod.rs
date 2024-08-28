use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("pichu_kaminari");
    acmd::install(agent);
    agent.install();
}