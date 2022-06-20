use super::*;
use globals::*;
// status script import

// FIGHTER_RYU_STATUS_KIND_DASH_BACK //
#[status_script(agent = "pfushigisou", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn pre_down_special(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        false,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        0,
        0,
        0,
        0
    );

    0.into()
}

#[status_script(agent = "pfushigisou", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn main_down_special(fighter: &mut L2CFighterCommon) -> L2CValue {
    // change summon anim depending on LR
    let motion = if PostureModule::lr(fighter.module_accessor) < 0.0 {
        Hash40::new("solar_beam")
    } else {
        Hash40::new("solar_beam")
    };

    MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);
    
    KineticModule::clear_speed_all(fighter.module_accessor);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);

    fighter.main_shift(solar_beam_main_loop)
}

/// main status loop for metaquick summon
unsafe extern "C" fn solar_beam_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    //fighter.set_joint_rotate("top", Vector3f::new(0.0, 0.0, -45.0));
/*
    // exit if the animation is not done yet
    if !MotionModule::is_end(fighter.module_accessor) {
        
        if fighter.is_situation(*SITUATION_KIND_AIR) && fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        } else if fighter.is_situation(*SITUATION_KIND_GROUND) && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
        return 0.into();
    }

    GroundModule::attach_ground(fighter.boma(), true);
*/
    if MotionModule::is_end(fighter.module_accessor) {
        // if the animation is over, transition to fall or wait
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        } else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
       }
    }

    1.into()
}

#[status_script(agent = "pfushigisou", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn end_down_special(fighter: &mut L2CFighterCommon) -> L2CValue {
    // re-enable energies and remove the screenwide effect
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    0.into()
}


extern "C" {
    #[link_name = "_ZN3app34WeaponSpecializer_PTrainerPTrainer22request_change_pokemonERNS_6WeaponE"]
    fn request_change_pokemon() -> bool;
}

#[skyline::hook(replace=request_change_pokemon)]
unsafe fn request_change_pokemon_hook() -> bool {
    return false;
}

pub fn install() {
    install_status_scripts!(
        pre_down_special,
        main_down_special,
        end_down_special
    );
}