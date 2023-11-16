use super::*;
use globals::*;

//=================================================================
//== FighterStatusModuleImpl::set_fighter_status_data
//=================================================================
#[skyline::hook(replace=FighterStatusModuleImpl::set_fighter_status_data)]
unsafe fn set_fighter_status_data_hook(boma: &mut BattleObjectModuleAccessor, arg2: bool, treaded_kind: i32, arg4: bool, arg5: bool, arg6: bool, log_mask_flag: u64, status_attr: u32, power_up_attack_bit: u32, arg10: u32) {
    let id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let mut new_status_attr = status_attr;

    if boma.is_fighter() {

        // this handles turnaround special/b-reversible moves
        if (boma.kind() == *FIGHTER_KIND_BRAVE
            && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_START, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_FAILURE]))
        || (boma.kind() == *FIGHTER_KIND_CAPTAIN
            && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_LW]))
        || (boma.kind() == *FIGHTER_KIND_CLOUD
            && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW]))
        || (boma.kind() == *FIGHTER_KIND_DONKEY
            && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_N]))
        || (boma.kind() == *FIGHTER_KIND_IKE
            && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_S]))
        || (boma.kind() == *FIGHTER_KIND_KIRBY
            && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_S]))
            || (WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_CAPTAIN && boma.is_status(*FIGHTER_KIRBY_STATUS_KIND_CAPTAIN_SPECIAL_N))
            || (WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_DONKEY && boma.is_status(*FIGHTER_KIRBY_STATUS_KIND_DONKEY_SPECIAL_N))
            || (WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_LITTLEMAC && boma.is_status_one_of(&[*FIGHTER_KIRBY_STATUS_KIND_LITTLEMAC_SPECIAL_N, *FIGHTER_KIRBY_STATUS_KIND_LITTLEMAC_SPECIAL_N_START]))
            || (WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_PACMAN && boma.is_status(*FIGHTER_KIRBY_STATUS_KIND_PACMAN_SPECIAL_N))
        || (boma.kind() == *FIGHTER_KIND_KROOL
            && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_START, *FIGHTER_STATUS_KIND_SPECIAL_LW]))
        || (boma.kind() == *FIGHTER_KIND_LINK
            && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_LINK_STATUS_KIND_SPECIAL_LW_BLAST]))
        || (boma.kind() == *FIGHTER_KIND_LITTLEMAC
            && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N_START, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_HI_START]))
        || (boma.kind() == *FIGHTER_KIND_MARIO
            && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT]))
        || (boma.kind() == *FIGHTER_KIND_MIIGUNNER
            && boma.is_status_one_of(&[*FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_1_GROUND, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_1_AIR, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_2_GROUND, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_2_AIR]))
        || (boma.kind() == *FIGHTER_KIND_PACMAN
            && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_N]))
        || (boma.kind() == *FIGHTER_KIND_PIKMIN
            && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW]))
        || (boma.kind() == *FIGHTER_KIND_RICHTER
            && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW]))
        || (boma.kind() == *FIGHTER_KIND_SAMUS
            && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1G, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2G, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A]))
        || (boma.kind() == *FIGHTER_KIND_SIMON
            && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW]))
        || (boma.kind() == *FIGHTER_KIND_TOONLINK
            && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW]))
        || (boma.kind() == *FIGHTER_KIND_YOUNGLINK
            && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW]))
        || (boma.kind() == *FIGHTER_KIND_ZELDA
            && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_2]))
        || (boma.kind() == *FIGHTER_KIND_SHIZUE
            && boma.is_status_one_of(&[*FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_START, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_LW_FIRE]))
        || (boma.kind() == *FIGHTER_KIND_PALUTENA
            && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW]))
        || (boma.kind() == *FIGHTER_KIND_KAMUI
            && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_LW_HIT]))
        || (boma.kind() == *FIGHTER_KIND_MURABITO
            && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW])
            && boma.is_situation(*SITUATION_KIND_AIR))
        || (boma.kind() == *FIGHTER_KIND_DEDEDE
            && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_LW]))
        || (boma.kind() == *FIGHTER_KIND_MIIFIGHTER
            && ((WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_WAZA_CUSTOMIZE_TO) == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_LW_1 && boma.is_status(*FIGHTER_STATUS_KIND_SPECIAL_LW))
            || (WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_WAZA_CUSTOMIZE_TO) == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_LW_3 && boma.is_status(*FIGHTER_STATUS_KIND_SPECIAL_LW))))
        || (boma.kind() == *FIGHTER_KIND_MIISWORDSMAN
            && (WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_WAZA_CUSTOMIZE_TO) == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_LW_3 && boma.is_status(*FIGHTER_STATUS_KIND_SPECIAL_LW)))
        {
            // if b-reverse flag does not already exist in status_attr bitmask
            if status_attr & *FIGHTER_STATUS_ATTR_START_TURN as u32 == 0 {
                // add b-reverse flag to status_attr bitmask
                new_status_attr = status_attr + *FIGHTER_STATUS_ATTR_START_TURN as u32;
            }
        }

    }

    original!()(boma, arg2, treaded_kind, arg4, arg5, arg6, log_mask_flag, new_status_attr, power_up_attack_bit, arg10)
}

pub fn install() {
    skyline::install_hooks!(
        set_fighter_status_data_hook,
    );
}
