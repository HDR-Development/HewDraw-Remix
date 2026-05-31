use super::*;

mod ground;
mod kinetic;
mod motion;
mod status;
mod work;

pub fn install() {
    ground::install();
    kinetic::install();
    motion::install();
    status::install();
    work::install();
}