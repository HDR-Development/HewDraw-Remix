use super::*;
use globals::*;
use smash2::*;
// status script import


pub fn install() {
    install_status_scripts!(
        arrow_haved_main,
        special_n_exit,
        arrow_fly_end,
        arrow_fly_init,
        arrow_stick_end,
        arrow_hit_stick_end,
        arrow_haved_end
    );
    install_agent_frames!(
        link_bowarrow,
        dedede_fix,
        villager_fix,
        isabelle_fix,
        kirby_fix,
        rosalina_fix
    );
    skyline::install_hook!(create_item);
}

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
    unk_0: u64,
    weird_flag: u64,
    unk_1_weird: u64,
    unk_approx_0: f32,
    unk_02: f32
}

#[skyline::hook(offset = 0x15daea0)]
pub unsafe fn create_item(item_manager: &smash2::app::ItemManager, create_item_param: *mut CreateItemParam, unk: bool, unk2: bool, unk3: bool) -> *mut BattleObject {
    original!()(item_manager,create_item_param,unk,unk2,unk3)
}

unsafe fn set_arrow_fuse_params(module_accessor: *mut BattleObjectModuleAccessor, item: i32, id: u32) {
    let owner_id = WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let owner_module_accessor = smash::app::sv_battle_object::module_accessor(owner_id);
    let kind = ItemModule::get_have_item_kind(owner_module_accessor,0);
    let link_object = utils::util::get_battle_object_from_id(owner_id);
    let arrow_id = (*(module_accessor)).battle_object_id;
    let arrow_object = utils::util::get_battle_object_from_id(arrow_id);
    let kind = if item == *ITEM_KIND_NONE {
        ItemModule::get_have_item_kind(owner_module_accessor,0)
    }
    else {
        item
    };
    if utility::get_kind(&mut *owner_module_accessor) == *FIGHTER_KIND_LINK {
        VarModule::set_int(link_object,vars::link::instance::TEAM_NO,TeamModule::team_no(owner_module_accessor) as i32);
        VarModule::set_int(link_object,vars::link::instance::FUSED_ITEM_KIND,kind);
        VarModule::on_flag(link_object,vars::link::instance::IS_ARROW_FUSE);
    }
    else {
        VarModule::set_int(link_object,vars::kirby::instance::TEAM_NO,TeamModule::team_no(owner_module_accessor) as i32);
        VarModule::set_int(link_object,vars::kirby::instance::FUSED_ITEM_KIND,kind);
        VarModule::on_flag(link_object,vars::kirby::instance::IS_ARROW_FUSE);
    }
    VarModule::set_int(arrow_object,vars::link_arrow::instance::FUSED_ITEM_KIND,kind);
    if kind == *ITEM_KIND_BOMBER {
        VarModule::set_int(arrow_object,vars::link_arrow::instance::FUSED_ITEM_TYPE,0);
        VarModule::set_int(arrow_object,vars::link_arrow::instance::FUSED_ITEM_POST_STATUS,*ITEM_BOMBER_STATUS_KIND_BORN2);
    }
    else if kind == *ITEM_KIND_KILLER
    || kind == *ITEM_KIND_BANANAGUN
    || kind == *ITEM_KIND_DOLPHINBOMB {
        VarModule::set_int(arrow_object,vars::link_arrow::instance::FUSED_ITEM_TYPE,1);
        VarModule::set_int(arrow_object,vars::link_arrow::instance::FUSED_ITEM_POST_STATUS,*ITEM_STATUS_KIND_THROW);
    }
    else if kind == *ITEM_KIND_LINKBOMB {
        VarModule::set_int(arrow_object,vars::link_arrow::instance::FUSED_ITEM_TYPE,0);
        VarModule::set_int(arrow_object,vars::link_arrow::instance::FUSED_ITEM_POST_STATUS,*ITEM_STATUS_KIND_BORN);
    }
    else {
        VarModule::set_int(arrow_object,vars::link_arrow::instance::FUSED_ITEM_TYPE,0);
        VarModule::set_int(arrow_object,vars::link_arrow::instance::FUSED_ITEM_POST_STATUS,*ITEM_STATUS_KIND_THROW);
    }
    let item_id = if id == *BATTLE_OBJECT_ID_INVALID as u32 {
        ItemModule::get_have_item_id(owner_module_accessor,0) as u32
    }
    else {
        id
    };
    if utility::get_kind(&mut *owner_module_accessor) == *FIGHTER_KIND_LINK {
        VarModule::set_int(link_object,vars::link::instance::FUSED_ITEM_ID,id as i32);
    }
    else {
        VarModule::set_int(link_object,vars::kirby::instance::FUSED_ITEM_ID,id as i32);
    }
    VarModule::set_int(arrow_object,vars::link_arrow::instance::FUSED_ITEM_ID,item_id as i32);
}

