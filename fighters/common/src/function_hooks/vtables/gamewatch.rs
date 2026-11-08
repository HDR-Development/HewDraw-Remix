pub fn install() {
    // Patches the const that the game checks for the 9 Critical Hit.
    let _ = skyline::patching::Patch::in_text(0xa82734).data(0x52801841u32);
}