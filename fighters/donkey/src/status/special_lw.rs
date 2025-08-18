use super::*;

// FIGHTER_STATUS_KIND_SPECIAL_LW

unsafe extern "C" fn special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // check for grounded grab (workaround due to HEAVY_PICKUP not liking trying to grab an opponent)
    if VarModule::is_flag(fighter.object(), vars::common::instance::IS_HEAVY_ATTACK) {
        VarModule::off_flag(fighter.object(), vars::common::instance::IS_HEAVY_ATTACK);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_landing"), 8.0, 1.0, false, 0.0, false, false);
    }
    else {
        // if you are grounded, pick up heavy item/spawn barrel
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            // change into the heavy item pickup status
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP.into(),false.into());
            return true.into();
        }

        // otherwise, proceed with airgrab
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw"), 0.0, 1.0, false, 0.0, false, false);
    }
    
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(special_lw_substatus as *const () as _));
    fighter.main_shift(special_lw_main_loop)
}

unsafe extern "C" fn special_lw_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR)
    && param_1.get_bool() {
        // enable fastfall
        fighter.sub_air_check_dive();

        // try to pick up an item nearby
        let frame = fighter.motion_frame();
        if frame > 5.0 && frame < 16.0 {
            let range = 20.0;
            fighter.try_pickup_item(range, Some(Hash40::new("top")), Some(&Vector2f{x: 10.0, y: 0.0}));
        }

        // if at any time during dspecial you are holding 
        // an item, transition into heavy pickup.
        if ItemModule::is_have_item(fighter.boma(), 0) {
            fighter.change_status_req(FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP.into(), false.into());
            grab!(fighter, MA_MSC_CMD_GRAB_CLEAR_ALL);
            return 1.into();
        }
    }
    0.into()
}

unsafe extern "C" fn special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_air = MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_lw");
    let situation = fighter.global_table[SITUATION_KIND].get_i32();
    if is_air {
        fighter.sub_air_check_dive();
        if CancelModule::is_enable_cancel(fighter.module_accessor) {
            if fighter.sub_transition_group_check_air_landing().get_bool()
            || fighter.sub_wait_ground_check_common(false.into()).get_bool()
            || fighter.sub_air_check_fall_common().get_bool() {
                return 1.into();
            }
        }
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            if fighter.status_frame() < 24 {
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw_landing"), -1.0, 1.0, 0.0, false, false);
            }
            else {
                WorkModule::set_float(fighter.module_accessor, 20.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
                return 1.into();
            }
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        let status = if is_air {
            FIGHTER_STATUS_KIND_FALL_SPECIAL
        }
        else {
            FIGHTER_STATUS_KIND_WAIT
        };
        fighter.change_status(status.into(), false.into());
    }
    1.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_main);
}