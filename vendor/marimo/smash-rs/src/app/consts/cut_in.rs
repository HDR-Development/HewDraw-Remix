#[repr(u32)]
pub enum CutInType {
    None = 0x0,
    LightingBowArrow = 0x1,
    KoCamera = 0x2,
    FighterSpecialZoom = 0x3,
    Finish = 0x4,
    FinalStartCamera = 0x5,
    MotionCamera = 0x6,
}

#[repr(u32)]
pub enum CutInSubType {
    None = 0x0,
    TrailFinish = 0x1,
}

#[repr(u32)]
pub enum CutInPriority {
    None = 0x0,
    KoCamera = 0x1,
    Finish = 0x2,
    Final = 0x3,
}