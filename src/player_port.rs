use ninput::*;
use dynamic::{consts::*, *};

pub const CONTROLLER_ID: [u32; 9] = [0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x20]; // 0x20 is the id for handheld

static mut ENABLE_SWAPPING: bool = false;

static mut ACTIVE_CONTROLLER: Option<u32> = None;
static mut PORT_ACTIVE: [bool; 8] = [true, false, false, false, false, false, false, false];
static mut NEXT_PORT: [i32; 9] = [1; 9];

// stores the raw argument data for the 8 player panes, to pass around different functions
static mut PORT_DATA: [Option<u64>; 8] = [None; 8];

static mut X_PRESSED: bool = false; // makes sure we only run related code once per button press
static mut ACTION: &str = "right";


// returns true/false depending on if the specified controller is performing the defined button macro
unsafe fn check_swap_macro(controller_id: u32) -> bool {
    if let Some(controller) = Controller::get_from_id(controller_id) {
        if controller.buttons.contains(Buttons::R) 
        && controller.pressed_buttons.contains(Buttons::X) {
            ACTION = "right"; // port will swap forwards
            return true;
        }
        else if controller.buttons.contains(Buttons::L) 
        && controller.pressed_buttons.contains(Buttons::X) {
            ACTION = "left"; // port will swap backwards
            return true;
        }
        else if controller.pressed_buttons.contains(Buttons::X) {
            ACTION = "out"; // port will disconnect without rejoining
            return true;
        }
    }

    return false;
}

// this hook runs when initializing player cards. used to prevent being able to swap in certain modes
#[skyline::hook(offset = 0x1a2665c, inline)]
unsafe fn set_enable_swap(ctx: &mut skyline::hooks::InlineCtx) {
    // by observing the first and last ids of the slots, we can determine how many are present
    let first_card = *ctx.registers[20].x.as_ref();
    let last_card = *ctx.registers[26].x.as_ref();
    // if the difference is 0x80, 8 slots are active and we can safely enable the feature
    if last_card == first_card + 0x80
    && !ENABLE_SWAPPING {
        ENABLE_SWAPPING = true;
        // println!("enabling port swapping!");
    }
}

// this address runs whenever the css is initialized. we can use this to clear neccesary data
#[skyline::hook(offset = 0x1a26ec0, inline)]
unsafe fn init_css(ctx: &mut skyline::hooks::InlineCtx) {
    // println!("refreshing port data");
    PORT_DATA = [None; 8];
    ACTIVE_CONTROLLER = None;
}

// this runs right afterwards, and stores the port data for this instance of the css
// this is required on each visit because the data is different for every css session, and cannot be reused without crashing
#[skyline::hook(offset = 0x1a13328, inline)]
unsafe fn set_port_data(ctx: &mut skyline::hooks::InlineCtx) {
    if PORT_DATA[0] == None { // this function loops on the CSS, so we make sure it only runs storage once per instance
        let base_port_id = *ctx.registers[8].x.as_ref();
        for i in 0..8 {
            let offset = (0x10 * i) as u64;
            PORT_DATA[i] = Some(base_port_id + offset);
            // println!("data for port {}: {:#x}", i + 1, base_port_id + offset);
        }
    }
}

pub const BACK_BUTTON: u64 = 0x1010f00e00;

// resets data when leaving the local battle session
// this function runs whenever a ui pane is  selected, so we look for the data for the "back" button on rule select
#[skyline::hook(offset = 0x2407280)]
unsafe fn reset_css_session(pane: u64, arg2: u64) {
    if pane == BACK_BUTTON {
        // println!("resetting for next css session, disabling swap");
        PORT_ACTIVE = [true, false, false, false, false, false, false, false];
        PORT_DATA = [None; 8];
        NEXT_PORT = [1; 9];
        ENABLE_SWAPPING = false;
    }

    original!()(pane, arg2)
}

#[skyline::from_offset(0x1a1e430)] // clears the controller's input and player hand
unsafe fn controller_input_off(arg1: u64, arg2: u64);

#[skyline::from_offset(0x1a1e860)] // unsure what this clears, but we run it just to be safe
unsafe fn controller_something_off(arg1: u64, port: u64);

#[skyline::from_offset(0x1a1e660)] // clears the player card from the css
unsafe fn controller_card_off(port: u64, arg2: u64);

#[skyline::from_offset(0x1a1f0b0)] // clears the player token
unsafe fn controller_token_off(arg1: u64, port: u64);

