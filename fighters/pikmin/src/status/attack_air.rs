use super::*;
use globals::*;

pub unsafe extern "C" fn attack_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air();
    fighter.main_shift(attack_air_main_loop)
}

unsafe extern "C" fn attack_air_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        let status = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING) {
            FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR
        }
        else {
            FIGHTER_STATUS_KIND_LANDING
        };
        fighter.change_status(status.into(), false.into());
        return 0.into();
    }
    else {
        if fighter.sub_transition_group_check_air_landing().get_bool() {
            return 0.into();
        }
    }
    let fall_special = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_ATTACK_AIR_WORK_FLAG_FALL_SPECIAL);
    if !fall_special {
        if CancelModule::is_enable_cancel(fighter.module_accessor) {
            if fighter.sub_wait_ground_check_common(false.into()).get_bool()
            || fighter.sub_air_check_fall_common().get_bool() {
                return 0.into();
            }
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        let status = if fall_special {
            FIGHTER_STATUS_KIND_FALL_SPECIAL
        }
        else {
            FIGHTER_STATUS_KIND_FALL
        };
        fighter.change_status(status.into(), false.into());
        return 0.into();
    }
    if !fighter.global_table[IS_STOPPING].get_bool() {
        fighter.sub_attack_air_uniq_process_exec_fix_pos();
        pikmin_attack_air_handle_pikmin(fighter);
    }
    0.into()
}

extern "C" {
    #[link_name = "\u{1}_ZN3app44FighterPikminLinkEventWeaponPikminConstraint13new_l2c_tableEv"]
    fn FighterPikminLinkEventWeaponPikminConstraint__new_l2c_table() -> L2CValue;

    #[link_name = "\u{1}_ZN3app46FighterPikminLinkEventWeaponPikminChangeMotion13new_l2c_tableEv"]
    fn FighterPikminLinkEventWeaponPikminChangeMotion__new_l2c_table() -> L2CValue;

    #[link_name = "\u{1}_ZN3app46FighterPikminLinkEventWeaponPikminChangeStatus13new_l2c_tableEv"]
    fn FighterPikminLinkEventWeaponPikminChangeStatus__new_l2c_table() -> L2CValue;

    #[link_name = "\u{1}_ZN3app40FighterPikminLinkEventWeaponPikminSyncLR13new_l2c_tableEv"]
    fn FighterPikminLinkEventWeaponPikminSyncLR__new_l2c_table() -> L2CValue;

    #[link_name = "\u{1}_ZN3app14LinkEventThrow13new_l2c_tableEv"]
    fn new_event_table() -> L2CValue;

    #[link_name = "\u{1}_ZN3app8lua_bind31LinkEvent__store_l2c_table_implEPKNS_9LinkEventE"]
    fn store_event_table(event: *const app::LinkEvent) -> L2CValue;
}

