use super::*;

mod special_n1;
mod special_n2;
mod special_n3;

pub fn install(agent: &mut Agent) {;
    special_n1::install(agent);
    special_n2::install(agent);
    special_n3::install(agent);
}