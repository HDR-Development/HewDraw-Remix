use super::css::get_ptr_to_controls;
use dynamic::ext::{ControllerMapping, InputKind};

/// Trait for the custom tag-based submenu navigation systems
pub trait TagSubMenu {
    fn decide(&self, index: usize) -> Option<Box<dyn TagSubMenu>>;
    fn cancel(&self) -> Option<Box<dyn TagSubMenu>>;
    fn get_title(&self) -> &'static [u8];
    fn get_button_text(&self, button_index: usize) -> Option<Vec<u8>>;
    fn get_button_count(&self) -> usize;
    fn get_start_index(&self) -> Option<usize>;
}

/// Controls the top-level of the submenu navigation
pub struct TopLevel {
    pub controls_id: usize,
}

// Constants to represent each potential button index
impl TopLevel {
    const PRESETS: usize = 0;
    const GAMECUBE: usize = 1;
    const PRO_CONTROLLER: usize = 2;
    const JOY_CONS: usize = 3;
    const COUNT: usize = 4;
}

impl TagSubMenu for TopLevel {
    fn decide(&self, index: usize) -> Option<Box<dyn TagSubMenu>> {
        match index {
            Self::PRESETS => Some(Box::new(ShortcutsMenu {
                controls_id: self.controls_id,
                start_button: None,
            })),
            Self::GAMECUBE => Some(Box::new(GamecubeMenu {
                controls_id: self.controls_id,
                start_button: None,
            })),
            Self::PRO_CONTROLLER => Some(Box::new(ProControllerMenu {
                controls_id: self.controls_id,
                start_button: None,
            })),
            Self::JOY_CONS => Some(Box::new(JoyConMenu {
                controls_id: self.controls_id,
                start_button: None,
            })),
            _ => None,
        }
    }

    fn cancel(&self) -> Option<Box<dyn TagSubMenu>> {
        None
    }

    fn get_title(&self) -> &'static [u8] {
        b"Edit Controls"
    }

    fn get_button_count(&self) -> usize {
        Self::COUNT
    }

    fn get_button_text(&self, button_index: usize) -> Option<Vec<u8>> {
        let name: &[u8] = match button_index {
            Self::PRESETS => b"Shortcuts",
            Self::GAMECUBE => b"Gamecube",
            Self::PRO_CONTROLLER => b"Pro Controller",
            Self::JOY_CONS => b"Joy-Cons",
            _ => return None,
        };

        Some(name.to_vec())
    }

    fn get_start_index(&self) -> Option<usize> {
        None
    }
}

pub struct ButtonSelector<F: Fn(&mut ControllerMapping, InputKind)> {
    controls_id: usize,
    return_to: Box<dyn Fn() -> Option<Box<dyn TagSubMenu>>>,
    initial: InputKind,
    set_input_kind: F,
}

impl<F: Fn(&mut ControllerMapping, InputKind)> ButtonSelector<F> {
    const ATTACK: usize = 0;
    const SPECIAL: usize = 1;
    const JUMP: usize = 2;
    const SHIELD: usize = 3;
    const GRAB: usize = 4;
    const PARRY: usize = 5;
    const TAUNT: usize = 6;
    const SHORT_HOP: usize = 7;
    const TILT_ATTACK: usize = 8;
    const SMASH_ATTACK: usize = 9;
    const COUNT: usize = 10;
}

impl<F: Fn(&mut ControllerMapping, InputKind)> TagSubMenu for ButtonSelector<F> {
    fn get_button_text(&self, button_index: usize) -> Option<Vec<u8>> {
        let name: &[u8] = match button_index {
            Self::ATTACK => b"Attack",
            Self::SPECIAL => b"Special",
            Self::JUMP => b"Jump",
            Self::SHIELD => b"Shield",
            Self::GRAB => b"Grab",
            Self::PARRY => b"Parry",
            Self::TAUNT => b"Taunt/Footstool",
            Self::SHORT_HOP => b"Short Hop",
            Self::TILT_ATTACK => b"Tilt",
            Self::SMASH_ATTACK => b"Smash",
            _ => return None,
        };

        Some(name.to_vec())
    }

