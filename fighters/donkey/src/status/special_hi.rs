use super::*;

// FIGHTER_STATUS_KIND_SPECIAL_HI

pub unsafe extern "C" fn special_hi_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR && StatusModule::prev_situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        GroundModule::set_cliff_check(fighter.module_accessor, app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES));
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_HI_FLAG_YACL_DEFAULT);
        fighter.select_cliff_hangdata_from_name("special_hi_slipoff");
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    // make grounded uspecial flat, so that moving forward and back isnt jarring
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        if fighter.motion_frame() > 16.0 && fighter.motion_frame() < 61.0 {
            // flattens dk out during uspecial
            fighter.set_joint_rotate("rot", Vector3f::new(0.0, 20.0, 50.0));

            // moves dk's trans bone slightly down to compensate for lifted feet during uspecial
            let slightly_lower = Vector3f{x:0.0, y: -4.0, z: 0.0 };
            ModelModule::set_joint_translate(fighter.boma(), Hash40::new("trans"), &slightly_lower, false, false);

            // leans left and right based on movement
            let movement_lean = 20.0 * KineticModule::get_sum_speed_x(fighter.boma(), *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            fighter.set_joint_rotate("trans", Vector3f::new(0.0, movement_lean, 0.0));
        }
    }

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_exec);
}
