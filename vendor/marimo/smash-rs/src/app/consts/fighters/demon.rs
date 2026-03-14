#[repr(u32)]
pub enum FighterDemonFinalModuleCall {
    StartInit = 0x0,
    StartExit = 0x1,
    ReadyInit = 0x2,
    ReadyExec = 0x3,
    ReadyExit = 0x4,
    Scene01Init = 0x5,
    Scene01Exec = 0x6,
    Scene01Exit = 0x7,
    EndInit = 0x8,
    EndExec = 0x9,
    EndExit = 0xA,
    Num = 0xB,
}