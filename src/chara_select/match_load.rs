use skyline::hooks::InlineCtx;
use smash2::app::FighterManager;
use smash::lib::lua_const::*;
use ultelier::sync_guest::{self as sync, BufferMode};
static mut SYNC_GUEST_MISSING: bool = false;
static mut LAST_BUFFER_MODE: Option<BufferMode> = None;

unsafe fn update_ssbusync_buffer_mode() {
    if SYNC_GUEST_MISSING {
        return;
    }

    let match_mode = utils::util::get_match_mode().0;
    let player_count = if let Some(fighter_manager) = FighterManager::instance() {
        fighter_manager.entry_count()
    } else {
        0
    };

    // only use double buffer mode for online modes with 2 or fewer players
    // prioritize stability for offline matches
    let online_modes = [
        45, // online arena smash
        58  // local wireless smash
    ];
    let desired_mode = if online_modes.contains(&match_mode) && player_count <= 2 {
        BufferMode::Double
    } else {
        BufferMode::Triple
    };

    let current_mode = match sync::env_flags() {
        Some(flags) => {
            if flags.contains(sync::EnvironmentFlags::TRIPLE_ENABLED) {
                BufferMode::Triple
            } else {
                BufferMode::Double
            }
        }
        None => {
            SYNC_GUEST_MISSING = true;
            println!("SSBU Sync Guest is not running, skipping buffer mode update.");
            return;
        }
    };

    if current_mode == desired_mode {
        LAST_BUFFER_MODE = Some(desired_mode);
        println!("Buffer mode is already {:?}, no update needed.", desired_mode);
        return;
    }
    println!("Updating buffer mode to {:?} from {:?} for match mode {} with {} active players", desired_mode, current_mode, match_mode, player_count);

    match sync::set_buffer_mode(desired_mode) {
        Some(true) => {
            LAST_BUFFER_MODE = Some(desired_mode);
        }
        Some(false) => {}
        None => {
            SYNC_GUEST_MISSING = true;
        }
    }
}

#[skyline::hook(offset = 0x1b7b814, inline)]
unsafe fn match_load(ctx: &mut InlineCtx) {
    update_ssbusync_buffer_mode();

    if !utils::one_player::one_player_entry() {
        return;
    }

    let result_ptr = ctx.registers[22].x() as *const u64;
    let pane = *result_ptr.add(1);
    if pane == 0 {
        return;
    }

    let internal = *(pane as *const u64);
    if internal == 0 {
        return;
    }

    let parent = *((internal as *const u8).add(0x18) as *const u64);
    if parent == 0 {
        return;
    }

    // hide timer
    *(parent as *mut u8).add(0x58) &= 0xFE;
}

pub fn install() {
    skyline::install_hooks!(
        match_load,
    );
}