#[status_script(agent = "link_bowarrow", status = WN_LINK_BOWARROW_STATUS_KIND_HAVED, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn arrow_haved_main(weapon: &mut L2CFighterBase) -> L2CValue {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let owner_module_accessor = smash::app::sv_battle_object::module_accessor(owner_id);
    let link_object = utils::util::get_battle_object_from_id(owner_id);
    if utility::get_kind(&mut *owner_module_accessor) == *FIGHTER_KIND_LINK {
        VarModule::set_int(link_object,vars::link::instance::FUSED_ITEM_KIND,*ITEM_KIND_NONE);
    }
    else {
        VarModule::set_int(link_object,vars::kirby::instance::FUSED_ITEM_KIND,*ITEM_KIND_NONE);
    }
    if ItemModule::is_have_item(owner_module_accessor,0) {
        set_arrow_fuse_params(weapon.module_accessor,*ITEM_KIND_NONE,*BATTLE_OBJECT_ID_INVALID as u32);
        let trait_flag = ItemModule::get_have_item_trait(owner_module_accessor,0);
        let kind = VarModule::get_int(weapon.battle_object,vars::link_arrow::instance::FUSED_ITEM_KIND);
        if kind != *ITEM_KIND_NONE
        && kind != *ITEM_KIND_ASSIST
        && kind != *ITEM_KIND_LINKARROW
        && trait_flag != *ITEM_TRAIT_FLAG_SWING as u64
        && trait_flag != *ITEM_TRAIT_FLAG_SHOOT as u64
        && trait_flag != *ITEM_TRAIT_FLAG_NONE as u64 {
            VarModule::on_flag(weapon.battle_object,vars::link_arrow::instance::IS_ARROW_FUSE);
        }
        else {
            VarModule::off_flag(weapon.battle_object,vars::link_arrow::instance::IS_ARROW_FUSE);
        }
    }
    else {
        VarModule::off_flag(weapon.battle_object,vars::link_arrow::instance::IS_ARROW_FUSE);
    }
    let is_fused = VarModule::is_flag(weapon.battle_object,vars::link_arrow::instance::IS_ARROW_FUSE);
    if is_fused {
        let item_id = VarModule::get_int(weapon.battle_object,vars::link_arrow::instance::FUSED_ITEM_ID) as u32;
        let item_boma = smash::app::sv_battle_object::module_accessor(item_id);
        LinkModule::remove_model_constraint(item_boma,true);
        if LinkModule::is_link(item_boma,*ITEM_LINK_NO_HAVE) {
            LinkModule::unlink_all(item_boma);
        }
        if LinkModule::is_link(item_boma,*ITEM_LINK_NO_HAVE) == false {
            VisibilityModule::set_whole(item_boma,true);
            LinkModule::link(item_boma,*ITEM_LINK_NO_HAVE,(*(weapon.module_accessor)).battle_object_id);
            LinkModule::set_model_constraint_pos_ort(item_boma,*ITEM_LINK_NO_HAVE,Hash40::new("top"),Hash40::new("top"),*CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32,true);
            PLAY_SE(weapon,Hash40::new("se_link_fuse"));
        }
    }
    if is_fused {
        MotionModule::change_motion(weapon.module_accessor,Hash40::new("haved"),0.0,1.0,false,0.0,false,false);
    }
    else {
        MotionModule::change_motion(weapon.module_accessor,Hash40::new("haved_hide"),0.0,1.0,false,0.0,false,false);
    }
    weapon.fastshift(L2CValue::Ptr(arrow_haved_main_loop as *const () as _))
}