    fn get_title(&self) -> &'static [u8] {
        b"Pick Button"
    }

    fn get_button_count(&self) -> usize {
        Self::COUNT
    }

    fn cancel(&self) -> Option<Box<dyn TagSubMenu>> {
        (self.return_to)()
    }

    fn decide(&self, index: usize) -> Option<Box<dyn TagSubMenu>> {
        let mut controls = unsafe { get_ptr_to_controls(self.controls_id) };

        let id = match index {
            Self::ATTACK => InputKind::Attack,
            Self::SPECIAL => InputKind::Special,
            Self::JUMP => InputKind::Jump,
            Self::SHIELD => InputKind::Guard,
            Self::GRAB => InputKind::Grab,
            Self::PARRY => InputKind::Parry,
            Self::TAUNT => InputKind::AppealHi,
            Self::SHORT_HOP => InputKind::JumpMini,
            Self::TILT_ATTACK => InputKind::TiltAttack,
            Self::SMASH_ATTACK => InputKind::SmashAttack,
            _ => return self.cancel(),
        };

        (self.set_input_kind)(controls.controls_mut(), id);
        self.cancel()
    }

    fn get_start_index(&self) -> Option<usize> {
        match self.initial {
            InputKind::Attack => Some(Self::ATTACK),
            InputKind::Special => Some(Self::SPECIAL),
            InputKind::Jump => Some(Self::JUMP),
            InputKind::Guard => Some(Self::SHIELD),
            InputKind::Grab => Some(Self::GRAB),
            InputKind::SmashAttack => Some(Self::SMASH_ATTACK),
            InputKind::AppealHi => Some(Self::TAUNT),
            InputKind::AppealS => Some(Self::TAUNT),
            InputKind::AppealLw => Some(Self::TAUNT),
            InputKind::Unset => None,
            InputKind::JumpMini => Some(Self::SHORT_HOP),
            InputKind::TiltAttack => Some(Self::TILT_ATTACK),
            InputKind::Parry => Some(Self::PARRY),
        }
    }
}

#[derive(Copy, Clone)]
pub struct GamecubeMenu {
    controls_id: usize,
    start_button: Option<usize>,
}

impl GamecubeMenu {
    const A: usize = 0;
    const B: usize = 1;
    const X: usize = 2;
    const Y: usize = 3;
    const Z: usize = 4;
    const L: usize = 5;
    const R: usize = 6;
    const DPAD_UP: usize = 7;
    const DPAD_LR: usize = 8;
    const DPAD_DOWN: usize = 9;
    const RIGHT_STICK: usize = 10;
    const TAP_JUMP: usize = 11;
    const PARRY_INPUT: usize = 12;
    const RIVALS_WJ: usize = 13;
    const STICK_SENS: usize = 14;
    const RUMBLE: usize = 15;
    const AB_SMASH: usize = 16;
    const COUNT: usize = 17;
}

impl TagSubMenu for GamecubeMenu {
    fn get_start_index(&self) -> Option<usize> {
        self.start_button
    }

    fn cancel(&self) -> Option<Box<dyn TagSubMenu>> {
        Some(Box::new(TopLevel {
            controls_id: self.controls_id,
        }))
    }

    fn get_button_count(&self) -> usize {
        Self::COUNT
    }

