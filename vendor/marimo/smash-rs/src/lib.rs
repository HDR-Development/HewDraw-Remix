#![feature(repr_simd)]
#![feature(simd_ffi)]
#![feature(const_trait_impl)]
#![feature(pointer_byte_offsets)]
#![feature(core_intrinsics)]
#![allow(incomplete_features)]
#![allow(improper_ctypes)] // For simd
#![allow(non_snake_case)]
pub mod app;
pub mod cpp;
mod lib_impl;
pub mod lua2cpp;
pub mod phx;

pub use lib_impl::lib;

pub(crate) use smash_macro::*;

#[macro_use]
extern crate bitflags;

#[macro_export]
macro_rules! size_of {
    ($ty:tt) => {
        std::mem::size_of::<$ty>()
    };
}

#[macro_export]
macro_rules! c_str {
    ($s:expr) => {
        [$s, "\0"].concat().as_bytes().as_ptr()
    };
}

#[cfg(feature = "type_assert")]
#[macro_use]
extern crate memoffset;

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct lua_State(u64);

#[cfg(feature = "type_assert")]
pub fn validate() {
    app::LinkEvent::assert();
    app::LinkEventCapture::assert();
    app::LinkEventCaptureItem::assert();
    app::LinkEventCaptureDriver::assert();
    app::LinkEventCaptureMimikkyu::assert();
    app::LinkEventCapturePulled::assert();
    app::LinkEventFinal::assert();
    app::LinkEventMask::assert();
    app::LinkEventPos::assert();
    app::LinkEventStarShot::assert();
    app::LinkEventThrow::assert();
    app::LinkEventTouchItem::assert();
    app::LinkEventYoshiTamagoDamageEffect::assert();
    app::FighterCloudLinkEventFinal::assert();
    app::FighterInklingLinkEventPaint::assert();
    app::FighterPikminLinkEventWeaponPikminChangeMotion::assert();
    app::FighterPikminLinkEventWeaponPikminChangeStatus::assert();
    app::FighterPikminLinkEventWeaponPikminConstraint::assert();
    app::FighterPikminLinkEventWeaponOnFlag::assert();
    app::FighterPikminLinkEventWeaponSetFloat::assert();
    app::FighterPikminLinkEventWeaponSetInt::assert();
    app::FighterPikminLinkEventWeaponSetPowerMulStatus::assert();
    app::FighterPikminLinkEventWeaponSyncLR::assert();
    app::FighterPikminLinkEventWeaponSyncPos::assert();
    app::FighterPokemonLinkEventChange::assert();
    app::FighterRidleyLinkEventMotion::assert();
    app::FighterRyuLinkEventFinalDeadDamage::assert();
    app::FighterRyuLinkEventFinalMoveTarget::assert();
    app::FighterElementLinkEventChange::assert();
    app::WeaponPickelTrolleyLinkEventConfirmMaterial::assert();
    app::WeaponPickelTrolleyLinkEventConsumeMaterial::assert();
    app::WeaponPickelTrolleyLinkEventDestroyed::assert();
    app::WeaponPickelTrolleyLinkEventGetParam::assert();
    app::WeaponPickelTrolleyLinkEventRemoveIfDistance::assert();
    app::WeaponPickelTrolleyLinkEventRemoveRailByGeneration::assert();
    app::WeaponPickelTrolleyLinkEventTurnTorchOn::assert();
    app::WeaponRobotHominglaserLinkEventBurst::assert();
    app::WeaponRobotHominglaserLinkEventSearch::assert();
    app::WeaponShizueFishingrodLinkEventCliff::assert();
    app::WeaponShizueFishingrodLinkEventCut::assert();
    app::WeaponShizueFishingrodLinkEventReel::assert();
    app::WeaponShizueFishingrodLinkEventShoot::assert();

    app::AttackAbsoluteData::assert();
    app::AttackData::assert();
    app::DamageInfo::assert();

    lib::L2CValue::assert();
    lib::L2CValueHack::assert();

    lua2cpp::L2CAgentBase::assert();
    lua2cpp::L2CAgentGeneratedBase::assert();
    lua2cpp::L2CFighterBase::assert();
    lua2cpp::L2CFighterCommon::assert();
    lua2cpp::L2CWeaponCommon::assert();

    lua2cpp::L2CFighterAIBase::assert();
    lua2cpp::L2CFighterAIActionBase::assert();
    lua2cpp::L2CFighterAIAnalystBase::assert();
    lua2cpp::L2CFighterAIModeBase::assert();

    lua2cpp::L2CFighterAnimcmdEffectCommon::assert();
    lua2cpp::L2CFighterAnimcmdExpressionCommon::assert();
    lua2cpp::L2CFighterAnimcmdGameCommon::assert();
    lua2cpp::L2CFighterAnimcmdSoundCommon::assert();
}
