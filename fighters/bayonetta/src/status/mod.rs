use super::*;

mod attack;
mod batwithin;
mod escape;
mod attackair;
mod specials;
mod jumpaerial;


pub fn install() {
    attack::install();
    batwithin::install();
    escape::install();
    attackair::install();
    specials::install();
    jumpaerial::install();
}