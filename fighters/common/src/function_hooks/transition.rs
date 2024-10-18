use super::*;
use globals::*;

//=================================================================
//== WorkModule::is_enable_transition_term
//== Note: Disable transition terms
//==        - Airdodge out of tumble
//==        - Airdodge out of footstool during footstool lockout
//=================================================================
#[skyline::hook(replace=WorkModule::is_enable_transition_term)]
unsafe fn is_enable_transition_term_hook(boma: &mut BattleObjectModuleAccessor, flag: i32) -> bool {
    // Fighters
    if boma.is_fighter() {
        let fighter_kind = boma.kind();
        let status_kind = StatusModule::status_kind(boma);
        let id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

        // Disallow airdodge out of tumble until you reach your stable fall speed
        if flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR
        && [*FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR].contains(&status_kind) {
            return false;
        }
    
        if flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN {
            if ([*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH].contains(&status_kind) && MotionModule::frame(boma) < ((MotionModule::end_frame(boma) * 0.5645).ln()) * 9.2157) {
                return false;
            }
        }
    
        if flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT {
            if ([*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH].contains(&status_kind) && MotionModule::frame(boma) < ((MotionModule::end_frame(boma) * 0.5645).ln()) * 9.2157) {
                return false;
            }
        }
    
        if flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_WALK && [*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH].contains(&status_kind) && MotionModule::frame(boma) < ((MotionModule::end_frame(boma) * 0.5645).ln()) * 9.2157 {
            return false;
        }
    
        // Allow dash, run, run_brake => taunt
        if [*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_U, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_S, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_LW].contains(&flag)
            && [*FIGHTER_STATUS_KIND_RUN_BRAKE].contains(&status_kind) {
            return true;
        }
    
        if flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI && VarModule::is_flag(boma.object(), vars::common::instance::UP_SPECIAL_CANCEL) {
            return false;
        }

        if flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND && VarModule::is_flag(boma.object(), vars::common::instance::UP_SPECIAL_CANCEL) {
            return false;
        }
    
        if flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S && (VarModule::is_flag(boma.object(), vars::common::instance::SIDE_SPECIAL_CANCEL) || VarModule::is_flag(boma.object(), vars::common::instance::SIDE_SPECIAL_CANCEL_NO_HIT)) {
            return false;
        }
    
        if flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND && (VarModule::is_flag(boma.object(), vars::common::instance::SIDE_SPECIAL_CANCEL) || VarModule::is_flag(boma.object(), vars::common::instance::SIDE_SPECIAL_CANCEL_NO_HIT)) {
            return false;
        }

        if fighter_kind == *FIGHTER_KIND_PEACH {
            if status_kind == *FIGHTER_STATUS_KIND_JUMP_AERIAL {
                if flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL || flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON {
                    if KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) < 0.0 {
                        return false;
                    }
                }
            }
        }

        // Disable Mii Swordfighter nspecial if the Tornado projectile is still active
        if fighter_kind == *FIGHTER_KIND_MIISWORDSMAN {
            if VarModule::get_int(boma.object(), vars::common::instance::GIMMICK_TIMER) > 0 && flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N {
                return false;
            }
        }

        if fighter_kind == *FIGHTER_KIND_TRAIL {
            if flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S {
                if (status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT))
                || VarModule::is_flag(boma.object(), vars::metaknight::instance::SPECIAL_S_HIT) {
                    return false;
                }
            }
        }

        //Disable Duck Hunt Down Special on a timer
        if boma.kind() == *FIGHTER_KIND_DUCKHUNT  {
            if VarModule::get_int(boma.object(), vars::duckhunt::instance::SPECIAL_LW_GUNMAN_TIMER) != 0 
            && flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW {
                return false;
            }
        }

        if fighter_kind == *FIGHTER_KIND_NANA {
            if ([*FIGHTER_STATUS_KIND_WAIT, 
                *FIGHTER_STATUS_KIND_TURN, 
                *FIGHTER_STATUS_KIND_WALK, 
                *FIGHTER_STATUS_KIND_WALK_BRAKE, 
                *FIGHTER_STATUS_KIND_RUN_BRAKE, 
                *FIGHTER_STATUS_KIND_JUMP_SQUAT,
                *FIGHTER_STATUS_KIND_SQUAT,
                *FIGHTER_STATUS_KIND_SQUAT_WAIT,
                *FIGHTER_STATUS_KIND_SQUAT_RV].contains(&status_kind)
            && flag == FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH) || 
            ([*FIGHTER_STATUS_KIND_DASH, 
                *FIGHTER_STATUS_KIND_TURN_DASH, 
                *FIGHTER_STATUS_KIND_RUN].contains(&status_kind)
            && flag == FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH) || 
            ([*FIGHTER_STATUS_KIND_DASH,
                *FIGHTER_STATUS_KIND_TURN_DASH, 
                *FIGHTER_STATUS_KIND_RUN].contains(&status_kind)
            && flag == FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN) {
                return true;
            }
        }
    }   
    original!()(boma, flag)
}

#[skyline::hook(replace=WorkModule::enable_transition_term)]
unsafe fn enable_transition_term_hook(boma: &mut BattleObjectModuleAccessor, flag: i32) -> bool {
    if flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN {
        VarModule::on_flag(boma.object(), vars::common::status::IS_DASH_TO_RUN_FRAME);
    }
    original!()(boma, flag)
}

pub fn install() {
    skyline::install_hooks!(
        is_enable_transition_term_hook,
        enable_transition_term_hook,
    );
}