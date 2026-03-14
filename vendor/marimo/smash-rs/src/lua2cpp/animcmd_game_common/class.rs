use std::ops::{Deref, DerefMut};

use crate::*;

use super::cpp::*;

#[repr(C)]
pub struct L2CFighterAnimcmdGameCommon {
    agent_base: lua2cpp::L2CAgentBase
}

impl Deref for L2CFighterAnimcmdGameCommon {
    type Target = lua2cpp::L2CAgentBase;

    fn deref(&self) -> &Self::Target {
        &self.agent_base
    }
}

impl DerefMut for L2CFighterAnimcmdGameCommon {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.agent_base
    }
}

impl L2CFighterAnimcmdGameCommon {
    pub fn game_TrailSpecialAirLwBackward(&mut self) -> lib::L2CValue {
        unsafe {
            game_TrailSpecialAirLwBackward(self).into()
        }
    }

    pub fn game_JumpBoardJumpCommonL(&mut self) -> lib::L2CValue {
        unsafe {
            game_JumpBoardJumpCommonL(self).into()
        }
    }

    pub fn game_JumpBoardJumpCommonM(&mut self) -> lib::L2CValue {
        unsafe {
            game_JumpBoardJumpCommonM(self).into()
        }
    }

    pub fn game_JumpBoardJumpCommonS(&mut self) -> lib::L2CValue {
        unsafe {
            game_JumpBoardJumpCommonS(self).into()
        }
    }

    pub fn game_SuicideBomb(&mut self) -> lib::L2CValue {
        unsafe {
            game_SuicideBomb(self).into()
        }
    }

    pub fn game_JustShieldBomber(&mut self) -> lib::L2CValue {
        unsafe {
            game_JustShieldBomber(self).into()
        }
    }

    pub fn game_HighSpeedDash(&mut self) -> lib::L2CValue {
        unsafe {
            game_HighSpeedDash(self).into()
        }
    }

    pub fn game_LuigiFinalShootIndirectCommon(&mut self) -> lib::L2CValue {
        unsafe {
            game_LuigiFinalShootIndirectCommon(self).into()
        }
    }

    pub fn game_LuigiFinalShootIndirectNoReactionCommon(&mut self) -> lib::L2CValue {
        unsafe {
            game_LuigiFinalShootIndirectNoReactionCommon(self).into()
        }
    }

    pub fn game_CaptureCutCommon(&mut self) -> lib::L2CValue {
        unsafe {
            game_CaptureCutCommon(self).into()
        }
    }

    pub fn game_StarShotCommon(&mut self) -> lib::L2CValue {
        unsafe {
            game_StarShotCommon(self).into()
        }
    }

    pub fn game_ScrewCommon(&mut self) -> lib::L2CValue {
        unsafe {
            game_ScrewCommon(self).into()
        }
    }

    pub fn game_RevengeshooterShootUpperCommon(&mut self) -> lib::L2CValue {
        unsafe {
            game_RevengeshooterShootUpperCommon(self).into()
        }
    }

    pub fn game_StaffShootUpperCommon(&mut self) -> lib::L2CValue {
        unsafe {
            game_StaffShootUpperCommon(self).into()
        }
    }

    pub fn game_DeathscytheSwingDashCommon2(&mut self) -> lib::L2CValue {
        unsafe {
            game_DeathscytheSwingDashCommon2(self).into()
        }
    }

    pub fn game_DeathscytheSwingDashCommon(&mut self) -> lib::L2CValue {
        unsafe {
            game_DeathscytheSwingDashCommon(self).into()
        }
    }

    pub fn game_DeathscytheSwing4Charge(&mut self) -> lib::L2CValue {
        unsafe {
            game_DeathscytheSwing4Charge(self).into()
        }
    }

    pub fn game_DeathscytheSwing4Common(&mut self) -> lib::L2CValue {
        unsafe {
            game_DeathscytheSwing4Common(self).into()
        }
    }

    pub fn game_DeathscytheSwing3Common(&mut self) -> lib::L2CValue {
        unsafe {
            game_DeathscytheSwing3Common(self).into()
        }
    }

    pub fn game_DeathscytheSwing1Common(&mut self) -> lib::L2CValue {
        unsafe {
            game_DeathscytheSwing1Common(self).into()
        }
    }

    pub fn game_KillSwordSwingDashCommon2(&mut self) -> lib::L2CValue {
        unsafe {
            game_KillSwordSwingDashCommon2(self).into()
        }
    }

    pub fn game_KillSwordSwingDashCommon(&mut self) -> lib::L2CValue {
        unsafe {
            game_KillSwordSwingDashCommon(self).into()
        }
    }

    pub fn game_SwordSwingDashCommon2(&mut self) -> lib::L2CValue {
        unsafe {
            game_SwordSwingDashCommon2(self).into()
        }
    }

    pub fn game_KillSwordSwing4CommonCharge(&mut self) -> lib::L2CValue {
        unsafe {
            game_KillSwordSwing4CommonCharge(self).into()
        }
    }

    pub fn game_KillSwordSwing4Common(&mut self) -> lib::L2CValue {
        unsafe {
            game_KillSwordSwing4Common(self).into()
        }
    }

    pub fn game_KillSwordSwing3Common(&mut self) -> lib::L2CValue {
        unsafe {
            game_KillSwordSwing3Common(self).into()
        }
    }

    pub fn game_KillSwordSwing1Common(&mut self) -> lib::L2CValue {
        unsafe {
            game_KillSwordSwing1Common(self).into()
        }
    }

    pub fn game_FirebarSwingDashCommon2(&mut self) -> lib::L2CValue {
        unsafe {
            game_FirebarSwingDashCommon2(self).into()
        }
    }

