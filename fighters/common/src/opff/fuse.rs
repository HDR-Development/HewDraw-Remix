use super::*;
use globals::*;
use smash::app::utility;

#[repr(C)]
pub struct CreateItemParam {
    founder_pos: Vector4f,
    item_pos: Vector4f,
    item_kind: ItemKind,
    another_battle_object_id: u32,
    variation_kind: i32,
    lr_dir: f32,
    owner_id: u32,
    unk_20: u32,
    pokeball_or_assist_kind: i32,
    unk_3c: f32,
    unk_40: u32,
    unk_44: u32,
    unk_48: f32,
    unk_4c: u32
}

#[skyline::from_offset(0x15db0b0)]
unsafe fn create_item(item_manager: *const smash_rs::app::ItemManager, create_item_param: *mut CreateItemParam, unk: bool, unk2: bool, unk3: bool) -> *mut BattleObject;

#[no_mangle]
pub unsafe extern "Rust" fn item_owner_id_ptr(item_id: u32) -> *mut u32 {
    let item = smash_rs::app::ItemManager::instance().unwrap().find_active_item_from_id(item_id);
    if item.is_null() {
        return std::ptr::null_mut();
    }
    (item as *mut u8).add(0x250) as *mut u32
}

// gives a released fused bomb back to whoever owned it before it was fused
#[no_mangle]
pub unsafe extern "Rust" fn restore_fused_item_owner(arrow_object: *mut BattleObject, item_boma: *mut BattleObjectModuleAccessor) {
    if utility::get_kind(&mut *item_boma) != *ITEM_KIND_LINKBOMB {
        return;
    }
    let owner_ptr = item_owner_id_ptr((*item_boma).battle_object_id);
    if !owner_ptr.is_null() {
        *owner_ptr = VarModule::get_int(arrow_object, vars::link_bowarrow::instance::FUSED_ITEM_OWNER_ID) as u32;
    }
}

