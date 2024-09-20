use super::*;
use globals::*;
// status script import

mod item_throw_heavy;
mod special_hi;
mod special_lw;
mod special_n;
mod catch_pull;
mod shoulder;
mod super_lift;
mod fall_special;

pub fn install(agent: &mut Agent) {
    item_throw_heavy::install(agent);
    special_hi::install(agent);
    special_lw::install(agent);
    special_n::install(agent);
    catch_pull::install(agent);
    shoulder::install(agent);
    super_lift::install(agent);
    fall_special::install(agent);
}
