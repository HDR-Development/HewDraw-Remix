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

unsafe fn rotate_bone(boma: &mut BattleObjectModuleAccessor, max_angle: f32, min_angle: f32, strength: f32) {
    let mut angle = min_angle.abs();
    if strength > 0.0 {
        angle = max_angle
    }
    let mut rotation = Vector3f{x: 0.0, y: 0.0, z: ((angle * -1.0 * strength) - 2.5)};
    let agent = utils::util::get_fighter_common_from_accessor(boma);
    agent.set_joint_rotate("bust", rotation);
}

unsafe fn bust_lean(boma: &mut BattleObjectModuleAccessor, lean_frame: f32, return_frame: f32, max_angle: f32, min_angle: f32) {
    let stick_y = ControlModule::get_stick_y(boma);
    let frame = MotionModule::frame(boma);
    let end_frame = MotionModule::end_frame(boma);
    let chest_y: f32 = VarModule::get_float(boma.object(), vars::dedede::instance::INHALE_STICK_Y);
    if frame >= 0.0 && frame < lean_frame {
        // linear interpolate to stick position,
        // while getting stick position still
        VarModule::set_float(boma.object(), vars::dedede::instance::INHALE_STICK_Y, stick_y);
        rotate_bone(boma, max_angle, min_angle, stick_y * ((frame as f32) / 30.0));
    } else if frame >= lean_frame && frame < return_frame {
        // rotate at selected angle for each frame
        rotate_bone(boma, max_angle, min_angle, chest_y);
    } else {
        // linear interpolate back to normal
        rotate_bone(boma, max_angle, min_angle, chest_y * (1.0 - ((frame - return_frame) / (end_frame - return_frame))));
    }
}

unsafe fn angled_inhale_shot(agent: &mut L2CFighterCommon) {
    if ArticleModule::is_exist(agent.boma(), *FIGHTER_DEDEDE_GENERATE_ARTICLE_GORDO){
        if agent.is_status(*FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_SHOT_OBJECT_HIT){
            bust_lean(agent.boma(), 6.0, 12.0, 20.0, -20.0);
        }
    }
}

