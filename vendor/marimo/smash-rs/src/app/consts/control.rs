bitflags! {
    pub struct ControlPadButton: i32 {
        const Attack      = 0x1;
        const Special     = 0x2;
        const Jump        = 0x4;
        const Guard       = 0x8;
        const Catch       = 0x10;
        const Smash       = 0x20;
        const JumpMini    = 0x40;
        const CStickOn    = 0x80;
        const StockShare  = 0x100;
        const AttackRaw   = 0x200;
        const AppealHi    = 0x400;
        const SpecialRaw  = 0x800;
        const AppealLw    = 0x1000;
        const AppealSL    = 0x2000;
        const AppealSR    = 0x4000;
        const FlickJump   = 0x8000;
        const GuardHold   = 0x10000;
        const SpecialRaw2 = 0x20000;

        const SpecialAll  = 0x20802;
        const AttackAll   = 0x201;
        const AppealAll   = 0x7400;
    }
}

bitflags! {
    #[repr(C)]
    #[allow(non_upper_case_globals)]
    pub struct CommandCat1: u32 {
        const AttackN = 0x1;
        const AttackS3 = 0x2;
        const AttackHi3 = 0x4;
        const AttackLw3 = 0x8;
        const AttackS4 = 0x10;
        const AttackHi4 = 0x20;
        const AttackLw4 = 0x40;
        const AttackAirN = 0x80;
        const AttackAirF = 0x100;
        const AttackAirB = 0x200;
        const AttackAirHi = 0x400;
        const AttackAirLw = 0x800;
        const SpecialN = 0x1000;
        const SpecialS = 0x2000;
        const SpecialHi = 0x4000;
        const SpecialLw = 0x8000;
        const Walk = 0x10000;
        const Dash = 0x20000;
        const Turn = 0x40000;
        const TurnDash = 0x80000;
        const Jump = 0x100000;
        const JumpButton = 0x200000;
        const AirEscape = 0x400000;
        const Squat = 0x800000;
        const Escape = 0x1000000;
        const EscapeF = 0x2000000;
        const EscapeB = 0x4000000;
        const WallJumpLeft = 0x8000000;
        const WallJumpRight = 0x10000000;
        const Catch = 0x20000000;
        const NoCmd = 0x40000000;
        const SpecialAny = 0xF000;
    }
}

bitflags! {
    #[repr(C)]
    #[allow(non_upper_case_globals)]
    pub struct CommandCat2: u32 {
        const AppealSL = 0x1;
        const AppealSR = 0x2;
        const AppealHi = 0x4;
        const AppealLw = 0x8;
        const AppealSmash = 0x10;
        const AttackDashAttackHi4 = 0x20;
        const FallJump = 0x40;
        const DashAttackS4 = 0x80;
        const DamageFallToFall = 0x100;
        const DownToDownStandFb = 0x200;
        const DownToDownStand = 0x400;
        const GuardToPass = 0x800;
        const SquatToSquatF = 0x1000;
        const SquatToSquatB = 0x2000;
        const TurnToEscapeF = 0x4000;
        const TurnToEscapeB = 0x8000;
        const StickEscapeF = 0x10000;
        const StickEscapeB = 0x20000;
        const StickEscape = 0x40000;
        const SpecialNReverseLr = 0x80000;
        const ThrowF = 0x100000;
        const ThrowB = 0x200000;
        const ThrowHi = 0x400000;
        const ThrowLw = 0x800000;
        const CommonGuard = 0x1000000;
        const AirLasso = 0x2000000;
        const AttackN2 = 0x4000000;
        const FinalReverseLr = 0x8000000;
    }
}

bitflags! {
    #[repr(C)]
    #[allow(non_upper_case_globals)]
    pub struct CommandCat3: u32 {
        const ItemLightThrowFb4 = 0x1;
        const ItemLightThrowHi4 = 0x2;
        const ItemLightThrowLw4 = 0x4;
        const ItemLightThrowHi = 0x8;
        const ItemLightThrowLw = 0x10;
        const ItemLightDrop = 0x20;
        const ItemLightThrowFb = 0x40;
        const ItemLightThrowAirFb = 0x80;
        const ItemLightThrowAirFb4 = 0x100;
        const ItemLightThrowAirHi = 0x200;
        const ItemLightThrowAirHi4 = 0x400;
        const ItemLightThrowAirLw = 0x800;
        const ItemLightThrowAirLw4 = 0x1000;
        const ItemLightDropAir = 0x2000;
        const ItemHeavyThrowFb = 0x4000;
        const ItemGetAir = 0x8000;
        const SpecialSSmash = 0x10000;
        const SpecialSSmashDash = 0x20000;
        const ItemLightThrow = 0x58;
        const ItemLightThrowAir = 0xA80;
        const ItemLightThrow4 = 0x7;
        const ItemLightThrow4Air = 0x1500;
        const ItemLightThrowAll = 0x5F;
        const ItemLightThrowAirAll = 0x1F80;
    }
}

bitflags! {
    #[repr(C)]
    #[allow(non_upper_case_globals)]
    pub struct CommandCat4: u32 {
        const SpecialNCommand = 0x1;
        const SpecialN2Command = 0x2;
        const SpecialSCommand = 0x4;
        const SpecialHiCommand = 0x8;
        const Command6n6 = 0x10;
        const Command4n4 = 0x20;
        const AttackCommand1 = 0x40;
        const SpecialHi2Command = 0x80;
        const SuperSpecialCommand = 0x100;
        const SuperSpecialRCommand = 0x200;
        const SuperSpecial2Command = 0x400;
        const SuperSpecial2RCommand = 0x800;
        const Command623nb = 0x1000;
        const Command623strict = 0x2000;
        const Command623along = 0x4000;
        const Command623blong = 0x8000;
        const Command623a = 0x10000;
        const Command2 = 0x20000;
        const Command3 = 0x40000;
        const Command1 = 0x80000;
        const Command6 = 0x100000;
        const Command4 = 0x200000;
        const Command8 = 0x400000;
        const Command9 = 0x800000;
        const Command7 = 0x1000000;
        const Command6n6ab = 0x2000000;
        const Command323catch = 0x4000000;
    }
}
