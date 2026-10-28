pub fn install() {
    let _ = skyline::patching::Patch::in_text(0xd59134).data(0x2A1703E1_u32);
}