unsafe fn arrow_haved_main_loop(_weapon: &mut L2CFighterBase) -> L2CValue {
    return L2CValue::I32(0)
}

#[status_script(agent = "link", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS)]
pub unsafe fn special_n_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    let bow_id = WorkModule::get_int(fighter.module_accessor,*FIGHTER_LINK_INSTANCE_WORK_ID_INT_BOW_ARTICLE_ID);
    ArticleModule::change_status_exist(fighter.module_accessor,bow_id,*WN_LINK_BOW_STATUS_KIND_BACK);
    if ArticleModule::is_exist(fighter.module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW) {
        if VarModule::is_flag(fighter.battle_object,vars::link::instance::IS_ARROW_FUSE) {
            let item_id = VarModule::get_int(fighter.battle_object,vars::link::instance::FUSED_ITEM_ID) as u32;
            let item_boma = smash::app::sv_battle_object::module_accessor(item_id);
            LinkModule::remove_model_constraint(item_boma,true);
            if LinkModule::is_link(item_boma,*ITEM_LINK_NO_HAVE) {
                LinkModule::unlink_all(item_boma);
                StatusModule::change_status_request(item_boma,*ITEM_STATUS_KIND_FALL,false);
            }
        }
    }
    ArticleModule::remove_exist(fighter.module_accessor,*FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW,ArticleOperationTarget(0));
    return L2CValue::I32(0)
}

#[status_script(agent = "link_bowarrow", status = WN_LINK_BOWARROW_STATUS_KIND_FLY, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn arrow_fly_end(weapon: &mut L2CFighterBase) -> L2CValue {
    if StatusModule::status_kind_next(weapon.module_accessor) == *WN_LINK_BOWARROW_STATUS_KIND_STICK
    || StatusModule::status_kind_next(weapon.module_accessor) == *WN_LINK_BOWARROW_STATUS_KIND_HIT_STICK
    || AttackModule::is_infliction_status(weapon.module_accessor,*COLLISION_KIND_MASK_HIT) {
        if VarModule::is_flag(weapon.battle_object,vars::link_arrow::instance::IS_ARROW_FUSE) {
            let item_id = VarModule::get_int(weapon.battle_object,vars::link_arrow::instance::FUSED_ITEM_ID) as u32;
            let item_boma = smash::app::sv_battle_object::module_accessor(item_id);
            LinkModule::remove_model_constraint(item_boma,true);
            if LinkModule::is_link(item_boma,*ITEM_LINK_NO_HAVE) {
                LinkModule::unlink_all(item_boma);
                let status = VarModule::get_int(weapon.battle_object,vars::link_arrow::instance::FUSED_ITEM_POST_STATUS);
                StatusModule::change_status_request(item_boma,status,false);
            }
        }
    }
    else {
        if VarModule::is_flag(weapon.battle_object,vars::link_arrow::instance::IS_ARROW_FUSE) {
            let item_id = VarModule::get_int(weapon.battle_object,vars::link_arrow::instance::FUSED_ITEM_ID) as u32;
            let item_manager = smash2::app::ItemManager::instance().unwrap();
            smash2::app::ItemManager::remove_item_from_id(item_manager,item_id);
        }
    }
    EffectModule::detach_all(weapon.module_accessor,5);
    return L2CValue::I32(0)
}

