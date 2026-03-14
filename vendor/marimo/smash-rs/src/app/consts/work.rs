#[allow(dead_code)]
pub mod transition_groups;

#[allow(dead_code)]
pub mod transition_terms;

#[allow(dead_code)]
pub mod work_ids;

#[repr(u32)]
pub enum BossCommonParamFloat {
    HpMin = 0x0,
    HpMax = 0x1,
    MultiPlayHpMul = 0x2,
    MultiCpuPlayHpMul = 0x3,
    MultiPlayPowerMul = 0x4,
    Final2ndHpMulMin = 0x5,
    Final2ndHpMulMax = 0x6,
    Final2ndPowerMulMin = 0x7,
    Final2ndPowerMulMax = 0x8,
    Final2ndHardHpMulMin = 0x9,
    Final2ndHardHpMulMax = 0xA,
    Final2ndHardPowerMulMin = 0xB,
    Final2ndHardPowerMulMax = 0xC,
    VeryEasyWaitTimeMul = 0xD,
    VeryEasyHpMul = 0xE,
    VeryEasyPowerMul = 0xF,
    StandardHpMul = 0x10,
    Standard2ndHpMul = 0x11,
    AngryWaitFrameMul = 0x12,
    TargetSearchNearestMul = 0x13,
    TargetSearchMostDamagedMul = 0x14,
    TargetSearchSelectedMul = 0x15,
    DamageMaxLv0 = 0x16,
    DamageMaxLv10 = 0x17,
    DamageMaxOverMul = 0x18,
}

#[repr(u32)]
pub enum BossCommonParamInt {
    TargetSearchSelectedCountThreshold = 0x0,
    BossStopFrame = 0x1,
    BossSlowMag1st = 0x2,
    BossSlowFrame1st = 0x3,
    BossSlowMag2nd = 0x4,
    BossSlowFrame2nd = 0x5,
    BgmFadeTime = 0x6,
    SeFadeTime = 0x7,
}