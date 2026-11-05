use super::*;
use utils::ext::*;

extern "C" {
    #[link_name = "ptrainer_swap_backwards_hook_inner"]
    fn ptrainer_swap_backwards_hook_inner(ctx: &mut skyline::hooks::InlineCtx);
}

#[skyline::hook(offset = 0x34cec44, inline)]
unsafe fn ptrainer_swap_backwards_hook(ctx: &mut skyline::hooks::InlineCtx) {
    ptrainer_swap_backwards_hook_inner(ctx);
}

// #[skyline::hook(offset = 0xf96330)]
// unsafe fn ptrainer_stub_death_switch() {}

pub fn install() {
    skyline::install_hooks!(
        ptrainer_swap_backwards_hook,
        //ptrainer_stub_death_switch,
    );
}