// returns the boma, object and fighter kind of the arrow's owner
#[no_mangle]
pub unsafe extern "Rust" fn fuse_get_owner(weapon_boma: *mut BattleObjectModuleAccessor) -> (*mut BattleObjectModuleAccessor, *mut BattleObject, i32) {
    let owner_id = WorkModule::get_int(weapon_boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let owner_boma = sv_battle_object::module_accessor(owner_id);
    let owner_object = get_battle_object_from_id(owner_id);
    let owner_kind = utility::get_kind(&mut *owner_boma);
    (owner_boma, owner_object, owner_kind)
}

// returns the boma and object of the given id if it is an active Link arrow
#[no_mangle]
pub unsafe extern "Rust" fn get_arrow(object_id: u32) -> Option<(*mut BattleObjectModuleAccessor, *mut BattleObject)> {
    if object_id == *BATTLE_OBJECT_ID_INVALID as u32 || !sv_battle_object::is_active(object_id) {
        return None;
    }
    let boma = sv_battle_object::module_accessor(object_id);
    if utility::get_category(&mut *boma) != *BATTLE_OBJECT_CATEGORY_WEAPON
    || utility::get_kind(&mut *boma) != *WEAPON_KIND_LINK_BOWARROW {
        return None;
    }
    Some((boma, get_battle_object_from_id(object_id)))
}

// attaches an item to the arrow's "top" bone, returning whether it was newly attached
#[no_mangle]
pub unsafe extern "Rust" fn fuse_attach_item(item_boma: *mut BattleObjectModuleAccessor, arrow_id: u32) -> bool {
    if !LinkModule::is_link(item_boma, *ITEM_LINK_NO_HAVE) {
        VisibilityModule::set_whole(item_boma, true);
        LinkModule::link(item_boma, *ITEM_LINK_NO_HAVE, arrow_id);
        LinkModule::set_model_constraint_pos_ort(item_boma, *ITEM_LINK_NO_HAVE, Hash40::new("top"), Hash40::new("top"), (*CONSTRAINT_FLAG_ORIENTATION | *CONSTRAINT_FLAG_POSITION) as u32, true);
        return true;
    }
    false
}

// unlinks an item from whatever it is attached to, returning whether it was attached
#[no_mangle]
pub unsafe extern "Rust" fn fuse_detach_item(item_boma: *mut BattleObjectModuleAccessor) -> bool {
    LinkModule::remove_model_constraint(item_boma, true);
    if LinkModule::is_link(item_boma, *ITEM_LINK_NO_HAVE) {
        LinkModule::unlink_all(item_boma);
        return true;
    }
    false
}

// deletes the item fused to an arrow, if any
#[no_mangle]
pub unsafe extern "Rust" fn fuse_remove_item(arrow_object: *mut BattleObject) {
    if !VarModule::is_flag(arrow_object, vars::link_bowarrow::instance::IS_ARROW_FUSE) {
        return;
    }
    let item_id = VarModule::get_int(arrow_object, vars::link_bowarrow::instance::FUSED_ITEM_ID) as u32;
    if sv_battle_object::is_active(item_id) {
        smash_rs::app::ItemManager::instance().unwrap().remove_item_from_id(item_id);
    }
    VarModule::off_flag(arrow_object, vars::link_bowarrow::instance::IS_ARROW_FUSE);
}

// stores the fused item's info on the arrow/owner
// pass ITEM_KIND_NONE / BATTLE_OBJECT_ID_INVALID to use the item the owner is currently holding.
#[no_mangle]
pub unsafe extern "Rust" fn fuse_set_arrow_params(arrow_boma: *mut BattleObjectModuleAccessor, item: i32, id: u32) {
    let (owner_boma, owner_object, owner_kind) = fuse_get_owner(arrow_boma);
    let arrow_object = get_battle_object_from_id((*arrow_boma).battle_object_id);

    let kind = if item == *ITEM_KIND_NONE {
        ItemModule::get_have_item_kind(owner_boma, 0)
    }
    else {
        item
    };
    let item_id = if id == *BATTLE_OBJECT_ID_INVALID as u32 {
        ItemModule::get_have_item_id(owner_boma, 0) as u32
    }
    else {
        id
    };
    VarModule::set_int(owner_object, vars::common::instance::FUSE_TEAM_NO, TeamModule::team_no(owner_boma) as i32);

    // 0: item travels with the arrow normally
    // 1: arrow is fired straight and fast
    let (item_type, post_status) = if kind == *ITEM_KIND_BOMBER {
        (0, *ITEM_BOMBER_STATUS_KIND_BORN2)
    }
    else if [*ITEM_KIND_KILLER, *ITEM_KIND_BANANAGUN, *ITEM_KIND_DOLPHINBOMB].contains(&kind) {
        (1, *ITEM_STATUS_KIND_THROW)
    }
    else if kind == *ITEM_KIND_LINKBOMB {
        (0, *ITEM_STATUS_KIND_BORN)
    }
    else {
        (0, *ITEM_STATUS_KIND_THROW)
    };
    VarModule::set_int(arrow_object, vars::link_bowarrow::instance::FUSED_ITEM_KIND, kind);
    VarModule::set_int(arrow_object, vars::link_bowarrow::instance::FUSED_ITEM_TYPE, item_type);
    VarModule::set_int(arrow_object, vars::link_bowarrow::instance::FUSED_ITEM_POST_STATUS, post_status);
    VarModule::set_int(arrow_object, vars::link_bowarrow::instance::FUSED_ITEM_ID, item_id as i32);
    let owner_ptr = item_owner_id_ptr(item_id);
    let item_owner_id = if owner_ptr.is_null() { *BATTLE_OBJECT_ID_INVALID as u32 } else { *owner_ptr };
    VarModule::set_int(arrow_object, vars::link_bowarrow::instance::FUSED_ITEM_OWNER_ID, item_owner_id as i32);
}

// spawns a new item of the given kind and fuses it to the given arrow
#[no_mangle]
pub unsafe extern "Rust" fn fuse_create_item(arrow_boma: *mut BattleObjectModuleAccessor, item: i32, founder_boma: *mut BattleObjectModuleAccessor, owner_id: u32) {
    let pos = Vector4f{x: PostureModule::pos_x(founder_boma), y: PostureModule::pos_y(founder_boma), z: PostureModule::pos_z(founder_boma), w: 0.0};
    let mut params = CreateItemParam {
        founder_pos: pos,
        item_pos: pos,
        item_kind: ItemKind(item),
        another_battle_object_id: *BATTLE_OBJECT_ID_INVALID as u32,
        variation_kind: *ITEM_VARIATION_NONE,
        lr_dir: PostureModule::lr(founder_boma),
        owner_id: owner_id,
        unk_20: 20,
        pokeball_or_assist_kind: *ITEM_KIND_NONE,
        unk_3c: 0.0,
        unk_40: 0,
        unk_44: 0,
        unk_48: 1.0,
        unk_4c: 0x63
    };
    let item_manager = smash_rs::app::ItemManager::instance().unwrap();
    let battle_object = create_item(item_manager, &mut params, false, false, false);
    if battle_object.is_null() {
        return;
    }
    let item_boma = (*battle_object).module_accessor;
    if ![*ITEM_KIND_HEALBALL, *ITEM_KIND_CHEWING, *ITEM_KIND_BOOMERANG].contains(&item) {
        StatusModule::change_status_request(item_boma, *ITEM_STATUS_KIND_HAVE, false);
    }
    if item == *ITEM_KIND_LINKBOMB {
        PostureModule::set_scale(item_boma, 1.3, false);
    }
    LinkModule::remove_model_constraint(item_boma, true);
    fuse_attach_item(item_boma, (*arrow_boma).battle_object_id);
    let arrow_object = get_battle_object_from_id((*arrow_boma).battle_object_id);
    VarModule::on_flag(arrow_object, vars::link_bowarrow::instance::IS_ARROW_FUSE);
    fuse_set_arrow_params(arrow_boma, item, (*item_boma).battle_object_id);
}

// ddd (and kirby with ddd's hat) re-creates the fused item when spitting the arrow back out
#[no_mangle]
pub unsafe extern "Rust" fn fuse_dedede_common(fighter: &mut L2CFighterCommon) {
    let status = if fighter.kind() == *FIGHTER_KIND_KIRBY {
        *FIGHTER_KIRBY_STATUS_KIND_DEDEDE_SPECIAL_N_SHOT_OBJECT_HIT
    }
    else {
        *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_SHOT_OBJECT_HIT
    };

    if !fighter.is_status(status) {
        VarModule::off_flag(fighter.battle_object, vars::common::instance::FUSE_IS_STORED_ITEM);
        VarModule::off_flag(fighter.battle_object, vars::common::instance::FUSE_IS_STORED_ITEM_CHECKED);
        return;
    }
    let obj_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_SPECIAL_N_WORK_INT_SHOT_OBJECT_ID) as u32;
    let (obj_boma, obj_object) = match get_arrow(obj_id) {
        Some(arrow) => arrow,
        None => return
    };

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_SPECIAL_N_FLAG_SHOT_OBJECT_SHOOT)
    && !VarModule::is_flag(fighter.battle_object, vars::common::instance::FUSE_IS_STORED_ITEM_CHECKED) {
        VarModule::on_flag(fighter.battle_object, vars::common::instance::FUSE_IS_STORED_ITEM_CHECKED);
        if VarModule::is_flag(obj_object, vars::link_bowarrow::instance::IS_ARROW_FUSE) {
            // skip if the arrow still carries its item
            let fused_item_id = VarModule::get_int(obj_object, vars::link_bowarrow::instance::FUSED_ITEM_ID) as u32;
            let is_fused_item_attached = sv_battle_object::is_active(fused_item_id)
                && LinkModule::is_link(sv_battle_object::module_accessor(fused_item_id), *ITEM_LINK_NO_HAVE);
            if !is_fused_item_attached {
                let fused_item = VarModule::get_int(obj_object, vars::link_bowarrow::instance::FUSED_ITEM_KIND);
                VarModule::set_int(fighter.battle_object, vars::common::instance::FUSE_STORED_ITEM_KIND, fused_item);
                VarModule::on_flag(fighter.battle_object, vars::common::instance::FUSE_IS_STORED_ITEM);
            }
        }
    }

    if VarModule::is_flag(fighter.battle_object, vars::common::instance::FUSE_IS_STORED_ITEM)
    && MotionModule::frame(fighter.module_accessor) >= 7.0 {
        let item = VarModule::get_int(fighter.battle_object, vars::common::instance::FUSE_STORED_ITEM_KIND);
        fuse_create_item(obj_boma, item, fighter.module_accessor, (*fighter.module_accessor).battle_object_id);
        VarModule::set_int(fighter.battle_object, vars::common::instance::FUSE_STORED_ITEM_KIND, *ITEM_KIND_NONE);
        VarModule::off_flag(fighter.battle_object, vars::common::instance::FUSE_IS_STORED_ITEM);
    }
}