    fn decide(&self, index: usize) -> Option<Box<dyn TagSubMenu>> {
        let mut controls = unsafe { get_ptr_to_controls(self.controls_id) };
        let controls = controls.controls_mut();
        let return_to = Self {
            controls_id: self.controls_id,
            start_button: Some(index),
        };
        match index {
            Self::A => Some(Box::new(ButtonSelector {
                controls_id: self.controls_id,
                return_to: Box::new(move || Some(Box::new(return_to))),
                initial: controls.gc_a,
                set_input_kind: |ctrls, input| ctrls.gc_a = input,
            })),
            Self::B => Some(Box::new(ButtonSelector {
                controls_id: self.controls_id,
                return_to: Box::new(move || Some(Box::new(return_to))),
                initial: controls.gc_b,
                set_input_kind: |ctrls, input| ctrls.gc_b = input,
            })),
            Self::X => Some(Box::new(ButtonSelector {
                controls_id: self.controls_id,
                return_to: Box::new(move || Some(Box::new(return_to))),
                initial: controls.gc_x,
                set_input_kind: |ctrls, input| ctrls.gc_x = input,
            })),
            Self::Y => Some(Box::new(ButtonSelector {
                controls_id: self.controls_id,
                return_to: Box::new(move || Some(Box::new(return_to))),
                initial: controls.gc_y,
                set_input_kind: |ctrls, input| ctrls.gc_y = input,
            })),
            Self::Z => Some(Box::new(ButtonSelector {
                controls_id: self.controls_id,
                return_to: Box::new(move || Some(Box::new(return_to))),
                initial: controls.gc_z,
                set_input_kind: |ctrls, input| ctrls.gc_z = input,
            })),
            Self::L => Some(Box::new(ButtonSelector {
                controls_id: self.controls_id,
                return_to: Box::new(move || Some(Box::new(return_to))),
                initial: controls.gc_l,
                set_input_kind: |ctrls, input| ctrls.gc_l = input,
            })),
            Self::R => Some(Box::new(ButtonSelector {
                controls_id: self.controls_id,
                return_to: Box::new(move || Some(Box::new(return_to))),
                initial: controls.gc_r,
                set_input_kind: |ctrls, input| ctrls.gc_r = input,
            })),
            Self::DPAD_UP => Some(Box::new(ButtonSelector {
                controls_id: self.controls_id,
                return_to: Box::new(move || Some(Box::new(return_to))),
                initial: controls.gc_dup,
                set_input_kind: |ctrls, input| ctrls.gc_dup = input,
            })),
            Self::DPAD_LR => Some(Box::new(ButtonSelector {
                controls_id: self.controls_id,
                return_to: Box::new(move || Some(Box::new(return_to))),
                initial: controls.gc_dlr,
                set_input_kind: |ctrls, input| ctrls.gc_dlr = input,
            })),
            Self::DPAD_DOWN => Some(Box::new(ButtonSelector {
                controls_id: self.controls_id,
                return_to: Box::new(move || Some(Box::new(return_to))),
                initial: controls.gc_ddown,
                set_input_kind: |ctrls, input| ctrls.gc_ddown = input,
            })),
            Self::RIGHT_STICK => Some(Box::new(ButtonSelector {
                controls_id: self.controls_id,
                return_to: Box::new(move || Some(Box::new(return_to))),
                initial: controls.pro_cstick,
                set_input_kind: |ctrls, input| ctrls.pro_cstick = input,
            })),
            Self::TAP_JUMP => {
                let mut controls = unsafe { get_ptr_to_controls(self.controls_id) };
                controls.controls_mut().gc_tapjump = !controls.controls_mut().gc_tapjump;
                Some(Box::new(Self {
                    controls_id: self.controls_id,
                    start_button: Some(index),
                }))
            }
            Self::PARRY_INPUT => {
                let mut controls = unsafe { get_ptr_to_controls(self.controls_id) };
                if controls.controls_mut().gc_absmash & 2 != 0 {
                    controls.controls_mut().gc_absmash &= !2;
                } else {
                    controls.controls_mut().gc_absmash |= 2;
                }
                Some(Box::new(Self {
                    controls_id: self.controls_id,
                    start_button: Some(index),
                }))
            }
            Self::RIVALS_WJ => {
                let mut controls = unsafe { get_ptr_to_controls(self.controls_id) };
                if controls.controls_mut().gc_absmash & 4 != 0 {
                    controls.controls_mut().gc_absmash &= !4;
                } else {
                    controls.controls_mut().gc_absmash |= 4;
                }
                Some(Box::new(Self {
                    controls_id: self.controls_id,
                    start_button: Some(index),
                }))
            }
            Self::STICK_SENS => {
                let mut controls = unsafe { get_ptr_to_controls(self.controls_id) };
                controls.controls_mut().gc_sensitivity =
                    (controls.controls_mut().gc_sensitivity + 1) % 3;
                Some(Box::new(Self {
                    controls_id: self.controls_id,
                    start_button: Some(index),
                }))
            }
            Self::RUMBLE => {
                let mut controls = unsafe { get_ptr_to_controls(self.controls_id) };
                controls.controls_mut().gc_rumble = !controls.controls_mut().gc_rumble;
                Some(Box::new(Self {
                    controls_id: self.controls_id,
                    start_button: Some(index),
                }))
            }
            Self::AB_SMASH => {
                let mut controls = unsafe { get_ptr_to_controls(self.controls_id) };
                if controls.controls_mut().gc_absmash & 1 != 0 {
                    controls.controls_mut().gc_absmash &= !1;
                } else {
                    controls.controls_mut().gc_absmash |= 1;
                }
                Some(Box::new(Self {
                    controls_id: self.controls_id,
                    start_button: Some(index),
                }))
            }
            _ => Some(Box::new(self.clone())),
        }
    }

