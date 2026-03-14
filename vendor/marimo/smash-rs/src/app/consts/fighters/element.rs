#[repr(u32)]
pub enum FighterElementFinalModuleCall {
    StartInit = 0x0,
    StartExit = 0x1,
    ReadyInit = 0x2,
    ReadyExec = 0x3,
    ReadyExitPre = 0x4,
    ReadyExit = 0x5,
    Scene01Init = 0x6,
    Scene01Exec = 0x7,
    Scene01Exit = 0x8,
    Scene02Init = 0x9,
    Scene02Exec = 0xA,
    Scene02Exit = 0xB,
    EndInit = 0xC,
    EndExec = 0xD,
    EndExit = 0xE,
    Num = 0xF,
}
