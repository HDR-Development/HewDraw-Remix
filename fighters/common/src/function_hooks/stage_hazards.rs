#![feature(proc_macro_hygiene)]

use skyline::{hook, install_hook};

extern "C" {
    #[link_name = "_ZN3app5stage12get_stage_idEv"]
    fn get_stage_id() -> u32;
}

#[skyline::hook(offset = 0x5209a0)]
unsafe fn area_manager_process(manager: *const u64) {
    let mut start = *manager.add(1);
    let end = *manager.add(2);
    while start != end {
        let current = *(start as *const u64);
        if *(current as *mut u8).add(0x20) == 0x1b && get_stage_id() == 0x8f {
            *(current as *mut bool).add(0x21) = false;
            *((current + 0x40) as *mut f32) = 0.0;
            *((current + 0x40) as *mut f32).add(1) = 0.0;
            *((current + 0x40) as *mut f32).add(2) = 0.0;
            *((current + 0x40) as *mut f32).add(3) = 0.0;
            *((current + 0x40) as *mut f32).add(4) = 0.0;
            *((current + 0x40) as *mut f32).add(5) = 0.0;
            *((current + 0x40) as *mut f32).add(6) = 0.0;
            *((current + 0x40) as *mut f32).add(7) = 0.0;
        }
        start = start + 0x8;
    }
    call_original!(manager)
}

#[skyline::hook(offset = 0x30f6160)]
unsafe fn stub() {}

static HAZARDLESS_STAGE_IDS: &[u32] = &[
    0x3b, // venom
    0x3e, // brinstar
    0x62, // skyworld
    0x68, // wario ware,
    0x6e, // halberd
    0x77, // summit
    0xcb, // find mii (StreetPass)
    0xb9, // reset bomb forest
    0xec, // skyloft,
    0x107, // wrecking crew
    0x10d, // wuhu island
    0x119, // duck hunt
];

#[skyline::hook(offset = 0x178a090, inline)]
unsafe fn init_stage(ctx: &mut skyline::hooks::InlineCtx) {
    if HAZARDLESS_STAGE_IDS.contains(&*ctx.registers[1].w.as_ref()) {
        *ctx.registers[3].w.as_mut() = 0;
    }
}

#[skyline::hook(offset = 0x3a9160, inline)]
unsafe fn handle_movement_grav_update(ctx: &mut skyline::hooks::InlineCtx) {
    let battle_object_world = *(((skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64) + 0x52b6558) as *const u64);
    *(battle_object_world as *mut u8).add(0x59) = 0x1;
}

#[skyline::hook(offset = 0x25fb9a4, inline)]
unsafe fn fix_hazards_for_online(ctx: &skyline::hooks::InlineCtx) {
  let ptr = *ctx.registers[1].x.as_ref();
  let stage_id = *(ptr as *const u16) as u32;
  if HAZARDLESS_STAGE_IDS.contains(&stage_id) {
    *(ptr as *mut bool).add(0x10) = false;
  }
}

#[skyline::hook(offset = 0x298123C, inline)]
unsafe fn lylat_no_rot(ctx: &mut skyline::hooks::InlineCtx) {
    if *ctx.registers[8].x.as_ref() == 3 {
        *ctx.registers[8].x.as_mut() = 5;
    }
}

pub fn install() {
    skyline::patching::Patch::in_text(0x298236c).data(0x52800008u32);
    skyline::patching::Patch::in_text(0x28444cc).data(0x52800009u32);
    skyline::patching::Patch::in_text(0x28440f4).data(0x52800009u32);
    skyline::patching::Patch::in_text(0x2844500).nop();
    skyline::patching::Patch::in_text(0x2844128).nop();

    skyline::install_hooks!(
        stub,
        area_manager_process,
        init_stage,
        handle_movement_grav_update,
        fix_hazards_for_online,
        lylat_no_rot,
    );
}