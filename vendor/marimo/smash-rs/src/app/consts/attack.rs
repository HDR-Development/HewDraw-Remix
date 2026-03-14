#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum AttackSetOffKind {
    Off = 0x0,
    On = 0x1,
    Thru = 0x2,
    NoStop = 0x3,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum AttackLRCheck {
    Pos = 0x0,
    Speed = 0x1,
    LR = 0x2,
    Forward = 0x3,
    Backward = 0x4,
    Part = 0x5,
    BackSlash = 0x6,
    Left = 0x7,
    Right = 0x8,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum AttackRegion {
    None = 0x0,
    Head = 0x1,
    Body = 0x2,
    Hip = 0x3,
    Punch = 0x4,
    Elbow = 0x5,
    Kick = 0x6,
    Knee = 0x7,
    Throw = 0x8,
    Object = 0x9,
    Sword = 0xA,
    Hammer = 0xB,
    Bomb = 0xC,
    Spin = 0xD,
    Bite = 0xE,
    Magic = 0xF,
    PSI = 0x10,
    Palutena = 0x11,
    Aura = 0x12,
    Bat = 0x13,
    Parasol = 0x14,
    Pikmin = 0x15,
    Water = 0x16,
    Whip = 0x17,
    Tail = 0x18,
    Energy = 0x19,
}
