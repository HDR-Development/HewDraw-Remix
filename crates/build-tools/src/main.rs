use std::path::Path;

pub fn main() {
    let rom_dst_path = Path::new(hdr_macros::rom_path!());
    let rom_src_path = Path::new(hdr_macros::rom_source_path!());
    build_tools::rebuild_romfs(rom_src_path, rom_dst_path);
}
