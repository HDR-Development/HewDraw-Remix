use super::*;

pub extern "C" fn stub() -> u64 { 0 }
/// we're misunderstanding the scope of these functions lol. 
/// leaving out the check for final status functions prevented b-reverrse of shotos NSpecial and DSpecial.

#[skyline::hook(offset = 0x62ea88, inline)]
unsafe fn final_breverse_0(ctx: &mut skyline::hooks::InlineCtx) {
    let boma = *(*ctx.registers[0].x.as_ref() as *mut *mut BattleObjectModuleAccessor).add(1);
    if ![
        *FIGHTER_KIND_RYU,
        *FIGHTER_KIND_KEN
    ].contains(&(*boma).kind()) {
        return;
    }
    if !(*boma).is_status_one_of(&[
        *FIGHTER_STATUS_KIND_FINAL,
        *FIGHTER_RYU_STATUS_KIND_FINAL2,
        *FIGHTER_RYU_STATUS_KIND_FINAL2_AIR_END,
        *FIGHTER_RYU_STATUS_KIND_FINAL2_FALL,
        *FIGHTER_RYU_STATUS_KIND_FINAL2_LANDING,
        *FIGHTER_RYU_STATUS_KIND_FINAL_AIR_END,
        *FIGHTER_RYU_STATUS_KIND_FINAL_FALL,
        *FIGHTER_RYU_STATUS_KIND_FINAL_HIT,
        *FIGHTER_RYU_STATUS_KIND_FINAL_JUMP,
        *FIGHTER_RYU_STATUS_KIND_FINAL_LANDING,
    ]) {
        return;
    }
    *ctx.registers[8].x.as_mut() = stub as *const () as u64;
}

#[skyline::hook(offset = 0x68f44c, inline)]
unsafe fn final_breverse_1(ctx: &mut skyline::hooks::InlineCtx) {
    let boma = *(*ctx.registers[0].x.as_ref() as *mut *mut BattleObjectModuleAccessor).add(1);
    if ![
        *FIGHTER_KIND_RYU,
        *FIGHTER_KIND_KEN
    ].contains(&(*boma).kind()) {
        return;
    }
    if !(*boma).is_status_one_of(&[
        *FIGHTER_STATUS_KIND_FINAL,
        *FIGHTER_RYU_STATUS_KIND_FINAL2,
        *FIGHTER_RYU_STATUS_KIND_FINAL2_AIR_END,
        *FIGHTER_RYU_STATUS_KIND_FINAL2_FALL,
        *FIGHTER_RYU_STATUS_KIND_FINAL2_LANDING,
        *FIGHTER_RYU_STATUS_KIND_FINAL_AIR_END,
        *FIGHTER_RYU_STATUS_KIND_FINAL_FALL,
        *FIGHTER_RYU_STATUS_KIND_FINAL_HIT,
        *FIGHTER_RYU_STATUS_KIND_FINAL_JUMP,
        *FIGHTER_RYU_STATUS_KIND_FINAL_LANDING,
    ]) {
        return;
    }
    *ctx.registers[8].x.as_mut() = stub as *const () as u64;
}

pub fn install() {
    skyline::install_hooks!(final_breverse_0, final_breverse_1);
}
