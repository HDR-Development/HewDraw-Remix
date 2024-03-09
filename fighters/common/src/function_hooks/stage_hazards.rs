#![feature(proc_macro_hygiene)]

use skyline::{hook, install_hook};

extern "C" {
    #[link_name = "_ZN3app5stage12get_stage_idEv"]
    fn get_stage_id() -> u32;
    fn get_current_stage_alt() -> usize;
}

#[skyline::hook(offset = 0x30F6DE0)]
unsafe fn stub(arg: u64) {
    // if get_stage_id() == 0x8f && get_current_stage_alt() == 0 {
        return;
    // } else {
    //     call_original!(arg);
    // }
}

#[skyline::hook(offset = 0x5209c0)]
unsafe fn area_manager_process(manager: *const u64) {
    let mut start = *manager.add(1);
    let end = *manager.add(2);
    while start != end {
        let current = *(start as *const u64);
        // if *(current as *mut u8).add(0x20) == 0x1b
        //     && (get_stage_id() == 0x8f && get_current_stage_alt() == 0)
        // {
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

static HAZARDLESS_STAGE_IDS: &[u32] = &[
    0x3b,  // venom
    0x3e,  // brinstar
    0x62,  // skyworld
    0x6e,  // halberd
    0x77,  // summit
    0xcb,  // find mii (StreetPass)
    0xb9,  // reset bomb forest
    0xec,  // skyloft,
    0x107, // wrecking crew
    0x10d, // wuhu island
];

#[skyline::hook(offset = 0x178ab60, inline)]
unsafe fn init_stage(ctx: &mut skyline::hooks::InlineCtx) {
    let stage_id = *ctx.registers[1].w.as_ref();
    // let is_alt_haz_off = ([0x59].contains(&stage_id) && get_current_stage_alt() == 0)
    //     || (stage_id == 0x68 && get_current_stage_alt() == 0);
    // if HAZARDLESS_STAGE_IDS.contains(&stage_id) || is_alt_haz_off {
    if HAZARDLESS_STAGE_IDS.contains(&stage_id) || stage_id == 0x68 || stage_id == 0x59 {
        *ctx.registers[3].w.as_mut() = 0;
    }
}

#[skyline::hook(offset = 0x3a9180, inline)]
unsafe fn handle_movement_grav_update(ctx: &mut skyline::hooks::InlineCtx) {
    let battle_object_world = *(((skyline::hooks::getRegionAddress(skyline::hooks::Region::Text)
        as u64)
        + 0x52b8558) as *const u64);
    *(battle_object_world as *mut u8).add(0x59) = 0x1;
}

#[skyline::hook(offset = 0x25fc624, inline)]
unsafe fn fix_hazards_for_online(ctx: &skyline::hooks::InlineCtx) {
    let ptr = *ctx.registers[1].x.as_ref();
    let stage_id = *(ptr as *const u16) as u32;
    // let is_alt_haz_off = ([0x59].contains(&stage_id) && get_current_stage_alt() == 0)
    //     || (stage_id == 0x68 && get_current_stage_alt() == 0);
    // if HAZARDLESS_STAGE_IDS.contains(&stage_id) || is_alt_haz_off {
    if HAZARDLESS_STAGE_IDS.contains(&stage_id) || stage_id == 0x68 || stage_id == 0x59 {
        *(ptr as *mut bool).add(0x10) = false;
    }
}

#[skyline::hook(offset = 0x2981EBC, inline)]
unsafe fn lylat_no_rot(ctx: &mut skyline::hooks::InlineCtx) {
    if *ctx.registers[8].x.as_ref() == 3 {
        *ctx.registers[8].x.as_mut() = 5;
    }
}

// 0x0 - asteroids
// 0x1 - space battle (big ships)
// 0x2 - corneria
// 0x3 - space battle (small ships)
// 0x4 - default haz off space
// #[skyline::hook(offset = 0x297D68C, inline)]
// unsafe fn lylat_set_form_hazards_off(ctx: &mut skyline::hooks::InlineCtx) {
//     // if get_current_stage_alt() == 0 {
//     //     *ctx.registers[8].x.as_mut() = 0x2;
//     // } else {
//         *ctx.registers[8].x.as_mut() = 0x4;
//     // }
// }

pub fn install() {
    // NOTE: The 0xc80 is from the 13.0.1 -> 13.0.2 port
    skyline::patching::Patch::in_text(0x298236c + 0xc80).data(0x52800008u32);
    skyline::patching::Patch::in_text(0x28444cc + 0xc80).data(0x52800009u32);
    skyline::patching::Patch::in_text(0x28440f4 + 0xc80).data(0x52800009u32);
    skyline::patching::Patch::in_text(0x2844500 + 0xc80).nop();
    skyline::patching::Patch::in_text(0x2844128 + 0xc80).nop();
    skyline::patching::Patch::in_text(0x4471134)
        .data(std::f32::INFINITY)
        .unwrap(); // palu temple
    skyline::patching::Patch::in_text(0x44723dc)
        .data(2880.0f32)
        .unwrap(); // palu temple
    skyline::patching::Patch::in_text(0x447142c)
        .data(-2880.0f32)
        .unwrap(); // palu temple

    skyline::install_hooks!(
        stub,
        init_stage,
        area_manager_process,
        handle_movement_grav_update,
        fix_hazards_for_online,
        lylat_no_rot,
        // lylat_set_form_hazards_off
    );
}
