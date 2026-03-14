use std::ops::{Deref, DerefMut};

use crate::*;

#[repr(C)]
#[repr(packed)]
pub struct LinkEvent {
    vtable: &'static lib::BasicEventVTable,
    pub id: u32,
    padding: u32,
    pub link_event_kind: phx::Hash40,
    pub receiver_boma: *mut app::BattleObjectModuleAccessor,
    pub sender_id: u32,
    pub no: u32,
    pub result: bool,
    pub padding3: [u8; 3]
}

impl Clone for LinkEvent {
    fn clone(&self) -> Self {
        Self {
            vtable: self.vtable,
            id: self.id,
            padding: self.padding,
            link_event_kind: self.link_event_kind,
            receiver_boma: self.receiver_boma,
            sender_id: self.sender_id,
            no: self.no,
            result: self.result,
            padding3: self.padding3
        }
    }
}

impl LinkEvent {
    pub fn get_id(&self) -> u32 {
        (self.vtable.get_id)(self as *const Self as *const lib::BasicEvent)
    }

    pub fn as_lua(&self) -> lib::L2CValue {
        self.into()
    }
}

#[cfg(feature = "type_assert")]
impl LinkEvent {
    pub fn assert() {
        assert_eq!(size_of!(LinkEvent), 0x2C);
        assert_eq!(offset_of!(LinkEvent, vtable), 0x0);
        assert_eq!(offset_of!(LinkEvent, id), 0x8);
        assert_eq!(offset_of!(LinkEvent, link_event_kind), 0x10);
        assert_eq!(offset_of!(LinkEvent, receiver_boma), 0x18);
        assert_eq!(offset_of!(LinkEvent, sender_id), 0x20);
        assert_eq!(offset_of!(LinkEvent, no), 0x24);
        assert_eq!(offset_of!(LinkEvent, result), 0x28);
    }
}

impl Into<lib::L2CValue> for &LinkEvent {
    fn into(self) -> lib::L2CValue {
        (self.vtable.into_l2cvalue)(self as *const LinkEvent as *const lib::BasicEvent).into()
    }
}