// villager/isabelle (and kirby with their hat) store the fused item when pocketing the arrow, and delete the original
// the item is re-created when the arrow is pulled back out
#[no_mangle]
pub unsafe extern "Rust" fn fuse_pocket_common(fighter: &mut L2CFighterCommon) {
    if ![*FIGHTER_KIND_MURABITO, *FIGHTER_KIND_SHIZUE, *FIGHTER_KIND_KIRBY].contains(&fighter.kind()) {
        return;
    }

    let search_statuses = if fighter.kind() == *FIGHTER_KIND_KIRBY {
        let copy_chara = WorkModule::get_int(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA);
        if ![*FIGHTER_KIND_MURABITO, *FIGHTER_KIND_SHIZUE].contains(&copy_chara) {
            VarModule::set_int(fighter.battle_object, vars::common::instance::FUSE_STORED_ITEM_KIND, *ITEM_KIND_NONE);
        }
        [*FIGHTER_KIRBY_STATUS_KIND_MURABITO_SPECIAL_N_SEARCH, *FIGHTER_KIRBY_STATUS_KIND_SHIZUE_SPECIAL_N_SEARCH]
    }
    else {
        [*FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_SEARCH, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_SEARCH]
    };

    // the pocket is lost on death
    if fighter.is_status(*FIGHTER_STATUS_KIND_DEAD) {
        VarModule::set_int(fighter.battle_object, vars::common::instance::FUSE_STORED_ITEM_KIND, *ITEM_KIND_NONE);
    }
    if !fighter.is_status_one_of(&search_statuses) {
        VarModule::set_int(fighter.battle_object, vars::common::instance::FUSE_STORED_TARGET_ID, *BATTLE_OBJECT_ID_INVALID);
        return;
    }

    // kirby's copy abilities use the copied fighter's instance work ids
    let obj_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TARGET_OBJECT_ID) as u32;
    if obj_id == *BATTLE_OBJECT_ID_INVALID as u32
    || obj_id == VarModule::get_int(fighter.battle_object, vars::common::instance::FUSE_STORED_TARGET_ID) as u32 {
        return;
    }

    // handle each pocketed object once, replacing whatever fused item was stored before
    VarModule::set_int(fighter.battle_object, vars::common::instance::FUSE_STORED_TARGET_ID, obj_id as i32);
    VarModule::set_int(fighter.battle_object, vars::common::instance::FUSE_STORED_ITEM_KIND, *ITEM_KIND_NONE);
    let obj_object = match get_arrow(obj_id) {
        Some((_, obj_object)) => obj_object,
        None => return
    };

    VarModule::set_int(fighter.battle_object, vars::common::instance::FUSE_TEAM_NO, TeamModule::team_no(fighter.module_accessor) as i32);
    if VarModule::is_flag(obj_object, vars::link_bowarrow::instance::IS_ARROW_FUSE) {
        let fused_item = VarModule::get_int(obj_object, vars::link_bowarrow::instance::FUSED_ITEM_KIND);
        VarModule::set_int(fighter.battle_object, vars::common::instance::FUSE_STORED_ITEM_KIND, fused_item);
        fuse_remove_item(obj_object);
    }
}