    fn get_title(&self) -> &'static [u8] {
        b"Gamecube"
    }

    fn get_button_text(&self, button_index: usize) -> Option<Vec<u8>> {
        let button_name: &[u8] = match button_index {
            Self::A => b"A",
            Self::B => b"B",
            Self::X => b"X",
            Self::Y => b"Y",
            Self::Z => b"Z",
            Self::L => b"L",
            Self::R => b"R",
            Self::DPAD_UP => b"DPad Up",
            Self::DPAD_LR => b"DPad Side",
            Self::DPAD_DOWN => b"DPad Down",
            Self::TAP_JUMP => {
                let mut controls = unsafe { get_ptr_to_controls(self.controls_id) };
                if controls.controls_mut().gc_tapjump {
                    b"Tap Jump: On"
                } else {
                    b"Tap Jump: Off"
                }
            }
            Self::PARRY_INPUT => {
                let mut controls = unsafe { get_ptr_to_controls(self.controls_id) };
                if controls.controls_mut().gc_absmash & 2 != 0 {
                    b"Parry Input: Taunt"
                } else {
                    b"Parry Input: Shield"
                }
            }
            Self::RIVALS_WJ => {
                let mut controls = unsafe { get_ptr_to_controls(self.controls_id) };
                if controls.controls_mut().gc_absmash & 4 != 0 {
                    b"Walljump: Button"
                } else {
                    b"Walljump: Flick"
                }
            }
            Self::STICK_SENS => {
                let mut controls = unsafe { get_ptr_to_controls(self.controls_id) };
                match controls.controls_mut().gc_sensitivity {
                    0 => b"Sensitivity: Low",
                    1 => b"Sensitivity: Med",
                    _ => b"Sensitivity: High",
                }
            }
            Self::RUMBLE => {
                let mut controls = unsafe { get_ptr_to_controls(self.controls_id) };
                if controls.controls_mut().gc_rumble {
                    b"Rumble: On"
                } else {
                    b"Rumble: Off"
                }
            }
            Self::AB_SMASH => {
                let mut controls = unsafe { get_ptr_to_controls(self.controls_id) };
                if controls.controls_mut().gc_absmash & 1 != 0 {
                    b"A+B Smash: On"
                } else {
                    b"A+B Smash: Off"
                }
            }
            _ => return None,
        };

        Some(button_name.to_vec())
    }
}

#[derive(Copy, Clone)]
pub struct ProControllerMenu {
    controls_id: usize,
    start_button: Option<usize>,
}

impl ProControllerMenu {
    const A: usize = 0;
    const B: usize = 1;
    const X: usize = 2;
    const Y: usize = 3;
    const L: usize = 4;
    const R: usize = 5;
    const ZL: usize = 6;
    const ZR: usize = 7;
    const DPAD_UP: usize = 8;
    const DPAD_LR: usize = 9;
    const DPAD_DOWN: usize = 10;
    const TAP_JUMP: usize = 11;
    const RIGHT_STICK: usize = 12;
    const PARRY_INPUT: usize = 13;
    const RIVALS_WJ: usize = 14;
    const STICK_SENS: usize = 15;
    const RUMBLE: usize = 16;
    const AB_SMASH: usize = 17;
    const COUNT: usize = 18;
}

impl TagSubMenu for ProControllerMenu {
    fn get_start_index(&self) -> Option<usize> {
        self.start_button
    }

    fn cancel(&self) -> Option<Box<dyn TagSubMenu>> {
        Some(Box::new(TopLevel {
            controls_id: self.controls_id,
        }))
    }

    fn get_button_count(&self) -> usize {
        Self::COUNT
    }

