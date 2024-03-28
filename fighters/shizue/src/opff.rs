// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;
 
unsafe fn fishing_rod_shield_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, frame: f32) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_START].contains(&status_kind) {
        if frame < 25.0 {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
                if situation_kind == *SITUATION_KIND_GROUND {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_GUARD_ON, true);
                }
            }
        }
    }
}

unsafe fn fair_scale(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if fighter.is_motion(Hash40::new("attack_air_f"))  {
        if fighter.motion_frame() > 13.0 || fighter.motion_frame() < 17.0 {
            ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("havel"), &Vector3f::new(1.075, 1.075, 1.075));
        }
    }
}

//Determine if fuel is past threshold
unsafe fn boost_ready(boma: &mut BattleObjectModuleAccessor) {
    if WorkModule::get_float(boma, *FIGHTER_MURABITO_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_FRAME) >= 100.0 {
        VarModule::on_flag(boma.object(), vars::shizue::status::IS_DETACH_BOOST);
    }
}

extern "Rust" {
    fn gimmick_flash(boma: &mut BattleObjectModuleAccessor);
}

//Use handy effects to show when Isabelle reaches below or above 50% fuel
unsafe fn fuel_indicators(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    let fuel = WorkModule::get_float(fighter.boma(), *FIGHTER_MURABITO_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_FRAME);
    if fuel >= 99.4 && fuel <= 100.0 && !fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_HI, 
        *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_WAIT,
        *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_TURN,
        *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_FLAP,
        *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_DETACH,
        *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_LANDING,
        ]) {
        gimmick_flash(fighter);
    } else if fuel >= 99.4 && fuel <= 100.0 {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0.0, 6.0, 0.0, 0.0, 0.0, 0.0, 0.7, true);
    }
}



//Cancel aerials on hit into Balloon Trip
unsafe fn balloon_special_cancel(fighter: &mut L2CFighterCommon) {
    let boma = fighter.boma();
    if fighter.is_status(*FIGHTER_STATUS_KIND_ATTACK_AIR)
    && (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD))
    && !fighter.is_in_hitlag() 
    && VarModule::is_flag(fighter.object(), vars::shizue::status::IS_DETACH_BOOST) {
        if fighter.is_cat_flag(Cat1::SpecialHi) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, false);
        }
    }
}

//Cancel anything on hit into Lloid Call
// unsafe fn lloid_special_cancel(fighter: &mut L2CFighterCommon) {
//     let boma = fighter.boma();
//     if (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD))
//     && !fighter.is_in_hitlag()  
//     && VarModule::is_flag(fighter.battle_object, vars::shizue::status::IS_LLOID_READY) {
//         if fighter.is_cat_flag(Cat1::SpecialLw) {
//             StatusModule::change_status_request_from_script(boma, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_LW_FIRE, false);
//         }
//     }
// }

//Reel in
unsafe fn reel_in(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, frame: f32) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if status_kind == *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_END {
        if frame < 4.0 {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
                if situation_kind == *SITUATION_KIND_GROUND {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_GUARD_ON, true);
                }
            }
        }
    }
}

//Lloid Trap Fire Jump Cancel
unsafe fn lloid_trap_fire_jc(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, stick_x: f32, facing: f32, frame: f32) {
    if status_kind == *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_LW_FIRE {
        if boma.status_frame() > 5 && !boma.is_in_hitlag() {
            boma.check_jump_cancel(false, false);
        }
    }
}

