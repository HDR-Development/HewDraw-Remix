use super::*;
use smash::app::BattleObjectModuleAccessor;
use globals::*;

// deletes articles that shouldn't be pocketable
unsafe fn ac_update(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    
    if status_kind == *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_SEARCH {
        let object_id = WorkModule::get_int(fighter.module_accessor,*FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TARGET_OBJECT_ID) as u32;
        if object_id == 0 || object_id == 0x50000000 {return;}
        let object_boma = sv_battle_object::module_accessor(object_id);
        let object_object = utils::util::get_battle_object_from_id(object_id);
        if !VarModule::has_var_module(object_object) { return; }
        if VarModule::is_flag(&mut *(*object_boma).object(), vars::common::status::NO_POCKET) {
            //Change Villager status
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_FAILURE,false);
            WorkModule::set_int(fighter.module_accessor, 0x50000000, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TARGET_OBJECT_ID);

            //fighter-specific code
            let owner_id = WorkModule::get_int(object_boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
            if sv_battle_object::is_active(owner_id) {
                let owner = get_battle_object_from_id(owner_id);
                // packun
                if (&mut *(*owner).module_accessor).kind() == *FIGHTER_KIND_PACKUN {
                    VarModule::set_float(owner, vars::packun::instance::FIRE_POS_X, 0.0);
                    VarModule::set_float(owner, vars::packun::instance::FIRE_POS_Y, 0.0);
                }
            }

            //Remove article
            let weapon = get_fighter_common_from_accessor(&mut *(object_boma));
            smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
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

#[no_mangle]
pub unsafe extern "Rust" fn ac_common(fighter: &mut L2CFighterCommon) {
    ac_update(fighter);
}