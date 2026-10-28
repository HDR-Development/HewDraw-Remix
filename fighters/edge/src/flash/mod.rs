use super::*;

mod acmd;
mod status;
mod vtable;

pub fn install() {
    let agent = &mut Agent::new("edge_flash");
    acmd::install(agent);
    status::install(agent);
    agent.install();

    vtable::install();
}