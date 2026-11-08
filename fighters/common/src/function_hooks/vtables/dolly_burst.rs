use super::*;
use utils::ext::*;

extern "C" {
    #[link_name = "burst_check_status_inner"]
    fn burst_check_status_inner(ctx: &mut skyline::hooks::InlineCtx);

    #[link_name = "burst_set_motion_inner"]
    fn burst_set_motion_inner(ctx: &mut skyline::hooks::InlineCtx);
}

#[skyline::hook(offset = 0x97569c, inline)]
unsafe extern "C" fn burst_check_status(ctx: &mut skyline::hooks::InlineCtx) {
    burst_check_status_inner(ctx);
}

#[skyline::hook(offset = 0x975b70, inline)]
unsafe extern "C" fn burst_set_motion(ctx: &mut skyline::hooks::InlineCtx) {
    burst_set_motion_inner(ctx);
}

pub fn install() {
    skyline::install_hooks!(
        burst_check_status,
        burst_set_motion
    );
}
