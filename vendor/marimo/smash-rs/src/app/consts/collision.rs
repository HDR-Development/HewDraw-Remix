bitflags! {
    #[repr(C)]
    #[derive(Default)]
    #[allow(non_upper_case_globals)]
    pub struct CollisionCategoryMask: u16 {
        const Fighter = 0x1;
        const Enemy = 0x2;
        const Item = 0x4;
        const Gimmick = 0x8;
        const ItemE = 0x10;
        const Floor = 0x20;
    }
}

bitflags! {
    #[repr(C)]
    #[allow(non_upper_case_globals)]
    pub struct CollisionSituationMask: u8 {
        const Ground = 0x1;
        const Air = 0x2;
        const Odd = 0x4;
    }
}

bitflags! {
    #[repr(C)]
    #[allow(non_upper_case_globals)]
    pub struct CollisionPartMask: u16 {
        const Etc = 0x2;
        const Body = 0x9;
        const Legs = 0xC;
        const Head = 0x10;
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum CollisionSoundLevel {
    Small = 0x0,
    Medium = 0x1,
    Large = 0x2,
    ExtraLarge = 0x3,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum CollisionSoundAttr {
    None = 0x0,
    Punch = 0x1,
    Kick = 0x2,
    CutUp = 0x3,
    Coin = 0x4,
    Bat = 0x5,
    Harisen = 0x6,
    Elec = 0x7,
    Fire = 0x8,
    Water = 0x9,
    Grass = 0xA,
    Bomb = 0xB,
    PeachWeapon = 0xC,
    Snake = 0xD,
    Ike = 0xE,
    Dedede = 0xF,
    Magic = 0x10,
    KameHit = 0x11,
    PeachBinta = 0x12,
    PeachFryingPan = 0x13,
    PeachGolf = 0x14,
    PeachTennis = 0x15,
    PeachParasol = 0x16,
    DaisyBinta = 0x17,
    DaisyFryingPan = 0x18,
    DaisyGolf = 0x19,
    DaisyTennis = 0x1A,
    DaisyParasol = 0x1B,
    Lucario = 0x1C,
    MarthSword = 0x1D,
    MarthFinal = 0x1E,
    MarioCoin = 0x1F,
    MarioFinal = 0x20,
    LuigiCoin = 0x21,
    NessBat = 0x22,
    Freeze = 0x23,
    MarioFireball = 0x24,
    MarioCoinLast = 0x25,
    MarioMant = 0x26,
    FoxBlaster = 0x27,
    LuigiAttackDash = 0x28,
    LuigiSmash = 0x29,
    MarioWaterPump = 0x2A,
    PacmanBell = 0x2B,
    GuruguruHit = 0x2C,
    LizardonFire = 0x2D,
    TrainHit = 0x2E,
    MarioDCoinLast = 0x2F,
    MarioDMant = 0x30,
    MarioDCapsule = 0x31,
    PacmanWater = 0x32,
    MiiGunnerBlaster = 0x33,
    RefletFinalSword = 0x34,
    RefletFinalFire = 0x35,
    RefletFinalElec = 0x36,
    DuckhuntFinal = 0x37,
    ShulkFinalDanban = 0x38,
    ShulkFinalRiki = 0x39,
    FalcoBlaster = 0x3A,
    RyuPunch = 0x3B,
    RyuKick = 0x3C,
    LucasBat = 0x3D,
    RyuFinal01 = 0x3E,
    RyuFinal02 = 0x3F,
    RyuFinal03 = 0x40,
    CloudHit = 0x41,
    CloudSmash01 = 0x42,
    CloudSmash02 = 0x43,
    CloudSmash03 = 0x44,
    CloudFinal01 = 0x45,
    CloudFinal02 = 0x46,
    CloudFinal03 = 0x47,
    BayonettaHit01 = 0x48,
    BayonettaHit02 = 0x49,
    YoshiBiteHit = 0x4A,
    YoshiEggHit = 0x4B,
    RoyHit = 0x4C,
    ChromHit = 0x4D,
    FoxTail = 0x4E,
    Heavy = 0x4F,
    Slap = 0x50,
    ItemHammer = 0x51,
    InklingHit = 0x52,
    MarioLocalCoin = 0x53,
    MarioLocalCoinLast = 0x54,
    FamicomHit = 0x55,
    ZenigameShellHit = 0x56,
    SamusScrew = 0x57,
    SamusDScrew = 0x58,
    SamusScrewFinish = 0x59,
    SamusDScrewFinish = 0x5A,
    KenPunch = 0x5B,
    KenKick = 0x5C,
    KenFinal01 = 0x5D,
    KenFinal02 = 0x5E,
    KenFinal03 = 0x5F,
    ShizueHammer = 0x60,
    SimonWhip = 0x61,
    SimonCross = 0x62,
    RichterWhip = 0x63,
    RichterCross = 0x64,
    SheikFinalPunch = 0x65,
    SheikFinalKick = 0x66,
    MetaknightFinalHit = 0x67,
    RobotFinalHit = 0x68,
    KenShoryu = 0x69,
    DiddyScratch = 0x6A,
    MiiEnemyGBlaster = 0x6B,
    ToonlinkHit = 0x6C,
    JackShot = 0x6D,
    BraveCriticalHit = 0x6E,
    BuddyWing = 0x6F,
    DollyPunch = 0x70,
    DollyKick = 0x71,
    DollyCritical = 0x72,
    DollySuperSpecial01 = 0x73,
    MasterAxe = 0x74,
    MasterArrowMax = 0x75,
    MasterAttack100End = 0x76,
    TantanPunch01 = 0x77,
    TantanPunch02 = 0x78,
    TantanPunch03 = 0x79,
    TantanFinal = 0x7A,
    CloudFinalAppendHit01 = 0x7B,
    CloudFinalAppendHit02 = 0x7C,
    FlameFinal = 0x7D,
    DemonPunch01 = 0x7E,
    DemonPunch02 = 0x7F,
    DemonKick = 0x80,
    DemonCatchAttack = 0x81,
    DemonFinal = 0x82,
    DemonThrowCommand = 0x83,
    DemonAttackSquat4 = 0x84,
    DemonAttackLw3 = 0x85,
    DemonAppeal = 0x86,
    TrailSlash = 0x87,
    TrailStab = 0x88,
    TrailCleave = 0x89,
    TrailCleaveSingle = 0x8A,
    TrailKick = 0x8B,
    TrailFinal = 0x8C,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum CollisionShapeType {
    Sphere = 0x0,
    AABB = 0x1,
    Capsule = 0x2,
}
