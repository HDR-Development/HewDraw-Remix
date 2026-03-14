use crate::*;

use super::class::*;

extern "C" {
    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon19effect_BGTrailFinalEv"]
    pub(super) fn effect_BGTrailFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;
    
    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon25effect_BGDemonFinalAfterREv"]
    pub(super) fn effect_BGDemonFinalAfterR(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon25effect_BGDemonFinalAfterLEv"]
    pub(super) fn effect_BGDemonFinalAfterL(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon19effect_BGDemonFinalEv"]
    pub(super) fn effect_BGDemonFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon18effect_DemonRageBGEv"]
    pub(super) fn effect_DemonRageBG(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon27effect_ELightTargetFinalEndEv"]
    pub(super) fn effect_ELightTargetFinalEnd(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon21effect_BGELightFinal2Ev"]
    pub(super) fn effect_BGELightFinal2(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon20effect_BGELightFinalEv"]
    pub(super) fn effect_BGELightFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon21effect_BGEFlameFinal2Ev"]
    pub(super) fn effect_BGEFlameFinal2(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon20effect_BGEFlameFinalEv"]
    pub(super) fn effect_BGEFlameFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon20effect_BGCloudFinal2Ev"]
    pub(super) fn effect_BGCloudFinal2(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon21effect_BGEdgeFinalEndEv"]
    pub(super) fn effect_BGEdgeFinalEnd(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon18effect_BGEdgeFinalEv"]
    pub(super) fn effect_BGEdgeFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon26effect_EdgeWingStartScreenEv"]
    pub(super) fn effect_EdgeWingStartScreen(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon23effect_EdgeFire3Screen2Ev"]
    pub(super) fn effect_EdgeFire3Screen2(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon23effect_EdgeFire3Screen1Ev"]
    pub(super) fn effect_EdgeFire3Screen1(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon23effect_BGPickelFinalEndEv"]
    pub(super) fn effect_BGPickelFinalEnd(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon21effect_BGPickelFinalLEv"]
    pub(super) fn effect_BGPickelFinalL(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon21effect_BGPickelFinalREv"]
    pub(super) fn effect_BGPickelFinalR(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon23effect_BGTantanFinalEndEv"]
    pub(super) fn effect_BGTantanFinalEnd(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon21effect_BGTantanFinalLEv"]
    pub(super) fn effect_BGTantanFinalL(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon21effect_BGTantanFinalREv"]
    pub(super) fn effect_BGTantanFinalR(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon23effect_BGMasterFinalEndEv"]
    pub(super) fn effect_BGMasterFinalEnd(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon20effect_BGMasterFinalEv"]
    pub(super) fn effect_BGMasterFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon19effect_BGDollyFinalEv"]
    pub(super) fn effect_BGDollyFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon19effect_BGBuddyFinalEv"]
    pub(super) fn effect_BGBuddyFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon19effect_BGBraveFinalEv"]
    pub(super) fn effect_BGBraveFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon18effect_BGJackFinalEv"]
    pub(super) fn effect_BGJackFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon28effect_JackFinalDamageCommonEv"]
    pub(super) fn effect_JackFinalDamageCommon(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon18effect_JackCounterEv"]
    pub(super) fn effect_JackCounter(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon21effect_BGPackunFinal2Ev"]
    pub(super) fn effect_BGPackunFinal2(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon21effect_BGPackunFinal1Ev"]
    pub(super) fn effect_BGPackunFinal1(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon21effect_BGCriticalHit2Ev"]
    pub(super) fn effect_BGCriticalHit2(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon21effect_BGItemGenesis3Ev"]
    pub(super) fn effect_BGItemGenesis3(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon21effect_BGItemGenesis2Ev"]
    pub(super) fn effect_BGItemGenesis2(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon23effect_BGItemBossgalagaEv"]
    pub(super) fn effect_BGItemBossgalaga(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon20effect_BGItemDragoonEv"]
    pub(super) fn effect_BGItemDragoon(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon17effect_BGItemSlowEv"]
    pub(super) fn effect_BGItemSlow(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon24effect_BGAssistNightmareEv"]
    pub(super) fn effect_BGAssistNightmare(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon23effect_BGAssistKozukataEv"]
    pub(super) fn effect_BGAssistKozukata(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon21effect_BGAssistTvgameEv"]
    pub(super) fn effect_BGAssistTvgame(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon23effect_BGAssistMonstersEv"]
    pub(super) fn effect_BGAssistMonsters(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon24effect_BGAssistKawashimaEv"]
    pub(super) fn effect_BGAssistKawashima(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon22effect_BGAssistShadow2Ev"]
    pub(super) fn effect_BGAssistShadow2(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon21effect_BGAssistShadowEv"]
    pub(super) fn effect_BGAssistShadow(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon24effect_BGAssistSkullkid2Ev"]
    pub(super) fn effect_BGAssistSkullkid2(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon23effect_BGAssistSkullkidEv"]
    pub(super) fn effect_BGAssistSkullkid(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon19effect_BGBossFinishEv"]
    pub(super) fn effect_BGBossFinish(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon30effect_BGFinishHitHpThrowTrailEv"]
    pub(super) fn effect_BGFinishHitHpThrowTrail(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon25effect_BGFinishHitHpTrailEv"]
    pub(super) fn effect_BGFinishHitHpTrail(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon20effect_BGFinishHitHpEv"]
    pub(super) fn effect_BGFinishHitHp(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon18effect_BGFinishHitEv"]
    pub(super) fn effect_BGFinishHit(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon20effect_BGCriticalHitEv"]
    pub(super) fn effect_BGCriticalHit(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon19effect_BGYoshiFinalEv"]
    pub(super) fn effect_BGYoshiFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon24effect_BGMetaknightFinalEv"]
    pub(super) fn effect_BGMetaknightFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon23effect_BGMiiGunnerFinalEv"]
    pub(super) fn effect_BGMiiGunnerFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon25effect_BGMiiFighterFinal2Ev"]
    pub(super) fn effect_BGMiiFighterFinal2(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon24effect_BGMiiFighterFinalEv"]
    pub(super) fn effect_BGMiiFighterFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon20effect_BGChromFinal2Ev"]
    pub(super) fn effect_BGChromFinal2(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon20effect_BGChromFinal1Ev"]
    pub(super) fn effect_BGChromFinal1(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon21effect_BGRichterFinalEv"]
    pub(super) fn effect_BGRichterFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon19effect_BGSimonFinalEv"]
    pub(super) fn effect_BGSimonFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon24effect_BGGaogaenFinalEndEv"]
    pub(super) fn effect_BGGaogaenFinalEnd(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon21effect_BGGaogaenFinalEv"]
    pub(super) fn effect_BGGaogaenFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon19effect_BGKroolFinalEv"]
    pub(super) fn effect_BGKroolFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon20effect_BGDededeFinalEv"]
    pub(super) fn effect_BGDededeFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon22effect_BGDuckhuntFinalEv"]
    pub(super) fn effect_BGDuckhuntFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon21effect_BGRockmanFinalEv"]
    pub(super) fn effect_BGRockmanFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon23effect_BGSzerosuitFinalEv"]
    pub(super) fn effect_BGSzerosuitFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon23effect_BGGamewatchFinalEv"]
    pub(super) fn effect_BGGamewatchFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon19effect_BGShulkFinalEv"]
    pub(super) fn effect_BGShulkFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon21effect_BGRefletFinal2Ev"]
    pub(super) fn effect_BGRefletFinal2(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon20effect_BGRefletFinalEv"]
    pub(super) fn effect_BGRefletFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon20effect_BGMariodFinalEv"]
    pub(super) fn effect_BGMariodFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon18effect_BGPitBFinalEv"]
    pub(super) fn effect_BGPitBFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon17effect_BGPitFinalEv"]
    pub(super) fn effect_BGPitFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon19effect_BGCloudFinalEv"]
    pub(super) fn effect_BGCloudFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon21effect_BGKoopajrFinalEv"]
    pub(super) fn effect_BGKoopajrFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon26effect_BGMiiswordsManFinalEv"]
    pub(super) fn effect_BGMiiswordsManFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon19effect_BGDiddyFinalEv"]
    pub(super) fn effect_BGDiddyFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon20effect_BGRidleyFinalEv"]
    pub(super) fn effect_BGRidleyFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon21effect_BGCaptainFinalEv"]
    pub(super) fn effect_BGCaptainFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon20effect_BGWarioFinal2Ev"]
    pub(super) fn effect_BGWarioFinal2(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon19effect_BGWarioFinalEv"]
    pub(super) fn effect_BGWarioFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon23effect_BGBayonettaFinalEv"]
    pub(super) fn effect_BGBayonettaFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon22effect_BGDonkeyFinal2bEv"]
    pub(super) fn effect_BGDonkeyFinal2b(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon21effect_BGDonkeyFinal2Ev"]
    pub(super) fn effect_BGDonkeyFinal2(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon20effect_BGDonkeyFinalEv"]
    pub(super) fn effect_BGDonkeyFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon18effect_BGPopoFinalEv"]
    pub(super) fn effect_BGPopoFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon26effect_BGKenFinalShippuEndEv"]
    pub(super) fn effect_BGKenFinalShippuEnd(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon23effect_BGKenFinalShippuEv"]
    pub(super) fn effect_BGKenFinalShippu(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon27effect_BGKenFinalShinryukenEv"]
    pub(super) fn effect_BGKenFinalShinryuken(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon28effect_BGRyuFinalShinkuhado2Ev"]
    pub(super) fn effect_BGRyuFinalShinkuhado2(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon27effect_BGRyuFinalShinkuhadoEv"]
    pub(super) fn effect_BGRyuFinalShinkuhado(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon30effect_BGRyuFinalShinsyoryuEndEv"]
    pub(super) fn effect_BGRyuFinalShinsyoryuEnd(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon27effect_BGRyuFinalShinsyoryuEv"]
    pub(super) fn effect_BGRyuFinalShinsyoryu(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon19effect_BGZeldaFinalEv"]
    pub(super) fn effect_BGZeldaFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon19effect_BGLuigiFinalEv"]
    pub(super) fn effect_BGLuigiFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon24effect_BGLittlemacFinal2Ev"]
    pub(super) fn effect_BGLittlemacFinal2(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon23effect_BGLittlemacFinalEv"]
    pub(super) fn effect_BGLittlemacFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon17effect_BGIkeFinalEv"]
    pub(super) fn effect_BGIkeFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon23effect_BGInklingFinalL2Ev"]
    pub(super) fn effect_BGInklingFinalL2(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon23effect_BGInklingFinalR2Ev"]
    pub(super) fn effect_BGInklingFinalR2(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon22effect_BGInklingFinalLEv"]
    pub(super) fn effect_BGInklingFinalL(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon22effect_BGInklingFinalREv"]
    pub(super) fn effect_BGInklingFinalR(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon19effect_BGKamuiFinalEv"]
    pub(super) fn effect_BGKamuiFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon22effect_BGGekkougaFinalEv"]
    pub(super) fn effect_BGGekkougaFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon22effect_BGPTrainerFinalEv"]
    pub(super) fn effect_BGPTrainerFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon19effect_BGSonicFinalEv"]
    pub(super) fn effect_BGSonicFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon19effect_BGGanonFinalEv"]
    pub(super) fn effect_BGGanonFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon20effect_BGLucinaFinalEv"]
    pub(super) fn effect_BGLucinaFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon20effect_BGShizueFinalEv"]
    pub(super) fn effect_BGShizueFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon22effect_BGMurabitoFinalEv"]
    pub(super) fn effect_BGMurabitoFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon22effect_BGPalutenaFinalEv"]
    pub(super) fn effect_BGPalutenaFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon19effect_BGPichuFinalEv"]
    pub(super) fn effect_BGPichuFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon21effect_BGPikachuFinalEv"]
    pub(super) fn effect_BGPikachuFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon19effect_BGKoopaFinalEv"]
    pub(super) fn effect_BGKoopaFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon19effect_BGSheikFinalEv"]
    pub(super) fn effect_BGSheikFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon23effect_BGYoungLinkFinalEv"]
    pub(super) fn effect_BGYoungLinkFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon28effect_BGYoungLinkFinalStartEv"]
    pub(super) fn effect_BGYoungLinkFinalStart(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon22effect_BGToonlinkFinalEv"]
    pub(super) fn effect_BGToonlinkFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon27effect_BGToonlinkFinalStartEv"]
    pub(super) fn effect_BGToonlinkFinalStart(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon19effect_BGRobotFinalEv"]
    pub(super) fn effect_BGRobotFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon21effect_BGLucarioFinalEv"]
    pub(super) fn effect_BGLucarioFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon19effect_BGSnakeFinalEv"]
    pub(super) fn effect_BGSnakeFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon23effect_BGPacmanFinalEndEv"]
    pub(super) fn effect_BGPacmanFinalEnd(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon20effect_BGPacmanFinalEv"]
    pub(super) fn effect_BGPacmanFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon21effect_BGWiifitFinal2Ev"]
    pub(super) fn effect_BGWiifitFinal2(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon20effect_BGWiifitFinalEv"]
    pub(super) fn effect_BGWiifitFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon19effect_BGDaisyFinalEv"]
    pub(super) fn effect_BGDaisyFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon19effect_BGPeachFinalEv"]
    pub(super) fn effect_BGPeachFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon21effect_BGRosettaFinalEv"]
    pub(super) fn effect_BGRosettaFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon17effect_BGRoyFinalEv"]
    pub(super) fn effect_BGRoyFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon19effect_BGMarthFinalEv"]
    pub(super) fn effect_BGMarthFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon20effect_BGSamusdFinalEv"]
    pub(super) fn effect_BGSamusdFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon19effect_BGSamusFinalEv"]
    pub(super) fn effect_BGSamusFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon19effect_BGKirbyFinalEv"]
    pub(super) fn effect_BGKirbyFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon21effect_BGPikminFinal2Ev"]
    pub(super) fn effect_BGPikminFinal2(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon20effect_BGPikminFinalEv"]
    pub(super) fn effect_BGPikminFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon20effect_BGMewtwoFinalEv"]
    pub(super) fn effect_BGMewtwoFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon19effect_BGPurinFinalEv"]
    pub(super) fn effect_BGPurinFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon19effect_BGLucasFinalEv"]
    pub(super) fn effect_BGLucasFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon18effect_BGNessFinalEv"]
    pub(super) fn effect_BGNessFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon18effect_BGLinkFinalEv"]
    pub(super) fn effect_BGLinkFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon19effect_BGMarioFinalEv"]
    pub(super) fn effect_BGMarioFinal(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon14effect_BGTest4Ev"]
    pub(super) fn effect_BGTest4(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon14effect_BGTest3Ev"]
    pub(super) fn effect_BGTest3(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon14effect_BGTest2Ev"]
    pub(super) fn effect_BGTest2(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon14effect_BGTest1Ev"]
    pub(super) fn effect_BGTest1(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon32effect_BayonettaWitchTimeScreen2Ev"]
    pub(super) fn effect_BayonettaWitchTimeScreen2(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon32effect_BayonettaWitchTimeScreen1Ev"]
    pub(super) fn effect_BayonettaWitchTimeScreen1(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon23effect_JustShieldScreenEv"]
    pub(super) fn effect_JustShieldScreen(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon17effect_ThunderGetEv"]
    pub(super) fn effect_ThunderGet(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon15effect_KnockOutEv"]
    pub(super) fn effect_KnockOut(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon17effect_AuraHitDmgEv"]
    pub(super) fn effect_AuraHitDmg(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon23effect_PurpleFireHitDmgEv"]
    pub(super) fn effect_PurpleFireHitDmg(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon16effect_IceHitDmgEv"]
    pub(super) fn effect_IceHitDmg(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon17effect_ElecHitDmgEv"]
    pub(super) fn effect_ElecHitDmg(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon17effect_FireHitDmgEv"]
    pub(super) fn effect_FireHitDmg(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon19effect_NormalHitDmgEv"]
    pub(super) fn effect_NormalHitDmg(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon26effect_TrailFinalDoorSpaceEv"]
    pub(super) fn effect_TrailFinalDoorSpace(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon24effect_TrailFinalKeyHoleEv"]
    pub(super) fn effect_TrailFinalKeyHole(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon17effect_DemonRage2Ev"]
    pub(super) fn effect_DemonRage2(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon16effect_DemonRageEv"]
    pub(super) fn effect_DemonRage(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon15effect_ElecHitLEv"]
    pub(super) fn effect_ElecHitL(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon17effect_ElecHitEffEv"]
    pub(super) fn effect_ElecHitEff(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon24effect_PickelNotGenerateEv"]
    pub(super) fn effect_PickelNotGenerate(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon24effect_ExplosionFlashingEv"]
    pub(super) fn effect_ExplosionFlashing(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon27effect_CloudFinalLimitBreakEv"]
    pub(super) fn effect_CloudFinalLimitBreak(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon22effect_CloudLimitBreakEv"]
    pub(super) fn effect_CloudLimitBreak(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon28effect_RefletSpecialLwDamageEv"]
    pub(super) fn effect_RefletSpecialLwDamage(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon19effect_ShulkCounterEv"]
    pub(super) fn effect_ShulkCounter(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon26effect_ShulkVisionOpponentEv"]
    pub(super) fn effect_ShulkVisionOpponent(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon18effect_ShulkVisionEv"]
    pub(super) fn effect_ShulkVision(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon27effect_MonadArtsDamageSmashEv"]
    pub(super) fn effect_MonadArtsDamageSmash(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon28effect_MonadArtsDamageBusterEv"]
    pub(super) fn effect_MonadArtsDamageBuster(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon17effect_MetamonEndEv"]
    pub(super) fn effect_MetamonEnd(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon25effect_ScopeAirFireCommonEv"]
    pub(super) fn effect_ScopeAirFireCommon(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon26effect_ScopeAirRapidCommonEv"]
    pub(super) fn effect_ScopeAirRapidCommon(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon26effect_ScopeAirStartCommonEv"]
    pub(super) fn effect_ScopeAirStartCommon(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon27effect_ScopeFireUpperCommonEv"]
    pub(super) fn effect_ScopeFireUpperCommon(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon28effect_ScopeRapidUpperCommonEv"]
    pub(super) fn effect_ScopeRapidUpperCommon(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon28effect_ScopeStartUpperCommonEv"]
    pub(super) fn effect_ScopeStartUpperCommon(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon33effect_DeathscytheSwingDashCommonEv"]
    pub(super) fn effect_DeathscytheSwingDashCommon(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon36effect_DeathscytheSwing4ChargeCommonEv"]
    pub(super) fn effect_DeathscytheSwing4ChargeCommon(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon30effect_DeathscytheSwing4CommonEv"]
    pub(super) fn effect_DeathscytheSwing4Common(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon30effect_DeathscytheSwing3CommonEv"]
    pub(super) fn effect_DeathscytheSwing3Common(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon30effect_DeathscytheSwing1CommonEv"]
    pub(super) fn effect_DeathscytheSwing1Common(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon31effect_KillSwordSwingDashCommonEv"]
    pub(super) fn effect_KillSwordSwingDashCommon(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon34effect_KillSwordSwing4CommonChargeEv"]
    pub(super) fn effect_KillSwordSwing4CommonCharge(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon28effect_KillSwordSwing4CommonEv"]
    pub(super) fn effect_KillSwordSwing4Common(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon28effect_KillSwordSwing3CommonEv"]
    pub(super) fn effect_KillSwordSwing3Common(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon28effect_KillSwordSwing1CommonEv"]
    pub(super) fn effect_KillSwordSwing1Common(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon23effect_FirebarSwingDashEv"]
    pub(super) fn effect_FirebarSwingDash(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon32effect_FirebarSwing4ChargeCommonEv"]
    pub(super) fn effect_FirebarSwing4ChargeCommon(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon20effect_FirebarSwing3Ev"]
    pub(super) fn effect_FirebarSwing3(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon20effect_FirebarSwing1Ev"]
    pub(super) fn effect_FirebarSwing1(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon20effect_ClubSwingDashEv"]
    pub(super) fn effect_ClubSwingDash(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon29effect_ClubSwing4ChargeCommonEv"]
    pub(super) fn effect_ClubSwing4ChargeCommon(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon17effect_ClubSwing3Ev"]
    pub(super) fn effect_ClubSwing3(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon17effect_ClubSwing1Ev"]
    pub(super) fn effect_ClubSwing1(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon24effect_LipStickSwingDashEv"]
    pub(super) fn effect_LipStickSwingDash(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon33effect_LipStickSwing4ChargeCommonEv"]
    pub(super) fn effect_LipStickSwing4ChargeCommon(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon21effect_LipStickSwing3Ev"]
    pub(super) fn effect_LipStickSwing3(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon21effect_LipStickSwing1Ev"]
    pub(super) fn effect_LipStickSwing1(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon29effect_StarRodSwingDashCommonEv"]
    pub(super) fn effect_StarRodSwingDashCommon(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon32effect_StarRodSwing4ChargeCommonEv"]
    pub(super) fn effect_StarRodSwing4ChargeCommon(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon26effect_StarRodSwing4CommonEv"]
    pub(super) fn effect_StarRodSwing4Common(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon26effect_StarRodSwing3CommonEv"]
    pub(super) fn effect_StarRodSwing3Common(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon26effect_StarRodSwing1CommonEv"]
    pub(super) fn effect_StarRodSwing1Common(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon25effect_BatSwingDashCommonEv"]
    pub(super) fn effect_BatSwingDashCommon(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon22effect_BatSwing4CommonEv"]
    pub(super) fn effect_BatSwing4Common(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon22effect_BatSwing3CommonEv"]
    pub(super) fn effect_BatSwing3Common(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon22effect_BatSwing1CommonEv"]
    pub(super) fn effect_BatSwing1Common(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon27effect_SwordSwingDashCommonEv"]
    pub(super) fn effect_SwordSwingDashCommon(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon30effect_SwordSwing4ChargeCommonEv"]
    pub(super) fn effect_SwordSwing4ChargeCommon(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon24effect_Swing4ImpactSwordEv"]
    pub(super) fn effect_Swing4ImpactSword(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon24effect_SwordSwing4CommonEv"]
    pub(super) fn effect_SwordSwing4Common(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon24effect_Swing3ImpactSwordEv"]
    pub(super) fn effect_Swing3ImpactSword(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon24effect_SwordSwing3CommonEv"]
    pub(super) fn effect_SwordSwing3Common(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon24effect_Swing1ImpactSwordEv"]
    pub(super) fn effect_Swing1ImpactSword(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon24effect_SwordSwing1CommonEv"]
    pub(super) fn effect_SwordSwing1Common(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon19effect_Swing4ImpactEv"]
    pub(super) fn effect_Swing4Impact(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon16effect_Override2Ev"]
    pub(super) fn effect_Override2(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon15effect_OverrideEv"]
    pub(super) fn effect_Override(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon22effect_ElecThrowDamageEv"]
    pub(super) fn effect_ElecThrowDamage(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon30effect_FewRemainingCaptureTimeEv"]
    pub(super) fn effect_FewRemainingCaptureTime(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon21effect_InvalidCaptureEv"]
    pub(super) fn effect_InvalidCapture(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon18effect_ColorNormalEv"]
    pub(super) fn effect_ColorNormal(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon16effect_WitchTimeEv"]
    pub(super) fn effect_WitchTime(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon23effect_GamerMotherLightEv"]
    pub(super) fn effect_GamerMotherLight(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon32effect_PalutenaSpecialLw2PenaltyEv"]
    pub(super) fn effect_PalutenaSpecialLw2Penalty(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon22effect_CommonPowerDownEv"]
    pub(super) fn effect_CommonPowerDown(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon20effect_CommonPowerUpEv"]
    pub(super) fn effect_CommonPowerUp(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon19effect_BarrelCannonEv"]
    pub(super) fn effect_BarrelCannon(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon12effect_CurryEv"]
    pub(super) fn effect_Curry(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon39effect_MiiSwordsmanShippuSlashChargeMaxEv"]
    pub(super) fn effect_MiiSwordsmanShippuSlashChargeMax(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon26effect_StreetpassPowerDownEv"]
    pub(super) fn effect_StreetpassPowerDown(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon24effect_StreetpassPowerUpEv"]
    pub(super) fn effect_StreetpassPowerUp(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon24effect_CaptureBossgalagaEv"]
    pub(super) fn effect_CaptureBossgalaga(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon19effect_DamageLevel3Ev"]
    pub(super) fn effect_DamageLevel3(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon19effect_DamageLevel2Ev"]
    pub(super) fn effect_DamageLevel2(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon19effect_DamageLevel1Ev"]
    pub(super) fn effect_DamageLevel1(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon19effect_SpeedBoosterEv"]
    pub(super) fn effect_SpeedBooster(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon21effect_AshleyDarkzoneEv"]
    pub(super) fn effect_AshleyDarkzone(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon18effect_TeleportEndEv"]
    pub(super) fn effect_TeleportEnd(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon20effect_TeleportStartEv"]
    pub(super) fn effect_TeleportStart(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon23effect_PowerPelletGreenEv"]
    pub(super) fn effect_PowerPelletGreen(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon24effect_PowerPelletYellowEv"]
    pub(super) fn effect_PowerPelletYellow(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon22effect_PowerPelletBlueEv"]
    pub(super) fn effect_PowerPelletBlue(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon21effect_PowerPelletRedEv"]
    pub(super) fn effect_PowerPelletRed(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon18effect_PowerPelletEv"]
    pub(super) fn effect_PowerPellet(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon17effect_InvincibleEv"]
    pub(super) fn effect_Invincible(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon21effect_FinalEndCommonEv"]
    pub(super) fn effect_FinalEndCommon(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon22effect_FinalAuraCommonEv"]
    pub(super) fn effect_FinalAuraCommon(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon14effect_RollingEv"]
    pub(super) fn effect_Rolling(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon20effect_PassiveCommonEv"]
    pub(super) fn effect_PassiveCommon(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon14effect_RebirthEv"]
    pub(super) fn effect_Rebirth(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon16effect_DamageSubEv"]
    pub(super) fn effect_DamageSub(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon28effect_DamageSongFallLandingEv"]
    pub(super) fn effect_DamageSongFallLanding(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon17effect_DamageSongEv"]
    pub(super) fn effect_DamageSong(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon19effect_KnockOutLoopEv"]
    pub(super) fn effect_KnockOutLoop(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon21effect_FuraFuraCommonEv"]
    pub(super) fn effect_FuraFuraCommon(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon18effect_ShieldBreakEv"]
    pub(super) fn effect_ShieldBreak(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon17effect_JustShieldEv"]
    pub(super) fn effect_JustShield(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon29effect_WiifitSpecialLwSuccessEv"]
    pub(super) fn effect_WiifitSpecialLwSuccess(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon30effect_WiifitSpecialNChargeMaxEv"]
    pub(super) fn effect_WiifitSpecialNChargeMax(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon21effect_WarioChargeMaxEv"]
    pub(super) fn effect_WarioChargeMax(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon16effect_ChargeMaxEv"]
    pub(super) fn effect_ChargeMax(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon16effect_SmashHoldEv"]
    pub(super) fn effect_SmashHold(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon24effect_FallSpecialCommonEv"]
    pub(super) fn effect_FallSpecialCommon(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon18effect_ScrewCommonEv"]
    pub(super) fn effect_ScrewCommon(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon18effect_CatchCommonEv"]
    pub(super) fn effect_CatchCommon(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon17effect_StarMutekiEv"]
    pub(super) fn effect_StarMuteki(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon22effect_GoldenHammerEndEv"]
    pub(super) fn effect_GoldenHammerEnd(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon19effect_GoldenHammerEv"]
    pub(super) fn effect_GoldenHammer(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon16effect_HammerEndEv"]
    pub(super) fn effect_HammerEnd(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon13effect_HammerEv"]
    pub(super) fn effect_Hammer(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon19effect_KikaiDamageLEv"]
    pub(super) fn effect_KikaiDamageL(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon19effect_KikaiDamageMEv"]
    pub(super) fn effect_KikaiDamageM(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon19effect_KikaiDamageSEv"]
    pub(super) fn effect_KikaiDamageS(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon24effect_GuardDamageCommonEv"]
    pub(super) fn effect_GuardDamageCommon(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon25effect_GuardLandingEffectEv"]
    pub(super) fn effect_GuardLandingEffect(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon20effect_GuardOnCommonEv"]
    pub(super) fn effect_GuardOnCommon(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon15effect_AuraHitFEv"]
    pub(super) fn effect_AuraHitF(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon17effect_AuraHitEffEv"]
    pub(super) fn effect_AuraHitEff(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon18effect_AuraHitEff2Ev"]
    pub(super) fn effect_AuraHitEff2(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon15effect_AuraHitLEv"]
    pub(super) fn effect_AuraHitL(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon15effect_AuraHitMEv"]
    pub(super) fn effect_AuraHitM(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon15effect_AuraHitSEv"]
    pub(super) fn effect_AuraHitS(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon18effect_CurseHitEffEv"]
    pub(super) fn effect_CurseHitEff(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon21effect_PurpleFireHitFEv"]
    pub(super) fn effect_PurpleFireHitF(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon23effect_PurpleFireHitEffEv"]
    pub(super) fn effect_PurpleFireHitEff(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon24effect_PurpleFireHitEff2Ev"]
    pub(super) fn effect_PurpleFireHitEff2(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon21effect_PurpleFireHitLEv"]
    pub(super) fn effect_PurpleFireHitL(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon21effect_PurpleFireHitMEv"]
    pub(super) fn effect_PurpleFireHitM(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon21effect_PurpleFireHitSEv"]
    pub(super) fn effect_PurpleFireHitS(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon15effect_ElecHitFEv"]
    pub(super) fn effect_ElecHitF(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon15effect_ElecHitMEv"]
    pub(super) fn effect_ElecHitM(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon15effect_ElecHitSEv"]
    pub(super) fn effect_ElecHitS(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon17effect_FireHitF_LEv"]
    pub(super) fn effect_FireHitF_L(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon17effect_FireHitEffEv"]
    pub(super) fn effect_FireHitEff(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon18effect_FireHitEff2Ev"]
    pub(super) fn effect_FireHitEff2(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon17effect_FireHitL_LEv"]
    pub(super) fn effect_FireHitL_L(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon17effect_FireHitM_LEv"]
    pub(super) fn effect_FireHitM_L(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon17effect_FireHitS_LEv"]
    pub(super) fn effect_FireHitS_L(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon15effect_FireHitFEv"]
    pub(super) fn effect_FireHitF(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon15effect_FireHitLEv"]
    pub(super) fn effect_FireHitL(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon15effect_FireHitMEv"]
    pub(super) fn effect_FireHitM(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon15effect_FireHitSEv"]
    pub(super) fn effect_FireHitS(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon17effect_DamageRushEv"]
    pub(super) fn effect_DamageRush(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon13effect_DamageEv"]
    pub(super) fn effect_Damage(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon23effect_GlobalInvincibleEv"]
    pub(super) fn effect_GlobalInvincible(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon10effect_XluEv"]
    pub(super) fn effect_Xlu(this: *mut L2CFighterAnimcmdEffectCommon);

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon21effect_LipStickSwing4Ev"]
    pub(super) fn effect_LipStickSwing4(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon17effect_ClubSwing4Ev"]
    pub(super) fn effect_ClubSwing4(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon20effect_FirebarSwing4Ev"]
    pub(super) fn effect_FirebarSwing4(this: *mut L2CFighterAnimcmdEffectCommon) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp29L2CFighterAnimcmdEffectCommon15effect_FistDownEv"]
    pub(super) fn effect_FistDown(this: *mut L2CFighterAnimcmdEffectCommon);
}