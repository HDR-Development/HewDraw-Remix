use super::*;

mod attack;
mod batwithin;
mod escape;
mod attackair;
mod specialairs;
mod specials;
mod jumpaerial;


pub fn install() {
    attack::install();
    batwithin::install();
    escape::install();
    attackair::install();
    specialairs::install();
    specials::install();
    jumpaerial::install();
}

pub fn add_statuses() {
    specials::install_custom();
}