    pub fn game_FirebarSwingDashCommon(&mut self) -> lib::L2CValue {
        unsafe {
            game_FirebarSwingDashCommon(self).into()
        }
    }

    pub fn game_FirebarSwing4Common(&mut self) -> lib::L2CValue {
        unsafe {
            game_FirebarSwing4Common(self).into()
        }
    }

    pub fn game_FirebarSwing3Common(&mut self) -> lib::L2CValue {
        unsafe {
            game_FirebarSwing3Common(self).into()
        }
    }

    pub fn game_FirebarSwing1Common(&mut self) -> lib::L2CValue {
        unsafe {
            game_FirebarSwing1Common(self).into()
        }
    }

    pub fn game_ClubSwingDashCommon2(&mut self) -> lib::L2CValue {
        unsafe {
            game_ClubSwingDashCommon2(self).into()
        }
    }

    pub fn game_ClubSwingDashCommon(&mut self) -> lib::L2CValue {
        unsafe {
            game_ClubSwingDashCommon(self).into()
        }
    }

    pub fn game_ClubSwing4Common(&mut self) -> lib::L2CValue {
        unsafe {
            game_ClubSwing4Common(self).into()
        }
    }

    pub fn game_ClubSwing3Common(&mut self) -> lib::L2CValue {
        unsafe {
            game_ClubSwing3Common(self).into()
        }
    }

    pub fn game_ClubSwing1Common(&mut self) -> lib::L2CValue {
        unsafe {
            game_ClubSwing1Common(self).into()
        }
    }

    pub fn game_LipStickSwingDashCommon2(&mut self) -> lib::L2CValue {
        unsafe {
            game_LipStickSwingDashCommon2(self).into()
        }
    }

    pub fn game_LipStickSwingDashCommon(&mut self) -> lib::L2CValue {
        unsafe {
            game_LipStickSwingDashCommon(self).into()
        }
    }

    pub fn game_LipStickSwing4Common(&mut self) -> lib::L2CValue {
        unsafe {
            game_LipStickSwing4Common(self).into()
        }
    }

    pub fn game_LipStickSwing3Common(&mut self) -> lib::L2CValue {
        unsafe {
            game_LipStickSwing3Common(self).into()
        }
    }

    pub fn game_LipStickSwing1Common(&mut self) -> lib::L2CValue {
        unsafe {
            game_LipStickSwing1Common(self).into()
        }
    }

    pub fn game_StarRodSwingDashCommon2(&mut self) -> lib::L2CValue {
        unsafe {
            game_StarRodSwingDashCommon2(self).into()
        }
    }

    pub fn game_StarRodSwingDashCommon(&mut self) -> lib::L2CValue {
        unsafe {
            game_StarRodSwingDashCommon(self).into()
        }
    }

    pub fn game_StarRodSwing4Common(&mut self) -> lib::L2CValue {
        unsafe {
            game_StarRodSwing4Common(self).into()
        }
    }

    pub fn game_StarRodSwing3Common(&mut self) -> lib::L2CValue {
        unsafe {
            game_StarRodSwing3Common(self).into()
        }
    }

    pub fn game_StarRodSwing1Common(&mut self) -> lib::L2CValue {
        unsafe {
            game_StarRodSwing1Common(self).into()
        }
    }

    pub fn game_BatSwingDashCommon2(&mut self) -> lib::L2CValue {
        unsafe {
            game_BatSwingDashCommon2(self).into()
        }
    }

    pub fn game_BatSwingDashCommon(&mut self) -> lib::L2CValue {
        unsafe {
            game_BatSwingDashCommon(self).into()
        }
    }

    pub fn game_BatSwing4Common2(&mut self) -> lib::L2CValue {
        unsafe {
            game_BatSwing4Common2(self).into()
        }
    }

    pub fn game_BatSwing4Common1(&mut self) -> lib::L2CValue {
        unsafe {
            game_BatSwing4Common1(self).into()
        }
    }

    pub fn game_BatSwing3Common(&mut self) -> lib::L2CValue {
        unsafe {
            game_BatSwing3Common(self).into()
        }
    }

    pub fn game_BatSwing1Common(&mut self) -> lib::L2CValue {
        unsafe {
            game_BatSwing1Common(self).into()
        }
    }

    pub fn game_SwordSwingDashCommon(&mut self) -> lib::L2CValue {
        unsafe {
            game_SwordSwingDashCommon(self).into()
        }
    }

    pub fn game_SwordSwing4Common(&mut self) -> lib::L2CValue {
        unsafe {
            game_SwordSwing4Common(self).into()
        }
    }

    pub fn game_SwordSwing3Common(&mut self) -> lib::L2CValue {
        unsafe {
            game_SwordSwing3Common(self).into()
        }
    }

    pub fn game_SwordSwing1Common(&mut self) -> lib::L2CValue {
        unsafe {
            game_SwordSwing1Common(self).into()
        }
    }

    pub fn game_StaffShootAirCommon(&mut self) {
        unsafe {
            game_StaffShootAirCommon(self)
        }
    }

    pub fn game_RevengeshooterShootAirCommon(&mut self) {
        unsafe {
            game_RevengeshooterShootAirCommon(self)
        }
    }
}

#[cfg(feature = "type_assert")]
impl L2CFighterAnimcmdGameCommon {
    pub fn assert() {
        assert_eq!(size_of!(L2CFighterAnimcmdGameCommon), 0xC8);
        assert_eq!(offset_of!(L2CFighterAnimcmdGameCommon, agent_base), 0x0);
    }
}