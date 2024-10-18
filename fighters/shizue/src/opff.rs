// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

//Determine if fuel is past threshold
unsafe fn boost_ready(boma: &mut BattleObjectModuleAccessor) {
    if WorkModule::get_float(boma, *FIGHTER_MURABITO_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_FRAME) >= 100.0 {
        VarModule::on_flag(boma.object(), vars::shizue::status::SPECIAL_HI_EARLY_RELEASE);
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

unsafe fn reel_in(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status(*FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_END) 
    && boma.motion_frame() < 4.0 
    && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) 
    && boma.is_situation(*SITUATION_KIND_GROUND) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_GUARD_ON, true);
    }

    // disable the opponent's collision when reeling them in
    if boma.is_status(*FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_HIT) {
        let caught_id = boma.get_int(*FIGHTER_SHIZUE_STATUS_WORK_ID_SPECIAL_S_INT_TARGET_OBJECT_ID);
        if sv_battle_object::category(caught_id as u32) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
            let caught_boma = sv_battle_object::module_accessor(caught_id as u32);
            GroundModule::set_collidable(caught_boma, false);
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
    if (MotionModule::frame(fighter.module_accessor) > 6.0 && fighter.is_motion_one_of(&[Hash40::new("special_hi"), Hash40::new("special_air_hi")])) || fighter.is_status_one_of(&[*FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_WAIT, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_FLAP]) {
        // Cancel balloon trip early if character is holding shield, allowing for movement
        if fighter.is_button_on(Buttons::Guard) || fighter.is_button_on(Buttons::Catch) || fighter.is_button_on(Buttons::AttackAll) {
            // Check if the user canceled before the initial swing, punishing them by setting their fuel to 0, else set their fuel to 80% of what they had
            if !fighter.is_motion_one_of(&[Hash40::new("special_hi"), Hash40::new("special_air_hi")]) {
                VarModule::on_flag(fighter.object(), vars::shizue::status::SPECIAL_HI_LATE_RELEASE);
                EffectModule::req_follow(fighter.module_accessor, Hash40::new("shizue_putaway_catch"), Hash40::new("bust"), &Vector3f::zero(), &Vector3f::zero(), 0.8, true, 0, 0, 0, 0, 0, false, false);
                VarModule::off_flag(fighter.object(), vars::shizue::status::SPECIAL_HI_EARLY_RELEASE);
            } 
            else {
                VarModule::off_flag(fighter.object(), vars::shizue::status::SPECIAL_HI_LATE_RELEASE);
                EffectModule::req_follow(fighter.module_accessor, Hash40::new("shizue_erase_smoke"), Hash40::new("bust"), &Vector3f::zero(), &Vector3f::zero(), 0.8, true, 0, 0, 0, 0, 0, false, false);
                LAST_EFFECT_SET_ALPHA(fighter, 0.75);
            }
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_DETACH, true);
        }
    }
}

//Add directional boost if they hit fuel threshold when cancelled
unsafe fn balloon_dash(fighter: &mut L2CFighterBase) {
    if VarModule::is_flag(fighter.object(), vars::shizue::status::SPECIAL_HI_EARLY_RELEASE)
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
        VarModule::off_flag(fighter.object(), vars::shizue::status::SPECIAL_HI_EARLY_RELEASE);
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

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    reel_in(boma);
    boost_ready(boma);
    fastfall_specials(fighter);
    balloon_cancel(fighter);
    balloon_dash(fighter);
    fuel_indicators(fighter);
}

// symbol-based call for villager/isabelle's common pocket opff
extern "Rust" {
    fn ac_common(fighter: &mut smash::lua2cpp::L2CFighterCommon);
}

pub extern "C" fn shizue_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		shizue_frame(fighter);
        ac_common(fighter);
    }
}

pub unsafe fn shizue_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, shizue_frame_wrapper);
}