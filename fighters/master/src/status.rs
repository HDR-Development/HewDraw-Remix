use super::*;
use globals::*;
// status script import

mod rebirth;

pub fn install(agent: &mut Agent) {
    rebirth::install(agent);
}
