use super::*;
use utils::ext::*;


#[skyline::hook(offset = 0x3d4180)]
unsafe fn article_module__generate_article(module: u64, article_kind: i32, arg3: bool, arg4: i32) {
    let boma = *(module as *mut *mut BattleObjectModuleAccessor).add(1);
    let status_module = *(boma as *const u64).add(0x8);
    let is_respawn_entry = StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_REBIRTH
        && ((*boma).is_motion_one_of(&[Hash40::new("entry_l"), Hash40::new("entry_r")])
            || (*boma).status_frame() <= 1);

    // Set your status kind to ENTRY
    // This allows entry anim articles to spawn properly during respawn
    if is_respawn_entry {
        *((status_module + 0x98) as *mut i32) = *FIGHTER_STATUS_KIND_ENTRY;  // StatusModule::status_kind
    }
    
    call_original!(module, article_kind, arg3, arg4);

    // Set your status kind back to REBIRTH after article generation
    // to proceed with proper respawn behavior
    if is_respawn_entry {    
        *((status_module + 0x98) as *mut i32) = *FIGHTER_STATUS_KIND_REBIRTH;  // StatusModule::status_kind
    }
}

pub fn install() {
    skyline::install_hooks!(
        article_module__generate_article,
    );
}