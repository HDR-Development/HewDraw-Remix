use super::*;
use ninput::*;
use parking_lot::RwLock;
// use crate::vsync::SsbuSync;

static ID_LIST: &[u32] = &[0, 1, 2, 3, 4, 5, 6, 7, 0x20];

static mut PORT_DATA: LazyLock<RwLock<PortData>> = LazyLock::new(|| 
    RwLock::new(PortData::default())
);

struct PortData {
    enable_swap: bool,
    root_card: u64,
    active_controllers: Vec<PortController>,
    swap_target: Option<PortController>
}
impl Default for PortData {
    fn default() -> Self {
        PortData {
            enable_swap: false,
            root_card: 0x0,
            active_controllers: Vec::new(),
            swap_target: None
        }
    }
}

trait PortExt {
    fn active_ports(&self) -> Vec<u8>;
    fn is_active_id(&self, id: &u32) -> bool;
    fn is_active_port(&self, port: u8) -> bool;
    fn controller_from_port(&mut self, port: u8) -> Option<&mut PortController>;
    fn controller_from_id(&mut self, id: u32) -> Option<&mut PortController>;
    fn remove_controller(&mut self, id: u32);
    fn reset(&mut self);
}
impl PortExt for PortData {
    fn active_ports(&self) -> Vec<u8> {
        let mut ports = Vec::new();
        for controller in &self.active_controllers {
            ports.push(controller.port.unwrap_or(0));
        }
        ports.sort();
        
        ports
    }
    fn is_active_id(&self, id: &u32) -> bool {
        self.active_controllers.iter().any(|x| x.id == *id)
    }
    fn is_active_port(&self, port: u8) -> bool {
        self.active_ports().contains(&port)
    }
    fn controller_from_port(&mut self, port: u8) -> Option<&mut PortController> {
        self.active_controllers.iter_mut().find(|x| x.port == Some(port))
    }
    fn controller_from_id(&mut self, id: u32) -> Option<&mut PortController> {
        self.active_controllers.iter_mut().find(|x| x.id == id)
    }
    fn remove_controller(&mut self, id: u32) {
        self.active_controllers.retain(|x| x.id != id);
        println!("Controller ID {} removed", id);
    }
    fn reset(&mut self) {
        *self = PortData::default();
    }
}

#[derive(Debug, Copy, Clone)]
struct PortController {
    id: u32,
    port: Option<u8>,
    player_info: Option<*const PlayerInfo>,
    operation: Option<SwapAction>,
    swap_lock: bool
}

#[derive(Debug, Copy, Clone)]
enum SwapAction { Right, Left, Out }

#[derive(Debug, PartialEq)]
enum PlayerKind { Player, CPU, Amiibo, None }

trait ControllerExt {
    fn get_controller(&self) -> Controller;
    fn is_supported_controller(&self) -> bool;
    fn left_button(&self) -> Buttons;
    fn right_button(&self) -> Buttons;
    fn swap_button(&self) -> Buttons;

    fn check_swap_macro(&self) -> Option<SwapAction>;
    fn player_kind(&self) -> PlayerKind;
}
impl ControllerExt for PortController {
    fn get_controller(&self) -> Controller {
        Controller::get_from_id(self.id).unwrap_or(Controller::new(7))
    }
    fn is_supported_controller(&self) -> bool {
        self.get_controller().is_supported_controller()
    }
    fn left_button(&self) -> Buttons {
        match self.get_controller().controller_style {
            ControllerStyle::LeftJoycon => Buttons::LEFT_SL,
            ControllerStyle::RightJoycon => Buttons::RIGHT_SL,
            _ => Buttons::L
        }
    }
    fn right_button(&self) -> Buttons {
        match self.get_controller().controller_style {
            ControllerStyle::LeftJoycon => Buttons::LEFT_SR,
            ControllerStyle::RightJoycon => Buttons::RIGHT_SR,
            _ => Buttons::R
        }
    }
    fn swap_button(&self) -> Buttons {
        match self.get_controller().controller_style {
            ControllerStyle::LeftJoycon => Buttons::RIGHT,
            ControllerStyle::RightJoycon => Buttons::Y,
            _ => Buttons::X
        }
    }

