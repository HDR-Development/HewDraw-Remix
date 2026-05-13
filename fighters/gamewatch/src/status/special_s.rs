use super::*;
use smash2;

unsafe extern "C" fn special_s_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let prev_kind = WorkModule::get_int(fighter.module_accessor, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_INT_SPECIAL_S_PREV_KIND);
    let kind = WorkModule::get_int(fighter.module_accessor, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_INT_SPECIAL_S_KIND);

    smashline::original_status(Init, fighter, *FIGHTER_STATUS_KIND_SPECIAL_S)(fighter);

    // Generates RNG and makes sure it isn't the same as the previous two Judge values
    // RNG values are 0 indexed, so 0 > 1, 1 > 2, etc
    let mut rng = sv_math::rand(hash40("fighter"), 9);
    while rng == prev_kind || rng == kind {
        rng = sv_math::rand(hash40("fighter"), 9);
    }

    // Calculate MATH
    let math_state = VarModule::get_int(fighter.battle_object, vars::gamewatch::instance::SPECIAL_S_MATH_STATE);
    match math_state {
        0 => {
            // start math if rolling a 9
            if rng == 8 {
                VarModule::set_int(fighter.battle_object, vars::gamewatch::instance::SPECIAL_S_MATH_STATE, 1);
            }
        },
        1 => {
            // take the next two rolls...
            VarModule::set_int(fighter.battle_object, vars::gamewatch::instance::SPECIAL_S_MATH_STATE, 2);
        },
        2 => {
            // ...and perform math!
            let result = (rng + 1) + (kind + 1);
            if result < 9 {
                // if the two rolls add up to less than 9, use the result
                VarModule::set_int(fighter.battle_object, vars::gamewatch::instance::SPECIAL_S_MATH_RESULT, rng + kind + 1);
            }
            else if result > 9 {
                // if not, subtract the two and use that instead
                let num = rng.max(kind) - rng.min(kind) - 1;
                VarModule::set_int(fighter.battle_object, vars::gamewatch::instance::SPECIAL_S_MATH_RESULT, num);
            }
            else {
                // for balancing reasons add another 50% chance to actually be a guaranteed 9
                let rand = sv_math::rand(hash40("fighter"), 2);
                if rand == 1 {
                    VarModule::set_int(fighter.battle_object, vars::gamewatch::instance::SPECIAL_S_MATH_RESULT, rng + kind + 1);
                }
                else {
                    VarModule::set_int(fighter.battle_object, vars::gamewatch::instance::SPECIAL_S_MATH_RESULT, rng);
                }
            }
            VarModule::set_int(fighter.battle_object, vars::gamewatch::instance::SPECIAL_S_MATH_STATE, 3);
        },
        3 => {
            // use the result for the next roll
            rng = VarModule::get_int(fighter.battle_object, vars::gamewatch::instance::SPECIAL_S_MATH_RESULT);
            VarModule::set_int(fighter.battle_object, vars::gamewatch::instance::SPECIAL_S_MATH_STATE, 0);
        }
        _ => {}
    }

    // Enables the TABEMONO flag if RNG is equal to 6 (7, but it's 0 indexed)
    WorkModule::set_flag(fighter.module_accessor, rng == 6, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HIT_TABEMONO);
    WorkModule::set_int(fighter.module_accessor, kind, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_INT_SPECIAL_S_PREV_KIND);
    WorkModule::set_int(fighter.module_accessor, rng, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_INT_SPECIAL_S_KIND);
    WorkModule::set_int(fighter.module_accessor, rng, *FIGHTER_INSTANCE_WORK_ID_INT_TRICK_SUB);

    VisibilityModule::set_status_default_int64(fighter.module_accessor, hash40("panel") as i64, hash40(&format!("no_{}", rng + 1)) as i64);

    // Makes sure we aren't showing the actual number on the first frame.
    let mut rng_dummy = sv_math::rand(hash40("fighter"), 9);
    VisibilityModule::set_int64(fighter.module_accessor, hash40("panel") as i64, hash40(&format!("no_{}", rng_dummy + 1)) as i64);

    WorkModule::set_int64(
        fighter.module_accessor,
        hash40(&format!("special_s_{}", rng + 1)) as i64,
        *FIGHTER_GAMEWATCH_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND
    );
    WorkModule::set_int64(
        fighter.module_accessor,
        hash40(&format!("special_air_s_{}", rng + 1)) as i64,
        *FIGHTER_GAMEWATCH_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND_AIR
    );

    return 0.into();
}

unsafe extern "C" fn special_s_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_motion_one_of(&[Hash40::new("special_s_4"), Hash40::new("special_air_s_4")]) {
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) && !fighter.is_in_hitlag() {
            fighter.check_jump_cancel(false, false, false);
        }
    }

    return smashline::original_status(Exec, fighter, *FIGHTER_STATUS_KIND_SPECIAL_S)(fighter);
}

unsafe extern "C" fn special_s_check_attack(fighter: &mut L2CFighterCommon, param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    if fighter.is_motion_one_of(&[Hash40::new("special_s_1"), Hash40::new("special_air_s_1")]) {
        if (&param_3["object_category_"]).get_i32() == *BATTLE_OBJECT_CATEGORY_FIGHTER {
            if (&param_3["kind_"]).get_i32() == *COLLISION_KIND_HIT {
                let object_id = (&param_3["object_id_"]).get_u32();
                let opponent_boma = sv_battle_object::module_accessor(object_id);
                if StatusModule::situation_kind(opponent_boma) == *SITUATION_KIND_AIR {
                    let opponent_object = utils::util::get_battle_object_from_accessor(opponent_boma);
                    VarModule::on_flag(opponent_object, vars::common::instance::FORCE_TUMBLE_NO_BOUNCE);
                    StatusModule::set_status_kind_interrupt(opponent_boma, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR);
                }
            }
        }
    }

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_init);
    agent.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_exec);
    agent.status(CheckAttack, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_check_attack);
}