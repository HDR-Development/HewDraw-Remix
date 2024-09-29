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
    let fighter = utils::util::get_fighter_common_from_accessor(boma);
    fighter.set_joint_rotate("bust", rotation);
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

unsafe fn angled_inhale_shot(fighter: &mut L2CFighterCommon) {
    if ArticleModule::is_exist(fighter.boma(), *FIGHTER_DEDEDE_GENERATE_ARTICLE_GORDO){
        if fighter.is_status(*FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_SHOT_OBJECT_HIT){
            bust_lean(fighter.boma(), 6.0, 12.0, 20.0, -20.0);
        }
    }
}

//Gordo recatch and waddledash
unsafe fn gordo_recatch(boma: &mut BattleObjectModuleAccessor, frame: f32, fighter: &mut L2CFighterCommon){
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
            && !VarModule::is_flag(fighter.battle_object, vars::dedede::instance::DISABLE_WADDLE_DASH){
                if fighter.status_frame() > 1 
                && fighter.status_frame() < 4 { //We don't want to go into recatch if we are in the middle of airdodge/landing
                    if StatusModule::status_kind(article_boma) != *WEAPON_DEDEDE_GORDO_STATUS_KIND_DEAD{
                        VarModule::inc_int(fighter.battle_object, vars::dedede::instance::RECATCH_COUNTER);
                        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEDEDE_INSTANCE_WORK_ID_FLAG_GORDO_GET);
                        StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_SPECIAL_S, false);
                    }        
                } 
                else{
                    VarModule::set_float(fighter.battle_object, vars::dedede::instance::PRE_GORDOTOSS_LR, char_lr);
                }   
            }
        }
    }
}

