use super::*;

// FIGHTER_STATUS_KIND_JUMP_AERIAL

pub unsafe extern "C" fn jump_aerial_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_0 = fighter.status_pre_JumpAerial_sub().get_i32() == 0;
    let should_end = is_0 as i32 & 1 == 0;
    if !should_end {
        StatusModule::init_settings(
            fighter.module_accessor,
            app::SituationKind(*SITUATION_KIND_AIR),
            *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION,
            *GROUND_CORRECT_KIND_AIR as u32,
            app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
            true,
            *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
            *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
            *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
            0,
        );
        FighterStatusModuleImpl::set_fighter_status_data(
            fighter.module_accessor,
            false,
            *FIGHTER_TREADED_KIND_ENABLE,
            true,
            false,
            true,
            0,
            *FIGHTER_STATUS_ATTR_INTO_DOOR as u32,
            0,
            0,
        );
    }
    return (should_end as i32).into();
}

pub unsafe extern "C" fn jump_aerial_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let aerial_damage_reaction = WorkModule::get_float(
        fighter.module_accessor,
        *FIGHTER_YOSHI_INSTANCE_WORK_ID_FLOAT_AERIAL_DAMAGE_REACTION,
    );
    DamageModule::set_no_reaction_mode_status(
        fighter.module_accessor,
        DamageNoReactionMode {
            _address: *DAMAGE_NO_REACTION_MODE_REACTION_VALUE as u8,
        },
        aerial_damage_reaction,
        -1.0,
        -1,
    );
    WorkModule::on_flag(
        fighter.module_accessor,
        *FIGHTER_YOSHI_INSTANCE_WORK_ID_FLAG_JUMP_AERIAL_ARMOR,
    );
    let turn_cont_value = WorkModule::get_param_float(
        fighter.module_accessor,
        hash40("jump_aerial"),
        hash40("turn_cont_value"),
    );
    if fighter.global_table[STICK_X].get_f32() * -1.0 * PostureModule::lr(fighter.module_accessor)
        > turn_cont_value
    {
        let turn_count = WorkModule::get_param_int(
            fighter.module_accessor,
            hash40("jump_aerial"),
            hash40("turn_count"),
        );
        WorkModule::set_int(
            fighter.module_accessor,
            turn_count,
            *FIGHTER_YOSHI_INSTANCE_WORK_ID_INT_AERIAL_TURN_COUNT,
        );
    } else {
        WorkModule::set_int(
            fighter.module_accessor,
            0,
            *FIGHTER_YOSHI_INSTANCE_WORK_ID_INT_AERIAL_TURN_COUNT,
        );
    }
    MotionModule::set_trans_move_speed_no_scale(fighter.module_accessor, true);
    fighter.status_JumpAerial();
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_JUMP_AERIAL, jump_aerial_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_JUMP_AERIAL, jump_aerial_main);
}