    fn decide(&self, index: usize) -> Option<Box<dyn TagSubMenu>> {
        let mut controls = unsafe { get_ptr_to_controls(self.controls_id) };
        let controls = controls.controls_mut();
        let return_to = Self {
            controls_id: self.controls_id,
            start_button: Some(index),
        };
        match index {
            Self::A => Some(Box::new(ButtonSelector {
                controls_id: self.controls_id,
                return_to: Box::new(move || Some(Box::new(return_to))),
                initial: controls.pro_a,
                set_input_kind: |ctrls, input| ctrls.pro_a = input,
            })),
            Self::B => Some(Box::new(ButtonSelector {
                controls_id: self.controls_id,
                return_to: Box::new(move || Some(Box::new(return_to))),
                initial: controls.pro_b,
                set_input_kind: |ctrls, input| ctrls.pro_b = input,
            })),
            Self::X => Some(Box::new(ButtonSelector {
                controls_id: self.controls_id,
                return_to: Box::new(move || Some(Box::new(return_to))),
                initial: controls.pro_x,
                set_input_kind: |ctrls, input| ctrls.pro_x = input,
            })),
            Self::Y => Some(Box::new(ButtonSelector {
                controls_id: self.controls_id,
                return_to: Box::new(move || Some(Box::new(return_to))),
                initial: controls.pro_y,
                set_input_kind: |ctrls, input| ctrls.pro_y = input,
            })),
            Self::L => Some(Box::new(ButtonSelector {
                controls_id: self.controls_id,
                return_to: Box::new(move || Some(Box::new(return_to))),
                initial: controls.pro_l,
                set_input_kind: |ctrls, input| ctrls.pro_l = input,
            })),
            Self::R => Some(Box::new(ButtonSelector {
                controls_id: self.controls_id,
                return_to: Box::new(move || Some(Box::new(return_to))),
                initial: controls.pro_r,
                set_input_kind: |ctrls, input| ctrls.pro_r = input,
            })),
            Self::ZL => Some(Box::new(ButtonSelector {
                controls_id: self.controls_id,
                return_to: Box::new(move || Some(Box::new(return_to))),
                initial: controls.pro_zl,
                set_input_kind: |ctrls, input| ctrls.pro_zl = input,
            })),
            Self::ZR => Some(Box::new(ButtonSelector {
                controls_id: self.controls_id,
                return_to: Box::new(move || Some(Box::new(return_to))),
                initial: controls.pro_zr,
                set_input_kind: |ctrls, input| ctrls.pro_zr = input,
            })),
            Self::DPAD_UP => Some(Box::new(ButtonSelector {
                controls_id: self.controls_id,
                return_to: Box::new(move || Some(Box::new(return_to))),
                initial: controls.pro_dup,
                set_input_kind: |ctrls, input| ctrls.pro_dup = input,
            })),
            Self::DPAD_LR => Some(Box::new(ButtonSelector {
                controls_id: self.controls_id,
                return_to: Box::new(move || Some(Box::new(return_to))),
                initial: controls.pro_dlr,
                set_input_kind: |ctrls, input| ctrls.pro_dlr = input,
            })),
            Self::DPAD_DOWN => Some(Box::new(ButtonSelector {
                controls_id: self.controls_id,
                return_to: Box::new(move || Some(Box::new(return_to))),
                initial: controls.pro_ddown,
                set_input_kind: |ctrls, input| ctrls.pro_ddown = input,
            })),
            Self::RIGHT_STICK => Some(Box::new(ButtonSelector {
                controls_id: self.controls_id,
                return_to: Box::new(move || Some(Box::new(return_to))),
                initial: controls.pro_cstick,
                set_input_kind: |ctrls, input| ctrls.pro_cstick = input,
            })),
            Self::TAP_JUMP => {
                let mut controls = unsafe { get_ptr_to_controls(self.controls_id) };
                controls.controls_mut().pro_tapjump = !controls.controls_mut().pro_tapjump;
                Some(Box::new(Self {
                    controls_id: self.controls_id,
                    start_button: Some(index),
                }))
            }
            Self::PARRY_INPUT => {
                let mut controls = unsafe { get_ptr_to_controls(self.controls_id) };
                if controls.controls_mut().pro_absmash & 2 != 0 {
                    controls.controls_mut().pro_absmash &= !2;
                } else {
                    controls.controls_mut().pro_absmash |= 2;
                }
                Some(Box::new(Self {
                    controls_id: self.controls_id,
                    start_button: Some(index),
                }))
            }
            Self::RIVALS_WJ => {
                let mut controls = unsafe { get_ptr_to_controls(self.controls_id) };
                if controls.controls_mut().pro_absmash & 4 != 0 {
                    controls.controls_mut().pro_absmash &= !4;
                } else {
                    controls.controls_mut().pro_absmash |= 4;
                }
                Some(Box::new(Self {
                    controls_id: self.controls_id,
                    start_button: Some(index),
                }))
            }
            Self::STICK_SENS => {
                let mut controls = unsafe { get_ptr_to_controls(self.controls_id) };
                controls.controls_mut().pro_sensitivity =
                    (controls.controls_mut().pro_sensitivity + 1) % 3;
                Some(Box::new(Self {
                    controls_id: self.controls_id,
                    start_button: Some(index),
                }))
            }
            Self::RUMBLE => {
                let mut controls = unsafe { get_ptr_to_controls(self.controls_id) };
                controls.controls_mut().pro_rumble = !controls.controls_mut().pro_rumble;
                Some(Box::new(Self {
                    controls_id: self.controls_id,
                    start_button: Some(index),
                }))
            }
            Self::AB_SMASH => {
                let mut controls = unsafe { get_ptr_to_controls(self.controls_id) };
                if controls.controls_mut().pro_absmash & 1 != 0 {
                    controls.controls_mut().pro_absmash &= !1;
                } else {
                    controls.controls_mut().pro_absmash |= 1;
                }
                Some(Box::new(Self {
                    controls_id: self.controls_id,
                    start_button: Some(index),
                }))
            }
            _ => Some(Box::new(self.clone())),
        }
    }

