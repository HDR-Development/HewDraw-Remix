// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

extern "Rust" {
    fn gimmick_flash(boma: &mut BattleObjectModuleAccessor);
}

/// handle speed application
unsafe fn check_apply_speeds(fighter: &mut smash::lua2cpp::L2CFighterCommon) {

    // handle speed application once
    if VarModule::is_flag(fighter.battle_object, vars::packun::instance::STANCE_ENABLE_CHANGE_SPEED) {
        if VarModule::get_int(fighter.battle_object, vars::packun::instance::CURRENT_STANCE) == 0 {
            apply_status_speed_mul(fighter, 1.0);
        } else if fighter.is_status_one_of(&[
            *FIGHTER_STATUS_KIND_ESCAPE_F,
            *FIGHTER_STATUS_KIND_ESCAPE_B,
            *FIGHTER_STATUS_KIND_SLIP_STAND_F,
            *FIGHTER_STATUS_KIND_SLIP_STAND_B,
            *FIGHTER_STATUS_KIND_DOWN_STAND_FB,
            *FIGHTER_STATUS_KIND_PASSIVE_FB]) {
                apply_status_speed_mul(fighter, 1.0);
        } else if VarModule::get_int(fighter.battle_object, vars::packun::instance::CURRENT_STANCE) == 1 {
            apply_status_speed_mul(fighter, 0.86);
        } else if VarModule::get_int(fighter.battle_object, vars::packun::instance::CURRENT_STANCE) == 2 {
            apply_status_speed_mul(fighter, 0.84);
        }
        VarModule::off_flag(fighter.battle_object, vars::packun::instance::STANCE_ENABLE_CHANGE_SPEED);
    }

    if fighter.status() != VarModule::get_int(fighter.battle_object, vars::packun::instance::STANCE_STATUS) {
        //println!("Status is changing!");
        VarModule::on_flag(fighter.battle_object, vars::packun::instance::STANCE_ENABLE_CHANGE_SPEED);
        VarModule::set_int(fighter.battle_object, vars::packun::instance::STANCE_STATUS, fighter.status());
        //println!("new stance status: {}", VarModule::get_int(fighter.battle_object, vars::packun::instance::STANCE_STATUS));
    }

    // dash & momentum transfer speeds
    if VarModule::get_int(fighter.battle_object, vars::packun::instance::CURRENT_STANCE) == 1 {
        // Putrid
        VarModule::set_float(fighter.battle_object, vars::common::instance::JUMP_SPEED_MAX_MUL, 1.0);
    }
    else if VarModule::get_int(fighter.battle_object, vars::packun::instance::CURRENT_STANCE) == 2 {
        // Prickly
        VarModule::set_float(fighter.battle_object, vars::common::instance::JUMP_SPEED_MAX_MUL, 0.88);
    }
}

/// applies the given multiplier to various speed stats of the given fighter. 
/// This should only be called once per status, or you will get some multiplicative effects
unsafe fn apply_status_speed_mul(fighter: &mut smash::lua2cpp::L2CFighterCommon, mul: f32) {
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
    let og_speed_mul = app::sv_kinetic_energy::get_speed_mul(fighter.lua_state_agent);

    // set the X motion speed multiplier (where movement is baked into an anim)
    lua_bind::FighterKineticEnergyMotion::set_speed_mul(fighter.get_motion_energy(), og_speed_mul * mul);

    // set the X motion accel multiplier for control energy (used in the air, during walk, fall, etc)
    lua_bind::FighterKineticEnergyController::mul_x_accel_mul(fighter.get_controller_energy(), mul);

    // set the X motion accel multiplier for control energy (used in the air, during walk, fall, etc)
    lua_bind::FighterKineticEnergyController::mul_x_accel_add(fighter.get_controller_energy(), mul);

    // set the X speed max multiplier for control energy (used in the air, during walk, fall, etc)
    lua_bind::FighterKineticEnergyController::mul_x_speed_max(fighter.get_controller_energy(), mul);
}

unsafe fn game_start_switch(fighter: &mut L2CFighterCommon) {
    if fighter.is_prev_status(*FIGHTER_STATUS_KIND_ENTRY) {
        if StatusModule::is_changing(fighter.module_accessor) {
            VarModule::on_flag(fighter.battle_object, vars::packun::status::STANCE_INIT);
        }
        if VarModule::is_flag(fighter.battle_object, vars::packun::status::STANCE_INIT) {
            if fighter.is_button_on(Buttons::AppealSL) {
                SET_STANCE(fighter, 0, false);
                VarModule::off_flag(fighter.battle_object, vars::packun::status::STANCE_INIT);
            }
            else if fighter.is_button_on(Buttons::AppealSR) {
                SET_STANCE(fighter, 1, false);
                VarModule::off_flag(fighter.battle_object, vars::packun::status::STANCE_INIT);
            }
            else if fighter.is_button_on(Buttons::AppealLw) {
                SET_STANCE(fighter, 2, false);
                VarModule::off_flag(fighter.battle_object, vars::packun::status::STANCE_INIT);
            }
        }
        if fighter.status_frame() > 94 {
            VarModule::off_flag(fighter.battle_object, vars::packun::status::STANCE_INIT);
        }
    }
}

unsafe fn maw_end_head(fighter: &mut L2CFighterCommon) {
    // fixes side special forcing normal head somewhere
    if fighter.is_prev_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_CHARGE,
        *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_CANCEL,
        *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_JUMP_CANCEL,
        *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_END,
        *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_SHOOT
    ])
    && VarModule::get_int(fighter.battle_object, vars::packun::instance::CURRENT_STANCE) == 1 {
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("headb"), true);
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("heada"), false);
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("heads"), false);
    }
}

unsafe fn lss_lc(fighter: &mut L2CFighterCommon) {
    if fighter.is_status_one_of(&[*FIGHTER_PACKUN_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_LW_FALL_END])
    || (fighter.is_status(*FIGHTER_PACKUN_STATUS_KIND_SPECIAL_LW_BITE) && fighter.is_flag(*FIGHTER_PACKUN_STATUS_SPECIAL_LW_FLAG_STALK_SHORTEN)) {
        fighter.check_land_cancel(Some(6.0));
    }
}

pub unsafe fn moveset(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    check_apply_speeds(fighter);
    game_start_switch(fighter);
    maw_end_head(fighter);
    lss_lc(fighter);
}

unsafe extern "C" fn plant_meter(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        if !sv_information::is_ready_go() && fighter.status_frame() < 1 {
            return;
        }

        utils::ui::UiManager::set_plant_meter_enable(fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32, true);
        utils::ui::UiManager::set_plant_meter_info(
            fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32,
            VarModule::get_int(fighter.battle_object, vars::packun::instance::CURRENT_STANCE)
        );
    }
}

pub extern "C" fn packun_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		packun_frame(fighter);
    }
}

pub unsafe fn packun_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, packun_frame_wrapper);
    agent.on_line(Main, plant_meter);
}