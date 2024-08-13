use super::*;
use globals::*;
// status script import
 
mod special_hi;

pub fn install(agent: &mut Agent) {
    special_hi::install(agent);
}
