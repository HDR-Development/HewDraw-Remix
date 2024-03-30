use super::*;
use globals::*;
// status script import

mod run;
mod special_s;
mod special_lw;

pub fn install(agent: &mut Agent) {
    run::install(agent);
    special_s::install(agent);
    special_lw::install(agent);
}
