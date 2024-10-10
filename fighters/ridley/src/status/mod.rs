use super::*;
use globals::*;
// status script import

mod special_hi_hover;
mod special_hi_charge;
mod special_n;
mod special_s;
mod special_lw;

pub fn install(agent: &mut Agent) {
    special_hi_hover::install(agent);
    special_hi_charge::install(agent);
    special_n::install(agent);
    special_s::install(agent);
    special_lw::install(agent);
}
