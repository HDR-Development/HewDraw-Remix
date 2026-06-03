use super::*;

mod special_s1;
mod special_s2;
mod special_s3;

pub fn install(agent: &mut Agent) {;
    special_s1::install(agent);
    special_s2::install(agent);
    special_s3::install(agent);
}