    fn get_title(&self) -> &'static [u8] {
        b"Pro Controller"
    }

    fn get_button_text(&self, button_index: usize) -> Option<Vec<u8>> {
        let button_name: &[u8] = match button_index {
            Self::A => b"A",
            Self::B => b"B",
            Self::X => b"X",
            Self::Y => b"Y",
            Self::L => b"L",
            Self::R => b"R",
            Self::ZL => b"ZL",
            Self::ZR => b"ZR",
            Self::DPAD_UP => b"DPad Up",
            Self::DPAD_LR => b"DPad Side",
            Self::DPAD_DOWN => b"DPad Down",
            Self::TAP_JUMP => {
                let mut controls = unsafe { get_ptr_to_controls(self.controls_id) };
                if controls.controls_mut().pro_tapjump {
                    b"Tap Jump: On"
                } else {
                    b"Tap Jump: Off"
                }
            }
            Self::PARRY_INPUT => {
                let mut controls = unsafe { get_ptr_to_controls(self.controls_id) };
                if controls.controls_mut().pro_absmash & 2 != 0 {
                    b"Parry Input: Taunt"
                } else {
                    b"Parry Input: Shield"
                }
            }
            Self::RIVALS_WJ => {
                let mut controls = unsafe { get_ptr_to_controls(self.controls_id) };
                if controls.controls_mut().pro_absmash & 4 != 0 {
                    b"Walljump: Button"
                } else {
                    b"Walljump: Flick"
                }
            }
            Self::STICK_SENS => {
                let mut controls = unsafe { get_ptr_to_controls(self.controls_id) };
                match controls.controls_mut().pro_sensitivity {
                    0 => b"Sensitivity: Low",
                    1 => b"Sensitivity: Med",
                    _ => b"Sensitivity: High",
                }
            }
            Self::RUMBLE => {
                let mut controls = unsafe { get_ptr_to_controls(self.controls_id) };
                if controls.controls_mut().pro_rumble {
                    b"Rumble: On"
                } else {
                    b"Rumble: Off"
                }
            }
            Self::AB_SMASH => {
                let mut controls = unsafe { get_ptr_to_controls(self.controls_id) };
                if controls.controls_mut().pro_absmash & 1 != 0 {
                    b"A+B Smash: On"
                } else {
                    b"A+B Smash: Off"
                }
            }
            _ => return None,
        };

        Some(button_name.to_vec())
    }
}

#[derive(Copy, Clone)]
pub struct JoyConMenu {
    controls_id: usize,
    start_button: Option<usize>,
}

impl JoyConMenu {
    const EAST: usize = 0;
    const SOUTH: usize = 1;
    const NORTH: usize = 2;
    const WEST: usize = 3;
    const SL: usize = 4;
    const SR: usize = 5;
    const BUMPER: usize = 6;
    const TRIGGER: usize = 7;
    const TAP_JUMP: usize = 8;
    const PARRY_INPUT: usize = 9;
    const RIVALS_WJ: usize = 10;
    const STICK_SENS: usize = 11;
    const RUMBLE: usize = 12;
    const AB_SMASH: usize = 13;
    const COUNT: usize = 14;
}

impl TagSubMenu for JoyConMenu {
    fn get_start_index(&self) -> Option<usize> {
        self.start_button
    }

    fn cancel(&self) -> Option<Box<dyn TagSubMenu>> {
        Some(Box::new(TopLevel {
            controls_id: self.controls_id,
        }))
    }

    fn get_button_count(&self) -> usize {
        Self::COUNT
    }

