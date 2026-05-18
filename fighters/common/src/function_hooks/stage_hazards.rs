#![feature(proc_macro_hygiene)]

use chrono::Datelike;
use skyline::{hook, install_hook};

extern "C" {
    #[link_name = "_ZN3app5stage12get_stage_idEv"]
    fn get_stage_id() -> u32;
    fn get_current_stage_alt() -> usize;
}

#[skyline::hook(offset = 0x30F6E00)]
unsafe fn stub(arg: u64) {
    if get_stage_id() == 0x8f && get_current_stage_alt() == 0 {
        return;
    } else {
        call_original!(arg);
    }
}

#[skyline::hook(offset = 0x5209c0)]
unsafe fn area_manager_process(manager: *const u64) {
    let mut start = *manager.add(1);
    let end = *manager.add(2);
    while start != end {
        let current = *(start as *const u64);
        if *(current as *mut u8).add(0x20) == 0x1b
            && (get_stage_id() == 0x8f && get_current_stage_alt() == 0)
        {
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


#[skyline::hook(offset = 0x178ab60, inline)]
unsafe fn init_stage(ctx: &mut skyline::hooks::InlineCtx) {
    let stage_id = ctx.registers[1].w();
    let is_alt_haz_off = ([0x59].contains(&stage_id) && get_current_stage_alt() == 0)
        || (stage_id == 0x68 && get_current_stage_alt() == 0);
    if is_alt_haz_off {
        ctx.registers[3].set_w(0);
    }
}

#[skyline::hook(offset = 0x3a9180, inline)]
unsafe fn handle_movement_grav_update(ctx: &mut skyline::hooks::InlineCtx) {
    let battle_object_world = *(((skyline::hooks::getRegionAddress(skyline::hooks::Region::Text)
        as u64)
        + 0x52b7558) as *const u64);
    *(battle_object_world as *mut u8).add(0x59) = 0x1;
}

#[skyline::hook(offset = 0x25fc644, inline)]
unsafe fn fix_hazards_for_online(ctx: &skyline::hooks::InlineCtx) {
    let ptr = ctx.registers[1].x();
    let stage_id = *(ptr as *const u16) as u32;
    let is_alt_haz_off = ([0x59].contains(&stage_id) && get_current_stage_alt() == 0)
        || (stage_id == 0x68 && get_current_stage_alt() == 0);
    if is_alt_haz_off {
        *(ptr as *mut bool).add(0x10) = false;
    }
}

#[skyline::hook(offset = 0x2981EDC, inline)]
unsafe fn lylat_no_rot(ctx: &mut skyline::hooks::InlineCtx) {
    if ctx.registers[8].x() == 3 {
        ctx.registers[8].set_x(5);
    }
}

// 0x0 - asteroids
// 0x1 - space battle (big ships)
// 0x2 - corneria
// 0x3 - space battle (small ships)
// 0x4 - default haz off space
#[skyline::hook(offset = 0x297D6AC, inline)]
unsafe fn lylat_set_form_hazards_off(ctx: &mut skyline::hooks::InlineCtx) {
    if get_current_stage_alt() == 0 {
        ctx.registers[8].set_x(0x2);
    } else {
        ctx.registers[8].set_x(0x4);
    }
}

#[skyline::hook(offset = 0x3098AFC, inline)]
unsafe fn yoshis_island_seasonal(ctx: &mut skyline::hooks::InlineCtx) {
    let now = chrono::Utc::now();
    let month = now.month();
    let season = match month {
        12 | 1 | 2 => 0x4, // winter
        3 | 4 | 5 => 0x1, // spring
        6 | 7 | 8 => 0x2, // summer
        9 | 10 | 11 => 0x3, // autumn
        _ => panic!("Yoshis Island - chrono::Utc::now().month() returned an invalid month value: {}", month),
    };
    ctx.registers[9].set_w(season);
}

// 0x1 - spring
// 0x2 - summer
// 0x3 - autumn
// 0x4 - winter
#[skyline::hook(offset = 0x3097AE8, inline)]
unsafe fn yoshis_island_seasonal_omega(ctx: &mut skyline::hooks::InlineCtx) {
    let now = chrono::Utc::now();
    let month = now.month();
    let season = match month {
        12 | 1 | 2 => 0x4, // winter
        3 | 4 | 5 => 0x1, // spring
        6 | 7 | 8 => 0x2, // summer
        9 | 10 | 11 => 0x3, // autumn
        _ => panic!("Yoshis Island - chrono::Utc::now().month() returned an invalid month value: {}", month),
    };
    ctx.registers[9].set_w(season);
}

pub fn install() {
    // NOTE: The 0xc80 is from the 13.0.1 -> 13.0.2 port
    // NOTE: The 0x20  is from the 13.0.2 -> 13.0.3 port
    skyline::patching::Patch::in_text(0x298236c + 0xc80 + 0x20).data(0x52800008u32);
    skyline::patching::Patch::in_text(0x28444cc + 0xc80 + 0x20).data(0x52800009u32);
    skyline::patching::Patch::in_text(0x28440f4 + 0xc80 + 0x20).data(0x52800009u32);
    skyline::patching::Patch::in_text(0x2844500 + 0xc80 + 0x20).nop();
    skyline::patching::Patch::in_text(0x2844128 + 0xc80 + 0x20).nop();
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
        // lylat_set_form_hazards_off,
        yoshis_island_seasonal,
        yoshis_island_seasonal_omega,
    );
}
