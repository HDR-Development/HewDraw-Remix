use std::path::Path;

use walkdir::WalkDir;

pub fn rebuild_xml_to_prc(root_src_path: &Path, root_dst_path: &Path) {
    let file =
        std::fs::File::open(root_src_path).expect("Root path did not contain valid UTF-8 data!");
    let mut file_reader = std::io::BufReader::new(file);
    let prc_struct =
        prc::xml::read_xml(&mut file_reader).expect("Failed to parse XML into PRC struct");
    prc::save(root_dst_path, &prc_struct).expect("Failed to write PRC to output path!");
}

pub fn rebuild_romfs(root_src_path: &Path, root_dst_path: &Path) {
    if root_dst_path.exists() {
        std::fs::remove_dir_all(root_dst_path).unwrap();
    }
    for entry in WalkDir::new(root_src_path) {
        if let Ok(entry) = entry {
            if entry.file_type().is_file() {
                let path = entry.path();
                let local_path = path
                    .strip_prefix(root_src_path)
                    .expect("Path in root was not in root! Possible symlink?");
                if let Some(parent) = local_path.parent() {
                    std::fs::create_dir_all(root_dst_path.join(parent))
                        .expect("Cannot create path in destination root!");
                    if let Some(extension) = local_path.extension() {
                        if extension == "xml" {
                            rebuild_xml_to_prc(
                                path,
                                &root_dst_path.join(local_path).with_extension("prc"),
                            );
                        } else if extension == "lua" {
                            std::fs::copy(
                                path,
                                &root_dst_path.join(local_path).with_extension("lc"),
                            )
                            .unwrap();
                        } else {
                            std::fs::copy(path, &root_dst_path.join(local_path)).unwrap();
                        }
                    }
                }
            }
        }
    }
}
