use super::*;

mod specialhi;
mod speciallw;
mod specialn;
mod specials;

pub fn install(agent: &mut Agent) {
    specialhi::install(agent);
    speciallw::install(agent);
    specialn::install(agent);
    specials::install(agent);
}