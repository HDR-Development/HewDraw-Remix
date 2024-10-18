use super::*;
mod attack;
mod attackx4;
mod attackdash;
mod attacks3;

mod attackair;

pub fn install() {
    attack::install();
    attackx4::install();
    attackdash::install();
    attacks3::install();

    attackair::install();
}