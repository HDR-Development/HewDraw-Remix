use super::*;
mod attack;
mod attackx4;
mod attackdash;

mod attackair;

pub fn install() {
    attack::install();
    attackx4::install();
    attackdash::install();

    attackair::install();
}