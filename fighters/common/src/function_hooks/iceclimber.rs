use super::*;

pub fn install() {
    unsafe {
        // Patches out disabling grabs
        skyline::patching::Patch::in_text(0xfba284).nop();
        skyline::patching::Patch::in_text(0xfb9780).nop();
        skyline::patching::Patch::in_text(0xfb9794).nop();
        skyline::patching::Patch::in_text(0xfb97a8).nop();

        // No more cheering or crying.
        skyline::patching::Patch::in_text(0xfb63e8).data(0x17FFFFE2_u32);
        skyline::patching::Patch::in_text(0xfba298).data(0x1400060C_u32);
        skyline::patching::Patch::in_text(0xfba52c).nop();
        skyline::patching::Patch::in_text(0xfba850).data(0x1400049E_u32);

        // fixes nana not inputting grab when popo is actionable
        skyline::patching::Patch::in_text(0x2f7dcc).data(0x140000E1_u32);
    }
}