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

extern "C" {
    #[link_name = "_ZN3app10pacmanboss11PACMAN_LIFEEv"]
    fn pacmanboss_life() -> i32;
}

// restores item state that is normally set up when a fighter throws the item
#[no_mangle]
pub unsafe extern "Rust" fn fuse_on_item_released(item_boma: *mut BattleObjectModuleAccessor) {
    if utility::get_kind(&mut *item_boma) == *ITEM_KIND_PACMANBOSS {
        WorkModule::on_flag(item_boma, *ITEM_PACMAN_INSTANCE_WORK_FLAG_THROW);
        WorkModule::off_flag(item_boma, *ITEM_PACMAN_INSTANCE_WORK_FLAG_DROP);
        WorkModule::off_flag(item_boma, *ITEM_PACMAN_INSTANCE_WORK_FLAG_HIT);
        WorkModule::off_flag(item_boma, *ITEM_PACMAN_INSTANCE_WORK_FLAG_HOP);
        WorkModule::set_int(item_boma, 0, *ITEM_PACMAN_INSTANCE_WORK_INT_BOUND_COUNT);
        WorkModule::set_int(item_boma, pacmanboss_life(), *ITEM_PACMAN_INSTANCE_WORK_INT_LIFE);
        WorkModule::set_int(item_boma, *PACMANBOSS_STATE_FIRST, *ITEM_PACMANBOSS_INSTANCE_WORK_INT_STATE);
        WorkModule::set_int(item_boma, 0, *ITEM_PACMANBOSS_INSTANCE_WORK_INT_THROW_FRAME);
        WorkModule::set_int(item_boma, 0, *ITEM_PACMANBOSS_INSTANCE_WORK_INT_ROT_FRAME);
        WorkModule::set_float(item_boma, 0.0, *ITEM_PACMANBOSS_INSTANCE_WORK_FLOAT_ROT_ANGLE);
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

unsafe fn owner_team_no_var(owner_kind: i32) -> i32 {
    if owner_kind == *FIGHTER_KIND_MURABITO {
        vars::murabito::instance::TEAM_NO
    }
    else if owner_kind == *FIGHTER_KIND_SHIZUE {
        vars::shizue::instance::TEAM_NO
    }
    else if owner_kind == *FIGHTER_KIND_KIRBY {
        vars::kirby::instance::TEAM_NO
    }
    else {
        vars::link::instance::TEAM_NO
    }
}

// returns the (stored item kind, pocket target id, team no) vars a fighter uses to pocket a fused arrow
unsafe fn pocket_vars(fighter_kind: i32) -> Option<(i32, i32, i32)> {
    if fighter_kind == *FIGHTER_KIND_MURABITO {
        Some((vars::murabito::instance::FUSED_ITEM_KIND, vars::murabito::instance::FUSE_POCKET_TARGET_ID, vars::murabito::instance::TEAM_NO))
    }
    else if fighter_kind == *FIGHTER_KIND_SHIZUE {
        Some((vars::shizue::instance::FUSED_ITEM_KIND, vars::shizue::instance::FUSE_POCKET_TARGET_ID, vars::shizue::instance::TEAM_NO))
    }
    else if fighter_kind == *FIGHTER_KIND_KIRBY {
        Some((vars::kirby::instance::FUSED_ITEM_KIND, vars::kirby::instance::FUSE_POCKET_TARGET_ID, vars::kirby::instance::TEAM_NO))
    }
    else {
        None
    }
}

// returns the boma and object of the given id if it is an active Link arrow
unsafe fn get_arrow(object_id: u32) -> Option<(*mut BattleObjectModuleAccessor, *mut BattleObject)> {
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

// returns the team the arrow's owner had when the arrow was fused
#[no_mangle]
pub unsafe extern "Rust" fn fuse_get_owner_team_no(weapon_boma: *mut BattleObjectModuleAccessor) -> i32 {
    let (_, owner_object, owner_kind) = fuse_get_owner(weapon_boma);
    VarModule::get_int(owner_object, owner_team_no_var(owner_kind))
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

    VarModule::set_int(owner_object, owner_team_no_var(owner_kind), TeamModule::team_no(owner_boma) as i32);
    if owner_kind == *FIGHTER_KIND_LINK {
        VarModule::set_int(owner_object, vars::link::instance::FUSED_ITEM_ID, item_id as i32);
    }
    else if owner_kind == *FIGHTER_KIND_KIRBY {
        VarModule::set_int(owner_object, vars::kirby::instance::FUSED_ITEM_ID, item_id as i32);
    }

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
    let (status, fuse_item_flag, checked_flag, fused_item_kind_var) = if fighter.kind() == *FIGHTER_KIND_KIRBY {
        (*FIGHTER_KIRBY_STATUS_KIND_DEDEDE_SPECIAL_N_SHOT_OBJECT_HIT, vars::kirby::instance::FUSE_DEDEDE_ITEM, vars::kirby::instance::FUSE_DEDEDE_ITEM_CHECKED, vars::kirby::instance::FUSE_DEDEDE_ITEM_KIND)
    }
    else {
        (*FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_SHOT_OBJECT_HIT, vars::dedede::instance::FUSE_ITEM, vars::dedede::instance::FUSE_ITEM_CHECKED, vars::dedede::instance::FUSED_ITEM_KIND)
    };
    if !fighter.is_status(status) {
        VarModule::off_flag(fighter.battle_object, fuse_item_flag);
        VarModule::off_flag(fighter.battle_object, checked_flag);
        return;
    }
    let obj_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_SPECIAL_N_WORK_INT_SHOT_OBJECT_ID) as u32;
    let (obj_boma, obj_object) = match get_arrow(obj_id) {
        Some(arrow) => arrow,
        None => return
    };

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_SPECIAL_N_FLAG_SHOT_OBJECT_SHOOT)
    && !VarModule::is_flag(fighter.battle_object, checked_flag) {
        VarModule::on_flag(fighter.battle_object, checked_flag);
        if VarModule::is_flag(obj_object, vars::link_bowarrow::instance::IS_ARROW_FUSE) {
            // skip if the arrow still carries its item
            let fused_item_id = VarModule::get_int(obj_object, vars::link_bowarrow::instance::FUSED_ITEM_ID) as u32;
            let is_fused_item_attached = sv_battle_object::is_active(fused_item_id)
                && LinkModule::is_link(sv_battle_object::module_accessor(fused_item_id), *ITEM_LINK_NO_HAVE);
            if !is_fused_item_attached {
                let fused_item = VarModule::get_int(obj_object, vars::link_bowarrow::instance::FUSED_ITEM_KIND);
                VarModule::set_int(fighter.battle_object, fused_item_kind_var, fused_item);
                VarModule::on_flag(fighter.battle_object, fuse_item_flag);
            }
        }
    }

    if VarModule::is_flag(fighter.battle_object, fuse_item_flag)
    && MotionModule::frame(fighter.module_accessor) >= 7.0 {
        let item = VarModule::get_int(fighter.battle_object, fused_item_kind_var);
        fuse_create_item(obj_boma, item, fighter.module_accessor, (*fighter.module_accessor).battle_object_id);
        VarModule::set_int(fighter.battle_object, fused_item_kind_var, *ITEM_KIND_NONE);
        VarModule::off_flag(fighter.battle_object, fuse_item_flag);
    }
}

// villager/isabelle (and kirby with their hat) store the fused item when pocketing the arrow, and delete the original
// the item is re-created when the arrow is pulled back out
#[no_mangle]
pub unsafe extern "Rust" fn fuse_pocket_common(fighter: &mut L2CFighterCommon) {
    let (fused_item_kind_var, target_id_var, team_no_var) = match pocket_vars(fighter.kind()) {
        Some(pocket_vars) => pocket_vars,
        None => return
    };
    let search_statuses = if fighter.kind() == *FIGHTER_KIND_KIRBY {
        let copy_chara = WorkModule::get_int(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA);
        if ![*FIGHTER_KIND_MURABITO, *FIGHTER_KIND_SHIZUE].contains(&copy_chara) {
            VarModule::set_int(fighter.battle_object, fused_item_kind_var, *ITEM_KIND_NONE);
        }
        [*FIGHTER_KIRBY_STATUS_KIND_MURABITO_SPECIAL_N_SEARCH, *FIGHTER_KIRBY_STATUS_KIND_SHIZUE_SPECIAL_N_SEARCH]
    }
    else {
        [*FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_SEARCH, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_SEARCH]
    };

    // the pocket is lost on death
    if fighter.is_status(*FIGHTER_STATUS_KIND_DEAD) {
        VarModule::set_int(fighter.battle_object, fused_item_kind_var, *ITEM_KIND_NONE);
    }
    if !fighter.is_status_one_of(&search_statuses) {
        VarModule::set_int(fighter.battle_object, target_id_var, *BATTLE_OBJECT_ID_INVALID);
        return;
    }

    // kirby's copy abilities use the copied fighter's instance work ids
    let obj_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TARGET_OBJECT_ID) as u32;
    if obj_id == *BATTLE_OBJECT_ID_INVALID as u32
    || obj_id == VarModule::get_int(fighter.battle_object, target_id_var) as u32 {
        return;
    }

    // handle each pocketed object once, replacing whatever fused item was stored before
    VarModule::set_int(fighter.battle_object, target_id_var, obj_id as i32);
    VarModule::set_int(fighter.battle_object, fused_item_kind_var, *ITEM_KIND_NONE);
    let obj_object = match get_arrow(obj_id) {
        Some((_, obj_object)) => obj_object,
        None => return
    };

    VarModule::set_int(fighter.battle_object, team_no_var, TeamModule::team_no(fighter.module_accessor) as i32);
    if VarModule::is_flag(obj_object, vars::link_bowarrow::instance::IS_ARROW_FUSE) {
        let fused_item = VarModule::get_int(obj_object, vars::link_bowarrow::instance::FUSED_ITEM_KIND);
        VarModule::set_int(fighter.battle_object, fused_item_kind_var, fused_item);
        fuse_remove_item(obj_object);
    }
}

