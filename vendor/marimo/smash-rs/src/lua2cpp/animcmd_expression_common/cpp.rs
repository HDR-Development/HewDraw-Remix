use crate::*;

use super::class::*;

extern "C" {
    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon29expression_TrailHoleHitRumbleEv"]
    pub(super) fn expression_TrailHoleHitRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon29expression_TrailBeamHitRumbleEv"]
    pub(super) fn expression_TrailBeamHitRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon33expression_TrailThunderHitRumble3Ev"]
    pub(super) fn expression_TrailThunderHitRumble3(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon33expression_TrailThunderHitRumble2Ev"]
    pub(super) fn expression_TrailThunderHitRumble2(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon32expression_TrailThunderHitRumbleEv"]
    pub(super) fn expression_TrailThunderHitRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon28expression_TrailIceHitRumbleEv"]
    pub(super) fn expression_TrailIceHitRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon29expression_TrailFireHitRumbleEv"]
    pub(super) fn expression_TrailFireHitRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon28expression_GetSpParts3RumbleEv"]
    pub(super) fn expression_GetSpParts3Rumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon28expression_GetSpParts2RumbleEv"]
    pub(super) fn expression_GetSpParts2Rumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon28expression_GetSpParts1RumbleEv"]
    pub(super) fn expression_GetSpParts1Rumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon30expression_SuperLeafFallRumbleEv"]
    pub(super) fn expression_SuperLeafFallRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon31expression_AvailableFinalRumbleEv"]
    pub(super) fn expression_AvailableFinalRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon27expression_KillerJumpRumbleEv"]
    pub(super) fn expression_KillerJumpRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon23expression_KillerRumbleEv"]
    pub(super) fn expression_KillerRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon21expression_BuryRumbleEv"]
    pub(super) fn expression_BuryRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon28expression_UsePowBlockRumbleEv"]
    pub(super) fn expression_UsePowBlockRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon29expression_GetSuperStarRumbleEv"]
    pub(super) fn expression_GetSuperStarRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon25expression_GetMetalRumbleEv"]
    pub(super) fn expression_GetMetalRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon25expression_EndSmallRumbleEv"]
    pub(super) fn expression_EndSmallRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon23expression_EndBigRumbleEv"]
    pub(super) fn expression_EndBigRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon25expression_GetSmallRumbleEv"]
    pub(super) fn expression_GetSmallRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon23expression_GetBigRumbleEv"]
    pub(super) fn expression_GetBigRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon25expression_SwimDiveRumbleEv"]
    pub(super) fn expression_SwimDiveRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon26expression_CameraHitRumbleEv"]
    pub(super) fn expression_CameraHitRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon28expression_StarFallEndRumbleEv"]
    pub(super) fn expression_StarFallEndRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon25expression_StarFallRumbleEv"]
    pub(super) fn expression_StarFallRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon21expression_DeadRumbleEv"]
    pub(super) fn expression_DeadRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon32expression_RocketBeltHoverRumbleEv"]
    pub(super) fn expression_RocketBeltHoverRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon28expression_BarrelScrewRumbleEv"]
    pub(super) fn expression_BarrelScrewRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon22expression_ScrewRumbleEv"]
    pub(super) fn expression_ScrewRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon25expression_WarpStarRumbleEv"]
    pub(super) fn expression_WarpStarRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon31expression_ItemShootEmptyRumbleEv"]
    pub(super) fn expression_ItemShootEmptyRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon26expression_FireCurryRumbleEv"]
    pub(super) fn expression_FireCurryRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon31expression_FireSuperScopeRumbleEv"]
    pub(super) fn expression_FireSuperScopeRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon33expression_ChargeSuperScopeRumbleEv"]
    pub(super) fn expression_ChargeSuperScopeRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon29expression_FireMagicPotRumbleEv"]
    pub(super) fn expression_FireMagicPotRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon31expression_FireFireFlowerRumbleEv"]
    pub(super) fn expression_FireFireFlowerRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon26expression_FireDrillRumbleEv"]
    pub(super) fn expression_FireDrillRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon36expression_FireSuperScopeRapidRumbleEv"]
    pub(super) fn expression_FireSuperScopeRapidRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon31expression_FireSteelDiverRumbleEv"]
    pub(super) fn expression_FireSteelDiverRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon26expression_FireStaffRumbleEv"]
    pub(super) fn expression_FireStaffRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon35expression_FireRevengeShooterRumbleEv"]
    pub(super) fn expression_FireRevengeShooterRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon30expression_FireBananaGunRumbleEv"]
    pub(super) fn expression_FireBananaGunRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon27expression_FireRayGunRumbleEv"]
    pub(super) fn expression_FireRayGunRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon26expression_HummerHitRumbleEv"]
    pub(super) fn expression_HummerHitRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon32expression_ItemHeavyThrow4RumbleEv"]
    pub(super) fn expression_ItemHeavyThrow4Rumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon31expression_ItemHeavyThrowRumbleEv"]
    pub(super) fn expression_ItemHeavyThrowRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon35expression_ItemLightThrowDashRumbleEv"]
    pub(super) fn expression_ItemLightThrowDashRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon32expression_ItemLightThrow4RumbleEv"]
    pub(super) fn expression_ItemLightThrow4Rumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon31expression_ItemLightThrowRumbleEv"]
    pub(super) fn expression_ItemLightThrowRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon35expression_ItemLightThrowDropRumbleEv"]
    pub(super) fn expression_ItemLightThrowDropRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon27expression_FinalCutinRumbleEv"]
    pub(super) fn expression_FinalCutinRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon25expression_CatchCutRumbleEv"]
    pub(super) fn expression_CatchCutRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon26expression_CatchPullRumbleEv"]
    pub(super) fn expression_CatchPullRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon28expression_GimmickPipeRumbleEv"]
    pub(super) fn expression_GimmickPipeRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon34expression_CaptureMasterHandRumbleEv"]
    pub(super) fn expression_CaptureMasterHandRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon30expression_CaptureNabbitRumbleEv"]
    pub(super) fn expression_CaptureNabbitRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon34expression_CaptureGimmckFishRumbleEv"]
    pub(super) fn expression_CaptureGimmckFishRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon27expression_DamageFallRumbleEv"]
    pub(super) fn expression_DamageFallRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon30expression_CaptureWaitLwRumbleEv"]
    pub(super) fn expression_CaptureWaitLwRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon30expression_CaptureWaitHiRumbleEv"]
    pub(super) fn expression_CaptureWaitHiRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon29expression_SwimDrownOutRumbleEv"]
    pub(super) fn expression_SwimDrownOutRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon26expression_SwimDrownRumbleEv"]
    pub(super) fn expression_SwimDrownRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon28expression_AirCatchHitRumbleEv"]
    pub(super) fn expression_AirCatchHitRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon28expression_SlipEscapeFRumbleEv"]
    pub(super) fn expression_SlipEscapeFRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon27expression_CliffJump2RumbleEv"]
    pub(super) fn expression_CliffJump2Rumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon27expression_CliffCatchRumbleEv"]
    pub(super) fn expression_CliffCatchRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon25expression_StopWallRumbleEv"]
    pub(super) fn expression_StopWallRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon26expression_SwallowedRumbleEv"]
    pub(super) fn expression_SwallowedRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon25expression_SleepEndRumbleEv"]
    pub(super) fn expression_SleepEndRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon26expression_SleepLoopRumbleEv"]
    pub(super) fn expression_SleepLoopRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon28expression_FuraFuraEndRumbleEv"]
    pub(super) fn expression_FuraFuraEndRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon31expression_FuraFuraStartDRumbleEv"]
    pub(super) fn expression_FuraFuraStartDRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon31expression_FuraFuraStartURumbleEv"]
    pub(super) fn expression_FuraFuraStartURumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon21expression_BindRumbleEv"]
    pub(super) fn expression_BindRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon25expression_FuraFuraRumbleEv"]
    pub(super) fn expression_FuraFuraRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon31expression_ShieldBreakFlyRumbleEv"]
    pub(super) fn expression_ShieldBreakFlyRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon27expression_AttachWallRumbleEv"]
    pub(super) fn expression_AttachWallRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon32expression_PassiveWallJumpRumbleEv"]
    pub(super) fn expression_PassiveWallJumpRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon24expression_PassiveRumbleEv"]
    pub(super) fn expression_PassiveRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon29expression_DownForwardURumbleEv"]
    pub(super) fn expression_DownForwardURumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon27expression_DownBoundURumbleEv"]
    pub(super) fn expression_DownBoundURumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon27expression_WallDamageRumbleEv"]
    pub(super) fn expression_WallDamageRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon27expression_CaptureCutRumbleEv"]
    pub(super) fn expression_CaptureCutRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon32expression_CapturePulledHiRumbleEv"]
    pub(super) fn expression_CapturePulledHiRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon32expression_DamageFlyMeteorRumbleEv"]
    pub(super) fn expression_DamageFlyMeteorRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon30expression_DamageFlyRollRumbleEv"]
    pub(super) fn expression_DamageFlyRollRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon29expression_DamageFlyTopRumbleEv"]
    pub(super) fn expression_DamageFlyTopRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon28expression_DamageFlyLwRumbleEv"]
    pub(super) fn expression_DamageFlyLwRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon27expression_DamageFlyNRumbleEv"]
    pub(super) fn expression_DamageFlyNRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon28expression_DamageFlyHiRumbleEv"]
    pub(super) fn expression_DamageFlyHiRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon27expression_DamageFlyLRumbleEv"]
    pub(super) fn expression_DamageFlyLRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon27expression_DamageFlyMRumbleEv"]
    pub(super) fn expression_DamageFlyMRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon26expression_JustGuardRumbleEv"]
    pub(super) fn expression_JustGuardRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon25expression_GuardLv5RumbleEv"]
    pub(super) fn expression_GuardLv5Rumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon25expression_GuardLv4RumbleEv"]
    pub(super) fn expression_GuardLv4Rumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon25expression_GuardLv3RumbleEv"]
    pub(super) fn expression_GuardLv3Rumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon25expression_GuardLv2RumbleEv"]
    pub(super) fn expression_GuardLv2Rumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon25expression_GuardLv1RumbleEv"]
    pub(super) fn expression_GuardLv1Rumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon25expression_Guard100RumbleEv"]
    pub(super) fn expression_Guard100Rumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon26expression_DamageLv5RumbleEv"]
    pub(super) fn expression_DamageLv5Rumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon26expression_DamageLv4RumbleEv"]
    pub(super) fn expression_DamageLv4Rumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon26expression_DamageLv3RumbleEv"]
    pub(super) fn expression_DamageLv3Rumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon26expression_DamageLv2RumbleEv"]
    pub(super) fn expression_DamageLv2Rumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon26expression_DamageLv1RumbleEv"]
    pub(super) fn expression_DamageLv1Rumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon26expression_Damage100RumbleEv"]
    pub(super) fn expression_Damage100Rumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon24expression_ReboundRumbleEv"]
    pub(super) fn expression_ReboundRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon25expression_GuardOffRumbleEv"]
    pub(super) fn expression_GuardOffRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon24expression_GuardOnRumbleEv"]
    pub(super) fn expression_GuardOnRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon25expression_StepPoseRumbleEv"]
    pub(super) fn expression_StepPoseRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon29expression_SamusEscapeBRumbleEv"]
    pub(super) fn expression_SamusEscapeBRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon29expression_SamusEscapeFRumbleEv"]
    pub(super) fn expression_SamusEscapeFRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon31expression_EscapeAirSlideRumbleEv"]
    pub(super) fn expression_EscapeAirSlideRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon26expression_EscapeAirRumbleEv"]
    pub(super) fn expression_EscapeAirRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon24expression_EscapeBRumbleEv"]
    pub(super) fn expression_EscapeBRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon24expression_EscapeFRumbleEv"]
    pub(super) fn expression_EscapeFRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon24expression_EscapeNRumbleEv"]
    pub(super) fn expression_EscapeNRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon37expression_LandingFallSpecialHVRumbleEv"]
    pub(super) fn expression_LandingFallSpecialHVRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon31expression_LandingHeavyHVRumbleEv"]
    pub(super) fn expression_LandingHeavyHVRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon31expression_LandingLightHVRumbleEv"]
    pub(super) fn expression_LandingLightHVRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon35expression_LandingFallSpecialRumbleEv"]
    pub(super) fn expression_LandingFallSpecialRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon29expression_LandingHeavyRumbleEv"]
    pub(super) fn expression_LandingHeavyRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon29expression_LandingLightRumbleEv"]
    pub(super) fn expression_LandingLightRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon25expression_StepJumpRumbleEv"]
    pub(super) fn expression_StepJumpRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon29expression_JumpAerialF1RumbleEv"]
    pub(super) fn expression_JumpAerialF1Rumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon32expression_JumpAerialFrontRumbleEv"]
    pub(super) fn expression_JumpAerialFrontRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon26expression_JumpFrontRumbleEv"]
    pub(super) fn expression_JumpFrontRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon25expression_JumpBackRumbleEv"]
    pub(super) fn expression_JumpBackRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon30expression_JumpFrontMiniRumbleEv"]
    pub(super) fn expression_JumpFrontMiniRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon29expression_JumpBackMiniRumbleEv"]
    pub(super) fn expression_JumpBackMiniRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon31expression_JumpAerialBackRumbleEv"]
    pub(super) fn expression_JumpAerialBackRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon29expression_JumpAerialF2RumbleEv"]
    pub(super) fn expression_JumpAerialF2Rumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon29expression_JumpAerialF3RumbleEv"]
    pub(super) fn expression_JumpAerialF3Rumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon29expression_JumpAerialF4RumbleEv"]
    pub(super) fn expression_JumpAerialF4Rumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon29expression_JumpAerialF5RumbleEv"]
    pub(super) fn expression_JumpAerialF5Rumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon28expression_StepPoseAirRumbleEv"]
    pub(super) fn expression_StepPoseAirRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon32expression_CapturePulledLwRumbleEv"]
    pub(super) fn expression_CapturePulledLwRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon28expression_CaptureJumpRumbleEv"]
    pub(super) fn expression_CaptureJumpRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon27expression_DownBoundDRumbleEv"]
    pub(super) fn expression_DownBoundDRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon27expression_CeilDamageRumbleEv"]
    pub(super) fn expression_CeilDamageRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon26expression_DownBackURumbleEv"]
    pub(super) fn expression_DownBackURumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon29expression_DownForwardDRumbleEv"]
    pub(super) fn expression_DownForwardDRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon26expression_DownBackDRumbleEv"]
    pub(super) fn expression_DownBackDRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon30expression_PassiveStandFRumbleEv"]
    pub(super) fn expression_PassiveStandFRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon30expression_PassiveStandBRumbleEv"]
    pub(super) fn expression_PassiveStandBRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon28expression_PassiveWallRumbleEv"]
    pub(super) fn expression_PassiveWallRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon28expression_PassiveCeilRumbleEv"]
    pub(super) fn expression_PassiveCeilRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon25expression_StopCeilRumbleEv"]
    pub(super) fn expression_StopCeilRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon28expression_SlipEscapeBRumbleEv"]
    pub(super) fn expression_SlipEscapeBRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon29expression_FireWalkMashRumbleEv"]
    pub(super) fn expression_FireWalkMashRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon25expression_ScrewAirRumbleEv"]
    pub(super) fn expression_ScrewAirRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp33L2CFighterAnimcmdExpressionCommon24expression_GetGoldRumbleEv"]
    pub(super) fn expression_GetGoldRumble(this: *mut L2CFighterAnimcmdExpressionCommon) -> lib::L2CValueHack;
}