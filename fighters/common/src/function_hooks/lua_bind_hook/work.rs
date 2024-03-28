use super::*;
use globals::*;

#[skyline::hook(replace=WorkModule::is_flag)]
unsafe fn is_flag_hook(boma: &mut BattleObjectModuleAccessor, flag: i32) -> bool {
    if boma.is_fighter() {
        if flag == *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_LANDING_TURN {
            return true;
        }
        else if flag == *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR {
            if boma.is_fighter()
            && (boma.kind() == *FIGHTER_KIND_CLOUD && boma.is_status(*FIGHTER_CLOUD_STATUS_KIND_SPECIAL_LW_CHARGE))
            || (boma.kind() == *FIGHTER_KIND_EDGE && boma.is_status(*FIGHTER_STATUS_KIND_SPECIAL_N))
            || (boma.kind() == *FIGHTER_KIND_LUCARIO && boma.is_status(*FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_HOLD))
            || (boma.kind() == *FIGHTER_KIND_MEWTWO && boma.is_status(*FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_N_HOLD))
            || (boma.kind() == *FIGHTER_KIND_MIIGUNNER && boma.is_status(*FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N1_HOLD))
            || (boma.kind() == *FIGHTER_KIND_SHEIK && boma.is_status(*FIGHTER_SHEIK_STATUS_KIND_SPECIAL_N_LOOP)) 
            || (boma.kind() == *FIGHTER_KIND_DONKEY && boma.is_status(*FIGHTER_DONKEY_STATUS_KIND_SPECIAL_N_LOOP)) 
            || (boma.kind() == *FIGHTER_KIND_MASTER && boma.is_status(*FIGHTER_STATUS_KIND_SPECIAL_N)) 
            || (boma.kind() == *FIGHTER_KIND_PACKUN && boma.is_status(*FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_CHARGE))
            || (boma.kind() == *FIGHTER_KIND_SAMUS && boma.is_status(*FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_H))
            || (boma.kind() == *FIGHTER_KIND_SAMUSD && boma.is_status(*FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_H))
            || (boma.kind() == *FIGHTER_KIND_WIIFIT && boma.is_status(*FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_N_HOLD))
            || (boma.kind() == *FIGHTER_KIND_BRAVE && boma.is_status(*FIGHTER_BRAVE_STATUS_KIND_SPECIAL_N_HOLD))
            || (boma.kind() == *FIGHTER_KIND_PACMAN && boma.is_status(*FIGHTER_PACMAN_STATUS_KIND_SPECIAL_N_HOLD))
            || (boma.kind() == *FIGHTER_KIND_REFLET && boma.is_status(*FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_HOLD))
            || (boma.kind() == *FIGHTER_KIND_KIRBY && !(WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_NONE) && boma.is_status_one_of(&[
                *FIGHTER_KIRBY_STATUS_KIND_EDGE_SPECIAL_N,
                *FIGHTER_KIRBY_STATUS_KIND_LUCARIO_SPECIAL_N_HOLD,
                *FIGHTER_KIRBY_STATUS_KIND_MEWTWO_SPECIAL_N_HOLD,
                *FIGHTER_KIRBY_STATUS_KIND_MIIGUNNER_SPECIAL_N1_HOLD,
                *FIGHTER_KIRBY_STATUS_KIND_SHEIK_SPECIAL_N_LOOP,
                *FIGHTER_KIRBY_STATUS_KIND_DONKEY_SPECIAL_N_LOOP,
                *FIGHTER_KIRBY_STATUS_KIND_MASTER_SPECIAL_N,
                *FIGHTER_KIRBY_STATUS_KIND_SAMUS_SPECIAL_N_H,
                *FIGHTER_KIRBY_STATUS_KIND_WIIFIT_SPECIAL_N_HOLD,
                *FIGHTER_KIRBY_STATUS_KIND_BRAVE_SPECIAL_N_HOLD,
                *FIGHTER_KIRBY_STATUS_KIND_PACMAN_SPECIAL_N_HOLD,
                *FIGHTER_KIRBY_STATUS_KIND_REFLET_SPECIAL_N_HOLD,
                ])) {
                return false;
            }
        }
    }
    original!()(boma, flag)
}

pub fn install() {
    skyline::install_hooks!(
        is_flag_hook
    );
}