// returns the fused item the arrow's owner has pocketed, if any
#[no_mangle]
pub unsafe extern "Rust" fn fuse_get_pocket_item(weapon_boma: *mut BattleObjectModuleAccessor) -> i32 {
    let (_, owner_object, owner_kind) = fuse_get_owner(weapon_boma);
    let item = match pocket_vars(owner_kind) {
        Some((fused_item_kind_var, _, _)) => VarModule::get_int(owner_object, fused_item_kind_var),
        None => *ITEM_KIND_NONE
    };
    // the var starts at 0 (ITEM_KIND_ASSIST), which can't be fused
    if item > *ITEM_KIND_ASSIST { item } else { *ITEM_KIND_NONE }
}

// re-creates the owner's pocketed fused item on the arrow being pulled back out
#[no_mangle]
pub unsafe extern "Rust" fn fuse_take_out_pocket_item(weapon_boma: *mut BattleObjectModuleAccessor) {
    let item = fuse_get_pocket_item(weapon_boma);
    if item == *ITEM_KIND_NONE {
        return;
    }
    let (owner_boma, owner_object, owner_kind) = fuse_get_owner(weapon_boma);
    fuse_create_item(weapon_boma, item, weapon_boma, (*owner_boma).battle_object_id);
    if let Some((fused_item_kind_var, _, _)) = pocket_vars(owner_kind) {
        VarModule::set_int(owner_object, fused_item_kind_var, *ITEM_KIND_NONE);
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
    let (is_arrow_fuse_var, fused_item_id_var) = if fighter.kind() == *FIGHTER_KIND_KIRBY {
        (vars::kirby::instance::IS_ARROW_FUSE, vars::kirby::instance::FUSED_ITEM_ID)
    }
    else {
        (vars::link::instance::IS_ARROW_FUSE, vars::link::instance::FUSED_ITEM_ID)
    };

    if VarModule::is_flag(fighter.battle_object, is_arrow_fuse_var)
    && ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW) {
        let article = ArticleModule::get_article(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW);
        if !article.is_null() {
            let arrow_id = smash::app::lua_bind::Article::get_battle_object_id(article) as u32;
            if sv_battle_object::is_active(arrow_id) {
                VarModule::off_flag(get_battle_object_from_id(arrow_id), vars::link_bowarrow::instance::IS_ARROW_FUSE);
            }
        }
        let item_id = VarModule::get_int(fighter.battle_object, fused_item_id_var) as u32;
        if sv_battle_object::is_active(item_id) {
            let item_boma = sv_battle_object::module_accessor(item_id);
            if fuse_detach_item(item_boma) {
                StatusModule::change_status_request(item_boma, *ITEM_STATUS_KIND_FALL, false);
            }
        }
    }
    VarModule::off_flag(fighter.battle_object, is_arrow_fuse_var);
}