    fn decide(&self, index: usize) -> Option<Box<dyn TagSubMenu>> {
        let mut controls = unsafe { get_ptr_to_controls(self.controls_id) };
        let controls = controls.controls_mut();
        let return_to = Self {
            controls_id: self.controls_id,
            start_button: Some(index),
        };
        match index {
            Self::EAST => Some(Box::new(ButtonSelector {
                controls_id: self.controls_id,
                return_to: Box::new(move || Some(Box::new(return_to))),
                initial: controls.joy_right,
                set_input_kind: |ctrls, input| ctrls.joy_right = input,
            })),
            Self::SOUTH => Some(Box::new(ButtonSelector {
                controls_id: self.controls_id,
                return_to: Box::new(move || Some(Box::new(return_to))),
                initial: controls.joy_down,
                set_input_kind: |ctrls, input| ctrls.joy_down = input,
            })),
            Self::WEST => Some(Box::new(ButtonSelector {
                controls_id: self.controls_id,
                return_to: Box::new(move || Some(Box::new(return_to))),
                initial: controls.joy_left,
                set_input_kind: |ctrls, input| ctrls.joy_left = input,
            })),
            Self::NORTH => Some(Box::new(ButtonSelector {
                controls_id: self.controls_id,
                return_to: Box::new(move || Some(Box::new(return_to))),
                initial: controls.joy_up,
                set_input_kind: |ctrls, input| ctrls.joy_up = input,
            })),
            Self::SL => Some(Box::new(ButtonSelector {
                controls_id: self.controls_id,
                return_to: Box::new(move || Some(Box::new(return_to))),
                initial: controls.joy_sl,
                set_input_kind: |ctrls, input| ctrls.joy_sl = input,
            })),
            Self::SR => Some(Box::new(ButtonSelector {
                controls_id: self.controls_id,
                return_to: Box::new(move || Some(Box::new(return_to))),
                initial: controls.joy_sr,
                set_input_kind: |ctrls, input| ctrls.joy_sr = input,
            })),
            Self::BUMPER => Some(Box::new(ButtonSelector {
                controls_id: self.controls_id,
                return_to: Box::new(move || Some(Box::new(return_to))),
                initial: controls.joy_shoulder,
                set_input_kind: |ctrls, input| ctrls.joy_shoulder = input,
            })),
            Self::TRIGGER => Some(Box::new(ButtonSelector {
                controls_id: self.controls_id,
                return_to: Box::new(move || Some(Box::new(return_to))),
                initial: controls.joy_zshoulder,
                set_input_kind: |ctrls, input| ctrls.joy_zshoulder = input,
            })),
            Self::TAP_JUMP => {
                let mut controls = unsafe { get_ptr_to_controls(self.controls_id) };
                controls.controls_mut().pro_tapjump = !controls.controls_mut().pro_tapjump;
                Some(Box::new(Self {
                    controls_id: self.controls_id,
                    start_button: Some(index),
                }))
            }
            Self::PARRY_INPUT => {
                let mut controls = unsafe { get_ptr_to_controls(self.controls_id) };
                if controls.controls_mut().pro_absmash & 2 != 0 {
                    controls.controls_mut().pro_absmash &= !2;
                } else {
                    controls.controls_mut().pro_absmash |= 2;
                }
                Some(Box::new(Self {
                    controls_id: self.controls_id,
                    start_button: Some(index),
                }))
            }
            Self::RIVALS_WJ => {
                let mut controls = unsafe { get_ptr_to_controls(self.controls_id) };
                if controls.controls_mut().pro_absmash & 4 != 0 {
                    controls.controls_mut().pro_absmash &= !4;
                } else {
                    controls.controls_mut().pro_absmash |= 4;
                }
                Some(Box::new(Self {
                    controls_id: self.controls_id,
                    start_button: Some(index),
                }))
            }
            Self::STICK_SENS => {
                let mut controls = unsafe { get_ptr_to_controls(self.controls_id) };
                controls.controls_mut().pro_sensitivity =
                    (controls.controls_mut().pro_sensitivity + 1) % 3;
                Some(Box::new(Self {
                    controls_id: self.controls_id,
                    start_button: Some(index),
                }))
            }
            Self::RUMBLE => {
                let mut controls = unsafe { get_ptr_to_controls(self.controls_id) };
                controls.controls_mut().pro_rumble = !controls.controls_mut().pro_rumble;
                Some(Box::new(Self {
                    controls_id: self.controls_id,
                    start_button: Some(index),
                }))
            }
            Self::AB_SMASH => {
                let mut controls = unsafe { get_ptr_to_controls(self.controls_id) };
                if controls.controls_mut().pro_absmash & 1 != 0 {
                    controls.controls_mut().pro_absmash &= !1;
                } else {
                    controls.controls_mut().pro_absmash |= 1;
                }
                Some(Box::new(Self {
                    controls_id: self.controls_id,
                    start_button: Some(index),
                }))
            }
            _ => Some(Box::new(self.clone())),
        }
    }

