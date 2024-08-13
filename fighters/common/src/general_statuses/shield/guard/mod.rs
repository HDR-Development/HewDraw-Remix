use super::*;
mod pre;
mod main;
pub mod end;

mod init;
mod exec;
mod exit;

pub fn install() {
    pre::install();
    main::install();
    end::install();
    init::install();
    exec::install();
    exit::install();
}
