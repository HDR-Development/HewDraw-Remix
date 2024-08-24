use super::*;

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

    //println!();
    //println!("Current math state: {}", math_state);

    match math_state {
        0 => {
            // start math if rolling a 9
            if rng == 8 {
                //println!("Rolled a 9, start math!");
                VarModule::set_int(fighter.battle_object, vars::gamewatch::instance::SPECIAL_S_MATH_STATE, 1);
            }
            // else {
            //     println!("Rolled a {}", rng + 1);
            // }
        },
        1 => {
            // take the next two rolls...
            //println!("Rolled a {}", rng + 1);
            //println!("Incrementing math state");
            VarModule::set_int(fighter.battle_object, vars::gamewatch::instance::SPECIAL_S_MATH_STATE, 2);
        },
        2 => {
            // ...and perform math!
            //println!("Math time!");
            //println!("Rolled a {}", rng + 1);
            let result = (rng + 1) + (kind + 1);
            if result < 9 {
                // if the two rolls add up to less than 9, use the result
                //println!("Less than 9, incoming roll: {}", rng + kind + 2);
                VarModule::set_int(fighter.battle_object, vars::gamewatch::instance::SPECIAL_S_MATH_RESULT, rng + kind + 1);
            }
            else if result > 9 {
                // if not, subtract the two and use that instead
                let num = rng.max(kind) - rng.min(kind) - 1;
                //println!("Greater than 9, incoming roll: {}", num);
                VarModule::set_int(fighter.battle_object, vars::gamewatch::instance::SPECIAL_S_MATH_RESULT, num);
            }
            else {
                // for balancing reasons add another 50% chance to actually be a guaranteed 9
                //println!("Result is a 9, one more rng check");
                let rand = sv_math::rand(hash40("fighter"), 2);
                if rand == 1 {
                    VarModule::set_int(fighter.battle_object, vars::gamewatch::instance::SPECIAL_S_MATH_RESULT, (rng - kind - 1).abs());
                }
                else {
                    VarModule::set_int(fighter.battle_object, vars::gamewatch::instance::SPECIAL_S_MATH_RESULT, rng);
                }
            }
            VarModule::set_int(fighter.battle_object, vars::gamewatch::instance::SPECIAL_S_MATH_STATE, 3);
        },
        3 => {
            // use the result for the next roll
            //println!("Resetting math state");
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

    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_init);
}