use super::*;
use globals::*;

pub fn install() {
  skyline::install_hooks!(
    disable_negative_edge,
    // enable_terry_inputs_for_shotos
  );
  skyline::patching::Patch::in_text(0x10d45a4).data(0x14000014u32); // enables terry's command inputs for shotos
}

// disables negative edge check for both shotos
#[skyline::hook(offset = 0x6b95a8, inline)]
unsafe fn disable_negative_edge(ctx: &mut skyline::hooks::InlineCtx) {
  *ctx.registers[22].w.as_mut() = 0x0;
}

// #[skyline::hook(offset = 0x10d4550)]
// unsafe fn enable_terry_inputs_for_shotos() {}