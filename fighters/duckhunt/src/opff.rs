// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn duck_jump_cancel(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_DUCKHUNT_STATUS_KIND_SPECIAL_HI_FLY)
    && fighter.motion_frame() > 20.0
    && fighter.is_cat_flag(Cat1::SpecialHi) {
        fighter.change_status_req(*FIGHTER_DUCKHUNT_STATUS_KIND_SPECIAL_HI_END, true);
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_DUCKHUNT_STATUS_KIND_SPECIAL_HI_END
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

extern "Rust" {
    fn gimmick_flash(boma: &mut BattleObjectModuleAccessor);
}

unsafe fn gunman_timer(fighter: &mut L2CFighterCommon) {
    let timer = VarModule::get_int(fighter.object(), vars::duckhunt::instance::GUNMAN_TIMER);
    if  timer != 0 {
        VarModule::set_int(fighter.object(), vars::duckhunt::instance::GUNMAN_TIMER, (timer-1));
    }
    if timer == 1 {
        gimmick_flash(fighter);
    }
}

pub extern "C" fn duckhunt_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
        duck_jump_cancel(fighter);
        gunman_timer(fighter);
        fastfall_specials(fighter);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, duckhunt_frame_wrapper);
}