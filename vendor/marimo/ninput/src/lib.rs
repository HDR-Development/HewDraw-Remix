#[macro_use]
extern crate bitflags;

macro_rules! bit {
    ($amount:expr) => {
        1 << ($amount)
    }
}

bitflags! {
    #[derive(Default)]
    #[repr(C)]
    pub struct Buttons: u64 {
        const A               = bit!(0);
        const B               = bit!(1);
        const X               = bit!(2);
        const Y               = bit!(3);
        const STICK_L         = bit!(4);
        const STICK_R         = bit!(5);
        const L               = bit!(6);
        const R               = bit!(7);
        const ZL              = bit!(8);
        const ZR              = bit!(9);
        const PLUS            = bit!(10);
        const MINUS           = bit!(11);
        const LEFT            = bit!(12);
        const UP              = bit!(13);
        const RIGHT           = bit!(14);
        const DOWN            = bit!(15);
        const STICK_L_LEFT    = bit!(16);
        const STICK_L_UP      = bit!(17);
        const STICK_L_RIGHT   = bit!(18);
        const STICK_L_DOWN    = bit!(19);
        const STICK_R_LEFT    = bit!(20);
        const STICK_R_UP      = bit!(21);
        const STICK_R_RIGHT   = bit!(22);
        const STICK_R_DOWN    = bit!(23);
        const LEFT_SL         = bit!(24);
        const LEFT_SR         = bit!(25);
        const RIGHT_SL        = bit!(26);
        const RIGHT_SR        = bit!(27);
        const PALMA           = bit!(28);
        const VERIFICATION    = bit!(29);
        const HANDHELD_LEFT_B = bit!(30);
        const LAGON_C_LEFT    = bit!(31);
        const LAGON_C_UP      = bit!(32);
        const LAGON_C_RIGHT   = bit!(33);
        const LAGON_C_DOWN    = bit!(34);
    }
}

