use super::*;

mod acmd;
mod opff;
//mod status;

pub fn install() {
    let agent = &mut Agent::new("master_axe");
    acmd::install(agent);
    opff::install(agent);
    //status::install(agent);
    agent.install();
}