use super::*;

mod acmd;
//mod status;

pub fn install() {
    let agent = &mut Agent::new("krool_ironball");
    acmd::install(agent);
    //status::install(agent);
    agent.install();
}