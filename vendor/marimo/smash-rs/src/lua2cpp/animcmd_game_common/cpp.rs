use crate::*;

use super::class::*;

extern "C" {
    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon30game_TrailSpecialAirLwBackwardEv"]
    pub(super) fn game_TrailSpecialAirLwBackward(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon25game_JumpBoardJumpCommonLEv"]
    pub(super) fn game_JumpBoardJumpCommonL(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon25game_JumpBoardJumpCommonMEv"]
    pub(super) fn game_JumpBoardJumpCommonM(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon25game_JumpBoardJumpCommonSEv"]
    pub(super) fn game_JumpBoardJumpCommonS(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon16game_SuicideBombEv"]
    pub(super) fn game_SuicideBomb(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon21game_JustShieldBomberEv"]
    pub(super) fn game_JustShieldBomber(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon18game_HighSpeedDashEv"]
    pub(super) fn game_HighSpeedDash(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon34game_LuigiFinalShootIndirectCommonEv"]
    pub(super) fn game_LuigiFinalShootIndirectCommon(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon44game_LuigiFinalShootIndirectNoReactionCommonEv"]
    pub(super) fn game_LuigiFinalShootIndirectNoReactionCommon(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon21game_CaptureCutCommonEv"]
    pub(super) fn game_CaptureCutCommon(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon19game_StarShotCommonEv"]
    pub(super) fn game_StarShotCommon(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon16game_ScrewCommonEv"]
    pub(super) fn game_ScrewCommon(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon35game_RevengeshooterShootUpperCommonEv"]
    pub(super) fn game_RevengeshooterShootUpperCommon(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon26game_StaffShootUpperCommonEv"]
    pub(super) fn game_StaffShootUpperCommon(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon32game_DeathscytheSwingDashCommon2Ev"]
    pub(super) fn game_DeathscytheSwingDashCommon2(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon31game_DeathscytheSwingDashCommonEv"]
    pub(super) fn game_DeathscytheSwingDashCommon(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon28game_DeathscytheSwing4ChargeEv"]
    pub(super) fn game_DeathscytheSwing4Charge(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon28game_DeathscytheSwing4CommonEv"]
    pub(super) fn game_DeathscytheSwing4Common(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon28game_DeathscytheSwing3CommonEv"]
    pub(super) fn game_DeathscytheSwing3Common(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon28game_DeathscytheSwing1CommonEv"]
    pub(super) fn game_DeathscytheSwing1Common(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon30game_KillSwordSwingDashCommon2Ev"]
    pub(super) fn game_KillSwordSwingDashCommon2(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon29game_KillSwordSwingDashCommonEv"]
    pub(super) fn game_KillSwordSwingDashCommon(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon26game_SwordSwingDashCommon2Ev"]
    pub(super) fn game_SwordSwingDashCommon2(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon32game_KillSwordSwing4CommonChargeEv"]
    pub(super) fn game_KillSwordSwing4CommonCharge(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon26game_KillSwordSwing4CommonEv"]
    pub(super) fn game_KillSwordSwing4Common(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon26game_KillSwordSwing3CommonEv"]
    pub(super) fn game_KillSwordSwing3Common(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon26game_KillSwordSwing1CommonEv"]
    pub(super) fn game_KillSwordSwing1Common(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon28game_FirebarSwingDashCommon2Ev"]
    pub(super) fn game_FirebarSwingDashCommon2(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon27game_FirebarSwingDashCommonEv"]
    pub(super) fn game_FirebarSwingDashCommon(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon24game_FirebarSwing4CommonEv"]
    pub(super) fn game_FirebarSwing4Common(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon24game_FirebarSwing3CommonEv"]
    pub(super) fn game_FirebarSwing3Common(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon24game_FirebarSwing1CommonEv"]
    pub(super) fn game_FirebarSwing1Common(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon25game_ClubSwingDashCommon2Ev"]
    pub(super) fn game_ClubSwingDashCommon2(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon24game_ClubSwingDashCommonEv"]
    pub(super) fn game_ClubSwingDashCommon(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon21game_ClubSwing4CommonEv"]
    pub(super) fn game_ClubSwing4Common(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon21game_ClubSwing3CommonEv"]
    pub(super) fn game_ClubSwing3Common(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon21game_ClubSwing1CommonEv"]
    pub(super) fn game_ClubSwing1Common(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon29game_LipStickSwingDashCommon2Ev"]
    pub(super) fn game_LipStickSwingDashCommon2(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon28game_LipStickSwingDashCommonEv"]
    pub(super) fn game_LipStickSwingDashCommon(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon25game_LipStickSwing4CommonEv"]
    pub(super) fn game_LipStickSwing4Common(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon25game_LipStickSwing3CommonEv"]
    pub(super) fn game_LipStickSwing3Common(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon25game_LipStickSwing1CommonEv"]
    pub(super) fn game_LipStickSwing1Common(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon28game_StarRodSwingDashCommon2Ev"]
    pub(super) fn game_StarRodSwingDashCommon2(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon27game_StarRodSwingDashCommonEv"]
    pub(super) fn game_StarRodSwingDashCommon(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon24game_StarRodSwing4CommonEv"]
    pub(super) fn game_StarRodSwing4Common(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon24game_StarRodSwing3CommonEv"]
    pub(super) fn game_StarRodSwing3Common(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon24game_StarRodSwing1CommonEv"]
    pub(super) fn game_StarRodSwing1Common(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon24game_BatSwingDashCommon2Ev"]
    pub(super) fn game_BatSwingDashCommon2(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon23game_BatSwingDashCommonEv"]
    pub(super) fn game_BatSwingDashCommon(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon21game_BatSwing4Common2Ev"]
    pub(super) fn game_BatSwing4Common2(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon21game_BatSwing4Common1Ev"]
    pub(super) fn game_BatSwing4Common1(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon20game_BatSwing3CommonEv"]
    pub(super) fn game_BatSwing3Common(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon20game_BatSwing1CommonEv"]
    pub(super) fn game_BatSwing1Common(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon25game_SwordSwingDashCommonEv"]
    pub(super) fn game_SwordSwingDashCommon(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon22game_SwordSwing4CommonEv"]
    pub(super) fn game_SwordSwing4Common(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon22game_SwordSwing3CommonEv"]
    pub(super) fn game_SwordSwing3Common(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon22game_SwordSwing1CommonEv"]
    pub(super) fn game_SwordSwing1Common(this: *mut L2CFighterAnimcmdGameCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon24game_StaffShootAirCommonEv"]
    pub(super) fn game_StaffShootAirCommon(this: *mut L2CFighterAnimcmdGameCommon);

    #[link_name = "_ZN7lua2cpp27L2CFighterAnimcmdGameCommon33game_RevengeshooterShootAirCommonEv"]
    pub(super) fn game_RevengeshooterShootAirCommon(this: *mut L2CFighterAnimcmdGameCommon);
}