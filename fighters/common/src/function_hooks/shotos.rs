use super::*;
use globals::*;

pub fn install() {
    skyline::install_hooks!(
        disable_negative_edge,
        // enable_terry_inputs_for_shotos
    );
    skyline::patching::Patch::in_text(0x10D45C4).data(0x14000014u32); // enables terry's command inputs for shotos

    // untested patches for kazuya's inputs for terry
    // skyline::patching::Patch::in_text(0x097094c).nop();
    // skyline::patching::Patch::in_text(0x0970954).nop();
    // skyline::patching::Patch::in_text(0x0970958).nop();
    // skyline::patching::Patch::in_text(0x097095c).nop();
    // skyline::patching::Patch::in_text(0x0970960).nop();
    // skyline::patching::Patch::in_text(0x0970964).nop();
    // skyline::patching::Patch::in_text(0x0970968).nop();
    // skyline::patching::Patch::in_text(0x097096c).nop();
    // skyline::patching::Patch::in_text(0x0970970).nop();
    // skyline::patching::Patch::in_text(0x0970974).nop();
    // skyline::patching::Patch::in_text(0x0970978).nop();
    // skyline::patching::Patch::in_text(0x097097c).nop();
    // skyline::patching::Patch::in_text(0x0970980).nop();
    // skyline::patching::Patch::in_text(0x0970984).nop();
    // skyline::patching::Patch::in_text(0x0970988).nop();
}

// disables negative edge check for both shotos
#[skyline::hook(offset = 0x6b95a8, inline)]
unsafe fn disable_negative_edge(ctx: &mut skyline::hooks::InlineCtx) {
    *ctx.registers[22].w.as_mut() = 0x0;
}

// #[skyline::hook(offset = 0x10d4550)]
// unsafe fn enable_terry_inputs_for_shotos() {}