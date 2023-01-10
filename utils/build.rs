use std::path::Path;
use walkdir::WalkDir;

static ROM_WATCH: &'static str = include_str!("./rom_watch.txt");

fn rebuild_xml_to_prc(root_src_path: &Path, root_dst_path: &Path) {
    let file = std::fs::File::open(root_src_path).expect("Root path did not contain valid UTF-8 data!");
    let mut file_reader = std::io::BufReader::new(file);
    let prc_struct = prc::xml::read_xml(&mut file_reader).expect("Failed to parse XML into PRC struct");
    prc::save(root_dst_path, &prc_struct).expect("Failed to write PRC to output path!");
}

fn rebuild_romfs(root_src_path: &Path, root_dst_path: &Path) {
    for entry in WalkDir::new(root_src_path) {
        if let Ok(entry) = entry { 
            if entry.file_type().is_file() {
                let path = entry.path();
                let local_path = path.strip_prefix(root_src_path).expect("Path in root was not in root! Possible symlink?");
                if let Some(parent) = local_path.parent() {
                    std::fs::create_dir_all(root_dst_path.join(parent)).expect("Cannot create path in destination root!");
                    if let Some(extension) = local_path.extension() {
                        if extension == "xml" {
                            rebuild_xml_to_prc(path, &root_dst_path.join(local_path).with_extension("prc"));
                        } else if extension == "lua" {
                            std::fs::copy(path, &root_dst_path.join(local_path).with_extension("lc"));
                        } else {
                            std::fs::copy(path, &root_dst_path.join(local_path));
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let rom_dst_path = Path::new(hdr_macros::rom_path!());
    let rom_src_path = Path::new(hdr_macros::rom_source_path!());
    rebuild_romfs(rom_src_path, rom_dst_path);
    let rom_path = hdr_macros::rom_source_path!();
    for file in ROM_WATCH.lines() {
        println!("cargo:rerun-if-changed={}{}", rom_path, file);
    }
    println!("cargo:rerun-if-changed=agent_params.txt");
}