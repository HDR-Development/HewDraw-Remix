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

unsafe fn repeated_falcon_punch_turnaround(fighter: &mut L2CFighterCommon) {
    if StatusModule::is_changing(fighter.module_accessor) {
        return;
    }
    let frame = fighter.motion_frame();
    if fighter.is_status(*FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_N_TURN)
    && 22.0 < frame && frame < 41.0
    && fighter.is_stick_backward()
    && fighter.stick_x().abs() > 0.1
    {
        fighter.change_status_req(*FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_N_TURN, true);
    }
}

unsafe fn yes(fighter: &mut L2CFighterCommon) {
    if fighter.is_motion(Hash40::new("attack_air_f")) && (14..17).contains(&(fighter.motion_frame() as u32)) {
        if !VarModule::is_flag(fighter.object(), vars::captain::status::YES) && fighter.is_button_on(Buttons::AppealAll) && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            PLAY_SE(fighter, Hash40::new("vc_captain_appeal03"));
            VarModule::on_flag(fighter.object(), vars::captain::status::YES);
        }
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_N_TURN,
        *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_HI_THROW,
        *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_WALL_END
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
        if fighter.is_flag(*FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
            if [*FIGHTER_KINETIC_TYPE_MOTION_AIR, *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE].contains(&KineticModule::get_kinetic_type(fighter.module_accessor)) {
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
                let speed_y = app::sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);

                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y, 0.0, 0.0, 0.0);
                app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
                
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                app::sv_kinetic_energy::enable(fighter.lua_state_agent);

                KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
            }
        }
    }
}


pub unsafe extern "C" fn captain_frame_wrapper(fighter: &mut L2CFighterCommon) {
    common::opff::fighter_common_opff(fighter);
    air_falcon_kick_jump_reset(fighter);
    repeated_falcon_punch_turnaround(fighter);
    yes(fighter);
    fastfall_specials(fighter);
}

pub fn install() {
    smashline::Agent::new("captain")
        .on_line(Main, captain_frame_wrapper)
        .install();
}