unsafe fn mask_toggle(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, frame: f32) {
    let mask_is_equipped = VarModule::is_flag(boma.object(), vars::dedede::instance::EQUIP_MASK);
    let mask_is_exist = ArticleModule::is_exist(boma, *FIGHTER_DEDEDE_GENERATE_ARTICLE_MASK);
    if fighter.is_motion(Hash40::new("appeal_s_r")){
        if frame as i32 == 3 
        && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
        && fighter.status() != *FIGHTER_STATUS_KIND_REBIRTH{
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_s_r_mask"), 3.0, 1.0, false, 0.0, false, false);
        }
    }
    if fighter.is_motion(Hash40::new("appeal_s_r_mask")){
        if !mask_is_exist && frame < 11.0{
            ArticleModule::generate_article(boma, *FIGHTER_DEDEDE_GENERATE_ARTICLE_MASK, false, -1);
            let article = ArticleModule::get_article(boma, *FIGHTER_DEDEDE_GENERATE_ARTICLE_MASK);
            let article_id = smash::app::lua_bind::Article::get_battle_object_id(article) as u32;
            let article_boma = sv_battle_object::module_accessor(article_id);
            ModelModule::set_scale(article_boma, 1.1);
            LinkModule::set_model_constraint_pos_ort(article_boma, *LINK_NO_ARTICLE, Hash40::new("attach"), Hash40::new("havel"),  *CONSTRAINT_FLAG_POSITION as u32 | *CONSTRAINT_FLAG_ORIENTATION as u32, false); 
        }
        if frame as i32 == 12{
            if !mask_is_equipped && mask_is_exist{
                let article = ArticleModule::get_article(boma, *FIGHTER_DEDEDE_GENERATE_ARTICLE_MASK);
                let article_id = smash::app::lua_bind::Article::get_battle_object_id(article) as u32;
                let article_boma = sv_battle_object::module_accessor(article_id);
                LinkModule::set_model_constraint_pos_ort(article_boma, *LINK_NO_ARTICLE, Hash40::new("have"), Hash40::new("head"),  *CONSTRAINT_FLAG_POSITION as u32 | *CONSTRAINT_FLAG_ORIENTATION as u32, false); 
                EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 18, 3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
                VarModule::on_flag(boma.object(), vars::dedede::instance::EQUIP_MASK);
            }
            else{
                ArticleModule::remove_exist(boma, *FIGHTER_DEDEDE_GENERATE_ARTICLE_MASK, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
                VarModule::off_flag(boma.object(), vars::dedede::instance::EQUIP_MASK);
            }
        }  
    }  
    //Model scale on the mask during up tilt 
    if fighter.is_motion(Hash40::new("attack_hi3"))
    && mask_is_exist{
        let article = ArticleModule::get_article(boma, *FIGHTER_DEDEDE_GENERATE_ARTICLE_MASK);
        let article_id = smash::app::lua_bind::Article::get_battle_object_id(article) as u32;
        let article_boma = sv_battle_object::module_accessor(article_id);
        if (fighter.motion_frame() > 7.0 && fighter.motion_frame() < 16.0) {
            ModelModule::set_scale(article_boma, 1.3);
        }
        else{
            ModelModule::set_scale(article_boma, 1.1);
        }
    }
    //Removing shaking from ftilt, dair, up air, rapid jab with the mask on
    if fighter.is_motion_one_of(&[Hash40::new("attack_s3_s"), Hash40::new("attack_air_hi"), Hash40::new("attack_100")])
    && mask_is_exist{
        ShakeModule::stop(boma);
    }

    //ensuring the flag resets when the mask does not exist
    if mask_is_equipped && !mask_is_exist{
        VarModule::off_flag(boma.object(), vars::dedede::instance::EQUIP_MASK);
    }

    // remove mask on entry and death
    if fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_ENTRY, *FIGHTER_STATUS_KIND_DEAD])
    && mask_is_equipped {
        ArticleModule::remove_exist(boma, *FIGHTER_DEDEDE_GENERATE_ARTICLE_MASK, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe fn super_jump_fail_edge_cancel(fighter: &mut L2CFighterCommon){
    if fighter.is_status(*FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_HI_FAILURE) && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        StatusModule::change_status_force(fighter.boma(), *FIGHTER_STATUS_KIND_FALL, false);
    }
}

unsafe fn jet_hammer_charge(fighter: &mut L2CFighterCommon, motion_kind: u64) {
    if fighter.is_motion_one_of(&[
        Hash40::new("special_lw_start"),
        Hash40::new("special_air_lw_start"),
        ]) {

            let is_hold = ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL);
            let charge = VarModule::get_int(fighter.battle_object, vars::dedede::instance::JET_CHARGE_FRAMES) as f32;
            let max_charge_frames = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("hold_max_f"));
            let charge_start_frame = 6.0;
            let charge_end_frame = 11.0;

        if (charge_start_frame..charge_end_frame).contains(&fighter.motion_frame()) && charge < (max_charge_frames) && is_hold{
            let mut rand = app::sv_math::randf(hash40("fighter"), 1.0);
            let mut rand_waist = app::sv_math::randf(hash40("fighter"), 1.0);

            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_DEDEDE_GENERATE_ARTICLE_JETHAMMER){
                let article = ArticleModule::get_article(fighter.module_accessor, *FIGHTER_DEDEDE_GENERATE_ARTICLE_JETHAMMER);
                let object_id = smash::app::lua_bind::Article::get_battle_object_id(article) as u32;
                let article_boma = sv_battle_object::module_accessor(object_id);
                if MotionModule::motion_kind(article_boma) != hash40("hold"){
                    MotionModule::change_motion(article_boma, Hash40::new("hold"), 0.0, 1.0, false, 0.0, false, false);
                }
            }

            ModelModule::set_joint_translate(fighter.module_accessor, Hash40::new("shoulderr"), &Vector3f{x:3.0, y:rand, z:-1.0}, false, false); 
            ModelModule::set_joint_translate(fighter.module_accessor, Hash40::new("waist"), &Vector3f{x:3.0, y:0.0, z:rand_waist}, false, false); 

            if charge % 30.0 == 0.0 && fighter.motion_frame() > 6.0{
                EFFECT_FOLLOW(fighter, Hash40::new("sys_level_up"), Hash40::new("hammer3"), 0.0, 0.0, 0.0, 0, 0, 0, 0.5, true);
            }
            if charge % 10.0 == 0.0{
                DamageModule::add_damage(fighter.module_accessor, 1.5, 0);
                FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -11, 0, 0, 0, 0, 0, 0.9, 15, 0, 4, 0, 0, 0, false);
            }

            let motion_rate = (charge_end_frame - charge_start_frame)/(max_charge_frames * 2.0);
            MotionModule::set_rate(fighter.module_accessor, motion_rate);
            VarModule::set_int(fighter.battle_object, vars::dedede::instance::JET_CHARGE_FRAMES, charge as i32 + 1);
            
        } 
        else{
            MotionModule::set_rate(fighter.module_accessor, 1.0);
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
    && (fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
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
        *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_ATTACK
        ]) 
        || fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_S)
        && fighter.motion_frame() > 18.0
    )
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
    gordo_recatch(boma, frame, fighter);
    angled_inhale_shot(fighter);
    super_jump_fail_edge_cancel(fighter);
    fastfall_specials(fighter);
    mask_toggle(fighter, boma, frame);
    jet_hammer_charge(fighter, motion_kind);
}

pub extern "C" fn dedede_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
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

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, dedede_frame_wrapper);
}
