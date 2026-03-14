use std::ops::{Deref, DerefMut};

use crate::*;

use super::cpp::*;

#[repr(C)]
pub struct L2CFighterAnimcmdEffectCommon {
    agent_base: lua2cpp::L2CAgentBase
}

impl Deref for L2CFighterAnimcmdEffectCommon {
    type Target = lua2cpp::L2CAgentBase;

    fn deref(&self) -> &Self::Target {
        &self.agent_base
    }
}

impl DerefMut for L2CFighterAnimcmdEffectCommon {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.agent_base
    }
}

impl L2CFighterAnimcmdEffectCommon {
    pub fn effect_BGTrailFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGTrailFinal(self).into()
        }
    }

    pub fn effect_BGDemonFinalAfterR(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGDemonFinalAfterR(self).into()
        }
    }

    pub fn effect_BGDemonFinalAfterL(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGDemonFinalAfterL(self).into()
        }
    }

    pub fn effect_BGDemonFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGDemonFinal(self).into()
        }
    }

    pub fn effect_DemonRageBG(&mut self) -> lib::L2CValue {
        unsafe {
            effect_DemonRageBG(self).into()
        }
    }

    pub fn effect_ELightTargetFinalEnd(&mut self) -> lib::L2CValue {
        unsafe {
            effect_ELightTargetFinalEnd(self).into()
        }
    }

    pub fn effect_BGELightFinal2(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGELightFinal2(self).into()
        }
    }

    pub fn effect_BGELightFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGELightFinal(self).into()
        }
    }

    pub fn effect_BGEFlameFinal2(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGEFlameFinal2(self).into()
        }
    }

    pub fn effect_BGEFlameFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGEFlameFinal(self).into()
        }
    }

    pub fn effect_BGCloudFinal2(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGCloudFinal2(self).into()
        }
    }

    pub fn effect_BGEdgeFinalEnd(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGEdgeFinalEnd(self).into()
        }
    }

    pub fn effect_BGEdgeFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGEdgeFinal(self).into()
        }
    }

    pub fn effect_EdgeWingStartScreen(&mut self) -> lib::L2CValue {
        unsafe {
            effect_EdgeWingStartScreen(self).into()
        }
    }

    pub fn effect_EdgeFire3Screen2(&mut self) -> lib::L2CValue {
        unsafe {
            effect_EdgeFire3Screen2(self).into()
        }
    }

    pub fn effect_EdgeFire3Screen1(&mut self) -> lib::L2CValue {
        unsafe {
            effect_EdgeFire3Screen1(self).into()
        }
    }

    pub fn effect_BGPickelFinalEnd(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGPickelFinalEnd(self).into()
        }
    }

    pub fn effect_BGPickelFinalL(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGPickelFinalL(self).into()
        }
    }

    pub fn effect_BGPickelFinalR(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGPickelFinalR(self).into()
        }
    }

    pub fn effect_BGTantanFinalEnd(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGTantanFinalEnd(self).into()
        }
    }

    pub fn effect_BGTantanFinalL(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGTantanFinalL(self).into()
        }
    }

    pub fn effect_BGTantanFinalR(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGTantanFinalR(self).into()
        }
    }

    pub fn effect_BGMasterFinalEnd(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGMasterFinalEnd(self).into()
        }
    }

    pub fn effect_BGMasterFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGMasterFinal(self).into()
        }
    }

    pub fn effect_BGDollyFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGDollyFinal(self).into()
        }
    }

    pub fn effect_BGBuddyFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGBuddyFinal(self).into()
        }
    }

    pub fn effect_BGBraveFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGBraveFinal(self).into()
        }
    }

    pub fn effect_BGJackFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGJackFinal(self).into()
        }
    }

    pub fn effect_JackFinalDamageCommon(&mut self) -> lib::L2CValue {
        unsafe {
            effect_JackFinalDamageCommon(self).into()
        }
    }

    pub fn effect_JackCounter(&mut self) {
        unsafe {
            effect_JackCounter(self)
        }
    }

    pub fn effect_BGPackunFinal2(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGPackunFinal2(self).into()
        }
    }

    pub fn effect_BGPackunFinal1(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGPackunFinal1(self).into()
        }
    }

    pub fn effect_BGCriticalHit2(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGCriticalHit2(self).into()
        }
    }

    pub fn effect_BGItemGenesis3(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGItemGenesis3(self).into()
        }
    }

    pub fn effect_BGItemGenesis2(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGItemGenesis2(self).into()
        }
    }

    pub fn effect_BGItemBossgalaga(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGItemBossgalaga(self).into()
        }
    }

    pub fn effect_BGItemDragoon(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGItemDragoon(self).into()
        }
    }

    pub fn effect_BGItemSlow(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGItemSlow(self).into()
        }
    }

    pub fn effect_BGAssistNightmare(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGAssistNightmare(self).into()
        }
    }

    pub fn effect_BGAssistKozukata(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGAssistKozukata(self).into()
        }
    }

    pub fn effect_BGAssistTvgame(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGAssistTvgame(self).into()
        }
    }

    pub fn effect_BGAssistMonsters(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGAssistMonsters(self).into()
        }
    }

    pub fn effect_BGAssistKawashima(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGAssistKawashima(self).into()
        }
    }

    pub fn effect_BGAssistShadow2(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGAssistShadow2(self).into()
        }
    }

    pub fn effect_BGAssistShadow(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGAssistShadow(self).into()
        }
    }

    pub fn effect_BGAssistSkullkid2(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGAssistSkullkid2(self).into()
        }
    }

    pub fn effect_BGAssistSkullkid(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGAssistSkullkid(self).into()
        }
    }

    pub fn effect_BGBossFinish(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGBossFinish(self).into()
        }
    }

    pub fn effect_BGFinishHitHpThrowTrail(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGFinishHitHpThrowTrail(self).into()
        }
    }

    pub fn effect_BGFinishHitHpTrail(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGFinishHitHpTrail(self).into()
        }
    }

    pub fn effect_BGFinishHitHp(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGFinishHitHp(self).into()
        }
    }

    pub fn effect_BGFinishHit(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGFinishHit(self).into()
        }
    }

    pub fn effect_BGCriticalHit(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGCriticalHit(self).into()
        }
    }

    pub fn effect_BGYoshiFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGYoshiFinal(self).into()
        }
    }

    pub fn effect_BGMetaknightFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGMetaknightFinal(self).into()
        }
    }

    pub fn effect_BGMiiGunnerFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGMiiGunnerFinal(self).into()
        }
    }

    pub fn effect_BGMiiFighterFinal2(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGMiiFighterFinal2(self).into()
        }
    }

    pub fn effect_BGMiiFighterFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGMiiFighterFinal(self).into()
        }
    }

    pub fn effect_BGChromFinal2(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGChromFinal2(self).into()
        }
    }

    pub fn effect_BGChromFinal1(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGChromFinal1(self).into()
        }
    }

    pub fn effect_BGRichterFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGRichterFinal(self).into()
        }
    }

    pub fn effect_BGSimonFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGSimonFinal(self).into()
        }
    }

    pub fn effect_BGGaogaenFinalEnd(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGGaogaenFinalEnd(self).into()
        }
    }

    pub fn effect_BGGaogaenFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGGaogaenFinal(self).into()
        }
    }

    pub fn effect_BGKroolFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGKroolFinal(self).into()
        }
    }

    pub fn effect_BGDededeFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGDededeFinal(self).into()
        }
    }

    pub fn effect_BGDuckhuntFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGDuckhuntFinal(self).into()
        }
    }

    pub fn effect_BGRockmanFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGRockmanFinal(self).into()
        }
    }

    pub fn effect_BGSzerosuitFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGSzerosuitFinal(self).into()
        }
    }

    pub fn effect_BGGamewatchFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGGamewatchFinal(self).into()
        }
    }

    pub fn effect_BGShulkFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGShulkFinal(self).into()
        }
    }

    pub fn effect_BGRefletFinal2(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGRefletFinal2(self).into()
        }
    }

    pub fn effect_BGRefletFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGRefletFinal(self).into()
        }
    }

    pub fn effect_BGMariodFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGMariodFinal(self).into()
        }
    }

    pub fn effect_BGPitBFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGPitBFinal(self).into()
        }
    }

    pub fn effect_BGPitFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGPitFinal(self).into()
        }
    }

    pub fn effect_BGCloudFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGCloudFinal(self).into()
        }
    }

    pub fn effect_BGKoopajrFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGKoopajrFinal(self).into()
        }
    }

    pub fn effect_BGMiiswordsManFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGMiiswordsManFinal(self).into()
        }
    }

    pub fn effect_BGDiddyFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGDiddyFinal(self).into()
        }
    }

    pub fn effect_BGRidleyFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGRidleyFinal(self).into()
        }
    }

    pub fn effect_BGCaptainFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGCaptainFinal(self).into()
        }
    }

    pub fn effect_BGWarioFinal2(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGWarioFinal2(self).into()
        }
    }

    pub fn effect_BGWarioFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGWarioFinal(self).into()
        }
    }

    pub fn effect_BGBayonettaFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGBayonettaFinal(self).into()
        }
    }

    pub fn effect_BGDonkeyFinal2b(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGDonkeyFinal2b(self).into()
        }
    }

    pub fn effect_BGDonkeyFinal2(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGDonkeyFinal2(self).into()
        }
    }

    pub fn effect_BGDonkeyFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGDonkeyFinal(self).into()
        }
    }

    pub fn effect_BGPopoFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGPopoFinal(self).into()
        }
    }

    pub fn effect_BGKenFinalShippuEnd(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGKenFinalShippuEnd(self).into()
        }
    }

    pub fn effect_BGKenFinalShippu(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGKenFinalShippu(self).into()
        }
    }

    pub fn effect_BGKenFinalShinryuken(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGKenFinalShinryuken(self).into()
        }
    }

    pub fn effect_BGRyuFinalShinkuhado2(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGRyuFinalShinkuhado2(self).into()
        }
    }

    pub fn effect_BGRyuFinalShinkuhado(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGRyuFinalShinkuhado(self).into()
        }
    }

    pub fn effect_BGRyuFinalShinsyoryuEnd(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGRyuFinalShinsyoryuEnd(self).into()
        }
    }

    pub fn effect_BGRyuFinalShinsyoryu(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGRyuFinalShinsyoryu(self).into()
        }
    }

    pub fn effect_BGZeldaFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGZeldaFinal(self).into()
        }
    }

    pub fn effect_BGLuigiFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGLuigiFinal(self).into()
        }
    }

    pub fn effect_BGLittlemacFinal2(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGLittlemacFinal2(self).into()
        }
    }

    pub fn effect_BGLittlemacFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGLittlemacFinal(self).into()
        }
    }

    pub fn effect_BGIkeFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGIkeFinal(self).into()
        }
    }

    pub fn effect_BGInklingFinalL2(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGInklingFinalL2(self).into()
        }
    }

    pub fn effect_BGInklingFinalR2(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGInklingFinalR2(self).into()
        }
    }

    pub fn effect_BGInklingFinalL(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGInklingFinalL(self).into()
        }
    }

    pub fn effect_BGInklingFinalR(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGInklingFinalR(self).into()
        }
    }

    pub fn effect_BGKamuiFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGKamuiFinal(self).into()
        }
    }

    pub fn effect_BGGekkougaFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGGekkougaFinal(self).into()
        }
    }

    pub fn effect_BGPTrainerFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGPTrainerFinal(self).into()
        }
    }

    pub fn effect_BGSonicFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGSonicFinal(self).into()
        }
    }

    pub fn effect_BGGanonFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGGanonFinal(self).into()
        }
    }

    pub fn effect_BGLucinaFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGLucinaFinal(self).into()
        }
    }

    pub fn effect_BGShizueFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGShizueFinal(self).into()
        }
    }

    pub fn effect_BGMurabitoFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGMurabitoFinal(self).into()
        }
    }

    pub fn effect_BGPalutenaFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGPalutenaFinal(self).into()
        }
    }

    pub fn effect_BGPichuFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGPichuFinal(self).into()
        }
    }

    pub fn effect_BGPikachuFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGPikachuFinal(self).into()
        }
    }

    pub fn effect_BGKoopaFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGKoopaFinal(self).into()
        }
    }

    pub fn effect_BGSheikFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGSheikFinal(self).into()
        }
    }

    pub fn effect_BGYoungLinkFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGYoungLinkFinal(self).into()
        }
    }

    pub fn effect_BGYoungLinkFinalStart(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGYoungLinkFinalStart(self).into()
        }
    }

    pub fn effect_BGToonlinkFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGToonlinkFinal(self).into()
        }
    }

    pub fn effect_BGToonlinkFinalStart(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGToonlinkFinalStart(self).into()
        }
    }

    pub fn effect_BGRobotFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGRobotFinal(self).into()
        }
    }

    pub fn effect_BGLucarioFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGLucarioFinal(self).into()
        }
    }

    pub fn effect_BGSnakeFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGSnakeFinal(self).into()
        }
    }

    pub fn effect_BGPacmanFinalEnd(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGPacmanFinalEnd(self).into()
        }
    }

    pub fn effect_BGPacmanFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGPacmanFinal(self).into()
        }
    }

    pub fn effect_BGWiifitFinal2(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGWiifitFinal2(self).into()
        }
    }

    pub fn effect_BGWiifitFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGWiifitFinal(self).into()
        }
    }

    pub fn effect_BGDaisyFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGDaisyFinal(self).into()
        }
    }

    pub fn effect_BGPeachFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGPeachFinal(self).into()
        }
    }

    pub fn effect_BGRosettaFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGRosettaFinal(self).into()
        }
    }

    pub fn effect_BGRoyFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGRoyFinal(self).into()
        }
    }

    pub fn effect_BGMarthFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGMarthFinal(self).into()
        }
    }

    pub fn effect_BGSamusdFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGSamusdFinal(self).into()
        }
    }

    pub fn effect_BGSamusFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGSamusFinal(self).into()
        }
    }

    pub fn effect_BGKirbyFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGKirbyFinal(self).into()
        }
    }

    pub fn effect_BGPikminFinal2(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGPikminFinal2(self).into()
        }
    }

    pub fn effect_BGPikminFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGPikminFinal(self).into()
        }
    }

    pub fn effect_BGMewtwoFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGMewtwoFinal(self).into()
        }
    }

    pub fn effect_BGPurinFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGPurinFinal(self).into()
        }
    }

    pub fn effect_BGLucasFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGLucasFinal(self).into()
        }
    }

    pub fn effect_BGNessFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGNessFinal(self).into()
        }
    }

    pub fn effect_BGLinkFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGLinkFinal(self).into()
        }
    }

    pub fn effect_BGMarioFinal(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGMarioFinal(self).into()
        }
    }

    pub fn effect_BGTest4(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGTest4(self).into()
        }
    }

    pub fn effect_BGTest3(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGTest3(self).into()
        }
    }

    pub fn effect_BGTest2(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGTest2(self).into()
        }
    }

    pub fn effect_BGTest1(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BGTest1(self).into()
        }
    }

    pub fn effect_BayonettaWitchTimeScreen2(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BayonettaWitchTimeScreen2(self).into()
        }
    }

    pub fn effect_BayonettaWitchTimeScreen1(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BayonettaWitchTimeScreen1(self).into()
        }
    }

    pub fn effect_JustShieldScreen(&mut self) -> lib::L2CValue {
        unsafe {
            effect_JustShieldScreen(self).into()
        }
    }

    pub fn effect_ThunderGet(&mut self) -> lib::L2CValue {
        unsafe {
            effect_ThunderGet(self).into()
        }
    }

    pub fn effect_KnockOut(&mut self) -> lib::L2CValue {
        unsafe {
            effect_KnockOut(self).into()
        }
    }

    pub fn effect_AuraHitDmg(&mut self) -> lib::L2CValue {
        unsafe {
            effect_AuraHitDmg(self).into()
        }
    }

    pub fn effect_PurpleFireHitDmg(&mut self) -> lib::L2CValue {
        unsafe {
            effect_PurpleFireHitDmg(self).into()
        }
    }

    pub fn effect_IceHitDmg(&mut self) -> lib::L2CValue {
        unsafe {
            effect_IceHitDmg(self).into()
        }
    }

    pub fn effect_ElecHitDmg(&mut self) -> lib::L2CValue {
        unsafe {
            effect_ElecHitDmg(self).into()
        }
    }

    pub fn effect_FireHitDmg(&mut self) -> lib::L2CValue {
        unsafe {
            effect_FireHitDmg(self).into()
        }
    }

    pub fn effect_NormalHitDmg(&mut self) -> lib::L2CValue {
        unsafe {
            effect_NormalHitDmg(self).into()
        }
    }

    pub fn effect_TrailFinalDoorSpace(&mut self) -> lib::L2CValue {
        unsafe {
            effect_TrailFinalDoorSpace(self).into()
        }
    }

    pub fn effect_TrailFinalKeyHole(&mut self) {
        unsafe {
            effect_TrailFinalKeyHole(self)
        }
    }

    pub fn effect_DemonRage2(&mut self) {
        unsafe {
            effect_DemonRage2(self)
        }
    }

    pub fn effect_DemonRage(&mut self) {
        unsafe {
            effect_DemonRage(self)
        }
    }

    pub fn effect_ElecHitL(&mut self) {
        unsafe {
            effect_ElecHitL(self)
        }
    }

    pub fn effect_ElecHitEff(&mut self) -> lib::L2CValue {
        unsafe {
            effect_ElecHitEff(self).into()
        }
    }

    pub fn effect_PickelNotGenerate(&mut self) -> lib::L2CValue {
        unsafe {
            effect_PickelNotGenerate(self).into()
        }
    }

    pub fn effect_ExplosionFlashing(&mut self) {
        unsafe {
            effect_ExplosionFlashing(self)
        }
    }

    pub fn effect_CloudFinalLimitBreak(&mut self) {
        unsafe {
            effect_CloudFinalLimitBreak(self)
        }
    }

    pub fn effect_CloudLimitBreak(&mut self) {
        unsafe {
            effect_CloudLimitBreak(self)
        }
    }

    pub fn effect_RefletSpecialLwDamage(&mut self) -> lib::L2CValue {
        unsafe {
            effect_RefletSpecialLwDamage(self).into()
        }
    }

    pub fn effect_ShulkCounter(&mut self) -> lib::L2CValue {
        unsafe {
            effect_ShulkCounter(self).into()
        }
    }

    pub fn effect_ShulkVisionOpponent(&mut self) {
        unsafe {
            effect_ShulkVisionOpponent(self)
        }
    }

    pub fn effect_ShulkVision(&mut self) {
        unsafe {
            effect_ShulkVision(self)
        }
    }

    pub fn effect_MonadArtsDamageSmash(&mut self) {
        unsafe {
            effect_MonadArtsDamageSmash(self)
        }
    }

    pub fn effect_MonadArtsDamageBuster(&mut self) {
        unsafe {
            effect_MonadArtsDamageBuster(self)
        }
    }

    pub fn effect_MetamonEnd(&mut self) {
        unsafe {
            effect_MetamonEnd(self)
        }
    }

    pub fn effect_ScopeAirFireCommon(&mut self) -> lib::L2CValue {
        unsafe {
            effect_ScopeAirFireCommon(self).into()
        }
    }

    pub fn effect_ScopeAirRapidCommon(&mut self) -> lib::L2CValue {
        unsafe {
            effect_ScopeAirRapidCommon(self).into()
        }
    }

    pub fn effect_ScopeAirStartCommon(&mut self) {
        unsafe {
            effect_ScopeAirStartCommon(self)
        }
    }

    pub fn effect_ScopeFireUpperCommon(&mut self) -> lib::L2CValue {
        unsafe {
            effect_ScopeFireUpperCommon(self).into()
        }
    }

    pub fn effect_ScopeRapidUpperCommon(&mut self) -> lib::L2CValue {
        unsafe {
            effect_ScopeRapidUpperCommon(self).into()
        }
    }

    pub fn effect_ScopeStartUpperCommon(&mut self) {
        unsafe {
            effect_ScopeStartUpperCommon(self)
        }
    }

    pub fn effect_DeathscytheSwingDashCommon(&mut self) -> lib::L2CValue {
        unsafe {
            effect_DeathscytheSwingDashCommon(self).into()
        }
    }

    pub fn effect_DeathscytheSwing4ChargeCommon(&mut self) {
        unsafe {
            effect_DeathscytheSwing4ChargeCommon(self)
        }
    }

    pub fn effect_DeathscytheSwing4Common(&mut self) -> lib::L2CValue {
        unsafe {
            effect_DeathscytheSwing4Common(self).into()
        }
    }

    pub fn effect_DeathscytheSwing3Common(&mut self) -> lib::L2CValue {
        unsafe {
            effect_DeathscytheSwing3Common(self).into()
        }
    }

    pub fn effect_DeathscytheSwing1Common(&mut self) -> lib::L2CValue {
        unsafe {
            effect_DeathscytheSwing1Common(self).into()
        }
    }

    pub fn effect_KillSwordSwingDashCommon(&mut self) -> lib::L2CValue {
        unsafe {
            effect_KillSwordSwingDashCommon(self).into()
        }
    }

    pub fn effect_KillSwordSwing4CommonCharge(&mut self) {
        unsafe {
            effect_KillSwordSwing4CommonCharge(self)
        }
    }

    pub fn effect_KillSwordSwing4Common(&mut self) -> lib::L2CValue {
        unsafe {
            effect_KillSwordSwing4Common(self).into()
        }
    }

    pub fn effect_KillSwordSwing3Common(&mut self) -> lib::L2CValue {
        unsafe {
            effect_KillSwordSwing3Common(self).into()
        }
    }

    pub fn effect_KillSwordSwing1Common(&mut self) -> lib::L2CValue {
        unsafe {
            effect_KillSwordSwing1Common(self).into()
        }
    }

    pub fn effect_FirebarSwingDash(&mut self) -> lib::L2CValue {
        unsafe {
            effect_FirebarSwingDash(self).into()
        }
    }

    pub fn effect_FirebarSwing4ChargeCommon(&mut self) {
        unsafe {
            effect_FirebarSwing4ChargeCommon(self)
        }
    }

    pub fn effect_FirebarSwing3(&mut self) -> lib::L2CValue {
        unsafe {
            effect_FirebarSwing3(self).into()
        }
    }

    pub fn effect_FirebarSwing1(&mut self) -> lib::L2CValue {
        unsafe {
            effect_FirebarSwing1(self).into()
        }
    }

    pub fn effect_ClubSwingDash(&mut self) -> lib::L2CValue {
        unsafe {
            effect_ClubSwingDash(self).into()
        }
    }

    pub fn effect_ClubSwing4ChargeCommon(&mut self) {
        unsafe {
            effect_ClubSwing4ChargeCommon(self)
        }
    }

    pub fn effect_ClubSwing3(&mut self) -> lib::L2CValue {
        unsafe {
            effect_ClubSwing3(self).into()
        }
    }

    pub fn effect_ClubSwing1(&mut self) -> lib::L2CValue {
        unsafe {
            effect_ClubSwing1(self).into()
        }
    }

    pub fn effect_LipStickSwingDash(&mut self) -> lib::L2CValue {
        unsafe {
            effect_LipStickSwingDash(self).into()
        }
    }

    pub fn effect_LipStickSwing4ChargeCommon(&mut self) {
        unsafe {
            effect_LipStickSwing4ChargeCommon(self)
        }
    }

    pub fn effect_LipStickSwing3(&mut self) -> lib::L2CValue {
        unsafe {
            effect_LipStickSwing3(self).into()
        }
    }

    pub fn effect_LipStickSwing1(&mut self) -> lib::L2CValue {
        unsafe {
            effect_LipStickSwing1(self).into()
        }
    }

    pub fn effect_StarRodSwingDashCommon(&mut self) -> lib::L2CValue {
        unsafe {
            effect_StarRodSwingDashCommon(self).into()
        }
    }

    pub fn effect_StarRodSwing4ChargeCommon(&mut self) {
        unsafe {
            effect_StarRodSwing4ChargeCommon(self)
        }
    }

    pub fn effect_StarRodSwing4Common(&mut self) -> lib::L2CValue {
        unsafe {
            effect_StarRodSwing4Common(self).into()
        }
    }

    pub fn effect_StarRodSwing3Common(&mut self) -> lib::L2CValue {
        unsafe {
            effect_StarRodSwing3Common(self).into()
        }
    }

    pub fn effect_StarRodSwing1Common(&mut self) -> lib::L2CValue {
        unsafe {
            effect_StarRodSwing1Common(self).into()
        }
    }

    pub fn effect_BatSwingDashCommon(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BatSwingDashCommon(self).into()
        }
    }

    pub fn effect_BatSwing4Common(&mut self) {
        unsafe {
            effect_BatSwing4Common(self)
        }
    }

    pub fn effect_BatSwing3Common(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BatSwing3Common(self).into()
        }
    }

    pub fn effect_BatSwing1Common(&mut self) -> lib::L2CValue {
        unsafe {
            effect_BatSwing1Common(self).into()
        }
    }

    pub fn effect_SwordSwingDashCommon(&mut self) -> lib::L2CValue {
        unsafe {
            effect_SwordSwingDashCommon(self).into()
        }
    }

    pub fn effect_SwordSwing4ChargeCommon(&mut self) {
        unsafe {
            effect_SwordSwing4ChargeCommon(self)
        }
    }

    pub fn effect_Swing4ImpactSword(&mut self) -> lib::L2CValue {
        unsafe {
            effect_Swing4ImpactSword(self).into()
        }
    }

    pub fn effect_SwordSwing4Common(&mut self) -> lib::L2CValue {
        unsafe {
            effect_SwordSwing4Common(self).into()
        }
    }

    pub fn effect_Swing3ImpactSword(&mut self) -> lib::L2CValue {
        unsafe {
            effect_Swing3ImpactSword(self).into()
        }
    }

    pub fn effect_SwordSwing3Common(&mut self) -> lib::L2CValue {
        unsafe {
            effect_SwordSwing3Common(self).into()
        }
    }

    pub fn effect_Swing1ImpactSword(&mut self) -> lib::L2CValue {
        unsafe {
            effect_Swing1ImpactSword(self).into()
        }
    }

    pub fn effect_SwordSwing1Common(&mut self) -> lib::L2CValue {
        unsafe {
            effect_SwordSwing1Common(self).into()
        }
    }

    pub fn effect_Swing4Impact(&mut self) -> lib::L2CValue {
        unsafe {
            effect_Swing4Impact(self).into()
        }
    }

    pub fn effect_Override2(&mut self) {
        unsafe {
            effect_Override2(self)
        }
    }

    pub fn effect_Override(&mut self) {
        unsafe {
            effect_Override(self)
        }
    }

    pub fn effect_ElecThrowDamage(&mut self) {
        unsafe {
            effect_ElecThrowDamage(self)
        }
    }

    pub fn effect_FewRemainingCaptureTime(&mut self) {
        unsafe {
            effect_FewRemainingCaptureTime(self)
        }
    }

    pub fn effect_InvalidCapture(&mut self) {
        unsafe {
            effect_InvalidCapture(self)
        }
    }

    pub fn effect_ColorNormal(&mut self) {
        unsafe {
            effect_ColorNormal(self)
        }
    }

    pub fn effect_WitchTime(&mut self) {
        unsafe {
            effect_WitchTime(self)
        }
    }

    pub fn effect_GamerMotherLight(&mut self) {
        unsafe {
            effect_GamerMotherLight(self)
        }
    }

    pub fn effect_PalutenaSpecialLw2Penalty(&mut self) {
        unsafe {
            effect_PalutenaSpecialLw2Penalty(self)
        }
    }

    pub fn effect_CommonPowerDown(&mut self) {
        unsafe {
            effect_CommonPowerDown(self)
        }
    }

    pub fn effect_CommonPowerUp(&mut self) {
        unsafe {
            effect_CommonPowerUp(self)
        }
    }

    pub fn effect_BarrelCannon(&mut self) {
        unsafe {
            effect_BarrelCannon(self)
        }
    }

    pub fn effect_Curry(&mut self) {
        unsafe {
            effect_Curry(self)
        }
    }

    pub fn effect_MiiSwordsmanShippuSlashChargeMax(&mut self) {
        unsafe {
            effect_MiiSwordsmanShippuSlashChargeMax(self)
        }
    }

    pub fn effect_StreetpassPowerDown(&mut self) {
        unsafe {
            effect_StreetpassPowerDown(self)
        }
    }

    pub fn effect_StreetpassPowerUp(&mut self) {
        unsafe {
            effect_StreetpassPowerUp(self)
        }
    }

    pub fn effect_CaptureBossgalaga(&mut self) {
        unsafe {
            effect_CaptureBossgalaga(self)
        }
    }

    pub fn effect_DamageLevel3(&mut self) {
        unsafe {
            effect_DamageLevel3(self)
        }
    }

    pub fn effect_DamageLevel2(&mut self) {
        unsafe {
            effect_DamageLevel2(self)
        }
    }

    pub fn effect_DamageLevel1(&mut self) {
        unsafe {
            effect_DamageLevel1(self)
        }
    }

    pub fn effect_SpeedBooster(&mut self) {
        unsafe {
            effect_SpeedBooster(self)
        }
    }

    pub fn effect_AshleyDarkzone(&mut self) {
        unsafe {
            effect_AshleyDarkzone(self)
        }
    }

    pub fn effect_TeleportEnd(&mut self) -> lib::L2CValue {
        unsafe {
            effect_TeleportEnd(self).into()
        }
    }

    pub fn effect_TeleportStart(&mut self) -> lib::L2CValue {
        unsafe {
            effect_TeleportStart(self).into()
        }
    }

    pub fn effect_PowerPelletGreen(&mut self) {
        unsafe {
            effect_PowerPelletGreen(self)
        }
    }

    pub fn effect_PowerPelletYellow(&mut self) {
        unsafe {
            effect_PowerPelletYellow(self)
        }
    }

    pub fn effect_PowerPelletBlue(&mut self) {
        unsafe {
            effect_PowerPelletBlue(self)
        }
    }

    pub fn effect_PowerPelletRed(&mut self) {
        unsafe {
            effect_PowerPelletRed(self)
        }
    }

    pub fn effect_PowerPellet(&mut self) {
        unsafe {
            effect_PowerPellet(self)
        }
    }

    pub fn effect_Invincible(&mut self) {
        unsafe {
            effect_Invincible(self)
        }
    }

    pub fn effect_FinalEndCommon(&mut self) {
        unsafe {
            effect_FinalEndCommon(self)
        }
    }

    pub fn effect_FinalAuraCommon(&mut self) {
        unsafe {
            effect_FinalAuraCommon(self)
        }
    }

    pub fn effect_Rolling(&mut self) {
        unsafe {
            effect_Rolling(self)
        }
    }

    pub fn effect_PassiveCommon(&mut self) -> lib::L2CValue {
        unsafe {
            effect_PassiveCommon(self).into()
        }
    }

    pub fn effect_Rebirth(&mut self) {
        unsafe {
            effect_Rebirth(self)
        }
    }

    pub fn effect_DamageSub(&mut self) -> lib::L2CValue {
        unsafe {
            effect_DamageSub(self).into()
        }
    }

    pub fn effect_DamageSongFallLanding(&mut self) -> lib::L2CValue {
        unsafe {
            effect_DamageSongFallLanding(self).into()
        }
    }

    pub fn effect_DamageSong(&mut self) {
        unsafe {
            effect_DamageSong(self)
        }
    }

    pub fn effect_KnockOutLoop(&mut self) {
        unsafe {
            effect_KnockOutLoop(self)
        }
    }

    pub fn effect_FuraFuraCommon(&mut self) {
        unsafe {
            effect_FuraFuraCommon(self)
        }
    }

    pub fn effect_ShieldBreak(&mut self) {
        unsafe {
            effect_ShieldBreak(self)
        }
    }

    pub fn effect_JustShield(&mut self) -> lib::L2CValue {
        unsafe {
            effect_JustShield(self).into()
        }
    }

    pub fn effect_WiifitSpecialLwSuccess(&mut self) {
        unsafe {
            effect_WiifitSpecialLwSuccess(self)
        }
    }

    pub fn effect_WiifitSpecialNChargeMax(&mut self) {
        unsafe {
            effect_WiifitSpecialNChargeMax(self)
        }
    }

    pub fn effect_WarioChargeMax(&mut self) {
        unsafe {
            effect_WarioChargeMax(self)
        }
    }

    pub fn effect_ChargeMax(&mut self) {
        unsafe {
            effect_ChargeMax(self)
        }
    }

    pub fn effect_SmashHold(&mut self) {
        unsafe {
            effect_SmashHold(self)
        }
    }

    pub fn effect_FallSpecialCommon(&mut self) {
        unsafe {
            effect_FallSpecialCommon(self)
        }
    }

    pub fn effect_ScrewCommon(&mut self) -> lib::L2CValue {
        unsafe {
            effect_ScrewCommon(self).into()
        }
    }

    pub fn effect_CatchCommon(&mut self) -> lib::L2CValue {
        unsafe {
            effect_CatchCommon(self).into()
        }
    }

    pub fn effect_StarMuteki(&mut self) {
        unsafe {
            effect_StarMuteki(self)
        }
    }

    pub fn effect_GoldenHammerEnd(&mut self) -> lib::L2CValue {
        unsafe {
            effect_GoldenHammerEnd(self).into()
        }
    }

    pub fn effect_GoldenHammer(&mut self) {
        unsafe {
            effect_GoldenHammer(self)
        }
    }

    pub fn effect_HammerEnd(&mut self) -> lib::L2CValue {
        unsafe {
            effect_HammerEnd(self).into()
        }
    }

    pub fn effect_Hammer(&mut self) {
        unsafe {
            effect_Hammer(self)
        }
    }

    pub fn effect_KikaiDamageL(&mut self) -> lib::L2CValue {
        unsafe {
            effect_KikaiDamageL(self).into()
        }
    }

    pub fn effect_KikaiDamageM(&mut self) -> lib::L2CValue {
        unsafe {
            effect_KikaiDamageM(self).into()
        }
    }

    pub fn effect_KikaiDamageS(&mut self) -> lib::L2CValue {
        unsafe {
            effect_KikaiDamageS(self).into()
        }
    }

    pub fn effect_GuardDamageCommon(&mut self) -> lib::L2CValue {
        unsafe {
            effect_GuardDamageCommon(self).into()
        }
    }

    pub fn effect_GuardLandingEffect(&mut self) -> lib::L2CValue {
        unsafe {
            effect_GuardLandingEffect(self).into()
        }
    }

    pub fn effect_GuardOnCommon(&mut self) -> lib::L2CValue {
        unsafe {
            effect_GuardOnCommon(self).into()
        }
    }

    pub fn effect_AuraHitF(&mut self) -> lib::L2CValue {
        unsafe {
            effect_AuraHitF(self).into()
        }
    }

    pub fn effect_AuraHitEff(&mut self) -> lib::L2CValue {
        unsafe {
            effect_AuraHitEff(self).into()
        }
    }

    pub fn effect_AuraHitEff2(&mut self) -> lib::L2CValue {
        unsafe {
            effect_AuraHitEff2(self).into()
        }
    }

    pub fn effect_AuraHitL(&mut self) {
        unsafe {
            effect_AuraHitL(self)
        }
    }

    pub fn effect_AuraHitM(&mut self) {
        unsafe {
            effect_AuraHitM(self)
        }
    }

    pub fn effect_AuraHitS(&mut self) -> lib::L2CValue {
        unsafe {
            effect_AuraHitS(self).into()
        }
    }

    pub fn effect_CurseHitEff(&mut self) {
        unsafe {
            effect_CurseHitEff(self)
        }
    }

    pub fn effect_PurpleFireHitF(&mut self) -> lib::L2CValue {
        unsafe {
            effect_PurpleFireHitF(self).into()
        }
    }

    pub fn effect_PurpleFireHitEff(&mut self) -> lib::L2CValue {
        unsafe {
            effect_PurpleFireHitEff(self).into()
        }
    }

    pub fn effect_PurpleFireHitEff2(&mut self) -> lib::L2CValue {
        unsafe {
            effect_PurpleFireHitEff2(self).into()
        }
    }

    pub fn effect_PurpleFireHitL(&mut self) {
        unsafe {
            effect_PurpleFireHitL(self)
        }
    }

    pub fn effect_PurpleFireHitM(&mut self) {
        unsafe {
            effect_PurpleFireHitM(self)
        }
    }

    pub fn effect_PurpleFireHitS(&mut self) -> lib::L2CValue {
        unsafe {
            effect_PurpleFireHitS(self).into()
        }
    }

    pub fn effect_ElecHitF(&mut self) {
        unsafe {
            effect_ElecHitF(self)
        }
    }

    pub fn effect_ElecHitM(&mut self) {
        unsafe {
            effect_ElecHitM(self)
        }
    }

    pub fn effect_ElecHitS(&mut self) {
        unsafe {
            effect_ElecHitS(self)
        }
    }

    pub fn effect_FireHitF_L(&mut self) -> lib::L2CValue {
        unsafe {
            effect_FireHitF_L(self).into()
        }
    }

    pub fn effect_FireHitEff(&mut self) -> lib::L2CValue {
        unsafe {
            effect_FireHitEff(self).into()
        }
    }

    pub fn effect_FireHitEff2(&mut self) -> lib::L2CValue {
        unsafe {
            effect_FireHitEff2(self).into()
        }
    }

    pub fn effect_FireHitL_L(&mut self) -> lib::L2CValue {
        unsafe {
            effect_FireHitL_L(self).into()
        }
    }

    pub fn effect_FireHitM_L(&mut self) -> lib::L2CValue {
        unsafe {
            effect_FireHitM_L(self).into()
        }
    }

    pub fn effect_FireHitS_L(&mut self) -> lib::L2CValue {
        unsafe {
            effect_FireHitS_L(self).into()
        }
    }

    pub fn effect_FireHitF(&mut self) -> lib::L2CValue {
        unsafe {
            effect_FireHitF(self).into()
        }
    }

    pub fn effect_FireHitL(&mut self) -> lib::L2CValue {
        unsafe {
            effect_FireHitL(self).into()
        }
    }

    pub fn effect_FireHitM(&mut self) -> lib::L2CValue {
        unsafe {
            effect_FireHitM(self).into()
        }
    }

    pub fn effect_FireHitS(&mut self) -> lib::L2CValue {
        unsafe {
            effect_FireHitS(self).into()
        }
    }

    pub fn effect_DamageRush(&mut self) -> lib::L2CValue {
        unsafe {
            effect_DamageRush(self).into()
        }
    }

    pub fn effect_Damage(&mut self) -> lib::L2CValue {
        unsafe {
            effect_Damage(self).into()
        }
    }

    pub fn effect_GlobalInvincible(&mut self) {
        unsafe {
            effect_GlobalInvincible(self)
        }
    }

    pub fn effect_Xlu(&mut self) {
        unsafe {
            effect_Xlu(self)
        }
    }

    pub fn effect_LipStickSwing4(&mut self) -> lib::L2CValue {
        unsafe {
            effect_LipStickSwing4(self).into()
        }
    }

    pub fn effect_ClubSwing4(&mut self) -> lib::L2CValue {
        unsafe {
            effect_ClubSwing4(self).into()
        }
    }

    pub fn effect_FirebarSwing4(&mut self) -> lib::L2CValue {
        unsafe {
            effect_FirebarSwing4(self).into()
        }
    }

    pub fn effect_FistDown(&mut self) {
        unsafe {
            effect_FistDown(self)
        }
    }
}

#[cfg(feature = "type_assert")]
impl L2CFighterAnimcmdEffectCommon {
    pub fn assert() {
        assert_eq!(size_of!(L2CFighterAnimcmdEffectCommon), 0xC8);
        assert_eq!(offset_of!(L2CFighterAnimcmdEffectCommon, agent_base), 0x0);
    }
}