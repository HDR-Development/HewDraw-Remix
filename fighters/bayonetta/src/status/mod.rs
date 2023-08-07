use super::*;

mod attack;
mod batwithin;
mod escape;
mod attackair;


pub fn install() {
    attack::install();
    batwithin::install();
    escape::install();
    attackair::install();
}