// this function loops while the css is active, so we can use it for running any real-time operations we need
// more importantly, this runs BEFORE controller initialization, which allows for instant reconnects
#[skyline::hook(offset = 0x1a2b570)]
unsafe fn css_main_loop(arg: u64) {
    if !ENABLE_SWAPPING {
        return original!()(arg);
    }

    // online arenas pass the usual check, so disable it if we are there
    if game_modes::get_melee_mode() == melee_mode::ARENA {
        ENABLE_SWAPPING = false;
        return original!()(arg);
    }

    if ACTIVE_CONTROLLER == None && PORT_DATA[0] != None && !X_PRESSED {
        // check each controller to see if they are performing the macro, and set it to "active" if so
        for controller in 0..9 {
            let id = CONTROLLER_ID[controller as usize];
            if check_swap_macro(id) {
                // println!("x pressed by controller {:#x}", controller);
                ACTIVE_CONTROLLER = Some(controller);
                let slot = NEXT_PORT[controller as usize];
                if let Some(port) = PORT_DATA[(slot - 1) as usize] {
                    controller_input_off(*(arg as *const u64), *((port + 8) as *const u64));
                    controller_something_off(0, port);
                    controller_card_off(port, 1);
                    controller_token_off(*(arg as *const u64), port);
                }
                // println!("disconnecting controller {} from slot {}", controller, slot);
                PORT_ACTIVE[(slot - 1) as usize] = false;

                if ACTION == "out" {
                    for i in 0..8 {
                        if !PORT_ACTIVE[i] { 
                            NEXT_PORT[controller as usize] = (i + 1) as i32;
                            break;
                         }
                    }
                }

                X_PRESSED = true;
            }
        }
    }

    if !ninput::any::is_press(ninput::Buttons::X) && X_PRESSED {
        X_PRESSED = false;
    }

    original!()(arg)
}

// this function runs right after a controller connects to the css, and assigns the player port to it
#[skyline::hook(offset = 0x1a31020)]
unsafe fn init_css_player(
    arg1: u64, // unknown. seems to always be the same; likely points to some kind of struct containing player data
    port: i32, // number of the player port that the game is going to try adding
    arg3: u64, // assigns the actual port that the joined player owns and controls
    arg4: u64 // assigns the port of the character card that gets loaded for the UI. will always be arg3 + 0x80
) {
    // see if we have an active controller for swapping ports. if not, return the original behavior
    let Some(controller) = ACTIVE_CONTROLLER else {
        // println!("port {} active", port);
        PORT_ACTIVE[(port - 1) as usize] = true;
        return original!()(arg1, port, arg3, arg4);
    };
    
    // bypass controller initialization if its a disconnect
    if ACTION == "out" {
        ACTIVE_CONTROLLER = None;
        ACTION = "right"; 
        return;
    }

    // println!("controller {} port {} data: {:#x} / {:#x}", id, port, arg3, arg4);

    // find the next available port to be loaded into
    let id = controller as usize;
    for i in 0..8 {
        if ACTION == "right" {
            NEXT_PORT[id] += 1;
            if NEXT_PORT[id] >= 9 { NEXT_PORT[id] = 1 };
        }
        else if ACTION == "left" {
            NEXT_PORT[id] -= 1;
            if NEXT_PORT[id] <= 0 { NEXT_PORT[id] = 8 };
        }

        if !PORT_ACTIVE[(NEXT_PORT[id] - 1) as usize] {
            break;
        }
        // println!("target port {} is active! skipping...", NEXT_PORT[id]);
    }

    // calculate the data offset needed in order to change our original port to the new one
    // we will then modify the original arguments by the needed amount to reach the target port
    // note: the game will always try to load the controller into the first available player slot (not occupied by another player or amiibo)
    let port_diff = NEXT_PORT[id] - port;
    let offset = (0x10 * port_diff.abs()) as u64;
    let new_port = if port_diff >= 0 { port + port_diff } else { port - port_diff };
    let new3 = if port_diff >= 0 { arg3 + offset } else { arg3 - offset };
    let new4 = if port_diff >= 0 { arg4 + offset } else { arg4 - offset };

    // println!("changing controller {}'s port from {} to {}, diff: {}", id, port, new_port, port_diff);
    PORT_ACTIVE[(new_port - 1) as usize] = true;
    ACTIVE_CONTROLLER = None; // clear the stored controller id now that the operation has performed

    original!()(arg1, new_port, new3, new4);
}

pub fn install() {
    skyline::install_hooks!(
        set_enable_swap,
        init_css,
        set_port_data,
        reset_css_session,
        css_main_loop,
        init_css_player
    );
}