impl Drop for LinkEvent {
    fn drop(&mut self) {
        (self.vtable.destructor)(self as *mut Self as *mut lib::BasicEvent)
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct LinkEventCapture {
    parent: LinkEvent,
    pub node: phx::Hash40,
    pub status: i32,
    pub stick_x: f32,
    pub stick_y: f32,
    pub motion_offset: i32,
    pub motion_offset_lw: i32,
    pub is_motion_hi_lw: bool,
    pub is_butterflynet: bool,
    pub ignore_distance: bool,
    pub constraint: bool,
    pub constraint_offset: phx::Vector3f,
    padding: u32,
    pub hit_no: u32,
}

impl Deref for LinkEventCapture {
    type Target = LinkEvent;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for LinkEventCapture {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

#[cfg(feature = "type_assert")]
impl LinkEventCapture {
    pub fn assert() {
        assert_eq!(size_of!(LinkEventCapture), 0x68);
        assert_eq!(offset_of!(LinkEventCapture, parent), 0x0);
        assert_eq!(offset_of!(LinkEventCapture, node), 0x30);
        assert_eq!(offset_of!(LinkEventCapture, status), 0x38);
        assert_eq!(offset_of!(LinkEventCapture, stick_x), 0x3C);
        assert_eq!(offset_of!(LinkEventCapture, stick_y), 0x40);
        assert_eq!(offset_of!(LinkEventCapture, motion_offset), 0x44);
        assert_eq!(offset_of!(LinkEventCapture, motion_offset_lw), 0x48);
        assert_eq!(offset_of!(LinkEventCapture, is_motion_hi_lw), 0x4C);
        assert_eq!(offset_of!(LinkEventCapture, is_butterflynet), 0x4D);
        assert_eq!(offset_of!(LinkEventCapture, ignore_distance), 0x4E);
        assert_eq!(offset_of!(LinkEventCapture, constraint), 0x4F);
        assert_eq!(offset_of!(LinkEventCapture, constraint_offset), 0x50);
        assert_eq!(offset_of!(LinkEventCapture, hit_no), 0x60);
    }
}

#[repr(C)]
#[repr(packed)]
pub struct LinkEventCaptureItem {
    parent: LinkEvent,
    pub lr: f32,
    pub joint_id: phx::Hash40,
    pub motion_kind: phx::Hash40,
    pub motion_rate: f32,
    pub is_motion_loop: bool,
    pub is_enable_capture_cut: bool,
    padding: u16,
    pub capture_cut_frame: f32,
    pub capture_cut_frame_damage: f32,
    pub capture_recovery: f32,
    pub capture_clatter_frame: f32,
    padding2: u64,
    pub capture_cut_speed: phx::Vector2f,
    padding3: u64,
    pub kinetic_type: i32
}

impl Clone for LinkEventCaptureItem {
    fn clone(&self) -> Self {
        Self {
            parent: self.parent.clone(),
            lr: self.lr,
            joint_id: self.joint_id,
            motion_kind: self.motion_kind,
            motion_rate: self.motion_rate,
            is_motion_loop: self.is_motion_loop,
            is_enable_capture_cut: self.is_enable_capture_cut,
            padding: 0,
            capture_cut_frame: self.capture_cut_frame,
            capture_cut_frame_damage: self.capture_cut_frame_damage,
            capture_recovery: self.capture_recovery,
            capture_clatter_frame: self.capture_clatter_frame,
            padding2: 0,
            capture_cut_speed: self.capture_cut_speed,
            padding3: 0,
            kinetic_type: self.kinetic_type
        }
    }
}

impl Deref for LinkEventCaptureItem {
    type Target = LinkEvent;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for LinkEventCaptureItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

#[cfg(feature = "type_assert")]
impl LinkEventCaptureItem {
    pub fn assert() {
        assert_eq!(size_of!(LinkEventCaptureItem), 0x74);
        assert_eq!(offset_of!(LinkEventCaptureItem, parent), 0x0);
        assert_eq!(offset_of!(LinkEventCaptureItem, lr), 0x2C);
        assert_eq!(offset_of!(LinkEventCaptureItem, joint_id), 0x30);
        assert_eq!(offset_of!(LinkEventCaptureItem, motion_kind), 0x38);
        assert_eq!(offset_of!(LinkEventCaptureItem, motion_rate), 0x40);
        assert_eq!(offset_of!(LinkEventCaptureItem, is_motion_loop), 0x44);
        assert_eq!(offset_of!(LinkEventCaptureItem, is_enable_capture_cut), 0x45);
        assert_eq!(offset_of!(LinkEventCaptureItem, capture_cut_frame), 0x48);
        assert_eq!(offset_of!(LinkEventCaptureItem, capture_cut_frame_damage), 0x4C);
        assert_eq!(offset_of!(LinkEventCaptureItem, capture_recovery), 0x50);
        assert_eq!(offset_of!(LinkEventCaptureItem, capture_clatter_frame), 0x54);
        assert_eq!(offset_of!(LinkEventCaptureItem, capture_cut_speed), 0x60);
        assert_eq!(offset_of!(LinkEventCaptureItem, kinetic_type), 0x70);
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct LinkEventCaptureDriver {
    parent: LinkEventCaptureItem,
    padding: u64,
    pub offset: phx::Vector3f,
    padding2: u32,
    pub fly_degree: f32,
    pub fly_distance: f32,
    pub max_change_degree: f32,
    pub fly_speed: f32,
    pub fly_max_speed: f32,
    pub fly_accel: f32,
    pub start_accel_frame: i32,
    pub dest_meter: f32,
    pub rot_z_speed: f32,
    pub clatter_end_xlu_frame: i32
}

impl Deref for LinkEventCaptureDriver {
    type Target = LinkEventCaptureItem;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for LinkEventCaptureDriver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

#[cfg(feature = "type_assert")]
impl LinkEventCaptureDriver {
    pub fn assert() {
        assert_eq!(size_of!(LinkEventCaptureDriver), 0xB8);
        assert_eq!(offset_of!(LinkEventCaptureDriver, parent), 0x0);
        assert_eq!(offset_of!(LinkEventCaptureDriver, offset), 0x80);
        assert_eq!(offset_of!(LinkEventCaptureDriver, fly_degree), 0x90);
        assert_eq!(offset_of!(LinkEventCaptureDriver, fly_distance), 0x94);
        assert_eq!(offset_of!(LinkEventCaptureDriver, max_change_degree), 0x98);
        assert_eq!(offset_of!(LinkEventCaptureDriver, fly_speed), 0x9C);
        assert_eq!(offset_of!(LinkEventCaptureDriver, fly_max_speed), 0xA0);
        assert_eq!(offset_of!(LinkEventCaptureDriver, fly_accel), 0xA4);
        assert_eq!(offset_of!(LinkEventCaptureDriver, start_accel_frame), 0xA8);
        assert_eq!(offset_of!(LinkEventCaptureDriver, dest_meter), 0xAC);
        assert_eq!(offset_of!(LinkEventCaptureDriver, rot_z_speed), 0xB0);
        assert_eq!(offset_of!(LinkEventCaptureDriver, clatter_end_xlu_frame), 0xB4);
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct LinkEventCaptureMimikkyu {
    parent: LinkEventCaptureItem,
    pub damage_mul: f32
}

#[cfg(feature = "type_assert")]
impl LinkEventCaptureMimikkyu {
    pub fn assert() {
        assert_eq!(size_of!(LinkEventCaptureMimikkyu), 0x78);
        assert_eq!(offset_of!(LinkEventCaptureMimikkyu, parent), 0x0);
        assert_eq!(offset_of!(LinkEventCaptureMimikkyu, damage_mul), 0x74);
    }
}

impl Deref for LinkEventCaptureMimikkyu {
    type Target = LinkEventCaptureItem;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for LinkEventCaptureMimikkyu {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct LinkEventCapturePulled {
    parent: LinkEvent,
    pub pull_speed: f32,
    pub capture_cut_frame: i32,
    pub capture_cut_damage: f32,
    pub capture_cut_frame_max: i32,
    pub capture_recovery: f32,
    pub capture_clatter_frame: f32,
}

impl Deref for LinkEventCapturePulled {
    type Target = LinkEvent;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for LinkEventCapturePulled {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

#[cfg(feature = "type_assert")]
impl LinkEventCapturePulled {
    pub fn assert() {
        assert_eq!(size_of!(LinkEventCapturePulled), 0x44);
        assert_eq!(offset_of!(LinkEventCapturePulled, parent), 0x0);
        assert_eq!(offset_of!(LinkEventCapturePulled, pull_speed), 0x2C);
        assert_eq!(offset_of!(LinkEventCapturePulled, capture_cut_frame), 0x30);
        assert_eq!(offset_of!(LinkEventCapturePulled, capture_cut_damage), 0x34);
        assert_eq!(offset_of!(LinkEventCapturePulled, capture_cut_frame_max), 0x38);
        assert_eq!(offset_of!(LinkEventCapturePulled, capture_recovery), 0x3C);
        assert_eq!(offset_of!(LinkEventCapturePulled, capture_clatter_frame), 0x40);
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct LinkEventFinal {
    parent: LinkEvent,
    pub motion_kind: phx::Hash40,
    pub motion_comp_frame: i32,
    pub req_status: i32,
    pub task_id: i32,
    pub offset_x: f32,
    pub offset_y: f32,
    pub frame: f32,
    pub target_pos: phx::Vector2f
}

impl Deref for LinkEventFinal {
    type Target = LinkEvent;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for LinkEventFinal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

#[cfg(feature = "type_assert")]
impl LinkEventFinal {
    pub fn assert() {
        assert_eq!(size_of!(LinkEventFinal), 0x58);
        assert_eq!(offset_of!(LinkEventFinal, parent), 0x0);
        assert_eq!(offset_of!(LinkEventFinal, motion_kind), 0x30);
        assert_eq!(offset_of!(LinkEventFinal, motion_comp_frame), 0x38);
        assert_eq!(offset_of!(LinkEventFinal, req_status), 0x3C);
        assert_eq!(offset_of!(LinkEventFinal, task_id), 0x40);
        assert_eq!(offset_of!(LinkEventFinal, offset_x), 0x44);
        assert_eq!(offset_of!(LinkEventFinal, offset_y), 0x48);
        assert_eq!(offset_of!(LinkEventFinal, frame), 0x4C);
        assert_eq!(offset_of!(LinkEventFinal, target_pos), 0x50);
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct LinkEventMask {
    parent: LinkEvent,
    pub depth_stencil: f32,
    pub scale_z: f32
}

impl Deref for LinkEventMask {
    type Target = LinkEvent;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for LinkEventMask {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

#[cfg(feature = "type_assert")]
impl LinkEventMask {
    pub fn assert() {
        assert_eq!(size_of!(LinkEventMask), 0x34);
        assert_eq!(offset_of!(LinkEventMask, parent), 0x0);
        assert_eq!(offset_of!(LinkEventMask, depth_stencil), 0x2C);
        assert_eq!(offset_of!(LinkEventMask, scale_z), 0x30);
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct LinkEventPos {
    parent: LinkEvent,
    padding: u32,
    pub pos: phx::Vector3f,
    padding2: u32,
}

impl Deref for LinkEventPos {
    type Target = LinkEvent;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for LinkEventPos {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

#[cfg(feature = "type_assert")]
impl LinkEventPos {
    pub fn assert() {
        assert_eq!(size_of!(LinkEventPos), 0x40);
        assert_eq!(offset_of!(LinkEventPos, parent), 0x0);
        assert_eq!(offset_of!(LinkEventPos, pos), 0x30);
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct LinkEventStarShot {
    parent: LinkEvent,
    padding: u32,
    pub task_id: i32,
    pub copy: bool
}

impl Deref for LinkEventStarShot {
    type Target = LinkEvent;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for LinkEventStarShot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

#[cfg(feature = "type_assert")]
impl LinkEventStarShot {
    pub fn assert() {
        assert_eq!(size_of!(LinkEventStarShot), 0x38);
        assert_eq!(offset_of!(LinkEventStarShot, parent), 0x0);
        assert_eq!(offset_of!(LinkEventStarShot, task_id), 0x30);
        assert_eq!(offset_of!(LinkEventStarShot, copy), 0x34);
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct LinkEventThrow {
    parent: LinkEvent,
    pub motion_kind: phx::Hash40,
    pub hit_group: i32,
    pub hit_no: i32,
    pub general_kind: i32,
    pub motion_rate: f32,
    pub motion_rate_default: bool
}

impl Deref for LinkEventThrow {
    type Target = LinkEvent;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for LinkEventThrow {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

#[cfg(feature = "type_assert")]
impl LinkEventThrow {
    pub fn assert() {
        assert_eq!(size_of!(LinkEventThrow), 0x50);
        assert_eq!(offset_of!(LinkEventThrow, parent), 0x0);
        assert_eq!(offset_of!(LinkEventThrow, motion_kind), 0x30);
        assert_eq!(offset_of!(LinkEventThrow, hit_group), 0x38);
        assert_eq!(offset_of!(LinkEventThrow, hit_no), 0x3C);
        assert_eq!(offset_of!(LinkEventThrow, general_kind), 0x40);
        assert_eq!(offset_of!(LinkEventThrow, motion_rate), 0x44);
        assert_eq!(offset_of!(LinkEventThrow, motion_rate_default), 0x48);
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct LinkEventTouchItem {
    parent: LinkEvent,
    padding: u32,
    pub item: u32,
    padding2: [u32; 3],
    pub touch_pos: phx::Vector3f,
    padding3: u32,
    pub touch_param: f32
}

impl Deref for LinkEventTouchItem {
    type Target = LinkEvent;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for LinkEventTouchItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

#[cfg(feature = "type_assert")]
impl LinkEventTouchItem {
    pub fn assert() {
        assert_eq!(size_of!(LinkEventTouchItem), 0x54);
        assert_eq!(offset_of!(LinkEventTouchItem, parent), 0x0);
        assert_eq!(offset_of!(LinkEventTouchItem, item), 0x30);
        assert_eq!(offset_of!(LinkEventTouchItem, touch_pos), 0x40);
        assert_eq!(offset_of!(LinkEventTouchItem, touch_param), 0x50);
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct LinkEventYoshiTamagoDamageEffect {
    parent: LinkEvent,
    padding: u32,
    pub damage: i32
}

impl Deref for LinkEventYoshiTamagoDamageEffect {
    type Target = LinkEvent;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for LinkEventYoshiTamagoDamageEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

#[cfg(feature = "type_assert")]
impl LinkEventYoshiTamagoDamageEffect {
    pub fn assert() {
        assert_eq!(size_of!(LinkEventYoshiTamagoDamageEffect), 0x34);
        assert_eq!(offset_of!(LinkEventYoshiTamagoDamageEffect, parent), 0x0);
        assert_eq!(offset_of!(LinkEventYoshiTamagoDamageEffect, damage), 0x30);
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct FighterCloudLinkEventFinal {
    parent: LinkEventFinal,
    padding: u64,
    pub bezier_param1: f32,
    pub bezier_param2: f32
}

impl Deref for FighterCloudLinkEventFinal {
    type Target = LinkEvent;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for FighterCloudLinkEventFinal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

#[cfg(feature = "type_assert")]
impl FighterCloudLinkEventFinal {
    pub fn assert() {
        assert_eq!(size_of!(FighterCloudLinkEventFinal), 0x68);
        assert_eq!(offset_of!(FighterCloudLinkEventFinal, parent), 0x0);
        assert_eq!(offset_of!(FighterCloudLinkEventFinal, bezier_param1), 0x60);
        assert_eq!(offset_of!(FighterCloudLinkEventFinal, bezier_param2), 0x64);
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct FighterInklingLinkEventPaint {
    parent: LinkEvent,
    padding: u32,
    pub pos: phx::Vec3,
    padding2: u32,
    pub half_length: phx::Vec2,
    padding3: u64,
}

impl Deref for FighterInklingLinkEventPaint {
    type Target = LinkEvent;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for FighterInklingLinkEventPaint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

#[cfg(feature = "type_assert")]
impl FighterInklingLinkEventPaint {
    pub fn assert() {
        assert_eq!(size_of!(FighterInklingLinkEventPaint), 0x50);
        assert_eq!(offset_of!(FighterInklingLinkEventPaint, parent), 0x0);
        assert_eq!(offset_of!(FighterInklingLinkEventPaint, pos), 0x30);
        assert_eq!(offset_of!(FighterInklingLinkEventPaint, half_length), 0x40);
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct FighterPikminLinkEventWeaponPikminChangeMotion {
    parent: LinkEvent,
    pub motion_kind: phx::Hash40,
    pub start_frame: f32,
    pub rate: f32,
    pub is_loop: bool,
    padding: [u8; 7]
}

impl Deref for FighterPikminLinkEventWeaponPikminChangeMotion {
    type Target = LinkEvent;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for FighterPikminLinkEventWeaponPikminChangeMotion {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

#[cfg(feature = "type_assert")]
impl FighterPikminLinkEventWeaponPikminChangeMotion {
    pub fn assert() {
        assert_eq!(size_of!(FighterPikminLinkEventWeaponPikminChangeMotion), 0x48);
        assert_eq!(offset_of!(FighterPikminLinkEventWeaponPikminChangeMotion, parent), 0x0);
        assert_eq!(offset_of!(FighterPikminLinkEventWeaponPikminChangeMotion, motion_kind), 0x30);
        assert_eq!(offset_of!(FighterPikminLinkEventWeaponPikminChangeMotion, start_frame), 0x38);
        assert_eq!(offset_of!(FighterPikminLinkEventWeaponPikminChangeMotion, rate), 0x3C);
        assert_eq!(offset_of!(FighterPikminLinkEventWeaponPikminChangeMotion, is_loop), 0x40);
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct FighterPikminLinkEventWeaponPikminChangeStatus {
    parent: LinkEvent,
    pub status_kind: i32
}

impl Deref for FighterPikminLinkEventWeaponPikminChangeStatus {
    type Target = LinkEvent;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for FighterPikminLinkEventWeaponPikminChangeStatus {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

#[cfg(feature = "type_assert")]
impl FighterPikminLinkEventWeaponPikminChangeStatus {
    pub fn assert() {
        assert_eq!(size_of!(FighterPikminLinkEventWeaponPikminChangeStatus), 0x30);
        assert_eq!(offset_of!(FighterPikminLinkEventWeaponPikminChangeStatus, parent), 0x0);
        assert_eq!(offset_of!(FighterPikminLinkEventWeaponPikminChangeStatus, status_kind), 0x2C);
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct FighterPikminLinkEventWeaponPikminConstraint {
    parent: LinkEvent,
    pub owner_joint_id: phx::Hash40,
    pub joint_id: phx::Hash40
}

impl Deref for FighterPikminLinkEventWeaponPikminConstraint {
    type Target = LinkEvent;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for FighterPikminLinkEventWeaponPikminConstraint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

#[cfg(feature = "type_assert")]
impl FighterPikminLinkEventWeaponPikminConstraint {
    pub fn assert() {
        assert_eq!(size_of!(FighterPikminLinkEventWeaponPikminConstraint), 0x40);
        assert_eq!(offset_of!(FighterPikminLinkEventWeaponPikminConstraint, parent), 0x0);
        assert_eq!(offset_of!(FighterPikminLinkEventWeaponPikminConstraint, owner_joint_id), 0x30);
        assert_eq!(offset_of!(FighterPikminLinkEventWeaponPikminConstraint, joint_id), 0x38);
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct FighterPikminLinkEventWeaponOnFlag {
    parent: LinkEvent,
    pub value_id: i32
}

impl Deref for FighterPikminLinkEventWeaponOnFlag {
    type Target = LinkEvent;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for FighterPikminLinkEventWeaponOnFlag {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

#[cfg(feature = "type_assert")]
impl FighterPikminLinkEventWeaponOnFlag {
    pub fn assert() {
        assert_eq!(size_of!(FighterPikminLinkEventWeaponOnFlag), 0x30);
        assert_eq!(offset_of!(FighterPikminLinkEventWeaponOnFlag, parent), 0x0);
        assert_eq!(offset_of!(FighterPikminLinkEventWeaponOnFlag, value_id), 0x2C);
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct FighterPikminLinkEventWeaponSetFloat {
    parent: LinkEvent,
    pub work_value: f32,
    pub work_id: i32,
}

impl Deref for FighterPikminLinkEventWeaponSetFloat {
    type Target = LinkEvent;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for FighterPikminLinkEventWeaponSetFloat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

#[cfg(feature = "type_assert")]
impl FighterPikminLinkEventWeaponSetFloat {
    pub fn assert() {
        assert_eq!(size_of!(FighterPikminLinkEventWeaponSetFloat), 0x34);
        assert_eq!(offset_of!(FighterPikminLinkEventWeaponSetFloat, parent), 0x0);
        assert_eq!(offset_of!(FighterPikminLinkEventWeaponSetFloat, work_value), 0x2C);
        assert_eq!(offset_of!(FighterPikminLinkEventWeaponSetFloat, work_id), 0x30);
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct FighterPikminLinkEventWeaponSetInt {
    parent: LinkEvent,
    pub work_value: i32,
    pub work_id: i32,
}

impl Deref for FighterPikminLinkEventWeaponSetInt {
    type Target = LinkEvent;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for FighterPikminLinkEventWeaponSetInt {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

#[cfg(feature = "type_assert")]
impl FighterPikminLinkEventWeaponSetInt {
    pub fn assert() {
        assert_eq!(size_of!(FighterPikminLinkEventWeaponSetInt), 0x34);
        assert_eq!(offset_of!(FighterPikminLinkEventWeaponSetInt, parent), 0x0);
        assert_eq!(offset_of!(FighterPikminLinkEventWeaponSetInt, work_value), 0x2C);
        assert_eq!(offset_of!(FighterPikminLinkEventWeaponSetInt, work_id), 0x30);
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct FighterPikminLinkEventWeaponSetPowerMulStatus {
    parent: LinkEvent,
    pub power_mul_status: f32,
}

impl Deref for FighterPikminLinkEventWeaponSetPowerMulStatus {
    type Target = LinkEvent;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for FighterPikminLinkEventWeaponSetPowerMulStatus {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

#[cfg(feature = "type_assert")]
impl FighterPikminLinkEventWeaponSetPowerMulStatus {
    pub fn assert() {
        assert_eq!(size_of!(FighterPikminLinkEventWeaponSetPowerMulStatus), 0x30);
        assert_eq!(offset_of!(FighterPikminLinkEventWeaponSetPowerMulStatus, parent), 0x0);
        assert_eq!(offset_of!(FighterPikminLinkEventWeaponSetPowerMulStatus, power_mul_status), 0x2C);
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct FighterPikminLinkEventWeaponSyncLR {
    parent: LinkEvent,
    pub lr: f32,
}

impl Deref for FighterPikminLinkEventWeaponSyncLR {
    type Target = LinkEvent;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for FighterPikminLinkEventWeaponSyncLR {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

#[cfg(feature = "type_assert")]
impl FighterPikminLinkEventWeaponSyncLR {
    pub fn assert() {
        assert_eq!(size_of!(FighterPikminLinkEventWeaponSyncLR), 0x30);
        assert_eq!(offset_of!(FighterPikminLinkEventWeaponSyncLR, parent), 0x0);
        assert_eq!(offset_of!(FighterPikminLinkEventWeaponSyncLR, lr), 0x2C);
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct FighterPikminLinkEventWeaponSyncPos {
    parent: LinkEvent,
    pub pos_x: f32,
    pub pos_y: f32,
}

impl Deref for FighterPikminLinkEventWeaponSyncPos {
    type Target = LinkEvent;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for FighterPikminLinkEventWeaponSyncPos {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

#[cfg(feature = "type_assert")]
impl FighterPikminLinkEventWeaponSyncPos {
    pub fn assert() {
        assert_eq!(size_of!(FighterPikminLinkEventWeaponSyncPos), 0x34);
        assert_eq!(offset_of!(FighterPikminLinkEventWeaponSyncPos, parent), 0x0);
        assert_eq!(offset_of!(FighterPikminLinkEventWeaponSyncPos, pos_x), 0x2C);
        assert_eq!(offset_of!(FighterPikminLinkEventWeaponSyncPos, pos_y), 0x30);
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct FighterPokemonLinkEventChange {
    parent: LinkEvent,
    pub object_id: u32,
}

impl Deref for FighterPokemonLinkEventChange {
    type Target = LinkEvent;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for FighterPokemonLinkEventChange {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

#[cfg(feature = "type_assert")]
impl FighterPokemonLinkEventChange {
    pub fn assert() {
        assert_eq!(size_of!(FighterPokemonLinkEventChange), 0x30);
        assert_eq!(offset_of!(FighterPokemonLinkEventChange, parent), 0x0);
        assert_eq!(offset_of!(FighterPokemonLinkEventChange, object_id), 0x2C);
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct FighterRidleyLinkEventMotion {
    parent: LinkEvent,
    pub motion_kind: phx::Hash40,
}

impl Deref for FighterRidleyLinkEventMotion {
    type Target = LinkEvent;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for FighterRidleyLinkEventMotion {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

#[cfg(feature = "type_assert")]
impl FighterRidleyLinkEventMotion {
    pub fn assert() {
        assert_eq!(size_of!(FighterRidleyLinkEventMotion), 0x38);
        assert_eq!(offset_of!(FighterRidleyLinkEventMotion, parent), 0x0);
        assert_eq!(offset_of!(FighterRidleyLinkEventMotion, motion_kind), 0x30);
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct FighterRyuLinkEventFinalDeadDamage {
    parent: LinkEvent,
}

impl Deref for FighterRyuLinkEventFinalDeadDamage {
    type Target = LinkEvent;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for FighterRyuLinkEventFinalDeadDamage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

// bools pack tighter and rust isn't perfect
impl FighterRyuLinkEventFinalDeadDamage {
    pub fn is_dead_damage(&self) -> bool {
        self.padding3[0] != 0
    }

    pub fn set_is_dead_damage(&mut self, is_dead_damage: bool) {
        if is_dead_damage {
            self.padding3[0] = 1;
        } else {
            self.padding3[0] = 0;
        }
    }
}

#[cfg(feature = "type_assert")]
impl FighterRyuLinkEventFinalDeadDamage {
    pub fn assert() {
        assert_eq!(size_of!(FighterRyuLinkEventFinalDeadDamage), 0x2C);
        assert_eq!(offset_of!(FighterRyuLinkEventFinalDeadDamage, parent), 0x0);
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct FighterRyuLinkEventFinalMoveTarget {
    parent: LinkEventFinal,
    padding: u64,
    pub ty: u32,
    pub speed: f32,
}

impl Deref for FighterRyuLinkEventFinalMoveTarget {
    type Target = LinkEvent;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for FighterRyuLinkEventFinalMoveTarget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

#[cfg(feature = "type_assert")]
impl FighterRyuLinkEventFinalMoveTarget {
    pub fn assert() {
        assert_eq!(size_of!(FighterRyuLinkEventFinalMoveTarget), 0x68);
        assert_eq!(offset_of!(FighterRyuLinkEventFinalMoveTarget, parent), 0x0);
        assert_eq!(offset_of!(FighterRyuLinkEventFinalMoveTarget, ty), 0x60);
        assert_eq!(offset_of!(FighterRyuLinkEventFinalMoveTarget, speed), 0x64);
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct FighterElementLinkEventChange {
    parent: LinkEvent,
    pub object_id: u32,
}

impl Deref for FighterElementLinkEventChange {
    type Target = LinkEvent;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for FighterElementLinkEventChange {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

#[cfg(feature = "type_assert")]
impl FighterElementLinkEventChange {
    pub fn assert() {
        assert_eq!(size_of!(FighterElementLinkEventChange), 0x30);
        assert_eq!(offset_of!(FighterElementLinkEventChange, parent), 0x0);
        assert_eq!(offset_of!(FighterElementLinkEventChange, object_id), 0x2C);
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct WeaponPickelTrolleyLinkEventConfirmMaterial {
    parent: LinkEvent,
}

impl Deref for WeaponPickelTrolleyLinkEventConfirmMaterial {
    type Target = LinkEvent;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for WeaponPickelTrolleyLinkEventConfirmMaterial {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

// bools pack tighter and rust isn't perfect
impl WeaponPickelTrolleyLinkEventConfirmMaterial {
    pub fn is_powered(&self) -> bool {
        self.padding3[0] != 0
    }

    pub fn is_rail_free(&self) -> bool {
        self.padding3[1] != 0
    }

    pub fn succeeded(&self) -> bool {
        self.padding3[2] != 0
    }

    pub fn set_is_powered(&mut self, is_powered: bool) {
        if is_powered {
            self.padding3[0] = 1;
        } else {
            self.padding3[0] = 0;
        }
    }

    pub fn set_is_rail_free(&mut self, is_rail_free: bool) {
        if is_rail_free {
            self.padding3[1] = 1;
        } else {
            self.padding3[1] = 0;
        }
    }

    pub fn set_succeeded(&mut self, succeeded: bool) {
        if succeeded {
            self.padding3[2] = 1;
        } else {
            self.padding3[2] = 0;
        }
    }
}

#[cfg(feature = "type_assert")]
impl WeaponPickelTrolleyLinkEventConfirmMaterial {
    pub fn assert() {
        assert_eq!(size_of!(WeaponPickelTrolleyLinkEventConfirmMaterial), 0x2C);
        assert_eq!(offset_of!(WeaponPickelTrolleyLinkEventConfirmMaterial, parent), 0x0);
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct WeaponPickelTrolleyLinkEventConsumeMaterial {
    parent: LinkEvent,
}

impl Deref for WeaponPickelTrolleyLinkEventConsumeMaterial {
    type Target = LinkEvent;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for WeaponPickelTrolleyLinkEventConsumeMaterial {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

// bools pack tighter and rust isn't perfect
impl WeaponPickelTrolleyLinkEventConsumeMaterial {
    pub fn is_powered(&self) -> bool {
        self.padding3[0] != 0
    }

    pub fn is_rail_free(&self) -> bool {
        self.padding3[1] != 0
    }

    pub fn set_is_powered(&mut self, is_powered: bool) {
        if is_powered {
            self.padding3[0] = 1;
        } else {
            self.padding3[0] = 0;
        }
    }

    pub fn set_is_rail_free(&mut self, is_rail_free: bool) {
        if is_rail_free {
            self.padding3[1] = 1;
        } else {
            self.padding3[1] = 0;
        }
    }
}

#[cfg(feature = "type_assert")]
impl WeaponPickelTrolleyLinkEventConsumeMaterial {
    pub fn assert() {
        assert_eq!(size_of!(WeaponPickelTrolleyLinkEventConsumeMaterial), 0x2C);
        assert_eq!(offset_of!(WeaponPickelTrolleyLinkEventConsumeMaterial, parent), 0x0);
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct WeaponPickelTrolleyLinkEventDestroyed {
    parent: LinkEvent,
    pub speed_x_mul: f32,
    pub speed_y: f32,
}

impl Deref for WeaponPickelTrolleyLinkEventDestroyed {
    type Target = LinkEvent;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for WeaponPickelTrolleyLinkEventDestroyed {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

#[cfg(feature = "type_assert")]
impl WeaponPickelTrolleyLinkEventDestroyed {
    pub fn assert() {
        assert_eq!(size_of!(WeaponPickelTrolleyLinkEventDestroyed), 0x34);
        assert_eq!(offset_of!(WeaponPickelTrolleyLinkEventDestroyed, parent), 0x0);
        assert_eq!(offset_of!(WeaponPickelTrolleyLinkEventDestroyed, speed_x_mul), 0x2C);
        assert_eq!(offset_of!(WeaponPickelTrolleyLinkEventDestroyed, speed_y), 0x30);
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct WeaponPickelTrolleyLinkEventGetParam {
    parent: LinkEvent,
    pub catch_time: f32,
    pub catch_time_add_by_damage: f32,
    pub catch_time_decrease: f32,
    pub catch_time_decrease_by_one_pattern: f32,
    pub catch_time_min_by_one_pattern: f32,
    pub catch_time_dec_by_clatter: f32
}

impl Deref for WeaponPickelTrolleyLinkEventGetParam {
    type Target = LinkEvent;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for WeaponPickelTrolleyLinkEventGetParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

#[cfg(feature = "type_assert")]
impl WeaponPickelTrolleyLinkEventGetParam {
    pub fn assert() {
        assert_eq!(size_of!(WeaponPickelTrolleyLinkEventGetParam), 0x44);
        assert_eq!(offset_of!(WeaponPickelTrolleyLinkEventGetParam, parent), 0x0);
        assert_eq!(offset_of!(WeaponPickelTrolleyLinkEventGetParam, catch_time), 0x2C);
        assert_eq!(offset_of!(WeaponPickelTrolleyLinkEventGetParam, catch_time_add_by_damage), 0x30);
        assert_eq!(offset_of!(WeaponPickelTrolleyLinkEventGetParam, catch_time_decrease), 0x34);
        assert_eq!(offset_of!(WeaponPickelTrolleyLinkEventGetParam, catch_time_decrease_by_one_pattern), 0x38);
        assert_eq!(offset_of!(WeaponPickelTrolleyLinkEventGetParam, catch_time_min_by_one_pattern), 0x3C);
        assert_eq!(offset_of!(WeaponPickelTrolleyLinkEventGetParam, catch_time_dec_by_clatter), 0x40);
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct WeaponPickelTrolleyLinkEventRemoveIfDistance {
    parent: LinkEvent,
    pub pos_x: f32,
    pub pos_y: f32,
    pub distance: f32,
}

impl Deref for WeaponPickelTrolleyLinkEventRemoveIfDistance {
    type Target = LinkEvent;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for WeaponPickelTrolleyLinkEventRemoveIfDistance {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

#[cfg(feature = "type_assert")]
impl WeaponPickelTrolleyLinkEventRemoveIfDistance {
    pub fn assert() {
        assert_eq!(size_of!(WeaponPickelTrolleyLinkEventRemoveIfDistance), 0x38);
        assert_eq!(offset_of!(WeaponPickelTrolleyLinkEventRemoveIfDistance, parent), 0x0);
        assert_eq!(offset_of!(WeaponPickelTrolleyLinkEventRemoveIfDistance, pos_x), 0x2C);
        assert_eq!(offset_of!(WeaponPickelTrolleyLinkEventRemoveIfDistance, pos_y), 0x30);
        assert_eq!(offset_of!(WeaponPickelTrolleyLinkEventRemoveIfDistance, distance), 0x34);
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct WeaponPickelTrolleyLinkEventRemoveRailByGeneration {
    parent: LinkEvent,
    pub from: i32,
    pub generation: i32,
    pub result: bool,
}

impl Deref for WeaponPickelTrolleyLinkEventRemoveRailByGeneration {
    type Target = LinkEvent;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for WeaponPickelTrolleyLinkEventRemoveRailByGeneration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

#[cfg(feature = "type_assert")]
impl WeaponPickelTrolleyLinkEventRemoveRailByGeneration {
    pub fn assert() {
        assert_eq!(size_of!(WeaponPickelTrolleyLinkEventRemoveRailByGeneration), 0x38);
        assert_eq!(offset_of!(WeaponPickelTrolleyLinkEventRemoveRailByGeneration, parent), 0x0);
        assert_eq!(offset_of!(WeaponPickelTrolleyLinkEventRemoveRailByGeneration, from), 0x2C);
        assert_eq!(offset_of!(WeaponPickelTrolleyLinkEventRemoveRailByGeneration, generation), 0x30);
        assert_eq!(offset_of!(WeaponPickelTrolleyLinkEventRemoveRailByGeneration, result), 0x34);
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct WeaponPickelTrolleyLinkEventTurnTorchOn {
    parent: LinkEvent,
    pub target_generation: i32,
}

impl Deref for WeaponPickelTrolleyLinkEventTurnTorchOn {
    type Target = LinkEvent;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for WeaponPickelTrolleyLinkEventTurnTorchOn {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

#[cfg(feature = "type_assert")]
impl WeaponPickelTrolleyLinkEventTurnTorchOn {
    pub fn assert() {
        assert_eq!(size_of!(WeaponPickelTrolleyLinkEventTurnTorchOn), 0x30);
        assert_eq!(offset_of!(WeaponPickelTrolleyLinkEventTurnTorchOn, parent), 0x0);
        assert_eq!(offset_of!(WeaponPickelTrolleyLinkEventTurnTorchOn, target_generation), 0x2C);
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct WeaponRobotHominglaserLinkEventBurst {
    parent: LinkEvent,
    pub index: i32,
}

impl Deref for WeaponRobotHominglaserLinkEventBurst {
    type Target = LinkEvent;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for WeaponRobotHominglaserLinkEventBurst {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

#[cfg(feature = "type_assert")]
impl WeaponRobotHominglaserLinkEventBurst {
    pub fn assert() {
        assert_eq!(size_of!(WeaponRobotHominglaserLinkEventBurst), 0x30);
        assert_eq!(offset_of!(WeaponRobotHominglaserLinkEventBurst, parent), 0x0);
        assert_eq!(offset_of!(WeaponRobotHominglaserLinkEventBurst, index), 0x2C);
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct WeaponRobotHominglaserLinkEventSearch {
    parent: LinkEvent,
    pub index: i32,
}

impl Deref for WeaponRobotHominglaserLinkEventSearch {
    type Target = LinkEvent;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for WeaponRobotHominglaserLinkEventSearch {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

#[cfg(feature = "type_assert")]
impl WeaponRobotHominglaserLinkEventSearch {
    pub fn assert() {
        assert_eq!(size_of!(WeaponRobotHominglaserLinkEventSearch), 0x30);
        assert_eq!(offset_of!(WeaponRobotHominglaserLinkEventSearch, parent), 0x0);
        assert_eq!(offset_of!(WeaponRobotHominglaserLinkEventSearch, index), 0x2C);
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct WeaponShizueFishingrodLinkEventCliff {
    parent: LinkEvent,
}

impl Deref for WeaponShizueFishingrodLinkEventCliff {
    type Target = LinkEvent;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for WeaponShizueFishingrodLinkEventCliff {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

// bools pack tighter and rust isn't perfect
impl WeaponShizueFishingrodLinkEventCliff {
    pub fn found(&self) -> bool {
        self.padding3[0] != 0
    }

    pub fn set_found(&mut self, found: bool) {
        if found {
            self.padding3[0] = 1;
        } else {
            self.padding3[0] = 0;
        }
    }
}

#[cfg(feature = "type_assert")]
impl WeaponShizueFishingrodLinkEventCliff {
    pub fn assert() {
        assert_eq!(size_of!(WeaponShizueFishingrodLinkEventCliff), 0x2C);
        assert_eq!(offset_of!(WeaponShizueFishingrodLinkEventCliff, parent), 0x0);
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct WeaponShizueFishingrodLinkEventCut {
    parent: LinkEvent,
    pub cut_joint_id: phx::Hash40,
}

impl Deref for WeaponShizueFishingrodLinkEventCut {
    type Target = LinkEvent;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for WeaponShizueFishingrodLinkEventCut {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

#[cfg(feature = "type_assert")]
impl WeaponShizueFishingrodLinkEventCut {
    pub fn assert() {
        assert_eq!(size_of!(WeaponShizueFishingrodLinkEventCut), 0x38);
        assert_eq!(offset_of!(WeaponShizueFishingrodLinkEventCut, parent), 0x0);
        assert_eq!(offset_of!(WeaponShizueFishingrodLinkEventCut, cut_joint_id), 0x30);
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct WeaponShizueFishingrodLinkEventReel {
    parent: LinkEvent,
    pub reel_speed: f32,
    pub pull_speed: f32,
    pub status_kind: i32,
    pub is_hook: bool
}

impl Deref for WeaponShizueFishingrodLinkEventReel {
    type Target = LinkEvent;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for WeaponShizueFishingrodLinkEventReel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

#[cfg(feature = "type_assert")]
impl WeaponShizueFishingrodLinkEventReel {
    pub fn assert() {
        assert_eq!(size_of!(WeaponShizueFishingrodLinkEventReel), 0x3C);
        assert_eq!(offset_of!(WeaponShizueFishingrodLinkEventReel, parent), 0x0);
        assert_eq!(offset_of!(WeaponShizueFishingrodLinkEventReel, reel_speed), 0x2C);
        assert_eq!(offset_of!(WeaponShizueFishingrodLinkEventReel, pull_speed), 0x30);
        assert_eq!(offset_of!(WeaponShizueFishingrodLinkEventReel, status_kind), 0x34);
        assert_eq!(offset_of!(WeaponShizueFishingrodLinkEventReel, is_hook), 0x38);
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct WeaponShizueFishingrodLinkEventShoot {
    parent: LinkEvent,
}

impl Deref for WeaponShizueFishingrodLinkEventShoot {
    type Target = LinkEvent;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for WeaponShizueFishingrodLinkEventShoot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

// bools pack tighter and rust isn't perfect
impl WeaponShizueFishingrodLinkEventShoot {
    pub fn flick(&self) -> bool {
        self.padding3[0] != 0
    }

    pub fn set_flick(&mut self, found: bool) {
        if found {
            self.padding3[0] = 1;
        } else {
            self.padding3[0] = 0;
        }
    }
}

#[cfg(feature = "type_assert")]
impl WeaponShizueFishingrodLinkEventShoot {
    pub fn assert() {
        assert_eq!(size_of!(WeaponShizueFishingrodLinkEventShoot), 0x2C);
        assert_eq!(offset_of!(WeaponShizueFishingrodLinkEventShoot, parent), 0x0);
    }
}