bitflags! {
    #[derive(Default)]
    #[repr(C)]
    pub struct Attributes: u32 {
        const IS_CONNECTED       = bit!(0);
        const IS_WIRED           = bit!(1);
        const IS_LEFT_CONNECTED  = bit!(2);
        const IS_LEFT_WIRED      = bit!(3);
        const IS_RIGHT_CONNECTED = bit!(4);
        const IS_RIGHT_WIRED     = bit!(5);
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum ControllerStyle {
    Invalid = 0,
    ProController = bit!(0),
    Handheld = bit!(1),
    DualJoycon = bit!(2),
    LeftJoycon = bit!(3),
    RightJoycon = bit!(4),
    GamecubeController = bit!(5),
    Pokeball = bit!(6),
    NES = bit!(7),
    HandheldNES = bit!(8),
    SNES = bit!(9),
    Nintendo64 = bit!(10),
    SegaGenesis = bit!(11),
    GenericExternal = bit!(29),
    Generic = bit!(30)
}

impl ControllerStyle {
    pub fn is_full_control(&self) -> bool {
        match self {
            Self::ProController | Self::Handheld | Self::DualJoycon => true,
            _ => false
        }
    }

    pub fn is_standard(&self) -> bool {
        match self {
            Self::LeftJoycon | Self::RightJoycon => true,
            _ => self.is_full_control()
        }
    }

    pub fn supports_analog_trigger(&self) -> bool {
        matches!(self, Self::GamecubeController)
    }
}

impl Buttons {
    pub fn is_up(&self) -> bool {
        self.intersects(Self::UP | Self::STICK_L_LEFT | Self::STICK_R_UP)
    }

    pub fn is_down(&self) -> bool {
        self.intersects(Self::DOWN | Self::STICK_L_DOWN | Self::STICK_R_DOWN)
    }

    pub fn is_left(&self) -> bool {
        self.intersects(Self::LEFT | Self::STICK_L_LEFT | Self::STICK_R_LEFT)
    }

    pub fn is_right(&self) -> bool {
        self.intersects(Self::RIGHT | Self::STICK_L_RIGHT | Self::STICK_R_RIGHT)
    }

    pub fn is_sl(&self) -> bool {
        self.intersects(Self::LEFT_SL | Self::RIGHT_SL)
    }

    pub fn is_sr(&self) -> bool {
        self.intersects(Self::LEFT_SR | Self::RIGHT_SR)
    }
    
    pub fn is_z(&self) -> bool {
        self.intersects(Self::ZL | Self::ZR)
    }

    pub fn up() -> Self {
        Self::UP | Self::STICK_L_UP | Self::STICK_R_UP
    }
    
    pub fn right() -> Self {
        Self::RIGHT | Self::STICK_L_RIGHT | Self::STICK_R_RIGHT
    }

    pub fn down() -> Self {
        Self::DOWN | Self::STICK_L_DOWN | Self::STICK_R_DOWN
    }

    pub fn left() -> Self {        
        Self::LEFT | Self::STICK_L_LEFT | Self::STICK_L_LEFT
    }
    
    pub fn z() -> Self {
        Self::ZL | Self::ZR
    }
}

#[derive(Copy, Clone, Default)]
#[repr(C)]
struct NpadCommonState {
    pub sampling_number: usize,
    pub buttons: Buttons,
    pub stick_l_x: i32,
    pub stick_l_y: i32,
    pub stick_r_x: i32,
    pub stick_r_y: i32,
    pub attributes: Attributes,
    _padding: u32
}

#[derive(Copy, Clone, Default)]
#[repr(C)]
struct NpadGcState {
    pub sampling_number: usize,
    pub buttons: Buttons,
    pub stick_l_x: i32,
    pub stick_l_y: i32,
    pub stick_r_x: i32,
    pub stick_r_y: i32,
    pub attributes: Attributes,
    pub left_trigger: u32,
    pub right_trigger: u32,
    _padding: u32
}

const TRIGGER_MAX: u32 = 0x7FFF;
const STICK_MAX: f32 = 0x7FFF as f32;

type NpadFullKeyState = NpadCommonState;
type NpadHandheldState = NpadCommonState;
type NpadJoyDualState = NpadCommonState;
type NpadJoyLeftState = NpadCommonState;
type NpadJoyRightState = NpadCommonState;

extern "C" {
    #[link_name = "\u{1}_ZN2nn3hid12GetNpadStateEPNS0_16NpadFullKeyStateERKj"]
    fn get_full_key_state(state: *mut NpadFullKeyState, id: *const u32);

    #[link_name = "\u{1}_ZN2nn3hid12GetNpadStateEPNS0_16NpadJoyDualStateERKj"]
    fn get_joy_dual_state(state: *mut NpadJoyDualState, id: *const u32);

    #[link_name = "\u{1}_ZN2nn3hid12GetNpadStateEPNS0_16NpadJoyLeftStateERKj"]
    fn get_joy_left_state(state: *mut NpadJoyLeftState, id: *const u32);

    #[link_name = "\u{1}_ZN2nn3hid12GetNpadStateEPNS0_17NpadJoyRightStateERKj"]
    fn get_joy_right_state(state: *mut NpadJoyRightState, id: *const u32);

    #[link_name = "\u{1}_ZN2nn3hid12GetNpadStateEPNS0_17NpadHandheldStateERKj"]
    fn get_handheld_state(state: *mut NpadHandheldState, id: *const u32);

    #[link_name = "\u{1}_ZN2nn3hid12GetNpadStateEPNS0_11NpadGcStateERKj"]
    fn get_gc_state(state: *mut NpadGcState, id: *const u32);

    #[link_name = "\u{1}_ZN2nn3hid15GetNpadStyleSetERKj"]
    fn get_style(id: *const u32) -> ControllerStyle;

    #[link_name = "\u{1}_ZN2nn3hid14InitializeNpadEv"]
    fn initialize_npad();

    #[link_name = "\u{1}_ZN2nn3hid24SetSupportedNpadStyleSetENS_4util10BitFlagSetILi32ENS0_12NpadStyleTagEEE"]
    fn set_supported_style_set(set: u32);

    #[link_name = "\u{1}_ZN2nn3hid6detail22SetSupportedNpadIdTypeEPKji"]
    fn set_supported_npad_ids(ids: *const u32, count: u32);
}

#[derive(Debug, Default, Copy, Clone)]
pub struct StickState {
    pub x: f32,
    pub y: f32
}

#[derive(Debug, Copy, Clone)]
pub struct AutorepeatInfo {
    pub enabled: bool,
    pub sample_count: u8,
    pub tracking_state: [u8; 64],
    pub autorepeat_buttons: Buttons
}

#[derive(Debug, Copy, Clone)]
pub struct Controller {
    pub buttons: Buttons,
    pub pressed_buttons: Buttons,
    pub released_buttons: Buttons,
    pub attributes: Attributes,
    pub left_stick: StickState,
    pub right_stick: StickState,
    pub trigger_l: f32,
    pub trigger_r: f32,
    pub controller_id: u32,
    pub autorepeat: AutorepeatInfo,
    pub controller_style: ControllerStyle,
}

impl Controller {
    fn update_from_common(&mut self, state: NpadCommonState) {
        let prev = self.buttons;
        self.buttons = state.buttons;
        self.attributes = state.attributes;
        self.pressed_buttons = self.buttons & !prev;
        self.released_buttons = prev & !self.buttons;
        self.left_stick = StickState {
            x: (state.stick_l_x as f32) / STICK_MAX,
            y: (state.stick_l_y as f32) / STICK_MAX
        };
        self.right_stick = StickState {
            x: (state.stick_r_x as f32) / STICK_MAX,
            y: (state.stick_r_y as f32) / STICK_MAX
        };
        self.trigger_l = if self.buttons.intersects(Buttons::L) { 1.0 } else { 0.0 };
        self.trigger_r = if self.buttons.intersects(Buttons::R) { 1.0 } else { 0.0 };
        self.autorepeat.autorepeat_buttons = Buttons::empty();
        if self.autorepeat.enabled {
            for x in 0..64 {
                if let Some(button) = Buttons::from_bits(1 << x) {
                    if self.buttons.intersects(button) {
                        self.autorepeat.tracking_state[x] += 1;
                        if self.autorepeat.tracking_state[x] >= self.autorepeat.sample_count {
                            self.autorepeat.tracking_state[x] = 0;
                            self.autorepeat.autorepeat_buttons |= button;
                        }
                    }
                }
            }
        }
    }   

    fn update_from_gamecube(&mut self, state: NpadGcState) {
        let prev = self.buttons;
        self.buttons = state.buttons;
        self.attributes = state.attributes;
        self.pressed_buttons = self.buttons & !prev;
        self.released_buttons = prev & !self.buttons;
        self.left_stick = StickState {
            x: (state.stick_l_x as f32) / STICK_MAX,
            y: (state.stick_l_y as f32) / STICK_MAX
        };
        self.right_stick = StickState {
            x: (state.stick_r_x as f32) / STICK_MAX,
            y: (state.stick_r_y as f32) / STICK_MAX
        };
        self.trigger_l = state.left_trigger as f32 / TRIGGER_MAX as f32;
        self.trigger_r = state.right_trigger as f32 / TRIGGER_MAX as f32;
        self.autorepeat.autorepeat_buttons = Buttons::empty();
        if self.autorepeat.enabled {
            for x in 0..64 {
                if let Some(button) = Buttons::from_bits(1 << x) {
                    if self.buttons.intersects(button) {
                        self.autorepeat.tracking_state[x] += 1;
                        if self.autorepeat.tracking_state[x] >= self.autorepeat.sample_count {
                            self.autorepeat.tracking_state[x] = 0;
                            self.autorepeat.autorepeat_buttons |= button;
                        }
                    }
                }
            }
        }
    }

    fn is_supported_style(style: ControllerStyle) -> bool {
        style.is_standard() || matches!(style, ControllerStyle::GamecubeController)
    }

    pub fn new(id: u32) -> Self {
        Self {
            buttons: Buttons::empty(),
            pressed_buttons: Buttons::empty(),
            released_buttons: Buttons::empty(),
            attributes: Attributes::empty(),
            left_stick: StickState::default(),
            right_stick: StickState::default(),
            trigger_l: 0.0,
            trigger_r: 0.0,
            controller_id: id,
            autorepeat: AutorepeatInfo { 
                enabled: false, 
                sample_count: 0, 
                tracking_state: [0; 64], 
                autorepeat_buttons: Buttons::empty() 
            },
            controller_style: ControllerStyle::Invalid,
        }
    }

    pub fn get_from_id(id: u32) -> Option<Self> {
        let style = unsafe {
            get_style(&id)
        };

        if Self::is_supported_style(style) {
            let mut controller = Self::new(id);
            controller.update();
            Some(controller)
        } else {
            None
        }
    }

    pub fn is_supported_controller(&self) -> bool {
        Self::is_supported_style(self.controller_style)
    }

    pub fn update(&mut self) {
        let style = unsafe {
            get_style(&self.controller_id)
        };

        self.controller_style = style;
        if !self.is_supported_controller() {
            return;
        }

        if self.controller_style.is_standard() {
            let mut state = NpadCommonState::default();
            match self.controller_style {
                ControllerStyle::ProController => unsafe {
                    get_full_key_state(&mut state, &self.controller_id);
                },
                ControllerStyle::Handheld => unsafe {
                    get_handheld_state(&mut state, &self.controller_id);
                },
                ControllerStyle::DualJoycon => unsafe {
                    get_joy_dual_state(&mut state, &self.controller_id);
                },
                ControllerStyle::LeftJoycon => unsafe {
                    get_joy_left_state(&mut state, &self.controller_id);
                },
                ControllerStyle::RightJoycon => unsafe {
                    get_joy_right_state(&mut state, &self.controller_id);
                },
                _ => unreachable!()
            }
            self.update_from_common(state);
        } else if matches!(self.controller_style, ControllerStyle::GamecubeController) {
            let mut state = NpadGcState::default();
            unsafe {
                get_gc_state(&mut state, &self.controller_id);
            }
            self.update_from_gamecube(state);
        }
    }
}

pub mod any {
    use super::*;
    
    pub fn combined_buttons() -> Buttons {
        let mut buttons = Buttons::default();
        for x in 0..8 {
            if let Some(controller) = Controller::get_from_id(x) {
                buttons |= controller.pressed_buttons;
            }
        }
        if let Some(controller) = Controller::get_from_id(0x20) {
            buttons |= controller.pressed_buttons;
        }
        buttons
    }

    pub fn is_press_any(buttons: Buttons) -> bool {
        for x in 0..7 {
            match Controller::get_from_id(x) {
                Some(controller) if controller.pressed_buttons.intersects(buttons) => return true,
                _ => {}
            }
        }
        if let Some(controller) = Controller::get_from_id(0x20) {
            controller.pressed_buttons.intersects(buttons)
        } else {
            false
        }
    }

    pub fn is_down_any(buttons: Buttons) -> bool {
        for x in 0..7 {
            match Controller::get_from_id(x) {
                Some(controller) if controller.buttons.intersects(buttons) => return true,
                _ => {}
            }
        }
        if let Some(controller) = Controller::get_from_id(0x20) {
            controller.buttons.intersects(buttons)
        } else {
            false
        }
    }

    pub fn is_release_any(buttons: Buttons) -> bool {
        for x in 0..7 {
            match Controller::get_from_id(x) {
                Some(controller) if controller.released_buttons.intersects(buttons) => return true,
                _ => {}
            }
        }
        if let Some(controller) = Controller::get_from_id(0x20) {
            controller.released_buttons.intersects(buttons)
        } else {
            false
        }
    }

    pub fn is_press(buttons: Buttons) -> bool {
        for x in 0..7 {
            match Controller::get_from_id(x) {
                Some(controller) if controller.pressed_buttons.contains(buttons) => return true,
                _ => {}
            }
        }
        if let Some(controller) = Controller::get_from_id(0x20) {
            controller.pressed_buttons.contains(buttons)
        } else {
            false
        }
    }
    
    pub fn is_down(buttons: Buttons) -> bool {
        for x in 0..7 {
            match Controller::get_from_id(x) {
                Some(controller) if controller.buttons.contains(buttons) => return true,
                _ => {}
            }
        }
        if let Some(controller) = Controller::get_from_id(0x20) {
            controller.buttons.contains(buttons)
        } else {
            false
        }
    }
    
    pub fn is_release(buttons: Buttons) -> bool {
        for x in 0..7 {
            match Controller::get_from_id(x) {
                Some(controller) if controller.released_buttons.contains(buttons) => return true,
                _ => {}
            }
        }
        if let Some(controller) = Controller::get_from_id(0x20) {
            controller.released_buttons.contains(buttons)
        } else {
            false
        }
    }
}

pub fn init() {
    unsafe {
        initialize_npad();
        set_supported_style_set(0x00_00_00_3F);
        static ID_LIST: &[u32] = &[0, 1, 2, 3, 4, 5, 6, 7, 0x10];
        set_supported_npad_ids(ID_LIST.as_ptr(), ID_LIST.len() as u32);
    }
}
