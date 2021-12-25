static ROM_WATCH: &'static str = include_str!("./rom_watch.txt");

fn main() {
    let rom_path = hdr_macros::rom_path!();
    for file in ROM_WATCH.lines() {
        println!("cargo:rerun-if-changed={}{}", rom_path, file);
    }
    println!("cargo:rerun-if-changed=agent_params.txt");
}