    fn check_swap_macro(&self) -> Option<SwapAction> {
        // make sure the controlled card is a player
        if self.player_kind() != PlayerKind::Player {
            return None;
        }

        let controller = self.get_controller();
        let (left, right, swap) = (self.left_button(), self.right_button(), self.swap_button());
        if controller.buttons.contains(swap) {
            // println!("swap button pressed");
            let hold_left = controller.buttons.contains(left);
            let hold_right = controller.buttons.contains(right);

            if hold_left && hold_right {
                return Some(SwapAction::Out); // port will disconnect without rejoining
            }
            else if hold_left {
                return Some(SwapAction::Left); // port will swap backwards
            }
            else if hold_right {
                return Some(SwapAction::Right); // port will swap forwards
            }
        }
        None
    }
    fn player_kind(&self) -> PlayerKind {
        unsafe {
            match self.player_info {
                Some(player) => {
                    match (*(*player).card).player_kind {
                        0 => PlayerKind::Player,
                        1 => PlayerKind::CPU,
                        2 => PlayerKind::Amiibo,
                        _ => PlayerKind::None
                    }
                },
                None => PlayerKind::None
            }
        }
    }
}

unsafe fn count_active_players(instance: CharaSelect) -> i32 {
    let mut active_players = 0;

    // Walk the known player-info array; 
    let mut addr = instance.player_base;
    for i in 0..8 {
        if addr as u64 == instance.player_max as u64 {
            break;
        }
        let player = (instance.player_base as u64 + (i * 0x10)) as *const PlayerInfo;
        if player.is_null() {
            break;
        }
        let card = (*player).card;
        if !card.is_null() {
            let is_player_or_cpu = (*card).player_kind == 0 || (*card).player_kind == 1;
            if is_player_or_cpu {
                active_players += 1;
            }
        }
        addr = (addr as u64 + 0x10) as *const PlayerInfo;
    }

    active_players
}

static mut IS_UNPRESSED : bool = false;
// this function loops while the css is active, allowing for runtime operations
#[skyline::hook(offset = 0x1a2b570)]
unsafe fn css_main_loop(arg: *const CharaSelect) {
    {
        if ninput::any::is_down(ninput::Buttons::MINUS) {
            if !IS_UNPRESSED {
                println!("Minus Pressed!");
                utils::open_modes_session();
            }
            IS_UNPRESSED = true;
        } else {
            IS_UNPRESSED = false;
        }

        let instance = *arg;
        let mut data = PORT_DATA.write();

        // run once on initialization
        if instance.frames_elapsed == 0 {
            data.reset();

            if instance.max_players_allowed != 8 || instance.local_wireless != 0 {
                data.enable_swap = false;
                println!("Port swapping is disabled.");
                
                return original!()(arg);
            }
            println!("Port swapping is enabled!");
            data.enable_swap = true;
            data.root_card = instance.first_player as u64;
        }
        
        // TODO: implement buffer swap for >2 players
        // if SsbuSync::ALLOW_BUFFER_SWAP() {
        //     let player_count = count_active_players(instance);
        //     crate::set_doubles_delay(player_count);
        //     ssbusync::Check_Buffer_Swap();
        // }
        
        // TODO: is this really the best way to check for online gamemodes?
        // let is_online = (instance.max_players_allowed != 8 || instance.local_wireless != 0);
        // if  SsbuSync::SyncEnv::online_only() {
        //     SsbuSync::online::ToggleOnlineFix(is_online);
        // }

        if !data.enable_swap || instance.ready_state != 0 {
            return original!()(arg);
        }

        // keep list of connected controllers up to date
        for id in ID_LIST {
            match Controller::get_from_id(*id) {
                Some(controller) => {
                    let mut addr = instance.player_base;
                    for i in 0..8 {
                        if addr as u64 == instance.player_max as u64 {
                            // println!("reached end of player list after {i} players");
                            break;
                        }
                        let player = (instance.player_base as u64 + (i * 0x10)) as *const PlayerInfo;
                        let card = *(*player).card;
                        let mut card_owner = card.controller_id;
                        if card_owner == 8 { card_owner = 0x20 };
                        // println!("Controller ID {id} checking for match on slot {i} with owner {card_owner}");
                        if card_owner == *id {
                            if !data.is_active_id(id) && controller.is_supported_controller() {
                                data.active_controllers.push(PortController {
                                    id: *id,
                                    port: Some(i as u8 + 1),
                                    player_info: Some(player),
                                    operation: None,
                                    swap_lock: false
                                });

                                println!("Controller ID {id} has been assigned to port {}", i + 1);
                            }

                            break;
                        }
                        
                        addr = (addr as u64 + 0x10) as *const PlayerInfo;
                    }
                }
                None => {
                    // println!("Controller ID {id} is not active.");
                    if data.is_active_id(id) {
                        data.remove_controller(*id);
                    }
                }
            }
        }

        // watch active controllers
        for i in 0..data.active_controllers.len() {
            let controller = data.active_controllers[i];
            if let Some(port) = controller.port {
                if port as u32 > instance.current_player_count {
                    println!("Controller ID {} is in an invalid port! Disconnecting...", controller.id);
                    data.remove_controller(controller.id);
                    break;
                }
            }

            if let Some(player) = controller.player_info {
                let card = (*player).card;
                let mut card_owner = (*card).controller_id;
                if card_owner == 8 { card_owner = 0x20 };
                if card_owner != controller.id {
                    println!("Controller ID {} has been disconnected! Removing...", controller.id);
                    data.remove_controller(controller.id);
                    break;
                }
            }

            // upgrade to mutable reference
            let controller = match data.controller_from_id(controller.id) {
                Some(controller) => controller,
                None => continue
            };
            if controller.swap_lock {
                if !controller.get_controller().buttons.contains(controller.swap_button()) {
                    controller.swap_lock = false;
                }
                break;
            }

            match controller.check_swap_macro() {
                Some(action) => {
                    // println!("Controller ID {:#x} triggered SwapAction: {:?}", controller.id, action);
                    controller.operation = Some(action);
                    
                    // disconnect controller from current port
                    // this hook runs BEFORE controller initialization, which allows for the controller to instantly reconnect if allowed
                    if let Some(player) = controller.player_info {
                        controller_input_off(arg, (*player).card);
                        controller_card_off(player, 1);
                        controller_something_off(instance.local_wireless, player);
                        controller_token_off(arg, player);
                    }
                    controller.swap_lock = true;
                    data.swap_target = Some(*controller);

                    break;
                }
                None => {}
            }
        }
    }


    original!()(arg)
}

