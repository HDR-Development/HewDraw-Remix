use super::*;
use globals::*;
//=================================================================
//== init_settings for edge_slipoffs module
//== Note: This is called from init_settings::init_settings_hook
//== Note: Forces GroundModule::correct to be called for
//         certain statuses
//== Note: JostleModule::set_team(boma, 0) is for walking through
//         other fighters
//=================================================================
pub unsafe fn init_settings_edges(boma: &mut BattleObjectModuleAccessor, situation: smash::app::SituationKind, arg3: i32, arg4: u32,
                              ground_cliff_check_kind: smash::app::GroundCliffCheckKind, arg6: bool,
                              arg7: i32, arg8: i32, arg9: i32, arg10: i32) -> u32 {
    /* "fix" forces GroundModule::correct to be called for the statuses we need */
    let mut fix = arg4;
    let fighter_kind = boma.kind();
    let status_kind = StatusModule::status_kind(boma);

    if boma.is_fighter() {

        if status_kind == *FIGHTER_STATUS_KIND_APPEAL {
            fix = *GROUND_CORRECT_KIND_GROUND as u32; /* GROUND_CORRECT_KIND_GROUND is i32 */
        }

        if [*FIGHTER_STATUS_KIND_WAIT,
            *FIGHTER_STATUS_KIND_DASH,
            *FIGHTER_STATUS_KIND_TURN,
            *FIGHTER_STATUS_KIND_TURN_DASH,
            *FIGHTER_STATUS_KIND_SQUAT,
            *FIGHTER_STATUS_KIND_SQUAT_WAIT,
            *FIGHTER_STATUS_KIND_SQUAT_F,
            *FIGHTER_STATUS_KIND_SQUAT_B,
            *FIGHTER_STATUS_KIND_SQUAT_RV,
            *FIGHTER_STATUS_KIND_LANDING,
            *FIGHTER_STATUS_KIND_LANDING_LIGHT,
            *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR,
            *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL,
            *FIGHTER_STATUS_KIND_LANDING_DAMAGE_LIGHT,
            *FIGHTER_STATUS_KIND_GUARD_DAMAGE,
            // *FIGHTER_STATUS_KIND_ESCAPE_AIR,
            // *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE,
            *FIGHTER_STATUS_KIND_DAMAGE,
            *FIGHTER_STATUS_KIND_AIR_LASSO_LANDING].contains(&status_kind) {
            fix = *GROUND_CORRECT_KIND_GROUND as u32;
        }

        if    (fighter_kind == *FIGHTER_KIND_EDGE && [*FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH, /* *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_CHARGED_RUSH,*/ *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_LANDING].contains(&status_kind) && StatusModule::prev_status_kind(boma, 0) != *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_CHARGED_RUSH)
           || (fighter_kind == *FIGHTER_KIND_KAMUI && [*FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_B,
                                                       *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_F,
                                                       *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_B_LANDING,
                                                       *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_F_LANDING].contains(&status_kind))
           || (fighter_kind == *FIGHTER_KIND_MIIFIGHTER && [*FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW2_START,
                                                            *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW2_KICK,
                                                            *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW2_LANDING,
                                                            *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW2_KICK_LANDING].contains(&status_kind))
           || (fighter_kind == *FIGHTER_KIND_MIISWORDSMAN && [*FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI1_END, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_END].contains(&status_kind))
           || (fighter_kind == *FIGHTER_KIND_SZEROSUIT && [*FIGHTER_SZEROSUIT_STATUS_KIND_SPECIAL_LW_FLIP,
                                                           *FIGHTER_SZEROSUIT_STATUS_KIND_SPECIAL_LW_KICK,
                                                           *FIGHTER_SZEROSUIT_STATUS_KIND_SPECIAL_LW_LANDING,
                                                           *FIGHTER_SZEROSUIT_STATUS_KIND_SPECIAL_LW_KICK_LANDING].contains(&status_kind))
           || (fighter_kind == *FIGHTER_KIND_BAYONETTA && [*FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D,
                                                           *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D_LANDING].contains(&status_kind))
           || (fighter_kind == *FIGHTER_KIND_DOLLY && [*FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_ATTACK,
                                                       *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_LANDING].contains(&status_kind)) 
           || (boma.kind() == *FIGHTER_KIND_KOOPAJR && boma.is_status(*FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_LANDING))
           || (boma.kind() == *FIGHTER_KIND_SHEIK && [*FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_ATTACK,
                                                       *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_LANDING].contains(&status_kind)) {
            fix = *GROUND_CORRECT_KIND_GROUND as u32;
        }
    }
    return fix
}

//=================================================================
//== GroundModule::correct
//== Note: This is the "can edge slippoff" function in Smash
//=================================================================
#[skyline::hook(replace=GroundModule::correct)]
unsafe fn correct_hook(boma: &mut BattleObjectModuleAccessor, kind: GroundCorrectKind) -> u64 {
    let status_kind = StatusModule::status_kind(boma);
    let situation_kind = StatusModule::situation_kind(boma);
    let fighter_kind = boma.kind();

    // It seems like everything gets caught as "landing"
    if boma.is_fighter() {
        if [
            *FIGHTER_STATUS_KIND_LANDING,
            *FIGHTER_STATUS_KIND_TURN_DASH,
            *FIGHTER_STATUS_KIND_DASH,
            *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL].contains(&status_kind) {
            return original!()(boma, GroundCorrectKind(1));
        }

        if ((fighter_kind == *FIGHTER_KIND_PIKACHU || fighter_kind == *FIGHTER_KIND_PICHU) &&
                [*FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_WEAK,
                 *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_ATTACK,
                 *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_END].contains(&status_kind))
            || (fighter_kind == *FIGHTER_KIND_CAPTAIN && status_kind == *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_END)
            || (fighter_kind == *FIGHTER_KIND_GANON && status_kind == *FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_END)
            || (fighter_kind == *FIGHTER_KIND_MIISWORDSMAN && [*FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_LW3_END].contains(&status_kind))
            || (fighter_kind == *FIGHTER_KIND_KOOPA && status_kind == *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_G)
            || (fighter_kind == *FIGHTER_KIND_DONKEY && situation_kind == *SITUATION_KIND_GROUND && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI)
            || (fighter_kind == *FIGHTER_KIND_GAOGAEN && situation_kind == *SITUATION_KIND_GROUND && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N)
            || (fighter_kind == *FIGHTER_KIND_LUIGI && situation_kind == *SITUATION_KIND_GROUND && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N) {
            return original!()(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        }
    }

    original!()(boma, kind)
}

extern "C" {
    #[link_name = "\u{1}_ZN3app11FighterUtil33get_ground_correct_kind_air_transERNS_26BattleObjectModuleAccessorEi"]
    fn get_ground_correct_kind_air_trans(boma: &mut BattleObjectModuleAccessor, something: i32) -> i32;
}

//=================================================================
//== FighterUtil::get_ground_correct_kind_air_trans
//== Note: Aerial ECB fixes for Link, Captain, Simon, Richter
//=================================================================
#[skyline::hook(replace=get_ground_correct_kind_air_trans)]
unsafe fn get_ground_correct_kind_air_trans_hook(boma: &mut BattleObjectModuleAccessor, something: i32) -> i32 {
    return *GROUND_CORRECT_KIND_AIR;
}

pub fn install() {
    skyline::install_hooks!(
        correct_hook,
        get_ground_correct_kind_air_trans_hook,
    );
}
