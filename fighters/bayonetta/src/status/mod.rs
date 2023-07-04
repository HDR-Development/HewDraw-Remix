use super::*;

mod attack;
mod batwithin;
mod escape;


pub fn install() {
    attack::install();
    batwithin::install();
    escape::install();
}