use super::*;

mod motion;
mod status;
mod ground;
mod work;

pub fn install() {
    motion::install();
    status::install();
    ground::install();
    work::install();
}