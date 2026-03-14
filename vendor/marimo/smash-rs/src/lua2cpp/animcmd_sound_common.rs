use std::ops::{Deref, DerefMut};

use crate::*;

#[repr(C)]
pub struct L2CFighterAnimcmdSoundCommon {
    agent_base: lua2cpp::L2CAgentBase
}

impl Deref for L2CFighterAnimcmdSoundCommon {
    type Target = lua2cpp::L2CAgentBase;

    fn deref(&self) -> &Self::Target {
        &self.agent_base
    }
}

impl DerefMut for L2CFighterAnimcmdSoundCommon {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.agent_base
    }
}

impl L2CFighterAnimcmdSoundCommon {
    #[allow(non_snake_case)]
    pub fn sound_CapturePulledWiifitHi(&mut self) -> lib::L2CValue {
        unsafe {
            sound_CapturePulledWiifitHi(self).into()
        }
    }
}

#[cfg(feature = "type_assert")]
impl L2CFighterAnimcmdSoundCommon {
    pub fn assert() {
        assert_eq!(size_of!(L2CFighterAnimcmdSoundCommon), 0xC8);
        assert_eq!(offset_of!(L2CFighterAnimcmdSoundCommon, agent_base), 0x0);
    }
}

extern "C" {
    #[link_name = "_ZN7lua2cpp28L2CFighterAnimcmdSoundCommon27sound_CapturePulledWiifitHiEv"]
    fn sound_CapturePulledWiifitHi(this: *mut L2CFighterAnimcmdSoundCommon) -> lib::L2CValueHack;
}