use super::*;

use globals::*;

unsafe fn float_cancel(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[STATUS_KIND] == FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR {
        let prev_status = [
            StatusModule::prev_status_kind(fighter.module_accessor, 0),
            StatusModule::prev_status_kind(fighter.module_accessor, 1)
        ];

        if prev_status.iter().any(|x| [*FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT, *FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT_START].contains(x)) {
            StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_LANDING, true);
        }
    }
}

#[utils::opff(FIGHTER_KIND_PEACH)]
unsafe fn peach_frame(fighter: &mut L2CFighterCommon) {
    float_cancel(fighter);
}