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
        if VarModule::is_flag(&mut *(*object_boma).object(), vars::common::status::NO_POCKET) {
            //Change Villager status
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_FAILURE,false);
            WorkModule::set_int(fighter.module_accessor, 0x50000000, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TARGET_OBJECT_ID);

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
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        ac_moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub unsafe fn ac_moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    let fighter_kind = boma.kind();
    ac_update(fighter);
}