//Gordo recatch and waddledash
unsafe fn gordo_recatch(boma: &mut BattleObjectModuleAccessor, frame: f32, agent: &mut L2CFighterCommon){
    if ArticleModule::is_exist(boma, *FIGHTER_DEDEDE_GENERATE_ARTICLE_GORDO){
        let article = ArticleModule::get_article(boma, *FIGHTER_DEDEDE_GENERATE_ARTICLE_GORDO);
        let object_id = smash::app::lua_bind::Article::get_battle_object_id(article) as u32;
        let article_boma = sv_battle_object::module_accessor(object_id);

        let char_pos = *PostureModule::pos(boma); //position of dedede
        let gordo_pos = *PostureModule::pos(article_boma); //position of gordo
        let char_lr = PostureModule::lr(boma); // LR value before we check everything
        let offset = Vector3f::new(5.0 * char_lr, 9.0, 0.0); //offset, if we need to move the area

        if ((gordo_pos.x - (char_pos.x + offset.x)).abs() < 19.0 && (gordo_pos.y - (char_pos.y + offset.y)).abs() < 15.0){
            if ((StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ESCAPE_AIR) 
            || ((StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_LANDING) && StatusModule::prev_status_kind(boma, 0) == *FIGHTER_STATUS_KIND_ESCAPE_AIR)) 
            && VarModule::is_flag(agent.battle_object, vars::dedede::instance::CAN_WADDLE_DASH_FLAG){
                if agent.status_frame() > 1 
                && agent.status_frame() < 4 { //We don't want to go into recatch if we are in the middle of airdodge/landing
                    if StatusModule::status_kind(article_boma) != *WEAPON_DEDEDE_GORDO_STATUS_KIND_DEAD{
                        VarModule::set_flag(agent.battle_object, vars::dedede::instance::CAN_WADDLE_DASH_FLAG, false);
                        VarModule::set_flag(agent.battle_object, vars::dedede::instance::IS_DASH_GORDO, true);
                        VarModule::inc_int(agent.battle_object, vars::dedede::instance::RECATCH_COUNTER);

                        VarModule::set_flag(agent.battle_object, vars::dedede::instance::IS_REMOVED_FLAG, true);
                        ArticleModule::remove(boma, *FIGHTER_DEDEDE_GENERATE_ARTICLE_GORDO, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL)); 

                        StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_SPECIAL_S, false);
                        
                        if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
                            KineticModule::mul_speed(agent.module_accessor, &Vector3f{x: 1.5, y: 0.0, z:1.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
                            MotionModule::change_motion(agent.module_accessor, Hash40::new("special_air_s_get"), 0.0, 1.0, false, 0.0, false, false);

                        }
                        else{
                            StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_SPECIAL_S, false);
                            MotionModule::change_motion(agent.module_accessor, Hash40::new("special_s_get"), 0.0, 1.0, false, 0.0, false, false);
                        }

                        //Prevents turnarounds
                        if ControlModule::get_stick_x(agent.module_accessor) * char_lr < 0.0{
                            PostureModule::reverse_lr(boma);
                            PostureModule::reverse_rot_y_lr(boma);
                            let new_char_lr = PostureModule::lr(boma);
                            if (char_lr != new_char_lr) && (ControlModule::get_stick_x(agent.module_accessor) * new_char_lr < 0.0){
                                PostureModule::reverse_lr(boma);
                                PostureModule::reverse_rot_y_lr(boma);
                            }
                        }      
                    }        
                }    
            }
        }
        // Re enables gordo dash if d3 has either done a gordodash, has landed, or is in jump squat
        if StatusModule::prev_status_kind(boma, 0) == *FIGHTER_STATUS_KIND_SPECIAL_S 
        || StatusModule::prev_status_kind(boma, 0) == *FIGHTER_STATUS_KIND_LANDING 
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_JUMP_SQUAT
        || WorkModule::is_flag(boma, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD){
            VarModule::set_flag(agent.battle_object, vars::dedede::instance::CAN_WADDLE_DASH_FLAG, true);
        }
        //Prevents B reversing when we are in the dash
        if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_SPECIAL_S 
        && VarModule::is_flag(agent.battle_object, vars::dedede::instance::IS_DASH_GORDO){
            if agent.status_frame() > 1 && agent.status_frame() < 4{
                ControlModule::reset_main_stick(boma);
            }
        }
        VarModule::set_flag(agent.battle_object, vars::dedede::instance::IS_REMOVED_FLAG, false);
    }
    //checking if gordo does not exist, but is not removed by this function
    else if !ArticleModule::is_exist(boma, *FIGHTER_DEDEDE_GENERATE_ARTICLE_GORDO) && !VarModule::is_flag(agent.battle_object, vars::dedede::instance::IS_REMOVED_FLAG){
        VarModule::set_flag(agent.battle_object, vars::dedede::instance::IS_DASH_GORDO, false);
        VarModule::set_int(agent.battle_object, vars::dedede::instance::RECATCH_COUNTER, 0); 
    }
}

unsafe fn super_jump_fail_edge_cancel(agent: &mut L2CFighterCommon){
    if agent.is_status(*FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_HI_FAILURE) && agent.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        StatusModule::change_status_force(agent.boma(), *FIGHTER_STATUS_KIND_FALL, false);
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
 
unsafe fn fastfall_specials(agent: &mut L2CFighterCommon) {
    if !agent.is_in_hitlag()
    && !StatusModule::is_changing(agent.module_accessor)
    && agent.is_status_one_of(&[
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
    && agent.is_situation(*SITUATION_KIND_AIR) {
        agent.sub_air_check_dive();
        if agent.is_flag(*FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
            if [*FIGHTER_KINETIC_TYPE_MOTION_AIR, *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE].contains(&KineticModule::get_kinetic_type(agent.module_accessor)) {
                agent.clear_lua_stack();
                lua_args!(agent, FIGHTER_KINETIC_ENERGY_ID_MOTION);
                let speed_y = app::sv_kinetic_energy::get_speed_y(agent.lua_state_agent);

                agent.clear_lua_stack();
                lua_args!(agent, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y, 0.0, 0.0, 0.0);
                app::sv_kinetic_energy::reset_energy(agent.lua_state_agent);
                
                agent.clear_lua_stack();
                lua_args!(agent, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                app::sv_kinetic_energy::enable(agent.lua_state_agent);

                KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, agent.module_accessor);
            }
        }
    }
}

pub unsafe fn moveset(agent: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    //bair_foot_rotation_scaling(boma);
    super_dedede_jump_quickfall(boma, frame);
    gordo_recatch(boma, frame, agent);
    angled_inhale_shot(agent);
    super_jump_fail_edge_cancel(agent);
    fastfall_specials(agent);
}

pub extern "C" fn dedede_frame_wrapper(agent: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(agent);
		dedede_frame(agent)
    }
}

pub unsafe fn dedede_frame(agent: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(agent) {
        moveset(agent, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}
pub fn install(agent: &mut Agent) {
    agent.on_line(Main, dedede_frame_wrapper);
}
