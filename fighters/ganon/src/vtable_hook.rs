use super::*;

#[skyline::hook(offset = 0xaa6800)]
pub unsafe extern "C" fn ganon_status_transition(_vtable: u64, fighter: &mut Fighter) {
    let module_accessor = fighter.battle_object.module_accessor;
    let prev_status = StatusModule::prev_status_kind(module_accessor, 0) as u64;
    let status = StatusModule::status_kind(module_accessor) as u64;
    if prev_status < 0x36 {
        if 1 << (prev_status & 0x3f) & 0xe00000000000u64 != 0
        && status & 0xfffffffe != 0x2e {
            ArticleModule::remove_exist(module_accessor, 1, ArticleOperationTarget(0));
        }
        if 1 << (prev_status & 0x3f) & 0x7000000000000u64 != 0
        && 1 < status - 0x31 {
            ArticleModule::remove_exist(module_accessor, 1, ArticleOperationTarget(0));
        }
        if 1 << (prev_status & 0x3f) & 0x38000000000000u64 != 0
        && status & 0xfffffffe != 0x34 {
            ArticleModule::remove_exist(module_accessor, 1, ArticleOperationTarget(0));
        }
        if prev_status == 0x2b {
            ArticleModule::remove_exist(module_accessor, 1, ArticleOperationTarget(0));
        }
    }
    if prev_status == 0x24 {
        if status != 0x25 {
            ArticleModule::remove_exist(module_accessor, 1, ArticleOperationTarget(0));
        }
    }
    else if prev_status == 0x25 {
        ArticleModule::remove_exist(module_accessor, 1, ArticleOperationTarget(0));
    }
    else if [0x2b, 0x2f, 0x32, 0x35].contains(&prev_status) {
        if status == 0x24 {
            ArticleModule::generate_article_enable(module_accessor, 1, false, -1);
        }
    }
}

pub fn install() {
    skyline::install_hooks!(
        ganon_status_transition
    );
}