unsafe extern "C" fn pikmin_attack_air_handle_pikmin(fighter: &mut L2CFighterCommon) {
    fighter.sub_attack_air_uniq_process_exec();
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PIKMIN_STATUS_ATTACK_AIR_WORK_FLAG_SYNC) {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PIKMIN_STATUS_ATTACK_AIR_WORK_FLAG_DETACH) {
            return;
        }
        pikmin_attack_air_handle_pikmin_detatch(fighter);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_PIKMIN_STATUS_ATTACK_AIR_WORK_FLAG_DETACH);
    }
    else {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_PIKMIN_STATUS_ATTACK_AIR_WORK_FLAG_SYNC);
        let fightermoduleaccessor = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut FighterModuleAccessor;
        FighterSpecializer_Pikmin::hold_pikmin(fightermoduleaccessor, 1);
        FighterSpecializer_Pikmin::update_hold_pikmin_param(fightermoduleaccessor);
        let hold_pikmin_num = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_NUM);
        if hold_pikmin_num <= 0 {
            return;
        }
        let pikmin_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_OBJECT_ID_0);
        let ret = LinkModule::link(fighter.module_accessor, *FIGHTER_PIKMIN_LINK_NO_PIKMIN, pikmin_id as u32);
        if ret & 1 != 0 {
            let mut link_event = FighterPikminLinkEventWeaponPikminConstraint__new_l2c_table();
            link_event["link_event_kind_"].assign(&L2CValue::Hash40(Hash40::new("fighter_pikmin_link_event_weapon_pikmin_constraint")));
            link_event[0xf2a5bf2beu64].assign(&L2CValue::Hash40(Hash40::new("top")));
            link_event["joint_id_"].assign(&L2CValue::Hash40(Hash40::new("top")));
            let object_id = fighter.global_table[OBJECT_ID].get_u32();
            link_event[0xaa79e68a2u64].assign(&L2CValue::U32(object_id));
            link_event_store_l2c_table(fighter, FIGHTER_PIKMIN_LINK_NO_PIKMIN.into(), link_event);
            LinkModule::set_attribute(fighter.module_accessor, *FIGHTER_PIKMIN_LINK_NO_PIKMIN, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_STOP as u8}, true);
            LinkModule::set_attribute(fighter.module_accessor, *FIGHTER_PIKMIN_LINK_NO_PIKMIN, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_ATTACK_STOP as u8}, true);

            let mut link_event = FighterPikminLinkEventWeaponPikminChangeMotion__new_l2c_table();
            link_event["link_event_kind_"].assign(&L2CValue::Hash40(Hash40::new("fighter_pikmin_link_event_weapon_pikmin_change_motion")));
            let motion = MotionModule::motion_kind(fighter.module_accessor);
            link_event["motion_kind_"].assign(&L2CValue::Hash40(Hash40::new_raw(motion)));
            link_event["start_frame_"].assign(&L2CValue::F32(0.0));
            link_event["rate_"].assign(&L2CValue::F32(1.0));
            link_event["loop_"].assign(&L2CValue::Bool(false));
            let object_id = fighter.global_table[OBJECT_ID].get_u32();
            link_event[0xaa79e68a2u64].assign(&L2CValue::U32(object_id));
            link_event_store_l2c_table(fighter, FIGHTER_PIKMIN_LINK_NO_PIKMIN.into(), link_event);

            let mut link_event = FighterPikminLinkEventWeaponPikminChangeStatus__new_l2c_table();
            link_event["link_event_kind_"].assign(&L2CValue::Hash40(Hash40::new("fighter_pikmin_link_event_weapon_pikmin_change_status")));
            link_event["status_kind_"].assign(&L2CValue::I32(*WEAPON_PIKMIN_PIKMIN_STATUS_KIND_ATTACK_AIR));
            let object_id = fighter.global_table[OBJECT_ID].get_u32();
            link_event[0xaa79e68a2u64].assign(&L2CValue::U32(object_id));
            link_event_store_l2c_table(fighter, FIGHTER_PIKMIN_LINK_NO_PIKMIN.into(), link_event);

            let mut link_event = FighterPikminLinkEventWeaponPikminSyncLR__new_l2c_table();
            link_event["link_event_kind_"].assign(&L2CValue::Hash40(Hash40::new("fighter_pikmin_link_event_weapon_pikmin_sync_lr")));
            let lr = PostureModule::lr(fighter.module_accessor);
            link_event["lr_"].assign(&L2CValue::F32(lr));
            let object_id = fighter.global_table[OBJECT_ID].get_u32();
            link_event[0xaa79e68a2u64].assign(&L2CValue::U32(object_id));
            link_event_store_l2c_table(fighter, FIGHTER_PIKMIN_LINK_NO_PIKMIN.into(), link_event);

            WorkModule::set_int(fighter.module_accessor, 1, *FIGHTER_PIKMIN_STATUS_ATTACK_AIR_WORK_INT_HAVE_PIKMIN);
        }
    }
}

unsafe extern "C" fn pikmin_attack_air_handle_pikmin_detatch(fighter: &mut L2CFighterCommon) {
    let fightermoduleaccessor = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut FighterModuleAccessor;
    FighterSpecializer_Pikmin::update_hold_pikmin_param(fightermoduleaccessor);
    let hold_pikmin_num = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_NUM);
    if hold_pikmin_num > 0 {
        if LinkModule::is_link(fighter.module_accessor, *FIGHTER_PIKMIN_LINK_NO_PIKMIN) {
            let mut link_event = new_event_table();
            link_event["link_event_kind_"].assign(&L2CValue::Hash40(Hash40::new_raw(0x39ab74e206)));
            let object_id = fighter.global_table[OBJECT_ID].get_u32();
            link_event[0xaa79e68a2u64].assign(&L2CValue::U32(object_id));
            link_event_store_l2c_table(fighter, FIGHTER_PIKMIN_LINK_NO_PIKMIN.into(), link_event);
    
            let mut link_event = FighterPikminLinkEventWeaponPikminChangeStatus__new_l2c_table();
            link_event["link_event_kind_"].assign(&L2CValue::Hash40(Hash40::new("fighter_pikmin_link_event_weapon_pikmin_change_status")));
            link_event["status_kind_"].assign(&L2CValue::I32(*WEAPON_PIKMIN_PIKMIN_STATUS_KIND_AIR_FOLLOW));
            let object_id = fighter.global_table[OBJECT_ID].get_u32();
            link_event[0xaa79e68a2u64].assign(&L2CValue::U32(object_id));
            link_event_store_l2c_table(fighter, FIGHTER_PIKMIN_LINK_NO_PIKMIN.into(), link_event);
        }
        FighterSpecializer_Pikmin::reduce_pikmin_all(fightermoduleaccessor);
    }
}

unsafe extern "C" fn link_event_store_l2c_table(fighter: &mut L2CFighterCommon, link_no: L2CValue, event: L2CValue) -> L2CValue {
    let callable: extern "C" fn() -> *mut app::LinkEvent = std::mem::transmute(event["new_instance_lua_"].get_ptr());
    let link_event = callable();
    lua_bind::LinkEvent::load_from_l2c_table(link_event, &event);
    LinkModule::send_event_parents_struct(fighter.module_accessor, link_no.get_i32(), link_event);
    let ret = store_event_table(link_event);
    let deleter: extern "C" fn(*mut app::LinkEvent) = std::mem::transmute(*((*(link_event as *const u64) + 0x8) as *const u64));
    deleter(link_event);
    ret
}
pub fn install() {
    smashline::Agent::new("pikmin")
        .status(Main, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_main)
        .install();
}
