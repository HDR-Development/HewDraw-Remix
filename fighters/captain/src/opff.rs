use super::*;
 
utils::import_noreturn!(common::opff::fighter_common_opff);

unsafe fn air_falcon_kick_jump_reset(fighter: &mut L2CFighterCommon) {
    if fighter.is_situation(*SITUATION_KIND_AIR)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_END,
        *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_WALL_END
    ])
    && fighter.get_num_used_jumps() == fighter.get_jump_count_max()
    {
        WorkModule::dec_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    }
}

unsafe fn falcon_punch_b_reverse(fighter: &mut L2CFighterCommon) {
    let frame = fighter.motion_frame();
    if fighter.is_motion(Hash40::new("special_air_n"))
    && frame < 5.0
    && fighter.is_stick_backward()
    {
        PostureModule::reverse_lr(fighter.module_accessor);
        PostureModule::update_rot_y_lr(fighter.module_accessor);
        if VarModule::is_flag(fighter.battle_object, vars::common::B_REVERSED) {
            return;
        }
        KineticModule::mul_speed(fighter.module_accessor, &Vector3f::new(-1.0, 1.0, 1.0), *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        VarModule::on_flag(fighter.battle_object, vars::common::B_REVERSED);
    }
}

unsafe fn falcon_kick_b_reverse(fighter: &mut L2CFighterCommon) {
    let frame = fighter.motion_frame();
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_LW)
    && 1.0 < frame && frame < 5.0
    && fighter.is_stick_backward() {
        PostureModule::reverse_lr(fighter.module_accessor);
        PostureModule::update_rot_y_lr(fighter.module_accessor);
        if VarModule::is_flag(fighter.battle_object, vars::common::B_REVERSED) {
            return;
        }
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        VarModule::on_flag(fighter.battle_object, vars::common::B_REVERSED);
    }
}

unsafe fn repeated_falcon_punch_turnaround(fighter: &mut L2CFighterCommon) {
    let frame = fighter.motion_frame();
    if fighter.is_status(*FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_N_TURN)
    && 21.0 < frame && frame < 40.0
    && fighter.is_stick_backward()
    && fighter.stick_x().abs() > 0.1
    {
        fighter.change_status_req(*FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_N_TURN, true);
    }
}

#[utils::macros::opff(FIGHTER_KIND_CAPTAIN )]
pub unsafe fn captain_frame_wrapper(fighter: &mut L2CFighterCommon) {
    common::opff::fighter_common_opff(fighter);

    air_falcon_kick_jump_reset(fighter);
    falcon_punch_b_reverse(fighter);
    falcon_kick_b_reverse(fighter);
    repeated_falcon_punch_turnaround(fighter);
}