use super::*;

#[status_script(agent = "donkey", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {

    // if you are grounded, pick up heavy item/spawn barrel
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {

        // if you aren't already holding an item, try to pick up one nearby
        if !ItemModule::is_have_item(fighter.module_accessor, 0) {
            ItemModule::pickup_item(fighter.module_accessor, ItemSize{_address: *ITEM_SIZE_HEAVY as u8}, *FIGHTER_HAVE_ITEM_WORK_MAIN, *ITEM_TRAIT_ALL, QuickItemTreatType{_address: *QUICK_ITEM_TREAT_TYPE_FORCE_HAVE as u8}, ItemPickupSearchMode{_address: *ITEM_PICKUP_SEARCH_MODE_NORMAL as u8});
        }
        if !ItemModule::is_have_item(fighter.module_accessor, 0) {
            ItemModule::pickup_item(fighter.module_accessor, ItemSize{_address: *ITEM_SIZE_LIGHT as u8}, *FIGHTER_HAVE_ITEM_WORK_MAIN, *ITEM_TRAIT_ALL, QuickItemTreatType{_address: *QUICK_ITEM_TREAT_TYPE_FORCE_HAVE as u8}, ItemPickupSearchMode{_address: *ITEM_PICKUP_SEARCH_MODE_NORMAL as u8});
        }
        
        // if you still arent holding an item, try to spawn a barrel
        if !ItemModule::is_have_item(fighter.module_accessor, 0) {
            VarModule::on_flag(fighter.object(), vars::donkey::instance::DID_SPAWN_BARREL);
            let itemmanager = smash2::app::ItemManager::instance().unwrap();
            let barrel_count = smash2::app::ItemManager::get_num_of_ownered_item(
                itemmanager, fighter.boma().battle_object_id, 
                smash2::app::ItemKind::Barrel);
            if barrel_count == 0 {
                ItemModule::have_item(fighter.module_accessor, ItemKind(*ITEM_KIND_BARREL),0,0,false,false);
                EFFECT(fighter, Hash40::new("donkey_handslap"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 0.3, 0, 0, 0, 0, 0, 0, false);
            }
        } else {
            VarModule::off_flag(fighter.object(), vars::donkey::instance::DID_SPAWN_BARREL);
        }
        
        // change into the heavy item pickup status either way
        fighter.change_status(FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP.into(),false.into());
        return true.into();
    }

    // otherwise, proceed with airgrab
    let motion = Hash40::new("special_air_lw");
    let kinetic = *FIGHTER_KINETIC_TYPE_FALL;
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING);
    KineticModule::change_kinetic(fighter.module_accessor, kinetic);
    MotionModule::change_motion(
        fighter.module_accessor,
        motion,
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(special_lw_substatus as *const () as _));
    fighter.main_shift(special_lw_main_loop)
}

unsafe extern "C" fn special_lw_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR)
    && param_1.get_bool() {
        // try to pick up an item nearby
        let frame = fighter.motion_frame();
        if frame > 5.0 && frame < 16.0 {
            let range = 20.0;
            fighter.try_pickup_item(range, Some(Hash40::new("top")), Some(&Vector2f{x: 10.0, y: 0.0}));
        }

        // if at any time during dspecial you are holding 
        // an item, transition into heavy pickup.
        if ItemModule::is_have_item(fighter.boma(), 0) {
            fighter.change_status_req(FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP.into(), false.into());
            grab!(fighter, MA_MSC_CMD_GRAB_CLEAR_ALL);
            return 1.into();
        }
    }
    0.into()
}

unsafe extern "C" fn special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_air = MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_lw");
    let situation = fighter.global_table[SITUATION_KIND].get_i32();
    if is_air {
        if CancelModule::is_enable_cancel(fighter.module_accessor) {
            if fighter.sub_transition_group_check_air_landing().get_bool()
            || fighter.sub_wait_ground_check_common(false.into()).get_bool()
            || fighter.sub_air_check_fall_common().get_bool() {
                return 1.into();
            }
        }
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        let status = if situation != *SITUATION_KIND_GROUND {
            FIGHTER_STATUS_KIND_FALL
        }
        else {
            WorkModule::set_float(fighter.module_accessor, 20.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
            FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL
        };
        fighter.change_status(status.into(), false.into());
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        let status = if is_air {
            FIGHTER_STATUS_KIND_FALL
        }
        else {
            FIGHTER_DONKEY_STATUS_KIND_SPECIAL_LW_LOOP
        };
        fighter.change_status(status.into(), false.into());
    }
    1.into()
}

pub fn install() {
    smashline::install_status_scripts!(
        special_lw_main
    );
}
