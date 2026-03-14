#![allow(improper_ctypes)]
#![allow(improper_ctypes_definitions)]
use std::{
    collections::{BTreeMap, HashMap},
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use hash40::Hash40;
use motion_lib::mlist::MList;
use motion_patch::{MotionMapExt, MotionPatch};
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use walkdir::WalkDir;

extern "C" {
    fn arcrop_register_event_callback(ty: u32, callback: extern "C" fn(u32));
    fn arcrop_register_callback(
        hash: Hash40,
        length: usize,
        cb: extern "C" fn(Hash40, *mut u8, usize, &mut usize) -> bool,
    ) -> bool;
    fn arcrop_load_file(hash: Hash40, buf: *mut u8, buf_len: usize, out_size: &mut usize) -> bool;
    fn initial_loading(ctx: &skyline::hooks::InlineCtx);
}

static MAP: Lazy<RwLock<HashMap<Hash40, Arc<PathBuf>>>> = Lazy::new(|| RwLock::new(HashMap::new()));
static DATA_MAP: Lazy<RwLock<HashMap<Hash40, Arc<MList>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

fn check_only_body(path: &str) {
    let root = Path::new(path);
    if !root.exists() {
        return;
    }

    for entry in WalkDir::new(root).min_depth(1).max_depth(1) {
        let Ok(entry) = entry else {
            continue;
        };

        let target_path = entry.path().join("motion/body/motion_patch.yaml");
        if !target_path.exists() {
            continue;
        }

        install_for_all_costumes(target_path);
    }
}

fn check_all(path: &str) {
    let root = Path::new(path);
    if !root.exists() {
        return;
    }

    for entry in WalkDir::new(root).min_depth(1).max_depth(1) {
        let Ok(entry) = entry else {
            continue;
        };

        for entry in WalkDir::new(entry.path().join("motion"))
            .min_depth(1)
            .max_depth(1)
        {
            let Ok(entry) = entry else {
                continue;
            };

            if !entry.file_type().is_dir() {
                continue;
            }

            let target_path = entry.path().join("motion_patch.yaml");
            if !target_path.exists() {
                continue;
            }

            install_for_all_costumes(target_path);
        }
    }
}

fn install_for_all_costumes(path: PathBuf) {
    let mut map = MAP.write();
    let mut data_map = DATA_MAP.write();

    let path = Arc::new(path);
    let parent = path.parent().unwrap();

    let mut data: Option<Arc<MList>> = None;

    let defaults = ["c00", "c01", "c02", "c03", "c04", "c05", "c06", "c07"];

    for name in defaults.iter() {
        let entry = parent.join(name);
        let mlist_path = entry.join("motion_list.bin");
        let hash = Hash40::new(mlist_path.to_str().unwrap().strip_prefix("mods:/").unwrap());

        map.insert(hash, path.clone());
        unsafe {
            arcrop_register_callback(hash, 5 * 1024 * 1024, callback);

            if USE_CACHE {
                let data = if let Some(data) = data.as_ref() {
                    data.clone()
                } else {
                    let motion_list = motion_lib::open(
                        Path::new("arc:/")
                            .join(parent.strip_prefix("mods:/").unwrap())
                            .join("c00/motion_list.bin"),
                    )
                    .unwrap();

                    let arc = Arc::new(motion_list);
                    data = Some(arc.clone());
                    arc
                };

                data_map.insert(hash, data);
            }
        }
    }

    for entry in WalkDir::new(parent).min_depth(1).max_depth(1) {
        let Ok(entry) = entry else {
            continue;
        };

        if !entry.file_type().is_dir() {
            continue;
        }

        if defaults.contains(&entry.file_name().to_str().unwrap()) {
            continue;
        }

        let mlist_path = entry.path().join("motion_list.bin");
        let hash = Hash40::new(mlist_path.to_str().unwrap().strip_prefix("mods:/").unwrap());

        map.insert(hash, path.clone());
        unsafe {
            arcrop_register_callback(hash, 5 * 1024 * 1024, callback);

            if USE_CACHE {
                let data = if let Some(data) = data.as_ref() {
                    data.clone()
                } else {
                    let motion_list = motion_lib::open(
                        Path::new("arc:/")
                            .join(parent.strip_prefix("mods:/").unwrap())
                            .join("c00/motion_list.bin"),
                    )
                    .unwrap();

                    let arc = Arc::new(motion_list);
                    data = Some(arc.clone());
                    arc
                };

                data_map.insert(hash, data);
            }
        }
    }
}

extern "C" fn callback(hash: Hash40, ptr: *mut u8, size: usize, out_size: &mut usize) -> bool {
    static RECURSION_GUARD: AtomicBool = AtomicBool::new(false);
    if RECURSION_GUARD.swap(true, Ordering::SeqCst) {
        return false;
    }

    unsafe {
        let mut mlist = None;

        if USE_CACHE {
            if let Some(list) = (*DATA_MAP.data_ptr()).get(&hash) {
                mlist = Some(list.clone());
            }
        }

        let time = std::time::Instant::now();
        if mlist.is_none() {
            let mut base_out_size = 0usize;

            if !arcrop_load_file(hash, ptr, size, &mut base_out_size) {
                panic!("No vanilla file to load");
            }
            println!("Loading original took {}s", time.elapsed().as_secs_f32());

            let source = motion_lib::open(format!("arc:/{hash}")).unwrap();
            println!("Parsing original took {}s", time.elapsed().as_secs_f32());
            mlist = Some(Arc::new(source));
        }

        let mut source = (*mlist.unwrap()).clone();

        let Some(path) = (*MAP.data_ptr()).get(&hash).cloned() else {
            RECURSION_GUARD.store(false, Ordering::SeqCst);
            return false;
        };
        let patch: BTreeMap<Hash40, MotionPatch> =
            serde_yaml::from_str(std::fs::read_to_string(path.as_path()).unwrap().as_str())
                .unwrap();
        patch.apply(&mut source);

        let mut data = std::io::Cursor::new(vec![]);
        motion_lib::write_stream(&mut data, &source).unwrap();
        println!("Writing new took {}s", time.elapsed().as_secs_f32());
        let data = data.into_inner();
        if data.len() > size {
            panic!("Motion list too big!!");
        }

        let slice = std::slice::from_raw_parts_mut(ptr, data.len());
        slice[..data.len()].copy_from_slice(&data);
        *out_size = data.len();
    }

    RECURSION_GUARD.store(false, Ordering::SeqCst);
    true
}

static mut USE_CACHE: bool = false;

#[skyline::hook(replace = initial_loading)]
unsafe fn init_loading_hook(ctx: &skyline::hooks::InlineCtx) {
    call_original!(ctx);
    for entry in walkdir::WalkDir::new("mods:/") {
        let Ok(entry) = entry else {
            continue;
        };
        if !entry.file_type().is_file() {
            continue;
        }

        if entry.path().file_name().unwrap().to_str().unwrap() == "motion_patch.yaml" {
            install_for_all_costumes(entry.path().to_path_buf());
        }
    }
}

extern "C" fn on_mod_fs_init(_: u32) {
    for entry in walkdir::WalkDir::new("mods:/") {
        let Ok(entry) = entry else {
            continue;
        };
        if !entry.file_type().is_file() {
            continue;
        }

        if entry.path().file_name().unwrap().to_str().unwrap() == "motion_patch.yaml" {
            install_for_all_costumes(entry.path().to_path_buf());
        }
    }
}

pub fn run(cache: bool) {
    unsafe {
        USE_CACHE = cache;
    }

    check_only_body("mods:/assist");
    check_only_body("mods:/boss");
    check_only_body("mods:/item");
    check_all("mods:/fighter");
}

pub fn install(cache: bool) {
    unsafe {
        USE_CACHE = cache;
        arcrop_register_event_callback(1, on_mod_fs_init);
    }
}
