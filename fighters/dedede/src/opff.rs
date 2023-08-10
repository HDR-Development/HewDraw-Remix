// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

// Up B early fall attack
unsafe fn super_dedede_jump_quickfall(boma: &mut BattleObjectModuleAccessor, frame: f32){
    if StatusModule::is_changing(boma) {
        return;
    }
    if boma.is_status(*FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_HI_JUMP)
    && (frame > 16.0 && frame < 36.0)
    {
        if ControlModule::get_stick_y(boma) < -0.5 {
            MotionModule::set_frame_sync_anim_cmd(boma, 35.0, true, true, false);
        }
    }
}

unsafe fn bair_foot_rotation_scaling(boma: &mut BattleObjectModuleAccessor) {
    // Rotation keyframes
    let start_frame = 0.0;
    let bend_frame = 3.0;
    let return_frame = 21.0;
    let straight_frame = 26.0;

    // Expansion keyframes
    let start_expand_frame = 5.0;
    let finish_expand_frame = 7.0;
    let return_expand_frame = 8.0;
    let normal_expand_frame = 10.0;

    let frame = MotionModule::frame(boma);
    let end_frame = MotionModule::end_frame(boma);
    let max_x_rotation_foot = 0.0;
    //let max_y_rotation_foot = -25.0;
    let max_y_rotation_foot = 0.0;
    let max_z_rotation_foot = 0.0;
    let mut rotation_foot = Vector3f{x: 0.0, y: 0.0, z: 0.0};

    let max_x_scale_foot = 2.0;
    let max_y_scale_foot = 1.25;
    let max_z_scale_foot = 1.25;

    if boma.is_motion(Hash40::new("attack_air_b")){
        if frame >= start_frame && frame < return_frame {
            // this has to be called every frame, or you snap back to the normal joint angle
            // interpolate to the respective bone bend angle
            // Foot bend
            let calc_x_rotate_foot = max_x_rotation_foot * (frame / (bend_frame - start_frame));
            let x_rotation_foot = calc_x_rotate_foot.clamp(0.0, max_x_rotation_foot);
            let calc_y_rotate_foot = max_y_rotation_foot * (frame / (bend_frame - start_frame));
            let y_rotation_foot = calc_y_rotate_foot.clamp(max_y_rotation_foot, 0.0);
            let calc_z_rotate_foot = max_z_rotation_foot * (frame / (bend_frame - start_frame));
            let z_rotation_foot = calc_z_rotate_foot.clamp(0.0, max_z_rotation_foot);
            rotation_foot = Vector3f{x: x_rotation_foot, y: y_rotation_foot, z: z_rotation_foot};
            ModelModule::set_joint_rotate(boma, Hash40::new("waist"), &rotation_foot, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
        } if frame >= return_frame && frame < straight_frame {
            // linear interpolate back to normal
            // Foot bend
            let calc_x_rotate_foot = max_x_rotation_foot  *(1.0 - (frame - return_frame) / (straight_frame - return_frame));
            let x_rotation_foot  = calc_x_rotate_foot.clamp(0.0, max_x_rotation_foot);
            let calc_y_rotate_foot = max_y_rotation_foot  *(1.0 - (frame - return_frame) / (straight_frame - return_frame));
            let y_rotation_foot  = calc_y_rotate_foot.clamp(max_y_rotation_foot, 0.0);
            let calc_z_rotate_foot = max_z_rotation_foot  *(1.0 - (frame - return_frame) / (straight_frame - return_frame));
            let z_rotation_foot  = calc_z_rotate_foot.clamp(0.0, max_z_rotation_foot);
            rotation_foot  = Vector3f{x: x_rotation_foot, y: y_rotation_foot, z: z_rotation_foot };
            ModelModule::set_joint_rotate(boma, Hash40::new("waist"), &rotation_foot, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
        }
        if frame >= start_expand_frame && frame < return_expand_frame{
            let calc_x_scale_foot = max_x_scale_foot * (frame / (finish_expand_frame - start_expand_frame));
            let x_scale_foot = calc_x_scale_foot.clamp(1.0, max_x_scale_foot);
            let calc_y_scale_foot = max_y_scale_foot * (frame / (finish_expand_frame - start_expand_frame));
            let y_scale_foot = calc_y_scale_foot.clamp(1.0, max_y_scale_foot);
            let calc_z_scale_foot = max_z_scale_foot * (frame / (finish_expand_frame - start_expand_frame));
            let z_scale_foot = calc_z_scale_foot.clamp(1.0, max_z_scale_foot);
            ModelModule::set_joint_scale(boma, Hash40::new("footr"), &Vector3f::new(x_scale_foot, y_scale_foot, z_scale_foot));
        }
        if frame >= return_expand_frame && frame < normal_expand_frame{
            let calc_x_scale_foot = max_x_scale_foot * (1.0 - (frame - return_expand_frame) / (normal_expand_frame - return_expand_frame));
            let x_scale_foot = calc_x_scale_foot.clamp(1.0, max_x_scale_foot);
            let calc_y_scale_foot = max_y_scale_foot * (1.0 - (frame - return_expand_frame) / (normal_expand_frame - return_expand_frame));
            let y_scale_foot = calc_y_scale_foot.clamp(1.0, max_y_scale_foot);
            let calc_z_scale_foot = max_z_scale_foot * (1.0 - (frame - return_expand_frame) / (normal_expand_frame - return_expand_frame));
            let z_scale_foot = calc_z_scale_foot.clamp(1.0, max_z_scale_foot);
            ModelModule::set_joint_scale(boma, Hash40::new("footr"), &Vector3f::new(x_scale_foot, y_scale_foot, z_scale_foot));
        }
    }
}
 
unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_END,
        *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_LOOP,
        *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_SPIT,
        *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_SWALLOW,
        *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_EAT_FALL,
        *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_EAT_JUMP1,
        *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_EAT_JUMP2,
        *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_EAT_TURN_AIR,
        *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_EAT_WAIT_FALL,
        *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_EAT_WAIT_JUMP,
        *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_EAT_PASS,
        *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_S_GET,
        *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_S_MISS,
        *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_HI_FAILURE,
        *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_HI_TURN,
        *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_JUMP,
        *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_FALL,
        *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_PASS
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

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    //bair_foot_rotation_scaling(boma);
    super_dedede_jump_quickfall(boma, frame);
    fastfall_specials(fighter);
}
#[utils::macros::opff(FIGHTER_KIND_DEDEDE )]
pub fn dedede_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		dedede_frame(fighter)
    }
}

pub unsafe fn dedede_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}