// kirby's inhale deletes the fused item
#[no_mangle]
pub unsafe extern "Rust" fn fuse_kirby_common(fighter: &mut L2CFighterCommon) {
    if !fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_LOOP) {
        return;
    }
    let obj_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_SPECIAL_N_WORK_INT_INHALE_OBJECT_ID) as u32;
    if let Some((_, obj_object)) = get_arrow(obj_id) {
        fuse_remove_item(obj_object);
    }
}

// rosalina's gravitational pull deletes the fused item
#[no_mangle]
pub unsafe extern "Rust" fn fuse_rosetta_common(fighter: &mut L2CFighterCommon) {
    if !fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_LW) {
        return;
    }
    let obj_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_ROSETTA_STATUS_SPECIAL_LW_INT_CAPTURE_OBJECT_ID) as u32;
    if let Some((_, obj_object)) = get_arrow(obj_id) {
        fuse_remove_item(obj_object);
    }
}

// drops the fused item if Link's (or Kirby's) NSpecial ends before the arrow is fired
#[no_mangle]
pub unsafe extern "Rust" fn fuse_special_n_exit(fighter: &mut L2CFighterCommon) {
    if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW) {
        return;
    }

    let arrow_article = ArticleModule::get_article(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW);
    if arrow_article.is_null() {
        return;
    }

    // exit if the arrow is not active
    let arrow_id = smash::app::lua_bind::Article::get_battle_object_id(arrow_article) as u32;
    if !sv_battle_object::is_active(arrow_id) {
        return;
    }

    // exit if the arrow is not fused
    let arrow_object = get_battle_object_from_id(arrow_id);
    if !VarModule::is_flag(arrow_object, vars::link_bowarrow::instance::IS_ARROW_FUSE) {
        return;
    }
    VarModule::off_flag(arrow_object, vars::link_bowarrow::instance::IS_ARROW_FUSE);

    // exit if fused item is not active
    let fused_item_id = VarModule::get_int(arrow_object, vars::link_bowarrow::instance::FUSED_ITEM_ID) as u32;
    if !sv_battle_object::is_active(fused_item_id) {
        return;
    }

    // detach the item from the arrow and make it fall
    let fused_item_boma = &mut (*sv_battle_object::module_accessor(fused_item_id));
    let fused_item_kind = VarModule::get_int(arrow_object, vars::link_bowarrow::instance::FUSED_ITEM_KIND);
    let item_next_status = if fused_item_kind == *ITEM_KIND_LINKBOMB {
        (*ITEM_STATUS_KIND_BORN) // links bomb blows up in his face when dropped
    }
    else {
        (*ITEM_STATUS_KIND_FALL)
    };
    fuse_detach_item(fused_item_boma);
    restore_fused_item_owner(arrow_object, fused_item_boma);
    StatusModule::change_status_request(fused_item_boma, item_next_status, false);
}
