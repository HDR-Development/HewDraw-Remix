use super::*;
use globals::*;

mod attack;
mod batwithin;
mod escape;
mod attackair;
mod specialairs;
mod specialn;
mod specials;
mod jumpaerial;

pub fn install(agent: &mut Agent) {
    attack::install(agent);
    batwithin::install(agent);
    escape::install(agent);
    attackair::install(agent);
    specialairs::install(agent);
    specialn::install(agent);
    specials::install(agent);
    jumpaerial::install(agent);
}