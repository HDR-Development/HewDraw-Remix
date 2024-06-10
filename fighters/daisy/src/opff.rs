use super::*;
use globals::*;
utils::import_noreturn!(common::opff::fighter_common_opff);

unsafe fn wall_bounce(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if status_kind == *FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_JUMP {
        let lr = PostureModule::lr(boma);
        let frame = MotionModule::frame(boma) as i32;
        let mut touch_wall = false;
        if lr > 0.0 {
            touch_wall = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_RIGHT as u32);
        } else {
            touch_wall = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_LEFT as u32);
        };
        if touch_wall && (1..25).contains(&frame){
                VarModule::on_flag(boma.object(), vars::peach::instance::IS_WALLBOUNCE);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_HIT_END, true);
        }
    }
    else if status_kind == *FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_HIT_END {
        if VarModule::is_flag(boma.object(), vars::daisy::instance::IS_WALLBOUNCE) {
            MotionModule::set_rate(boma, 0.6);
        }
    } else {
        VarModule::off_flag(boma.object(), vars::daisy::instance::IS_WALLBOUNCE);
    }
}

unsafe fn up_special_freefall_land_cancel(fighter: &mut L2CFighterCommon) {
    if fighter.is_prev_status(*FIGHTER_STATUS_KIND_FALL_SPECIAL)
    && fighter.is_status(*FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL) {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_LANDING, false);
    }
}

