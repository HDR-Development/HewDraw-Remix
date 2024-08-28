use super::*;
use globals::*;

mod attack;
mod batwithin;
mod escape;
mod attackair;
mod specialairs;
mod specialn;
mod specials;
mod specialhi;
mod jumpaerial;
mod wait;

pub fn install(agent: &mut Agent) {
    attack::install(agent);
    batwithin::install(agent);
    escape::install(agent);
    attackair::install(agent);
    specialairs::install(agent);
    specialn::install(agent);
    specials::install(agent);
    specialhi::install(agent);
    jumpaerial::install(agent);
    wait::install(agent);
}