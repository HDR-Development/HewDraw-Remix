#[repr(u32)]
pub enum FighterEdgeFinalModuleCall {
    StartInit = 0x0,
    StartExit = 0x1,
    ReadyInit = 0x2,
    ReadyExec = 0x3,
    ReadyExitPre = 0x4,
    ReadyExit = 0x5,
    Scene01InitPre = 0x6,
    Scene01Init = 0x7,
    Scene01Exec = 0x8,
    Scene01Exit = 0x9,
    EndInit = 0xA,
    EndExec = 0xB,
    EndExit = 0xC,
    Num = 0xD,
}