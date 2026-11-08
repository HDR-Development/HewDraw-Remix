use super::*;

pub unsafe extern "C" fn start_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NORMAL,
        *GROUND_CORRECT_KIND_NONE as u32,
        smashline::skyline_smash::app::GroundCliffCheckKind(0),
        false,
        0,
        0,
        0,
        0,
    );

    return 0.into();
}

pub unsafe extern "C" fn start_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let stone_count = 5;
    VarModule::set_int(weapon.battle_object, vars::plizardon_rock::status::MAX_STONES, stone_count);
    VarModule::set_int(weapon.battle_object, vars::plizardon_rock::status::REMAINING_STONES, stone_count);
    VarModule::set_int(weapon.battle_object, vars::plizardon_rock::status::SPAWN_COOLDOWN, 0);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("haved"), 0.0, 1.0, false, 0.0, false, false);
    
    //HAVE constraint. Tie the Rock's "have" bone to Zard's "throw" bone
    //Pretty sure most things until set model constraint arent necessary...
    let owner = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    LinkModule::remove_model_constraint(weapon.module_accessor, true);
    if LinkModule::is_link(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT) {
        LinkModule::unlink_all(weapon.module_accessor);
    }
    if LinkModule::is_link(weapon.module_accessor, *ITEM_LINK_NO_HAVE) == false {
        LinkModule::link(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT, owner);
        LinkModule::set_model_constraint_pos_ort(weapon.module_accessor, *LINK_NO_CONSTRAINT, Hash40::new("have"), Hash40::new("throw"), (*CONSTRAINT_FLAG_ORIENTATION | *CONSTRAINT_FLAG_POSITION) as u32, true);
    }

    weapon.fastshift(L2CValue::Ptr(start_main_loop as *const () as _)).into()
}

unsafe extern "C" fn start_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if VarModule::is_flag(weapon.battle_object, vars::plizardon_rock::status::ENABLE_BREAK) {
        VisibilityModule::set_whole(weapon.module_accessor, false);

        //spawn stones
        VarModule::dec_int(weapon.battle_object, vars::plizardon_rock::status::SPAWN_COOLDOWN);
        if VarModule::get_int(weapon.battle_object, vars::plizardon_rock::status::SPAWN_COOLDOWN) <= 0 {
            VarModule::set_int(weapon.battle_object, vars::plizardon_rock::status::SPAWN_COOLDOWN, 1);
            let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
            let owner = util::get_battle_object_from_id(owner_id);
            let mut owner_boma = &mut *(*owner).module_accessor;
            ArticleModule::generate_article(owner_boma, articles::plizardon::ROCKSTONE, false, -1) as u32;
            WorkModule::inc_int(owner_boma, *FIGHTER_PLIZARDON_STATUS_BREATH_WORK_INT_GENERATE_COUNT);
            if VarModule::countdown_int(weapon.battle_object, vars::plizardon_rock::status::REMAINING_STONES, 0) {
                notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
            }
        }
    }

    return 0.into();
}

pub unsafe extern "C" fn start_exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
    delete_if_orphaned(weapon);
    return 0.into();
}
pub unsafe extern "C" fn start_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    return 0.into();
}

unsafe extern "C" fn delete_if_orphaned(weapon: &mut L2CWeaponCommon) {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let mut should_delete = false;
    if !sv_battle_object::is_active(owner_id) {
        should_delete = true;
    }
    else {
        let owner = util::get_battle_object_from_id(owner_id);
        let mut owner_boma = &mut *(*owner).module_accessor;
        let status = StatusModule::status_kind(owner_boma);
        if [*FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_REBIRTH].contains(&status) {
            should_delete = true;
        }
    }
    if should_delete {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, statuses::plizardon_rock::HAVED, start_pre);
    agent.status(Main, statuses::plizardon_rock::HAVED, start_main);
    agent.status(Exec, statuses::plizardon_rock::HAVED, start_exec);
    agent.status(End, statuses::plizardon_rock::HAVED, start_end);
}