    fn get_title(&self) -> &'static [u8] {
        b"Single JoyCon"
    }

    fn get_button_text(&self, button_index: usize) -> Option<Vec<u8>> {
        let button_name: &[u8] = match button_index {
            Self::EAST => b"East",
            Self::SOUTH => b"South",
            Self::WEST => b"West",
            Self::NORTH => b"North",
            Self::SL => b"SL",
            Self::SR => b"SR",
            Self::BUMPER => b"Bumper",
            Self::TRIGGER => b"Trigger",
            Self::TAP_JUMP => {
                let mut controls = unsafe { get_ptr_to_controls(self.controls_id) };
                if controls.controls_mut().pro_tapjump {
                    b"Tap Jump: On"
                } else {
                    b"Tap Jump: Off"
                }
            }
            Self::PARRY_INPUT => {
                let mut controls = unsafe { get_ptr_to_controls(self.controls_id) };
                if controls.controls_mut().pro_absmash & 2 != 0 {
                    b"Parry Input: Taunt"
                } else {
                    b"Parry Input: Shield"
                }
            }
            Self::RIVALS_WJ => {
                let mut controls = unsafe { get_ptr_to_controls(self.controls_id) };
                if controls.controls_mut().pro_absmash & 4 != 0 {
                    b"Walljump: Button"
                } else {
                    b"Walljump: Flick"
                }
            }
            Self::STICK_SENS => {
                let mut controls = unsafe { get_ptr_to_controls(self.controls_id) };
                match controls.controls_mut().pro_sensitivity {
                    0 => b"Sensitivity: Low",
                    1 => b"Sensitivity: Med",
                    _ => b"Sensitivity: High",
                }
            }
            Self::RUMBLE => {
                let mut controls = unsafe { get_ptr_to_controls(self.controls_id) };
                if controls.controls_mut().pro_rumble {
                    b"Rumble: On"
                } else {
                    b"Rumble: Off"
                }
            }
            Self::AB_SMASH => {
                let mut controls = unsafe { get_ptr_to_controls(self.controls_id) };
                if controls.controls_mut().pro_absmash & 1 != 0 {
                    b"A+B Smash: On"
                } else {
                    b"A+B Smash: Off"
                }
            }
            _ => return None,
        };

        Some(button_name.to_vec())
    }
}

#[derive(Copy, Clone)]
pub struct ShortcutsMenu {
    controls_id: usize,
    start_button: Option<usize>,
}

impl ShortcutsMenu {
    const NO_TAP: usize = 0;
    const NO_RUMBLE: usize = 1;
    const NO_AB_SMASH: usize = 2;
    const TILT_STICK: usize = 3;
    const L_JUMP: usize = 4;
    const L_PARRY: usize = 5;
    const COUNT: usize = 6;
}

impl TagSubMenu for ShortcutsMenu {
    fn decide(&self, index: usize) -> Option<Box<dyn TagSubMenu>> {
        let this = Self {
            controls_id: self.controls_id,
            start_button: Some(index),
        };

        let mut controls = unsafe { get_ptr_to_controls(self.controls_id) };
        let controls = controls.controls_mut();

        match index {
            Self::NO_TAP => {
                controls.gc_tapjump = false;
                controls.pro_tapjump = false;
                controls.joy_tapjump = false;
            }
            Self::NO_RUMBLE => {
                controls.gc_rumble = false;
                controls.pro_rumble = false;
                controls.joy_rumble = false;
            }
            Self::NO_AB_SMASH => {
                controls.gc_absmash &= !1;
                controls.pro_absmash &= !1;
                controls.joy_absmash &= !1;
            }
            Self::TILT_STICK => {
                controls.gc_cstick = InputKind::Attack;
                controls.pro_cstick = InputKind::Attack;
            }
            Self::L_JUMP => {
                controls.gc_l = InputKind::Jump;
                controls.pro_zl = InputKind::Jump;
            }
            Self::L_PARRY => {
                controls.gc_l = InputKind::Parry;
                controls.pro_zl = InputKind::Parry;
            }
            _ => {}
        }

        Some(Box::new(this))
    }

    fn cancel(&self) -> Option<Box<dyn TagSubMenu>> {
        Some(Box::new(TopLevel {
            controls_id: self.controls_id,
        }))
    }

    fn get_title(&self) -> &'static [u8] {
        b"Shortcuts"
    }

    fn get_button_count(&self) -> usize {
        Self::COUNT
    }

    fn get_button_text(&self, button_index: usize) -> Option<Vec<u8>> {
        let name: &[u8] = match button_index {
            Self::NO_TAP => b"No Tap",
            Self::NO_RUMBLE => b"No Rumble",
            Self::NO_AB_SMASH => b"No AB Smash",
            Self::TILT_STICK => b"Tilt Stick",
            Self::L_JUMP => b"L/ZL Jump",
            Self::L_PARRY => b"L/ZL Parry",
            _ => return None,
        };

        Some(name.to_vec())
    }

    fn get_start_index(&self) -> Option<usize> {
        self.start_button
    }
}
