use std::{
    collections::HashSet,
    sync::{LazyLock, Mutex},
};

use crate::StagePage;

pub static STAGE_MANAGER: LazyLock<Mutex<StageManager>> = LazyLock::new(|| Mutex::new(StageManager::new()));

pub struct StageManager {
    pub selected_panel: Option<i32>,
    pub selected_preview: Option<i32>,
    pub stage_pages: Option<Vec<StagePage>>,
    pub random_stage_indexes: Option<Vec<i32>>,
    pub stage_loading: Option<bool>,
    pub perma_striked_stages: HashSet<i32>,
}

impl StageManager {
    pub fn new() -> Self {
        Self {
            selected_panel: None,
            selected_preview: None,
            stage_pages: None,
            random_stage_indexes: None,
            stage_loading: None,
            perma_striked_stages: HashSet::new(),
        }
    }
}
