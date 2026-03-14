use std::ops::{Deref, DerefMut};

use crate::*;

use super::cpp::*;

#[repr(C)]
pub struct L2CFighterAnimcmdExpressionCommon {
    agent_base: lua2cpp::L2CAgentBase
}

impl Deref for L2CFighterAnimcmdExpressionCommon {
    type Target = lua2cpp::L2CAgentBase;

    fn deref(&self) -> &Self::Target {
        &self.agent_base
    }
}

impl DerefMut for L2CFighterAnimcmdExpressionCommon {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.agent_base
    }
}

impl L2CFighterAnimcmdExpressionCommon {
    pub fn expression_TrailHoleHitRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_TrailHoleHitRumble(self).into()
        }
    }

    pub fn expression_TrailBeamHitRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_TrailBeamHitRumble(self).into()
        }
    }

    pub fn expression_TrailThunderHitRumble3(&mut self) -> lib::L2CValue {
        unsafe {
            expression_TrailThunderHitRumble3(self).into()
        }
    }

    pub fn expression_TrailThunderHitRumble2(&mut self) -> lib::L2CValue {
        unsafe {
            expression_TrailThunderHitRumble2(self).into()
        }
    }

    pub fn expression_TrailThunderHitRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_TrailThunderHitRumble(self).into()
        }
    }

    pub fn expression_TrailIceHitRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_TrailIceHitRumble(self).into()
        }
    }

    pub fn expression_TrailFireHitRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_TrailFireHitRumble(self).into()
        }
    }

    pub fn expression_GetSpParts3Rumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_GetSpParts3Rumble(self).into()
        }
    }

    pub fn expression_GetSpParts2Rumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_GetSpParts2Rumble(self).into()
        }
    }

    pub fn expression_GetSpParts1Rumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_GetSpParts1Rumble(self).into()
        }
    }

    pub fn expression_SuperLeafFallRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_SuperLeafFallRumble(self).into()
        }
    }

    pub fn expression_AvailableFinalRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_AvailableFinalRumble(self).into()
        }
    }

    pub fn expression_KillerJumpRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_KillerJumpRumble(self).into()
        }
    }

    pub fn expression_KillerRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_KillerRumble(self).into()
        }
    }

    pub fn expression_BuryRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_BuryRumble(self).into()
        }
    }

    pub fn expression_UsePowBlockRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_UsePowBlockRumble(self).into()
        }
    }

    pub fn expression_GetSuperStarRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_GetSuperStarRumble(self).into()
        }
    }

    pub fn expression_GetMetalRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_GetMetalRumble(self).into()
        }
    }

    pub fn expression_EndSmallRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_EndSmallRumble(self).into()
        }
    }

    pub fn expression_EndBigRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_EndBigRumble(self).into()
        }
    }

    pub fn expression_GetSmallRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_GetSmallRumble(self).into()
        }
    }

    pub fn expression_GetBigRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_GetBigRumble(self).into()
        }
    }

    pub fn expression_SwimDiveRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_SwimDiveRumble(self).into()
        }
    }

    pub fn expression_CameraHitRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_CameraHitRumble(self).into()
        }
    }

    pub fn expression_StarFallEndRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_StarFallEndRumble(self).into()
        }
    }

    pub fn expression_StarFallRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_StarFallRumble(self).into()
        }
    }

    pub fn expression_DeadRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_DeadRumble(self).into()
        }
    }

    pub fn expression_RocketBeltHoverRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_RocketBeltHoverRumble(self).into()
        }
    }

    pub fn expression_BarrelScrewRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_BarrelScrewRumble(self).into()
        }
    }

    pub fn expression_ScrewRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_ScrewRumble(self).into()
        }
    }

    pub fn expression_WarpStarRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_WarpStarRumble(self).into()
        }
    }

    pub fn expression_ItemShootEmptyRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_ItemShootEmptyRumble(self).into()
        }
    }

    pub fn expression_FireCurryRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_FireCurryRumble(self).into()
        }
    }

    pub fn expression_FireSuperScopeRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_FireSuperScopeRumble(self).into()
        }
    }

    pub fn expression_ChargeSuperScopeRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_ChargeSuperScopeRumble(self).into()
        }
    }

    pub fn expression_FireMagicPotRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_FireMagicPotRumble(self).into()
        }
    }

    pub fn expression_FireFireFlowerRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_FireFireFlowerRumble(self).into()
        }
    }

    pub fn expression_FireDrillRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_FireDrillRumble(self).into()
        }
    }

    pub fn expression_FireSuperScopeRapidRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_FireSuperScopeRapidRumble(self).into()
        }
    }

    pub fn expression_FireSteelDiverRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_FireSteelDiverRumble(self).into()
        }
    }

    pub fn expression_FireStaffRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_FireStaffRumble(self).into()
        }
    }

    pub fn expression_FireRevengeShooterRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_FireRevengeShooterRumble(self).into()
        }
    }

    pub fn expression_FireBananaGunRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_FireBananaGunRumble(self).into()
        }
    }

    pub fn expression_FireRayGunRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_FireRayGunRumble(self).into()
        }
    }

    pub fn expression_HummerHitRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_HummerHitRumble(self).into()
        }
    }

    pub fn expression_ItemHeavyThrow4Rumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_ItemHeavyThrow4Rumble(self).into()
        }
    }

    pub fn expression_ItemHeavyThrowRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_ItemHeavyThrowRumble(self).into()
        }
    }

    pub fn expression_ItemLightThrowDashRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_ItemLightThrowDashRumble(self).into()
        }
    }

    pub fn expression_ItemLightThrow4Rumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_ItemLightThrow4Rumble(self).into()
        }
    }

    pub fn expression_ItemLightThrowRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_ItemLightThrowRumble(self).into()
        }
    }

    pub fn expression_ItemLightThrowDropRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_ItemLightThrowDropRumble(self).into()
        }
    }

    pub fn expression_FinalCutinRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_FinalCutinRumble(self).into()
        }
    }

    pub fn expression_CatchCutRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_CatchCutRumble(self).into()
        }
    }

    pub fn expression_CatchPullRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_CatchPullRumble(self).into()
        }
    }

    pub fn expression_GimmickPipeRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_GimmickPipeRumble(self).into()
        }
    }

    pub fn expression_CaptureMasterHandRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_CaptureMasterHandRumble(self).into()
        }
    }

    pub fn expression_CaptureNabbitRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_CaptureNabbitRumble(self).into()
        }
    }

    pub fn expression_CaptureGimmckFishRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_CaptureGimmckFishRumble(self).into()
        }
    }

    pub fn expression_DamageFallRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_DamageFallRumble(self).into()
        }
    }

    pub fn expression_CaptureWaitLwRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_CaptureWaitLwRumble(self).into()
        }
    }

    pub fn expression_CaptureWaitHiRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_CaptureWaitHiRumble(self).into()
        }
    }

    pub fn expression_SwimDrownOutRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_SwimDrownOutRumble(self).into()
        }
    }

    pub fn expression_SwimDrownRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_SwimDrownRumble(self).into()
        }
    }

    pub fn expression_AirCatchHitRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_AirCatchHitRumble(self).into()
        }
    }

    pub fn expression_SlipEscapeFRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_SlipEscapeFRumble(self).into()
        }
    }

    pub fn expression_CliffJump2Rumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_CliffJump2Rumble(self).into()
        }
    }

    pub fn expression_CliffCatchRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_CliffCatchRumble(self).into()
        }
    }

    pub fn expression_StopWallRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_StopWallRumble(self).into()
        }
    }

    pub fn expression_SwallowedRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_SwallowedRumble(self).into()
        }
    }

    pub fn expression_SleepEndRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_SleepEndRumble(self).into()
        }
    }

    pub fn expression_SleepLoopRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_SleepLoopRumble(self).into()
        }
    }

    pub fn expression_FuraFuraEndRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_FuraFuraEndRumble(self).into()
        }
    }

    pub fn expression_FuraFuraStartDRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_FuraFuraStartDRumble(self).into()
        }
    }

    pub fn expression_FuraFuraStartURumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_FuraFuraStartURumble(self).into()
        }
    }

    pub fn expression_BindRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_BindRumble(self).into()
        }
    }

    pub fn expression_FuraFuraRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_FuraFuraRumble(self).into()
        }
    }

    pub fn expression_ShieldBreakFlyRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_ShieldBreakFlyRumble(self).into()
        }
    }

    pub fn expression_AttachWallRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_AttachWallRumble(self).into()
        }
    }

    pub fn expression_PassiveWallJumpRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_PassiveWallJumpRumble(self).into()
        }
    }

    pub fn expression_PassiveRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_PassiveRumble(self).into()
        }
    }

    pub fn expression_DownForwardURumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_DownForwardURumble(self).into()
        }
    }

    pub fn expression_DownBoundURumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_DownBoundURumble(self).into()
        }
    }

    pub fn expression_WallDamageRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_WallDamageRumble(self).into()
        }
    }

    pub fn expression_CaptureCutRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_CaptureCutRumble(self).into()
        }
    }

    pub fn expression_CapturePulledHiRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_CapturePulledHiRumble(self).into()
        }
    }

    pub fn expression_DamageFlyMeteorRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_DamageFlyMeteorRumble(self).into()
        }
    }

    pub fn expression_DamageFlyRollRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_DamageFlyRollRumble(self).into()
        }
    }

    pub fn expression_DamageFlyTopRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_DamageFlyTopRumble(self).into()
        }
    }

    pub fn expression_DamageFlyLwRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_DamageFlyLwRumble(self).into()
        }
    }

    pub fn expression_DamageFlyNRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_DamageFlyNRumble(self).into()
        }
    }

    pub fn expression_DamageFlyHiRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_DamageFlyHiRumble(self).into()
        }
    }

    pub fn expression_DamageFlyLRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_DamageFlyLRumble(self).into()
        }
    }

    pub fn expression_DamageFlyMRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_DamageFlyMRumble(self).into()
        }
    }

    pub fn expression_JustGuardRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_JustGuardRumble(self).into()
        }
    }

    pub fn expression_GuardLv5Rumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_GuardLv5Rumble(self).into()
        }
    }

    pub fn expression_GuardLv4Rumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_GuardLv4Rumble(self).into()
        }
    }

    pub fn expression_GuardLv3Rumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_GuardLv3Rumble(self).into()
        }
    }

    pub fn expression_GuardLv2Rumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_GuardLv2Rumble(self).into()
        }
    }

    pub fn expression_GuardLv1Rumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_GuardLv1Rumble(self).into()
        }
    }

    pub fn expression_Guard100Rumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_Guard100Rumble(self).into()
        }
    }

    pub fn expression_DamageLv5Rumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_DamageLv5Rumble(self).into()
        }
    }

    pub fn expression_DamageLv4Rumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_DamageLv4Rumble(self).into()
        }
    }

    pub fn expression_DamageLv3Rumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_DamageLv3Rumble(self).into()
        }
    }

    pub fn expression_DamageLv2Rumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_DamageLv2Rumble(self).into()
        }
    }

    pub fn expression_DamageLv1Rumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_DamageLv1Rumble(self).into()
        }
    }

    pub fn expression_Damage100Rumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_Damage100Rumble(self).into()
        }
    }

    pub fn expression_ReboundRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_ReboundRumble(self).into()
        }
    }

    pub fn expression_GuardOffRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_GuardOffRumble(self).into()
        }
    }

    pub fn expression_GuardOnRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_GuardOnRumble(self).into()
        }
    }

    pub fn expression_StepPoseRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_StepPoseRumble(self).into()
        }
    }

    pub fn expression_SamusEscapeBRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_SamusEscapeBRumble(self).into()
        }
    }

    pub fn expression_SamusEscapeFRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_SamusEscapeFRumble(self).into()
        }
    }

    pub fn expression_EscapeAirSlideRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_EscapeAirSlideRumble(self).into()
        }
    }

    pub fn expression_EscapeAirRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_EscapeAirRumble(self).into()
        }
    }

    pub fn expression_EscapeBRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_EscapeBRumble(self).into()
        }
    }

    pub fn expression_EscapeFRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_EscapeFRumble(self).into()
        }
    }

    pub fn expression_EscapeNRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_EscapeNRumble(self).into()
        }
    }

    pub fn expression_LandingFallSpecialHVRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_LandingFallSpecialHVRumble(self).into()
        }
    }

    pub fn expression_LandingHeavyHVRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_LandingHeavyHVRumble(self).into()
        }
    }

    pub fn expression_LandingLightHVRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_LandingLightHVRumble(self).into()
        }
    }

    pub fn expression_LandingFallSpecialRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_LandingFallSpecialRumble(self).into()
        }
    }

    pub fn expression_LandingHeavyRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_LandingHeavyRumble(self).into()
        }
    }

    pub fn expression_LandingLightRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_LandingLightRumble(self).into()
        }
    }

    pub fn expression_StepJumpRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_StepJumpRumble(self).into()
        }
    }

    pub fn expression_JumpAerialF1Rumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_JumpAerialF1Rumble(self).into()
        }
    }

    pub fn expression_JumpAerialFrontRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_JumpAerialFrontRumble(self).into()
        }
    }

    pub fn expression_JumpFrontRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_JumpFrontRumble(self).into()
        }
    }

    pub fn expression_JumpBackRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_JumpBackRumble(self).into()
        }
    }

    pub fn expression_JumpFrontMiniRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_JumpFrontMiniRumble(self).into()
        }
    }

    pub fn expression_JumpBackMiniRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_JumpBackMiniRumble(self).into()
        }
    }

    pub fn expression_JumpAerialBackRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_JumpAerialBackRumble(self).into()
        }
    }

    pub fn expression_JumpAerialF2Rumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_JumpAerialF2Rumble(self).into()
        }
    }

    pub fn expression_JumpAerialF3Rumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_JumpAerialF3Rumble(self).into()
        }
    }

    pub fn expression_JumpAerialF4Rumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_JumpAerialF4Rumble(self).into()
        }
    }

    pub fn expression_JumpAerialF5Rumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_JumpAerialF5Rumble(self).into()
        }
    }

    pub fn expression_StepPoseAirRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_StepPoseAirRumble(self).into()
        }
    }

    pub fn expression_CapturePulledLwRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_CapturePulledLwRumble(self).into()
        }
    }

    pub fn expression_CaptureJumpRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_CaptureJumpRumble(self).into()
        }
    }

    pub fn expression_DownBoundDRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_DownBoundDRumble(self).into()
        }
    }

    pub fn expression_CeilDamageRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_CeilDamageRumble(self).into()
        }
    }

    pub fn expression_DownBackURumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_DownBackURumble(self).into()
        }
    }

    pub fn expression_DownForwardDRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_DownForwardDRumble(self).into()
        }
    }

    pub fn expression_DownBackDRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_DownBackDRumble(self).into()
        }
    }

    pub fn expression_PassiveStandFRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_PassiveStandFRumble(self).into()
        }
    }

    pub fn expression_PassiveStandBRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_PassiveStandBRumble(self).into()
        }
    }

    pub fn expression_PassiveWallRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_PassiveWallRumble(self).into()
        }
    }

    pub fn expression_PassiveCeilRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_PassiveCeilRumble(self).into()
        }
    }

    pub fn expression_StopCeilRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_StopCeilRumble(self).into()
        }
    }

    pub fn expression_SlipEscapeBRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_SlipEscapeBRumble(self).into()
        }
    }

    pub fn expression_FireWalkMashRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_FireWalkMashRumble(self).into()
        }
    }

    pub fn expression_ScrewAirRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_ScrewAirRumble(self).into()
        }
    }

    pub fn expression_GetGoldRumble(&mut self) -> lib::L2CValue {
        unsafe {
            expression_GetGoldRumble(self).into()
        }
    }
}

#[cfg(feature = "type_assert")]
impl L2CFighterAnimcmdExpressionCommon {
    pub fn assert() {
        assert_eq!(size_of!(L2CFighterAnimcmdExpressionCommon), 0xC8);
        assert_eq!(offset_of!(L2CFighterAnimcmdExpressionCommon, agent_base), 0x0);
    }
}