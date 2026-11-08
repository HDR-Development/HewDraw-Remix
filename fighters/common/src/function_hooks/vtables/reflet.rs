pub fn install() {
    // Allows Robin to start with Levin Sword in normal matches
    skyline::patching::Patch::in_text(0x1005d30).nop();
}