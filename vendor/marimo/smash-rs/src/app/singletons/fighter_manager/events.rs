use super::*;
use std::ops::{Deref, DerefMut};

/// The event IDs that the game uses internally for it's fighter events
#[repr(u32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum FighterEventID {
    ChangeTagVisibility = 0xF,
    UIDamageUpdate = 0x11,
    JackUpdateRebelGauge = 0x58,
    BraveUpdateMenu = 0x5F,
    BraveUpdateMenu2 = 0x60,
    BraveUpdateMenu3 = 0x61,
    BraveUpdateMenu4 = 0x63,
}

macro_rules! impl_event {
    ($name:tt, $id:tt) => {
        #[sealed::sealed]
        impl FighterEventInheriter for $name {
            fn get_fighter_event_id() -> FighterEventID {
                FighterEventID::$id
            }
        }

        impl Deref for $name {
            type Target = FighterEvent;

            fn deref(&self) -> &Self::Target {
                &self.base
            }
        }

        impl DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.base
            }
        }

        impl AsRef<FighterEvent> for $name {
            fn as_ref(&self) -> &FighterEvent {
                &self.base
            }
        }

        impl AsMut<FighterEvent> for $name {
            fn as_mut(&mut self) -> &mut FighterEvent {
                &mut self.base
            }
        }
    };
    ($(($name:tt, $id:tt))*) => {
        $(
            impl_event!($name, $id);
        )*
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct ChangeTagVisibilityEvent {
    base: FighterEvent,
    pub is_visible: bool,
    pub should_fade: bool
}

impl ChangeTagVisibilityEvent {
    pub fn new(entry_id: app::FighterEntryID, is_visible: bool, should_fade: bool) -> Self {
        Self {
            base: FighterEvent::new(FighterEventID::ChangeTagVisibility, entry_id),
            is_visible,
            should_fade
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct UIDamageUpdateEvent {
    base: FighterEvent,
    pub total_damage: f32,
    pub change_in_damage: f32,
    pub is_increase: bool,
    pub is_decrease: bool
}

impl UIDamageUpdateEvent {
    pub fn new(entry_id: app::FighterEntryID, total_damage: f32, change_in_damage: f32, is_damage: bool, is_heal: bool) -> Self {
        Self {
            base: FighterEvent::new(FighterEventID::UIDamageUpdate, entry_id),
            total_damage,
            change_in_damage,
            is_increase: is_damage,
            is_decrease: is_heal
        }
    }

    pub fn is_absolute(&self) -> bool {
        !self.is_increase && !self.is_decrease
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct JackUpdateRebelGaugeEvent {
    base: FighterEvent,
    pub ratio: f32
}

impl JackUpdateRebelGaugeEvent {
    pub fn new(entry_id: app::FighterEntryID, ratio: f32) -> Self {
        Self {
            base: FighterEvent::new(FighterEventID::JackUpdateRebelGauge, entry_id),
            ratio
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct BraveSetMenuCommand {    // set command for slot
    base: FighterEvent,
    menu_entry: u32,            // 1-indexed
    command_id: i32,            // id of command
    mp_cost: f32
}

#[repr(C)]
#[derive(Debug)]
pub struct BraveEnableMenuCommand {   // set if command is enabled
    base: FighterEvent,
    menu_entry: u32,            // 1-indexed
    command_enabled: bool       // if command is enabled
}

#[repr(C)]
#[derive(Debug)]
pub struct BraveShowMenu {   // display menu
    base: FighterEvent,
}

#[repr(C)]
#[derive(Debug)]
pub struct BraveSetMenuSelectedCommand {   // set the selected entry on menu open
    base: FighterEvent,
    menu_entry: u32,
}

impl BraveSetMenuCommand {
    pub fn new(entry_id: app::FighterEntryID, menu_entry: u32, command_id: i32, mp_cost: f32) -> Self {
        Self {
            base: FighterEvent::new(FighterEventID::BraveUpdateMenu, entry_id),
            menu_entry,
            command_id,
            mp_cost
        }
    }
}

impl BraveEnableMenuCommand {
    pub fn new(entry_id: app::FighterEntryID, menu_entry: u32, command_enabled: bool) -> Self {
        Self {
            base: FighterEvent::new(FighterEventID::BraveUpdateMenu2, entry_id),
            menu_entry,
            command_enabled
        }
    }
}

impl BraveShowMenu {
    pub fn new(entry_id: app::FighterEntryID) -> Self {
        Self {
            base: FighterEvent::new(FighterEventID::BraveUpdateMenu3, entry_id)
        }
    }
}

impl BraveSetMenuSelectedCommand {
    pub fn new(entry_id: app::FighterEntryID, menu_entry: u32) -> Self {
        Self {
            base: FighterEvent::new(FighterEventID::BraveUpdateMenu4, entry_id),
            menu_entry
        }
    }
}

impl_event!(
    (JackUpdateRebelGaugeEvent, JackUpdateRebelGauge)
    (UIDamageUpdateEvent, UIDamageUpdate)
    (ChangeTagVisibilityEvent, ChangeTagVisibility)
    (BraveSetMenuCommand, BraveUpdateMenu)
    (BraveEnableMenuCommand, BraveUpdateMenu2)
    (BraveShowMenu, BraveUpdateMenu3)
    (BraveSetMenuSelectedCommand, BraveUpdateMenu4)
);