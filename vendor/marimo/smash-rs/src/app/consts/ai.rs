pub mod FighterAI;
pub mod FighterAIAct;
pub mod FighterAIAnalyst;
pub mod FighterAIStat;

#[repr(u8)]
pub enum FighterAICPType {
    Normal = 0x0,
    Stay = 0x1,
    Walk = 0x2,
    Jump = 0x3,
    Squat = 0x4,
    Escape = 0x5,
    EscapeNoRoll = 0x6,
    Chicken = 0x7,
    Attacknear = 0x8,
    Freak = 0x9,
    Cooperate = 0xA,
    Item = 0xB,
    Itemless = 0xC,
    Zako1 = 0xD,
    Zako2 = 0xE,
    Allstar = 0xF,
    StrongZako1 = 0x10,
    StrongZako2 = 0x11,
    Metal = 0x12,
    Giant = 0x13,
    Koopag = 0x14,
    Attackp1 = 0x15,
    Attackp2 = 0x16,
    Attackp3 = 0x17,
    Attackp4 = 0x18,
    TrainingSpecial = 0x19,
    TrainingSmashS = 0x1A,
    Appealer = 0x1B,
    Shielder = 0x1C,
    Swimmer = 0x1D,
    Shooter = 0x1E,
    Swinger = 0x1F,
    KirbyCopy = 0x20,
    SmashFLow = 0x21,
    Sham = 0x22,
    Challenger = 0x23,
    Manual = 0x24,
    Kiiladarz1 = 0x25,
    Kiiladarz2 = 0x26,
    Debug = 0x27,
    ParamMax = 0x28,
    ShdKirby = 0x29,
}

#[repr(u8)]
pub enum FighterAICPSlideType {
    MeleeCP = 0x0,
    None = 0x1,
    Weak = 0x2,
    Strong = 0x3
}

#[repr(u32)]
pub enum FighterAIUniqStat {
    Null = 0x0,
    GroundFree = 0x1,
    AirFree = 0x2,
    Dash = 0x3,
    Down = 0x4,
    Damage = 0x5,
    Cliff = 0x6,
    CliffAct = 0x7,
    Ladder = 0x8,
    Piyo = 0x9,
    PiyoS = 0xA,
    PiyoL = 0xB,
    Catch = 0xC,
    CatchEnd = 0xD,
    Swim = 0xE,
    AtWall = 0xF,
    FallSp = 0x10,
    Dragoon = 0x11,
    Genesis = 0x12,
    Killer = 0x13,
    Warpstar = 0x14,
    Attack = 0x15,
    AttackGround = 0x16,
    AtkHold = 0x17,
    AtkHoldFast = 0x18,
    AtkCatch = 0x19,
    Guard = 0x1A,
    Counter = 0x1B,
    JumpSquat = 0x1C,
    Fixdmg = 0x1D,
    Escape = 0x1E,
    Rebirth = 0x1F,
    LucarioSpecialS = 0x20,
    LucarioSpecialSC2 = 0x21,
    GamewatchSpecialLw = 0x22,
    Barrel = 0x23,
    Spring = 0x24,
    MewtwoThrown = 0x25,
    MasterSpecialN = 0x26,
}

impl FighterAIUniqStat {
    #[deprecated = "This constant holds the same value as AtkCatch, it doesn't appear to go used in game so be cautious when using it"]
    pub const GUARD_BREAK_ATTACK: FighterAIUniqStat = FighterAIUniqStat::AtkCatch;
}

#[repr(u32)]
pub enum FighterAIWeaponType {
    Weapon = 0x0,
    Item = 0x1,
    Stage = 0x2,
    Fighter = 0x3,
    Drop = 0x4,
    Item2 = 0x5,
    Enemy = 0x6,
}


#[repr(u32)]
pub enum FighterAIAttackPhase {
    None = 0x0,
    Prepare = 0x1,
    Start = 0x2,
    Active = 0x3,
    End = 0x4,
}


bitflags! {
    #[repr(C)]
    pub struct FighterAICPFlag: u16 {
        const Run = 0x1;
        const Attack = 0x2;
        const AtkAir = 0x4;
        const AtkJump = 0x8;
        const Shot = 0x10;
        const Shield = 0x20;
        const ChkFt = 0x40;
        const ChkZone = 0x80;
        const NearFt = 0x100;
        const Roll = 0x200;
        const Passive = 0x400;
    }
}

bitflags! {
    #[repr(C)]
    pub struct FighterAIWeaponFlag: u16 {
        const Enable = 0x1;
        const Accel = 0x2;
        const FloorSet = 0x4;
        const OnScreen = 0x8;
        const Shield = 0x10;
        const Reflector = 0x20;
        const Absorber = 0x40;
        const PutInPocket = 0x80;
        const Capture = 0x100;
        const NoAttack = 0x200;
        const SpeedDamage = 0x400;
        const Explosion30 = 0x800;
        const Explosion10 = 0x1000;
        const Eatable = 0x2000;
        const NotShot = 0x4000;
    }
}