// this function runs right after a controller connects to the css
#[skyline::hook(offset = 0x1a31020)]
unsafe fn init_css_player(
    arg1: u64, // pointer to an unknown struct that contains some css / player data
    port: i32, // number of the player port that the game is going to try adding (1-8)
    arg3: u64, // determines the actual port that the joined player owns and controls
    arg4: u64 // determines the port of the character card that gets loaded for the UI
) {
    let mut data = PORT_DATA.write();
    match data.swap_target {
        Some(controller) => {
            data.swap_target = None;
            if controller.operation.is_none() {
                return original!()(arg1, port, arg3, arg4);
            }
            
            let mut direction = 1;
            match controller.operation.unwrap_or(SwapAction::Out) {
                SwapAction::Right => {
                    print!("Swapping Controller ID {} right to ", controller.id);
                }
                SwapAction::Left => {
                    print!("Swapping Controller ID {} left to ", controller.id);
                    direction = -1;
                }
                SwapAction::Out => {
                    data.remove_controller(controller.id);
                    return;
                }
            }

            let current_port = controller.port.unwrap_or(1) as i8 - 1;
            let mut target_port = (8 + current_port + direction) % 8;
            for _ in 0..8 {
                let port_num = target_port as u8 + 1;
                if !data.is_active_port(port_num) {
                    print!("port {}\n", port_num);
                    let base = data.root_card as u64;
                    let card = base + (0x10 * target_port as u64);

                    let controller = match data.controller_from_id(controller.id) {
                        Some(controller) => controller,
                        None => return
                    };
                    
                    // offset player data
                    if let Some(player) = controller.player_info {
                        let player = player as i64;
                        let diff = target_port - current_port;
                        let new = player + (0x10 * diff as i64);
                        controller.player_info = Some(new as *const PlayerInfo);
                    }
                    controller.port = Some(port_num);

                    // alter which port is assigned
                    let offset = card as i64 - arg3 as i64;
                    return original!()(
                        arg1, 
                        port_num as i32, 
                        (arg3 as i64 + offset) as u64,
                        (arg4 as i64 + offset) as u64
                    );
                }
                target_port = (8 + target_port + direction) % 8;
            }
            print!("...nothing! no available ports found!\n");

            return;
        }
        None => {}
    }

    original!()(arg1, port, arg3, arg4)
}

#[skyline::from_offset(0x1a1e430)] // clears the controller's input and player hand
unsafe fn controller_input_off(css_instance: *const CharaSelect, player_card: *const PlayerCard);

#[skyline::from_offset(0x1a1e660)] // clears the player card from the css
unsafe fn controller_card_off(player: *const PlayerInfo, unk: u64);

#[skyline::from_offset(0x1a1e860)] // unsure what this clears, but we run it just to be safe
unsafe fn controller_something_off(wireless_state: u32, player: *const PlayerInfo);

#[skyline::from_offset(0x1a1f0b0)] // clears the player token
unsafe fn controller_token_off(css_instance: *const CharaSelect, player: *const PlayerInfo);

pub fn install() {
    skyline::install_hooks!(
        css_main_loop,
        init_css_player
    );
}
