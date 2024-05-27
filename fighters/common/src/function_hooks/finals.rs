use super::*;

pub extern "C" fn stub() -> u64 { 0 }

#[skyline::hook(offset = 0x62ea88, inline)]
unsafe fn final_breverse_0(ctx: &mut skyline::hooks::InlineCtx) {
    let boma = *(*ctx.registers[0].x.as_ref() as *mut *mut BattleObjectModuleAccessor).add(1);
    if ![
        *FIGHTER_KIND_RYU,
        *FIGHTER_KIND_KEN
    ].contains(&(*boma).kind()) {
        return;
    } else {
        *ctx.registers[8].x.as_mut() = stub as *const () as u64;
    }
}

#[skyline::hook(offset = 0x68f44c, inline)]
unsafe fn final_breverse_1(ctx: &mut skyline::hooks::InlineCtx) {
    let boma = *(*ctx.registers[0].x.as_ref() as *mut *mut BattleObjectModuleAccessor).add(1);
    if ![
        *FIGHTER_KIND_RYU,
        *FIGHTER_KIND_KEN
    ].contains(&(*boma).kind()) {
        return;
    } else {
        *ctx.registers[8].x.as_mut() = stub as *const () as u64;
    }
}

pub fn install() {
    skyline::install_hooks!(final_breverse_0, final_breverse_1);
}