#[status_script(agent = "link_bowarrow", status = WN_LINK_BOWARROW_STATUS_KIND_FLY, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
pub unsafe fn arrow_fly_init(weapon: &mut L2CFighterBase) -> L2CValue {
    original!(weapon);
    if VarModule::is_flag(weapon.battle_object,vars::link_arrow::instance::IS_ARROW_FUSE) {
        if VarModule::get_int(weapon.battle_object,vars::link_arrow::instance::FUSED_ITEM_TYPE) == 1 {
            let lr = PostureModule::lr(weapon.module_accessor);
            weapon.clear_lua_stack();
            lua_args!(weapon,*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,12.0*lr,0.0);
            sv_kinetic_energy::set_speed(weapon.lua_state_agent);
            weapon.clear_lua_stack();
            lua_args!(weapon,*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,0.0,0.0);
            sv_kinetic_energy::set_accel(weapon.lua_state_agent);
            KineticModule::enable_energy(weapon.module_accessor,*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
            AttackModule::set_power_mul(weapon.module_accessor,2.5);
        }
    }
    return L2CValue::I32(0)
}

#[status_script(agent = "link_bowarrow", status = WN_LINK_BOWARROW_STATUS_KIND_STICK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn arrow_stick_end(weapon: &mut L2CFighterBase) -> L2CValue {
    if VarModule::is_flag(weapon.battle_object,vars::link_arrow::instance::IS_REFLECT_FIX) {
        let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        let owner_module_accessor = smash::app::sv_battle_object::module_accessor(owner_id);
        let link_object = utils::util::get_battle_object_from_id(owner_id);
        let team_no = if utility::get_kind(&mut *owner_module_accessor) == *FIGHTER_KIND_MURABITO {
            VarModule::get_int(link_object,vars::murabito::instance::TEAM_NO)
        }
        else if utility::get_kind(&mut *owner_module_accessor) == *FIGHTER_KIND_SHIZUE {
            VarModule::get_int(link_object,vars::shizue::instance::TEAM_NO)
        }
        else if utility::get_kind(&mut *owner_module_accessor) == *FIGHTER_KIND_KIRBY {
            VarModule::get_int(link_object,vars::kirby::instance::TEAM_NO)
        }
        else {
            VarModule::get_int(link_object,vars::link::instance::TEAM_NO)
        };
        TeamModule::set_team(weapon.module_accessor,team_no,true);
        TeamModule::set_team_owner_id(weapon.module_accessor,(*(owner_module_accessor)).battle_object_id);
        VarModule::off_flag(weapon.battle_object,vars::link_arrow::instance::IS_REFLECT_FIX);
    }
    return L2CValue::I32(0)
}

#[status_script(agent = "link_bowarrow", status = WN_LINK_BOWARROW_STATUS_KIND_HIT_STICK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn arrow_hit_stick_end(weapon: &mut L2CFighterBase) -> L2CValue {
    arrow_stick_end(weapon)
}

#[smashline::weapon_frame(agent = WEAPON_KIND_LINK_BOWARROW, main)]
pub fn link_bowarrow(weapon: &mut L2CFighterBase) {
    unsafe {
        let owner_module_accessor = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        if AttackModule::is_infliction(weapon.module_accessor,*COLLISION_KIND_MASK_REFLECTOR)
        && StatusModule::status_kind(weapon.module_accessor) == *WN_LINK_BOWARROW_STATUS_KIND_FLY
        && VarModule::is_flag(weapon.battle_object,vars::link_arrow::instance::IS_ARROW_FUSE) {
            VarModule::on_flag(weapon.battle_object,vars::link_arrow::instance::IS_REFLECT_FIX);
            let item_id = VarModule::get_int(weapon.battle_object,vars::link_arrow::instance::FUSED_ITEM_ID) as u32;
            let item_boma = smash::app::sv_battle_object::module_accessor(item_id);
            let team_no = TeamModule::team_no(weapon.module_accessor) as i32;
            let team_owner_id = TeamModule::team_owner_id(weapon.module_accessor) as u32;
            TeamModule::set_team(item_boma,team_no,true);
            TeamModule::set_team_owner_id(item_boma,team_owner_id);
        }
    }
}

