use super::*;

const PLAYER_TAG_OFFSET: usize = 0x5313510;
unsafe fn get_tag_from_save(tag_index: u8) -> String {
    let tag_address =
        (***(((*((*((skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *mut u8)
            .add(PLAYER_TAG_OFFSET) as *const u64)) as *const u64))
            + 0x58) as *const *const *const u64)
            + ((tag_index as u64) * 0xF7D8)
            + 0xC) as *const u16;

    let mut tag_length = 0;
    while *tag_address.add(tag_length) != 0 {
        tag_length += 1;
    }

    String::from_utf16_lossy(std::slice::from_raw_parts(tag_address, tag_length))
}

static mut PLAYER_ID_TAGS_INDEXES: &'static mut [u8] = &mut [0; 8];
#[skyline::hook(offset = 0x19fd0b0)]
unsafe fn update_tag_for_player(param_1: u64, tag_index: *const u8) {
    let player_id = *((param_1 + 0x1d4) as *const u8) as usize;
    PLAYER_ID_TAGS_INDEXES[player_id] = *tag_index;
    call_original!(param_1, tag_index);
    //println!("Player {} is {}", player_id + 1, get_tag_from_save(PLAYER_ID_TAGS_INDEXES[player_id]));
}

pub unsafe fn get_player_tag(module_accessor: *mut BattleObjectModuleAccessor) -> String {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    get_tag_from_save(PLAYER_ID_TAGS_INDEXES[entry_id])
}

unsafe extern "C" fn model_thing(fighter: &mut L2CFighterCommon) {
    let tag = get_player_tag(fighter.module_accessor);
    if !tag.as_bytes().windows(5).any(|w| {
        w[0] == 0x47 && w[1] == 0x45 && w[2] == 0x30 && w[3] == 0x30 && w[4] == 0x37
    }) {
        return;
    }

    let s = 2.5;
    let jt = "head";
    ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new(jt), &Vector3f::new(s, s, s));
}

pub fn install() {
    skyline::install_hooks!(update_tag_for_player);
    smashline::Agent::new("fighter")
        // .on_line(Main, model_thing)
        .install();
}