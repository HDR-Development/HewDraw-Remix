use super::*;
pub mod reverse_hits;
pub mod edge_slipoffs;
pub mod ledges;
pub mod get_param;
pub mod change_motion;
pub mod transition;
pub mod djcancel;
pub mod init_settings;
pub mod momentum_transfer;
pub mod dash_dancing;
pub mod directional_influence;
pub mod hitstun;
pub mod change_status;
pub mod is_flag;
pub mod controls;

pub fn install() {
    reverse_hits::install();
    edge_slipoffs::install();
    ledges::install();
    get_param::install();
    change_motion::install();
    transition::install();
    djcancel::install();
    init_settings::install();
    hitstun::install();
    change_status::install();
    is_flag::install();
    controls::install();
    momentum_transfer::install();
    //dash_dancing::install();

    unsafe {
        // Handles getting rid of the kill zoom
        const NOP: u32 = 0xD503201Fu32;
        skyline::patching::patch_data(utils::offsets::kill_zoom_regular(), &NOP);
        skyline::patching::patch_data(utils::offsets::kill_zoom_throw(), &NOP);
        // Changes full hops to calculate vertical velocity identically to short hops
        skyline::patching::patch_data(0x6d2188, &0x52800015u32);
    }
}