#[smashline::fighter_frame(agent = FIGHTER_KIND_DEDEDE, main)]
pub fn dedede_fix(fighter: &mut L2CFighterCommon) {
    unsafe {
        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_SHOT_OBJECT_HIT {
            let obj_id = WorkModule::get_int(fighter.module_accessor,*FIGHTER_DEDEDE_STATUS_SPECIAL_N_WORK_INT_SHOT_OBJECT_ID) as u32;
            let obj_boma = sv_battle_object::module_accessor(obj_id);
            let obj_object = utils::util::get_battle_object_from_id(obj_id);
            if utility::get_kind(&mut *obj_boma) == *WEAPON_KIND_LINK_BOWARROW {
                let owner_id = WorkModule::get_int(obj_boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
                let owner_module_accessor = smash::app::sv_battle_object::module_accessor(owner_id);
                let link_object = utils::util::get_battle_object_from_id(owner_id);
                let fused_item = if utility::get_kind(&mut *owner_module_accessor) == *FIGHTER_KIND_MURABITO {
                    VarModule::get_int(link_object,vars::murabito::instance::FUSED_ITEM_KIND)
                }
                else if utility::get_kind(&mut *owner_module_accessor) == *FIGHTER_KIND_SHIZUE {
                    VarModule::get_int(link_object,vars::shizue::instance::FUSED_ITEM_KIND)
                }
                else if utility::get_kind(&mut *owner_module_accessor) == *FIGHTER_KIND_KIRBY {
                    VarModule::get_int(link_object,vars::kirby::instance::FUSED_ITEM_KIND)
                }
                else {
                    VarModule::get_int(link_object,vars::link::instance::FUSED_ITEM_KIND)
                };
                VarModule::set_int(fighter.battle_object,vars::dedede::instance::FUSED_ITEM_KIND,fused_item);
            }
            if WorkModule::is_flag(fighter.module_accessor,*FIGHTER_DEDEDE_STATUS_SPECIAL_N_FLAG_SHOT_OBJECT_SHOOT)
            && VarModule::get_int(fighter.battle_object,vars::dedede::instance::FUSED_ITEM_KIND) > *ITEM_KIND_NONE {
                VarModule::on_flag(fighter.battle_object,vars::dedede::instance::FUSE_ITEM);
            }
            if VarModule::is_flag(fighter.battle_object,vars::dedede::instance::FUSE_ITEM)
            && MotionModule::frame(fighter.module_accessor) >= 7.0 {
                let item = VarModule::get_int(fighter.battle_object,vars::dedede::instance::FUSED_ITEM_KIND);
                let mut params = CreateItemParam {
                    founder_pos: Vector4f{x: PostureModule::pos_x(fighter.module_accessor), y: PostureModule::pos_y(fighter.module_accessor), z: PostureModule::pos_z(fighter.module_accessor), w: 0.0},
                    item_pos: Vector4f{x: PostureModule::pos_x(fighter.module_accessor), y: PostureModule::pos_y(fighter.module_accessor), z: PostureModule::pos_z(fighter.module_accessor), w: 0.0},
                    item_kind: ItemKind(item),
                    another_battle_object_id: *BATTLE_OBJECT_ID_INVALID as u32,
                    variation_kind: *ITEM_VARIATION_NONE,
                    lr_dir: PostureModule::lr(fighter.module_accessor),
                    owner_id: (*(fighter.module_accessor)).battle_object_id,
                    unk_20: 20,
                    pokeball_or_assist_kind: *ITEM_KIND_NONE,
                    unk_0: 0,
                    weird_flag: 0x633F800000,
                    unk_1_weird: 1,
                    unk_approx_0: 0.0,
                    unk_02: 0.0
                };
                let item_manager = smash2::app::ItemManager::instance().unwrap();
                let battle_object = create_item(item_manager,&mut params,false,false,false);
                let item_boma = (*battle_object).module_accessor;
                if item != *ITEM_KIND_HEALBALL
                && item != *ITEM_KIND_CHEWING
                && item != *ITEM_KIND_BOOMERANG {
                    StatusModule::change_status_request(item_boma,*ITEM_STATUS_KIND_HAVE,false);
                }
                if item == *ITEM_KIND_LINKBOMB {
                    PostureModule::set_scale(item_boma,1.3,false);
                }
                LinkModule::remove_model_constraint(item_boma,true);
                if LinkModule::is_link(item_boma,*ITEM_LINK_NO_HAVE) == false {
                    VisibilityModule::set_whole(item_boma,true);
                    LinkModule::link(item_boma,*ITEM_LINK_NO_HAVE,obj_id);
                    LinkModule::set_model_constraint_pos_ort(item_boma,*ITEM_LINK_NO_HAVE,Hash40::new("top"),Hash40::new("top"),*CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32,true);
                }
                VarModule::on_flag(obj_object,vars::link_arrow::instance::IS_ARROW_FUSE);
                set_arrow_fuse_params(obj_boma,item,(*(item_boma)).battle_object_id);
                VarModule::set_int(fighter.battle_object,vars::dedede::instance::FUSED_ITEM_KIND,*ITEM_KIND_NONE);
                VarModule::off_flag(fighter.battle_object,vars::dedede::instance::FUSE_ITEM);
            }
        }
    }
}

#[smashline::fighter_frame(agent = FIGHTER_KIND_MURABITO, main)]
pub fn villager_fix(fighter: &mut L2CFighterCommon) {
    unsafe {
        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_SEARCH {
            let obj_id = WorkModule::get_int(fighter.module_accessor,*FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TARGET_OBJECT_ID) as u32;
            let obj_boma = sv_battle_object::module_accessor(obj_id);
            let obj_object = utils::util::get_battle_object_from_id(obj_id);
            if utility::get_kind(&mut *obj_boma) == *WEAPON_KIND_LINK_BOWARROW {
                let owner_id = WorkModule::get_int(obj_boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
                let owner_module_accessor = smash::app::sv_battle_object::module_accessor(owner_id);
                let link_object = utils::util::get_battle_object_from_id(owner_id);
                let fused_item = if utility::get_kind(&mut *owner_module_accessor) == *FIGHTER_KIND_KIRBY {
                    VarModule::get_int(link_object,vars::kirby::instance::FUSED_ITEM_KIND)
                }
                else if utility::get_kind(&mut *owner_module_accessor) == *FIGHTER_KIND_SHIZUE {
                    VarModule::get_int(link_object,vars::shizue::instance::FUSED_ITEM_KIND)
                }
                else {
                    VarModule::get_int(link_object,vars::link::instance::FUSED_ITEM_KIND)
                };
                VarModule::set_int(fighter.battle_object,vars::murabito::instance::FUSED_ITEM_KIND,fused_item);
                VarModule::set_int(fighter.battle_object,vars::murabito::instance::TEAM_NO,TeamModule::team_no(fighter.module_accessor) as i32);
                let item_id = VarModule::get_int(obj_object,vars::link_arrow::instance::FUSED_ITEM_ID) as u32;
                let item_manager = smash2::app::ItemManager::instance().unwrap();
                smash2::app::ItemManager::remove_item_from_id(item_manager,item_id);
            }
        }
    }
}

#[smashline::fighter_frame(agent = FIGHTER_KIND_SHIZUE, main)]
pub fn isabelle_fix(fighter: &mut L2CFighterCommon) {
    unsafe {
        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_SEARCH {
            let obj_id = WorkModule::get_int(fighter.module_accessor,*FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TARGET_OBJECT_ID) as u32;
            let obj_boma = sv_battle_object::module_accessor(obj_id);
            let obj_object = utils::util::get_battle_object_from_id(obj_id);
            if utility::get_kind(&mut *obj_boma) == *WEAPON_KIND_LINK_BOWARROW {
                let owner_id = WorkModule::get_int(obj_boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
                let owner_module_accessor = smash::app::sv_battle_object::module_accessor(owner_id);
                let link_object = utils::util::get_battle_object_from_id(owner_id);
                let fused_item = if utility::get_kind(&mut *owner_module_accessor) == *FIGHTER_KIND_KIRBY {
                    VarModule::get_int(link_object,vars::kirby::instance::FUSED_ITEM_KIND)
                }
                else if utility::get_kind(&mut *owner_module_accessor) == *FIGHTER_KIND_MURABITO {
                    VarModule::get_int(link_object,vars::murabito::instance::FUSED_ITEM_KIND)
                }
                else {
                    VarModule::get_int(link_object,vars::link::instance::FUSED_ITEM_KIND)
                };
                VarModule::set_int(fighter.battle_object,vars::shizue::instance::FUSED_ITEM_KIND,fused_item);
                VarModule::set_int(fighter.battle_object,vars::shizue::instance::TEAM_NO,TeamModule::team_no(fighter.module_accessor) as i32);
                let item_id = VarModule::get_int(obj_object,vars::link_arrow::instance::FUSED_ITEM_ID) as u32;
                let item_manager = smash2::app::ItemManager::instance().unwrap();
                smash2::app::ItemManager::remove_item_from_id(item_manager,item_id);
            }
        }
    }
}

#[smashline::fighter_frame(agent = FIGHTER_KIND_KIRBY, main)]
pub fn kirby_fix(fighter: &mut L2CFighterCommon) {
    unsafe {
        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_LOOP {
            let obj_id = WorkModule::get_int(fighter.module_accessor,*FIGHTER_KIRBY_STATUS_SPECIAL_N_WORK_INT_INHALE_OBJECT_ID) as u32;
            let obj_object = utils::util::get_battle_object_from_id(obj_id);
            let obj_boma = sv_battle_object::module_accessor(obj_id);
            let item_id = if utility::get_kind(&mut *obj_boma) == *WEAPON_KIND_LINK_BOWARROW {
                VarModule::get_int(obj_object,vars::link_arrow::instance::FUSED_ITEM_ID) as u32
            }
            else {
                *BATTLE_OBJECT_ID_INVALID as u32
            };
            let item_manager = smash2::app::ItemManager::instance().unwrap();
            smash2::app::ItemManager::remove_item_from_id(item_manager,item_id);
        }
    }
}

const FIGHTER_ROSETTA_STATUS_SPECIAL_LW_INT_CAPTURE_OBJECT_ID: i32 = 0x11000006;

#[smashline::fighter_frame(agent = FIGHTER_KIND_ROSETTA, main)]
pub fn rosalina_fix(fighter: &mut L2CFighterCommon) {
    unsafe {
        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_LW {
            let obj_id = WorkModule::get_int(fighter.module_accessor,FIGHTER_ROSETTA_STATUS_SPECIAL_LW_INT_CAPTURE_OBJECT_ID) as u32;
            let obj_object = utils::util::get_battle_object_from_id(obj_id);
            let obj_boma = sv_battle_object::module_accessor(obj_id);
            let item_id = if utility::get_kind(&mut *obj_boma) == *WEAPON_KIND_LINK_BOWARROW {
                VarModule::get_int(obj_object,vars::link_arrow::instance::FUSED_ITEM_ID) as u32
            }
            else {
                *BATTLE_OBJECT_ID_INVALID as u32
            };
            let item_manager = smash2::app::ItemManager::instance().unwrap();
            smash2::app::ItemManager::remove_item_from_id(item_manager,item_id);
        }
    }
}

#[status_script(agent = "link_bowarrow", status = WN_LINK_BOWARROW_STATUS_KIND_HAVED, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn arrow_haved_end(weapon: &mut L2CFighterBase) -> L2CValue {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let owner_module_accessor = smash::app::sv_battle_object::module_accessor(owner_id);
    let link_object = utils::util::get_battle_object_from_id(owner_id);
    if utility::get_kind(&mut *owner_module_accessor) == *FIGHTER_KIND_MURABITO
    || utility::get_kind(&mut *owner_module_accessor) == *FIGHTER_KIND_SHIZUE {
        let item = if utility::get_kind(&mut *owner_module_accessor) == *FIGHTER_KIND_MURABITO {
            VarModule::get_int(link_object,vars::murabito::instance::FUSED_ITEM_KIND)
        }
        else {
            VarModule::get_int(link_object,vars::shizue::instance::FUSED_ITEM_KIND)
        };
        if item != *ITEM_KIND_NONE {
            let mut params = CreateItemParam {
                founder_pos: Vector4f{x: PostureModule::pos_x(weapon.module_accessor), y: PostureModule::pos_y(weapon.module_accessor), z: PostureModule::pos_z(weapon.module_accessor), w: 0.0},
                item_pos: Vector4f{x: PostureModule::pos_x(weapon.module_accessor), y: PostureModule::pos_y(weapon.module_accessor), z: PostureModule::pos_z(weapon.module_accessor), w: 0.0},
                item_kind: ItemKind(item),
                another_battle_object_id: *BATTLE_OBJECT_ID_INVALID as u32,
                variation_kind: *ITEM_VARIATION_NONE,
                lr_dir: PostureModule::lr(weapon.module_accessor),
                owner_id: (*(owner_module_accessor)).battle_object_id,
                unk_20: 20,
                pokeball_or_assist_kind: *ITEM_KIND_NONE,
                unk_0: 0,
                weird_flag: 0x633F800000,
                unk_1_weird: 1,
                unk_approx_0: 0.0,
                unk_02: 0.0
            };
            let item_manager = smash2::app::ItemManager::instance().unwrap();
            let battle_object = create_item(item_manager,&mut params,false,false,false);
            let item_boma = (*battle_object).module_accessor;
            if item != *ITEM_KIND_HEALBALL
            && item != *ITEM_KIND_CHEWING
            && item != *ITEM_KIND_BOOMERANG {
                StatusModule::change_status_request(item_boma,*ITEM_STATUS_KIND_HAVE,false);
            }
            if item == *ITEM_KIND_LINKBOMB {
                PostureModule::set_scale(item_boma,1.3,false);
            }
            LinkModule::remove_model_constraint(item_boma,true);
            if LinkModule::is_link(item_boma,*ITEM_LINK_NO_HAVE) == false {
                VisibilityModule::set_whole(item_boma,true);
                LinkModule::link(item_boma,*ITEM_LINK_NO_HAVE,(*(weapon.module_accessor)).battle_object_id);
                LinkModule::set_model_constraint_pos_ort(item_boma,*ITEM_LINK_NO_HAVE,Hash40::new("top"),Hash40::new("top"),*CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32,true);
            }
            VarModule::on_flag(weapon.battle_object,vars::link_arrow::instance::IS_ARROW_FUSE);
            set_arrow_fuse_params(weapon.module_accessor,item,(*(item_boma)).battle_object_id);
            if utility::get_kind(&mut *owner_module_accessor) == *FIGHTER_KIND_MURABITO {
                VarModule::set_int(link_object,vars::murabito::instance::FUSED_ITEM_KIND,*ITEM_KIND_NONE);
            }
            else {
                VarModule::set_int(link_object,vars::shizue::instance::FUSED_ITEM_KIND,*ITEM_KIND_NONE);
            }
        }
    }
    return L2CValue::I32(0)
}