//Balloon Trip Cancel
unsafe fn balloon_cancel(fighter: &mut L2CFighterCommon) {
    if StatusModule::is_changing(fighter.module_accessor) {
        return;
    }
    if (MotionModule::frame(fighter.module_accessor) > 6.0 && fighter.is_motion_one_of(&[Hash40::new("special_hi"), Hash40::new("special_air_hi")])) || fighter.is_status_one_of(&[*FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_WAIT, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_FLAP]) {
        // Cancel balloon trip early if character is holding shield, allowing for movement
        if fighter.is_button_on(Buttons::Guard) || fighter.is_button_on(Buttons::Catch) || fighter.is_button_on(Buttons::AttackAll) {
            // Check if the user canceled before the initial swing, punishing them by setting their fuel to 0, else set their fuel to 80% of what they had
            if !fighter.is_motion_one_of(&[Hash40::new("special_hi"), Hash40::new("special_air_hi")]) {
                if WorkModule::get_float(fighter.module_accessor, *FIGHTER_MURABITO_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_FRAME) > 100.0 {
                    VarModule::set_float(fighter.object(), vars::shizue::instance::STORED_BALLOON_POWER, WorkModule::get_float(fighter.module_accessor, *FIGHTER_MURABITO_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_FRAME) - 100.0);
                }
                else {
                    VarModule::set_float(fighter.object(), vars::shizue::instance::STORED_BALLOON_POWER, 1.0);
                }
                VarModule::on_flag(fighter.object(), vars::shizue::status::IS_NOT_QUICK_RELEASE);
                EffectModule::req_follow(fighter.module_accessor, Hash40::new("shizue_putaway_catch"), Hash40::new("bust"), &Vector3f::zero(), &Vector3f::zero(), 0.8, true, 0, 0, 0, 0, 0, false, false);
                VarModule::off_flag(fighter.object(), vars::shizue::status::IS_DETACH_BOOST);
            } 
            else {
                VarModule::off_flag(fighter.object(), vars::shizue::status::IS_NOT_QUICK_RELEASE);
                VarModule::set_float(fighter.object(), vars::shizue::instance::STORED_BALLOON_POWER, 1.0);
                EffectModule::req_follow(fighter.module_accessor, Hash40::new("shizue_erase_smoke"), Hash40::new("bust"), &Vector3f::zero(), &Vector3f::zero(), 0.8, true, 0, 0, 0, 0, 0, false, false);
                LAST_EFFECT_SET_ALPHA(fighter, 0.75);
            }
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_DETACH, true);
        }
    }
}

//Add directional boost if they hit fuel threshold when cancelled
unsafe fn balloon_dash(fighter: &mut L2CFighterBase) {
    if VarModule::is_flag(fighter.object(), vars::shizue::status::IS_DETACH_BOOST)
    && fighter.is_status(*FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_DETACH) {
        let lr = PostureModule::lr(fighter.boma());
        let mut x_component = fighter.stick_x() * lr * 3.5;
        let mut y_component = fighter.stick_y() * 2.0 - 1.0;
        let mut flip = *EF_FLIP_NONE;
        if fighter.stick_y() >= 0.90 {
            y_component = 1.0;
        }
        let rot = (fighter.stick_y() / fighter.stick_x() * lr).atan().to_degrees();
        let motion_vec = Vector3f::new(x_component, y_component, 0.0);
        KineticModule::add_speed(fighter.boma(), &motion_vec);
        PLAY_STATUS(fighter, Hash40::new("se_shizue_special_l02"));
        if lr == 1.0 {
            flip = *EFFECT_AXIS_Y;
        } 
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("shizue_clayrocket_jet"), Hash40::new("shizue_clayrocket_jet"), Hash40::new("top"), 0, 6.0, 0, rot + 90.0, 0, 180, 1.5, true, flip);
        VarModule::off_flag(fighter.object(), vars::shizue::status::IS_DETACH_BOOST);
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_POCKET,
        *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_SEARCH,
        *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_FAILURE,
        *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_TAKE_OUT,
        *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_END,
        *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_DETACH,
        *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_LW_FAILURE,
        *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_LW_FIRE
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
    //fishing_rod_shield_cancel(boma, status_kind, situation_kind, frame);
    reel_in(boma, status_kind, situation_kind, frame);
    //lloid_trap_fire_jc(boma, status_kind, situation_kind, cat[0], stick_x, facing, frame);
    boost_ready(boma);
    fastfall_specials(fighter);
    balloon_cancel(fighter);
    balloon_dash(fighter);
    balloon_special_cancel(fighter);
    fuel_indicators(fighter);
    fair_scale(fighter);
}

pub extern "C" fn shizue_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		shizue_frame(fighter);
    }
}

pub unsafe fn shizue_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, shizue_frame_wrapper);
}
