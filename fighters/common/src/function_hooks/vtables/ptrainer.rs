use super::*;
use utils::ext::*;

#[skyline::hook(offset = 0x34ce8e4, inline)]
unsafe fn ptrainer_swap_backwards_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let object = *ctx.registers[20].x.as_ref() as *mut BattleObject;
    if VarModule::is_flag(object, vars::ptrainer::instance::IS_SWITCH_BACKWARDS) {
        let new = match *ctx.registers[8].x.as_ref() {
            0 => 1,
            1 => 2,
            2 => 0,
            _ => unreachable!()
        };

        *ctx.registers[8].x.as_mut() = new;
    }
}

#[skyline::hook(offset = 0xf96330)]
unsafe fn ptrainer_stub_death_switch() {}

pub fn install() {
    skyline::install_hooks!(
        ptrainer_swap_backwards_hook,
        ptrainer_stub_death_switch
    );
}