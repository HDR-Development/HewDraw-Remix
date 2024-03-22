mod containers;
mod types;

use skyline::hooks::InlineCtx;
use smash_arc::SearchLookup;
use self::types::FilesystemInfo;

fn get_pane_from_layout(layout_data: u64, name: &str) -> Option<u64> {
    unsafe {
        let pane_udata = crate::online::get_pane_by_name(layout_data, name.as_ptr());
        if pane_udata[1] != 0 {
            Some(pane_udata[1])
        } else {
            None
        }
    }
}

#[skyline::from_offset(0x37782c0)] // 0x3777640
unsafe fn replace_texture(pane: u64, index: &u32);

#[skyline::from_offset(0x353d6d0)] // 0x353ca50
unsafe fn get_filepath_index_by_hash40(index: &mut u32, hash40: u64);

#[skyline::hook(offset = 0x1ee9ebc, inline)] // 0x1ee93dc
unsafe fn among_us_baby(ctx: &InlineCtx) {
    let layout_view = *ctx.registers[0].x.as_ref();

    let pane = get_pane_from_layout(layout_view, "perry\0").unwrap();
    let mut index = 0u32;

    let hash = STAGE_HASH;
    let alt = get_current_stage_alt();
    let file_suffix = if alt != 0 {
        hash40::hash40(&format!("_s{alt:02}.bntx"))
    } else {
        hash40::hash40(".bntx")
    };

    let stage_hash = hash40::Hash40(STAGE_HASH).concat(file_suffix);

    get_filepath_index_by_hash40(&mut index, stage_hash.0);
    replace_texture(pane, &index);
}

static mut STAGE_HASH: u64 = 0x0;

const DLC: &[&'static str] = &[
    "battlefield_s",
    "brave_altar",
    "buddy_spiral",
    "demon_dojo",
    "dolly_stadium",
    "fe_shrine",
    "ff_cave",
    "jack_mementoes",
    "pickel_world",
    "tantan_spring",
    "trail_castle",
    "xeno_alst",
];

#[skyline::hook(offset = 0x25fdf38, inline)] // 0x25fd2b8
unsafe fn incoming_stage_load(ctx: &InlineCtx) {
    let search = FilesystemInfo::instance().unwrap().search();
    let Ok(path) = search.get_path_list_entry_from_hash(*ctx.registers[8].x.as_ref()) else {
        return;
    };

    let Ok(parent_path) = search.get_path_list_entry_from_hash(path.parent.hash40()) else {
        return;
    };

    let file_name = hash40::Hash40(parent_path.file_name.hash40().0);

    let root_path = if DLC
        .iter()
        .any(|dlc_name| hash40::hash40(*dlc_name) == file_name)
    {
        "ui/replace_patch/stage/stage_2/stage_2_"
    } else {
        "ui/replace/stage/stage_2/stage_2_"
    };
    if file_name == hash40::hash40("battlefield_s") {
        STAGE_HASH = hash40::hash40(root_path).concat(hash40::hash40("battlefields")).0;
    } else {
        STAGE_HASH = hash40::hash40(root_path).concat(file_name).0;
    }
}

static mut SHOULD_PLAY: bool = false;

#[skyline::from_offset(0x3777730)] // 0x3776ab0
unsafe fn play_animation(layout: u64, anim: *const u8);

#[skyline::hook(offset = 0x2310b48, inline)] // 0x2310068
unsafe fn play_out_anim(_: &InlineCtx) {
    SHOULD_PLAY = true;
}

#[skyline::hook(offset = 0x1343184, inline)] // 0x1343174
unsafe fn stop_play_anim(_: &InlineCtx) {
    SHOULD_PLAY = false;
}

#[skyline::hook(offset = 0x22d3494, inline)] // 0x22d29b4
unsafe fn should_play_out_anim(ctx: &mut InlineCtx) {
    if !SHOULD_PLAY {
        return;
    }

    *ctx.registers[8].x.as_mut() = 0;
}

extern "C" {
    fn get_current_stage_alt() -> usize;
}

pub fn install() {
    skyline::install_hooks!(
        among_us_baby,
        incoming_stage_load,
        should_play_out_anim,
        play_out_anim,
        stop_play_anim
    );
}
