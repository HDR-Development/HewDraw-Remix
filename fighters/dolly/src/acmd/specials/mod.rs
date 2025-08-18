use super::*;

mod special_command;
mod special_hi;
mod special_lw;
mod special_n;
mod special_sb;
mod special_sf;
mod super_special;

pub fn install(agent: &mut Agent) {
    special_command::install(agent);
    special_hi::install(agent);
    special_lw::install(agent);
    special_n::install(agent);
    special_sb::install(agent);
    special_sf::install(agent);
    super_special::install(agent);
}