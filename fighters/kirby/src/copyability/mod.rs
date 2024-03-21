use super::*;

mod acmd;
mod opff;

pub fn install(agent: &mut Agent) {
    acmd::install(agent);
    opff::install();
}