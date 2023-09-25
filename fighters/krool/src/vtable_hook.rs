use super::*;

// Covers Canonball Suction (Applies to Kirby as well) and Crown Catch

#[skyline::hook(offset = 0xc06530)]
pub unsafe extern "C" fn krool_func(_vtable: u64, fighter: &mut Fighter) -> u64 {
    let module_accessor = (fighter.battle_object).module_accessor;
    let status = StatusModule::status_kind(module_accessor);
    let kind = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND);
    if (status == 0x1e2 || status == 0x319)
    && WorkModule::is_flag(module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_N_SUCTION_IRONBALL) {
        let status = if kind == *FIGHTER_KIND_KIRBY {
            0x31b
        }
        else {
            0x1e4
        };
        StatusModule::change_status_request(module_accessor, status, false);
    }
    else if kind != *FIGHTER_KIND_KIRBY
    && WorkModule::is_flag(module_accessor, 0x200000EC) { // FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_CATCH_CROWN
        if ArticleModule::is_exist(module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_CROWN) {
            if status == *FIGHTER_KROOL_STATUS_KIND_SPECIAL_S_FAILURE {
                StatusModule::change_status_request(module_accessor, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_S_CATCH, false);
                WorkModule::off_flag(module_accessor, 0x200000EB); // FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_DROP_CROWN
            }
        }
        WorkModule::off_flag(module_accessor, 0x200000EC);
    }
    0
}

pub fn install() {
    skyline::install_hooks!(
        krool_func
    );
}
