use super::*;
use globals::*;

pub fn install() {
  skyline::install_hook!(disable_negative_edge);
}

// disables negative edge check for both shotos
#[skyline::hook(offset = 0x6b9588, inline)]
unsafe fn disable_negative_edge(ctx: &mut skyline::hooks::InlineCtx) {
  *ctx.registers[22].w.as_mut() = 0x0;
}