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
        
        // disable jumping while using floats
        if [*FIGHTER_STATUS_KIND_FALL,
        *FIGHTER_STATUS_KIND_FALL_AERIAL,
        *FIGHTER_STATUS_KIND_JUMP,
        *FIGHTER_STATUS_KIND_JUMP_AERIAL,
        *FIGHTER_STATUS_KIND_CLIFF_JUMP1,
        *FIGHTER_STATUS_KIND_CLIFF_JUMP2,
        *FIGHTER_STATUS_KIND_CLIFF_JUMP3,
        *FIGHTER_STATUS_KIND_ATTACK_AIR,
        *FIGHTER_STATUS_KIND_DAMAGE_FALL].contains(&status_kind)
        && WorkModule::is_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_FALL_SLOWLY)
        || VarModule::is_flag(boma.object(), vars::common::instance::OMNI_FLOAT) {
            if [*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL].contains(&flag) {
                    return false;
            }
        }
        
        // handle tilt attack input stopping you from getting a smash attack
        if [*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START,].contains(&flag)
            && boma.is_cat_flag(CatHdr::TiltAttack) {
                return false;
        }
    
        // Disallow airdodge out of tumble until you reach your stable fall speed
        if flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR
            && ([*FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR].contains(&status_kind)
            || (status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FALL && get_fighter_common_from_accessor(boma).global_table[CURRENT_FRAME].get_i32() <= 20))  {
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
    
        // Disable transition to double jump if you have float juice and are holding down
        if [*FIGHTER_KIND_SAMUSD, *FIGHTER_KIND_MEWTWO, *FIGHTER_KIND_REFLET].contains(&fighter_kind) {
            if [*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON].contains(&flag) {
                if boma.left_stick_y() < -0.66 {
                    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_SUPERLEAF_FALL_SLOWLY_FRAME) > 0 {
                        return false;
                    }
                }
            }
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
        //disable airdodge out of aerial tele
        if fighter_kind == *FIGHTER_KIND_MEWTWO {
            if VarModule::is_flag(boma.object(), vars::common::instance::UP_SPECIAL_CANCEL) 
            && !VarModule::is_flag(boma.object(), vars::mewtwo::instance::GROUNDED_TELEPORT) {
                if flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR { 
                    return false;
                }
            }
        }
        /*
        // Marth & Lucina - special fair FAF
        if fighter_kind == FIGHTER_KIND_MARTH || fighter_kind == *FIGHTER_KIND_LUCINA {
            if MotionModule::motion_kind(boma) == hash40("attack_air_f") {
                if flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR && ControlModule::get_attack_air_kind(boma) == FIGHTER_COMMAND_ATTACK_AIR_KIND_F {
                    if MotionModule::frame(boma) < 38.0 {
                        return false;
                    }
                }
            }
        }
        */

        // Meta Knight - Disable use of specials midair again after hitting them during the current airtime
        if fighter_kind == FIGHTER_KIND_METAKNIGHT {
            if     (flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N && VarModule::is_flag(boma.object(), vars::metaknight::instance::NEUTRAL_SPECIAL_HIT))
                || (flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S && VarModule::is_flag(boma.object(), vars::metaknight::instance::SIDE_SPECIAL_HIT))
                || (flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI && VarModule::is_flag(boma.object(), vars::metaknight::instance::UP_SPECIAL_HIT))
                || (flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW && VarModule::is_flag(boma.object(), vars::metaknight::instance::DOWN_SPECIAL_HIT)) {
                return false;
            }
        }

        //Disable sonic airdodge out of up-b and enable followup neutral B after a successful hit
        if fighter_kind == *FIGHTER_KIND_SONIC {
            /*
            if SONIC_NO_AIRDODGE[id] && flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR {
                return false;
            }
            */
            /*
            if status_kind == *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_HIT && flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N {
                return true;
            }
            */
        }

        // Disable Mii Swordfighter nspecial if the Tornado projectile is still active
        if fighter_kind == *FIGHTER_KIND_MIISWORDSMAN {
            if VarModule::get_int(boma.object(), vars::common::instance::GIMMICK_TIMER) > 0 && flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N {
                return false;
            }
        }

        // disable palutena projectiles during aegis reflector
        if fighter_kind == *FIGHTER_KIND_PALUTENA 
            && ArticleModule::is_exist(boma, *FIGHTER_PALUTENA_GENERATE_ARTICLE_REFLECTIONBOARD)
            && (flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S
            || flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N) {
                return false;
        }

        if fighter_kind == *FIGHTER_KIND_TRAIL {
            if flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S {
                if (status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT))
                || VarModule::is_flag(boma.object(), vars::metaknight::instance::SIDE_SPECIAL_HIT) {
                    return false;
                }
            }
        }

        //Disable Duck Hunt Down Special on a timer
        if boma.kind() == *FIGHTER_KIND_DUCKHUNT  {
            if VarModule::get_int(boma.object(), vars::duckhunt::instance::GUNMAN_TIMER) != 0 
            && flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW {
                    return false
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
        enable_transition_term_hook
    );
}
