bitflags! {
    #[repr(C)]
    #[allow(non_upper_case_globals)]
    pub struct FighterFacial: u32 {
        const None = 0x1;
        const Curry = 0x2;
        const Damage = 0x4;
        const Special = 0x8;
        const Pikmin = 0x10;
        const FinalFear = 0x20;
        const BombFear = 0x40;
        const Special2 = 0x80;
        const Finish = 0x100;
        const Poison = 0x200;
        const Stop = 0x400;
        const Sleep = 0x800;
        const Fistdown = 0x1000;
    }
}

#[repr(C)]
pub struct FighterAvailableFinal(u32);