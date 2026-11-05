use super::*;

extern "C" {
    #[link_name = "pickel_stuff_hook_inner"]
    fn pickel_stuff_hook_inner(ctx: &mut skyline::hooks::InlineCtx);
}

// lets the "stuff" article generate in new statuses
#[skyline::hook(offset = 0xf13d5c, inline)]
unsafe fn pickel_stuff_hook(ctx: &mut skyline::hooks::InlineCtx) {
    pickel_stuff_hook_inner(ctx);
}

pub fn install() {
    skyline::install_hooks!(
        pickel_stuff_hook
    );
}
