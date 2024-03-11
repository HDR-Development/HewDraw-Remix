use super::*;
use globals::*;

utils::import_noreturn!(common::opff::fighter_common_opff);

unsafe fn dspecial_cancels(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_CLOUD_STATUS_KIND_SPECIAL_LW_END)
    && fighter.is_situation(*SITUATION_KIND_AIR)
    && WorkModule::get_int(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_LW_INT_CANCEL_STATUS) == *FIGHTER_STATUS_KIND_ESCAPE_AIR
    {
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_NONE, *FIGHTER_CLOUD_STATUS_SPECIAL_LW_INT_CANCEL_STATUS);
    }
}

// Fixes bug where touching ground during upB puts you into special fall for 1f before landing
unsafe fn up_special_proper_landing(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI)
    && fighter.is_situation(*SITUATION_KIND_GROUND)
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_FALL) {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, false);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_FALL);
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S2,
        *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S3,
        *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_HI2_FALL,
        *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_LW_CHARGE,
        *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_LW_END
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

pub unsafe extern "C" fn cloud_frame_wrapper(fighter: &mut L2CFighterCommon) {
    common::opff::fighter_common_opff(fighter);

    dspecial_cancels(fighter);
    up_special_proper_landing(fighter);
    fastfall_specials(fighter);
}

pub unsafe extern "C" fn is_cloud_rock(object_boma: *mut BattleObjectModuleAccessor) -> bool {
    if utility::get_kind(&mut *object_boma) == *WEAPON_KIND_SHEIK_NEEDLE {
        let owner_id = WorkModule::get_int(object_boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
        let owner_boma = smash::app::sv_battle_object::module_accessor(owner_id);
        let owner_kind = utility::get_kind(&mut *owner_boma);
        if owner_kind == *FIGHTER_KIND_CLOUD {
            return true;
        }
    }
    return false;
}

//To prevent crashes when Isabelle/Villager pocket
pub unsafe extern "C" fn ac_update(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    
    if status_kind == *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_SEARCH {
        let object_id = WorkModule::get_int(fighter.module_accessor,*FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TARGET_OBJECT_ID) as u32;
        if object_id == 0 || object_id == 0x50000000 {return;}
        let object_boma = sv_battle_object::module_accessor(object_id);
        if is_cloud_rock(object_boma) {
            //Change Villager status
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_FAILURE,false);
            WorkModule::set_int(fighter.module_accessor, 0x50000000, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TARGET_OBJECT_ID);

            //Remove object
            //let weapon = get_fighter_common_from_accessor(object_boma);
            //smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
            WorkModule::set_int(object_boma, -1, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
            AttackModule::clear_all(object_boma);

            let pos = *PostureModule::pos(object_boma);
            EffectModule::req(
                object_boma,
                Hash40::new("sys_erace_smoke"),
                &Vector3f{x:pos.x,y:pos.y+2.0,z:pos.z},
                &Vector3f{x:0.0,y:0.0,z:0.0},
                0.625,
                0,
                -1,
                false,
                0
            );
            
        }
    }
}

pub fn install() {
    smashline::Agent::new("cloud")
        .on_line(Main, cloud_frame_wrapper)
        .install();
    Agent::new("murabito")
        .on_line(Main, ac_update)
        .install();
    Agent::new("shizue")
        .on_line(Main, ac_update)
        .install();
}