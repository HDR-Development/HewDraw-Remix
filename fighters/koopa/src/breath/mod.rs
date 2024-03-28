use super::*;

mod acmd;
mod status;

pub fn install() {
    let agent = &mut Agent::new("koopa_breath");
    acmd::install(agent);
    status::instal(agent);
    agent.install();
}