// transition daisy into unique animations for her third jump
unsafe fn triple_jump_motion(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    if fighter.is_motion_one_of(&[Hash40::new("jump_aerial_f"), Hash40::new("jump_aerial_b")])
    && boma.get_num_used_jumps() == boma.get_jump_count_max() {
        if fighter.is_motion(Hash40::new("jump_aerial_f")) {
            MotionModule::change_motion(boma, Hash40::new("jump_aerial_f2"), 0.0, 1.0, false, 0.0, false, false);
        } else {
            MotionModule::change_motion(boma, Hash40::new("jump_aerial_b2"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
}

// forcibly remove daisy's parasol from select animations
unsafe fn parasol_removal(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    let is_parasol_motion = fighter.is_motion_one_of(&[
        Hash40::new("entry_l"),
        Hash40::new("entry_r"),
        Hash40::new("fall_special"),
        Hash40::new("landing_fall_special")
    ]);

    if is_parasol_motion
    && ArticleModule::is_exist(boma, *FIGHTER_DAISY_GENERATE_ARTICLE_KASSAR) {
        ArticleModule::remove_exist(boma, *FIGHTER_DAISY_GENERATE_ARTICLE_KASSAR, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

// handles the hit team for daisy's vegetable item, allowing it to be hit around
unsafe fn set_vegetable_team(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    if ItemModule::get_have_item_kind(fighter.boma(), 0) == *ITEM_KIND_DAISYDAIKON {
        let item_id = ItemModule::get_have_item_id(boma, 0) as u32;
        let item_boma = sv_battle_object::module_accessor(item_id);
        VarModule::set_int(boma.object(), vars::daisy::instance::VEGETABLE_ID, item_id as i32);
        let team = TeamModule::hit_team_no(boma) as i32;
        TeamModule::set_team(item_boma, team, true);
        TeamModule::set_hit_team(item_boma, team);
    } else {
        let item_id = VarModule::get_int(boma.object(), vars::daisy::instance::VEGETABLE_ID) as u32;
        let item_boma = sv_battle_object::module_accessor(item_id);
        // measure how far the item is from daisy
        let x_distance = PostureModule::pos_x(boma) - PostureModule::pos_x(item_boma);
        let y_distance = (PostureModule::pos_y(boma) + 10.0) - PostureModule::pos_y(item_boma);
        // make sure the item is far enough away from daisy to prevent oddities with hitting herself
        let is_separated = x_distance.abs() > 15.0 || y_distance.abs() > 15.0;

        if TeamModule::hit_team_no(item_boma) as i32 != -1
        && is_separated {
            //println!("changing {} hit team to universal", TeamModule::hit_team_no(item_boma));
            TeamModule::set_hit_team(item_boma, -1);
        }

        let current_damage = DamageModule::damage(item_boma, 0);
        let prev_damage = VarModule::get_float(boma.object(), vars::daisy::instance::VEGETABLE_DAMAGE);
        if current_damage > prev_damage && is_separated { 
            if !AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) {
                let num_players = Fighter::get_fighter_entry_count();
                let mut opponent_team = 7;
                for i in 0..num_players{
                    let opponent_boma = sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(i));
                    if AttackModule::is_infliction(opponent_boma, *COLLISION_KIND_MASK_HIT) {
                        opponent_team = TeamModule::hit_team_no(opponent_boma) as i32;
                        //println!("Hit by opponent on team {}", opponent_team);
                        break;
                    }
                }
                //println!("Changing to team {}", opponent_team);
                TeamModule::set_team(item_boma, opponent_team, false);
                StatusModule::change_status_force(item_boma, *ITEM_STATUS_KIND_THROW, true); // resets the throw status so the hitbox doesn't clear
            } else {
                //println!("Hit by Daisy");
                let team = TeamModule::hit_team_no(boma) as i32;
                TeamModule::set_team(item_boma, team, false);
                StatusModule::change_status_force(item_boma, *ITEM_STATUS_KIND_THROW, true); // resets the throw status so the hitbox doesn't clear
            }
            VarModule::set_float(boma.object(), vars::daisy::instance::VEGETABLE_DAMAGE, current_damage);
        } else if current_damage != prev_damage {
            // handle the variable in other scenarios. mostly on item despawn
            VarModule::set_float(boma.object(), vars::daisy::instance::VEGETABLE_DAMAGE, current_damage);
        }
    }
}

use vars::daisy::instance::*;

// fuck effect iteration lol
unsafe fn effect_helper(fighter: &mut L2CFighterCommon) {
    // enter effect edit mode if all buttons are held, otherwise end the function
    if !VarModule::is_flag(fighter.object(), IS_EDIT_EFF) {
        if fighter.is_button_on(Buttons::Attack)
        && fighter.is_button_on(Buttons::Special)
        && fighter.is_button_on(Buttons::Guard)
        && fighter.is_button_on(Buttons::AppealAll) {
            println!("Entering effect editing mode."); 
            let vec1 = Vector4f{ x: 0.85, y: 0.85, z: 0.85, w: 0.2};
            let vec2 = Vector4f{ x: 0.0, y: 1.0, z: 0.0, w: 0.8};
            ColorBlendModule::set_main_color(fighter.boma(), &vec1, &vec2, 0.21, 2.2, 5, true);
            MotionModule::set_rate(fighter.boma(), 0.0);
            VarModule::on_flag(fighter.object(), IS_EDIT_EFF);

            // display what the player will be changing by default
            let mode = VarModule::get_int(fighter.object(), EFF_MODE);
            let axis = match VarModule::get_int(fighter.object(), EFF_AXIS) {
                0 => "x",
                1 => "y",
                _ => "z"
            };
            match mode {
                0 => println!("editing {} position", axis),
                1 => println!("editing {} rotation", axis),
                _ => println!("editing effect scale")
            }

            return;
        } 
        
        else { return; }
    }

    // if the code made it to this point, that means we are in effect editing mode

    // cycle between the different modes with left taunt
    if fighter.is_button_trigger(Buttons::AppealSL) {
        let mode = VarModule::get_int(fighter.object(), EFF_MODE);
        let axis = match VarModule::get_int(fighter.object(), EFF_AXIS) {
            0 => "x",
            1 => "y",
            _ => "z"
        };
        match mode {
            0 => {
                VarModule::set_int(fighter.object(), EFF_MODE, 1);
                println!("editing {} rotation", axis);
            }
            1 => {
                VarModule::set_int(fighter.object(), EFF_MODE, 2);
                println!("editing effect scale");
            }
            _ => {
                VarModule::set_int(fighter.object(), EFF_MODE, 0);
                println!("editing {} position", axis);
            }
        }
    }

    // cycle between the different axis with right taunt
    if fighter.is_button_trigger(Buttons::AppealSR)
    && VarModule::get_int(fighter.object(), EFF_MODE) != 2 {
        let mode = match VarModule::get_int(fighter.object(), EFF_MODE) {
            0 => "position",
            _ => "rotation"
        };
        let axis = VarModule::get_int(fighter.object(), EFF_AXIS);
        match axis {
            0 => {
                VarModule::set_int(fighter.object(), EFF_AXIS, 1);
                println!("editing y {}", mode);
            }
            1 => {
                VarModule::set_int(fighter.object(), EFF_AXIS, 2);
                println!("editing z {}", mode);
            }
            _ => {
                VarModule::set_int(fighter.object(), EFF_AXIS, 0);
                println!("editing x {}", mode);
            }
        }
    }

    // adjust the value with up and down taunt
    if fighter.is_button_trigger(Buttons::AppealHi | Buttons::AppealLw) {
        let mode = VarModule::get_int(fighter.object(), EFF_MODE);
        let axis = VarModule::get_int(fighter.object(), EFF_AXIS);
        // set whether to increase or decrease the value depending on the button
        let mul = 
            if fighter.is_button_trigger(Buttons::AppealHi) { 1.0 }
            else { -1.0 };
        // define how much we are changing the value by
        let increment = match mode {
            /* position */ 0 => 0.5,
            /* rotation */ 1 => 5.0,
            /* scale */ _ => 0.05
        };
        // figure out which value we are supposed to be changing
        let eff_const = 
            if mode == 0 { match axis {
                0 => EFF_POS_X,
                1 => EFF_POS_Y,
                _ => EFF_POS_Z
            } } 
            else if mode == 1 { match axis {
                0 => EFF_ROT_X,
                1 => EFF_ROT_Y,
                _ => EFF_ROT_Z
            } }
            else {
                EFF_SCALE
            };

        // change the value
        VarModule::add_float(fighter.object(), eff_const, increment * mul);

        // print our current effect settings to the terminal
        println!(
            "Effect parameters:
            Position - x: {}, y: {}, z: {}
            Rotation - x: {}, y: {}, z: {}
            Scale - {:.2}",
            VarModule::get_float(fighter.object(), EFF_POS_X),
            VarModule::get_float(fighter.object(), EFF_POS_Y),
            VarModule::get_float(fighter.object(), EFF_POS_Z),
            VarModule::get_float(fighter.object(), EFF_ROT_X),
            VarModule::get_float(fighter.object(), EFF_ROT_Y),
            VarModule::get_float(fighter.object(), EFF_ROT_Z),
            VarModule::get_float(fighter.object(), EFF_SCALE)
        );
    }

    // exit effect editing mode by pressing the attack button
    if fighter.is_button_trigger(Buttons::Attack) {
        println!("Exiting effect editing mode.");
        ColorBlendModule::cancel_main_color(fighter.boma(), 0);
        MotionModule::set_rate(fighter.boma(), 1.0);
        VarModule::off_flag(fighter.object(), IS_EDIT_EFF);
    }

    // to utilize these dynamic values in an effect script, use the following code
    // 
    // let x_pos = VarModule::get_float(agent.object(), vars::daisy::instance::EFF_POS_X);
    // let y_pos = VarModule::get_float(agent.object(), vars::daisy::instance::EFF_POS_Y);
    // let z_pos = VarModule::get_float(agent.object(), vars::daisy::instance::EFF_POS_Z);
    // let x_rot = VarModule::get_float(agent.object(), vars::daisy::instance::EFF_ROT_X);
    // let y_rot = VarModule::get_float(agent.object(), vars::daisy::instance::EFF_ROT_Y);
    // let z_rot = VarModule::get_float(agent.object(), vars::daisy::instance::EFF_ROT_Z);
    // let scale = VarModule::get_float(agent.object(), vars::daisy::instance::EFF_SCALE);
    // 
    // args: x_pos, y_pos, z_pos, x_rot, y_rot, z_rot, scale
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_PEACH_STATUS_KIND_SPECIAL_N_HIT,
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
    wall_bounce(boma, status_kind);
    //up_special_freefall_land_cancel(fighter);
    triple_jump_motion(fighter, boma);
    parasol_removal(fighter, boma);
    set_vegetable_team(fighter, boma);
    effect_helper(fighter);
    fastfall_specials(fighter);
}

pub unsafe extern "C" fn daisy_frame_wrapper(fighter: &mut L2CFighterCommon) {
    common::opff::fighter_common_opff(fighter);
    daisy_frame(fighter)
}

pub unsafe fn daisy_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, daisy_frame_wrapper);
}
