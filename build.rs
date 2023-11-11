use std::path::Path;

fn main() {
    let rom_src_path = Path::new(hdr_macros::rom_source_path!());
    println!(
        "cargo:rerun-if-changed={}",
        rom_src_path.join("../rom_watch.txt").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        rom_src_path.join("../agent_params.txt").display()
    );
    println!("cargo:rerun-if-changed={}", rom_src_path.display());
    println!("cargo:rustc-env=CARGO_NET_GIT_FETCH_WITH_CLI=true"); // force cargo to use git to fetch repos
}
