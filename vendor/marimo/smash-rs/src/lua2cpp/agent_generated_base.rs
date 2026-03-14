use std::ops::{Deref, DerefMut};

use crate::*;

#[repr(C)]
pub struct L2CAgentGeneratedBase {
    agent_base: lua2cpp::L2CAgentBase
}

impl Deref for L2CAgentGeneratedBase {
    type Target = lua2cpp::L2CAgentBase;

    fn deref(&self) -> &Self::Target {
        &self.agent_base
    }
}

impl DerefMut for L2CAgentGeneratedBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.agent_base
    }
}

impl L2CAgentGeneratedBase {
    pub fn construct_bind_const_value_table() {
        unsafe {
            construct_bind_const_value_table()
        }
    }
}

#[cfg(feature = "type_assert")]
impl L2CAgentGeneratedBase {
    pub fn assert() {
        assert_eq!(size_of!(L2CAgentGeneratedBase), 0xC8);
        assert_eq!(offset_of!(L2CAgentGeneratedBase, agent_base), 0x0);
    }
}

extern "C" {
    #[link_name = "_ZN7lua2cpp21L2CAgentGeneratedBase32construct_bind_const_value_tableEv"]
    fn construct_bind_const_value_table();
}