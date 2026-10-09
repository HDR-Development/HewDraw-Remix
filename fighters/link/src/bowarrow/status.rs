use super::*;

// symbol-based calls for link's arrow fuse
extern "Rust" {
    fn fuse_set_arrow_params(arrow_boma: *mut BattleObjectModuleAccessor, item: i32, id: u32);
    fn fuse_create_item(arrow_boma: *mut BattleObjectModuleAccessor, item: i32, founder_boma: *mut BattleObjectModuleAccessor, owner_id: u32);
    fn fuse_detach_item(item_boma: *mut BattleObjectModuleAccessor) -> bool;
    fn fuse_attach_item(item_boma: *mut BattleObjectModuleAccessor, arrow_id: u32) -> bool;
    fn fuse_remove_item(arrow_object: *mut BattleObject);
    fn fuse_get_owner_team_no(weapon_boma: *mut BattleObjectModuleAccessor) -> i32;
}

unsafe fn get_owner(weapon_boma: *mut BattleObjectModuleAccessor) -> (*mut BattleObjectModuleAccessor, *mut BattleObject, i32) {
    let owner_id = WorkModule::get_int(weapon_boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let owner_boma = sv_battle_object::module_accessor(owner_id);
    let owner_object = get_battle_object_from_id(owner_id);
    let owner_kind = utility::get_kind(&mut *owner_boma);
    (owner_boma, owner_object, owner_kind)
}

// WN_LINK_BOWARROW_STATUS_KIND_HAVED //

unsafe extern "C" fn haved_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let (owner_boma, owner_object, owner_kind) = get_owner(weapon.module_accessor);
    let mut is_fused = false;
    if ItemModule::is_have_item(owner_boma, 0) {
        let kind = ItemModule::get_have_item_kind(owner_boma, 0);
        let trait_flag = ItemModule::get_have_item_trait(owner_boma, 0);
        // trait flags are bitmasks, so swing/shoot items can carry other bits as well
        let is_swing_or_shoot = trait_flag & (*ITEM_TRAIT_FLAG_SWING | *ITEM_TRAIT_FLAG_SHOOT) as u64 != 0;
        // projectile items that fire the arrow straight (see fuse_set_arrow_params)
        let is_projectile = [*ITEM_KIND_KILLER, *ITEM_KIND_BANANAGUN, *ITEM_KIND_DOLPHINBOMB].contains(&kind);
        is_fused = ![*ITEM_KIND_NONE, *ITEM_KIND_ASSIST, *ITEM_KIND_LINKARROW].contains(&kind)
            && trait_flag != *ITEM_TRAIT_FLAG_NONE as u64
            && (is_projectile || !is_swing_or_shoot);
        if is_fused {
            fuse_set_arrow_params(weapon.module_accessor, *ITEM_KIND_NONE, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    }
    VarModule::set_flag(weapon.battle_object, vars::link_bowarrow::instance::IS_ARROW_FUSE, is_fused);
    if owner_kind == *FIGHTER_KIND_LINK {
        VarModule::set_flag(owner_object, vars::link::instance::IS_ARROW_FUSE, is_fused);
    }
    else if owner_kind == *FIGHTER_KIND_KIRBY {
        VarModule::set_flag(owner_object, vars::kirby::instance::IS_ARROW_FUSE, is_fused);
    }

    if is_fused {
        // move the held item from the owner's hand onto the arrow
        let item_id = VarModule::get_int(weapon.battle_object, vars::link_bowarrow::instance::FUSED_ITEM_ID) as u32;
        let item_boma = sv_battle_object::module_accessor(item_id);
        fuse_detach_item(item_boma);
        if fuse_attach_item(item_boma, (*weapon.module_accessor).battle_object_id) {
            // TODO: PLAY_SE(weapon, Hash40::new("se_link_fuse"));
        }
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("haved"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("haved_hide"), 0.0, 1.0, false, 0.0, false, false);
    }
    weapon.fastshift(L2CValue::Ptr(haved_main_loop as *const () as _))
}

unsafe extern "C" fn haved_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

// re-creates the item stored by Villager/Isabelle when a pocketed fused arrow is pulled back out
unsafe extern "C" fn haved_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let (owner_boma, owner_object, owner_kind) = get_owner(weapon.module_accessor);
    let fused_item_kind_var = if owner_kind == *FIGHTER_KIND_MURABITO {
        vars::murabito::instance::FUSED_ITEM_KIND
    }
    else if owner_kind == *FIGHTER_KIND_SHIZUE {
        vars::shizue::instance::FUSED_ITEM_KIND
    }
    else {
        return 0.into();
    };
    let item = VarModule::get_int(owner_object, fused_item_kind_var);
    if item != *ITEM_KIND_NONE && item > *ITEM_KIND_ASSIST {
        fuse_create_item(weapon.module_accessor, item, weapon.module_accessor, (*owner_boma).battle_object_id);
        VarModule::set_int(owner_object, fused_item_kind_var, *ITEM_KIND_NONE);
    }
    0.into()
}

// WN_LINK_BOWARROW_STATUS_KIND_FLY //

unsafe extern "C" fn fly_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    smashline::original_status(Init, weapon, *WN_LINK_BOWARROW_STATUS_KIND_FLY)(weapon);
    if VarModule::is_flag(weapon.battle_object, vars::link_bowarrow::instance::IS_ARROW_FUSE)
    && VarModule::get_int(weapon.battle_object, vars::link_bowarrow::instance::FUSED_ITEM_TYPE) == 1 {
        // projectile items fly straight and fast, and hit harder
        let lr = PostureModule::lr(weapon.module_accessor);
        weapon.clear_lua_stack();
        lua_args!(weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 12.0 * lr, 0.0);
        sv_kinetic_energy::set_speed(weapon.lua_state_agent);
        weapon.clear_lua_stack();
        lua_args!(weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
        sv_kinetic_energy::set_accel(weapon.lua_state_agent);
        KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
        AttackModule::set_power_mul(weapon.module_accessor, 2.5);
    }
    0.into()
}

unsafe extern "C" fn fly_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if VarModule::is_flag(weapon.battle_object, vars::link_bowarrow::instance::IS_ARROW_FUSE) {
        let status_next = StatusModule::status_kind_next(weapon.module_accessor);
        if status_next == *WN_LINK_BOWARROW_STATUS_KIND_STICK
        || status_next == *WN_LINK_BOWARROW_STATUS_KIND_HIT_STICK
        || AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_HIT) {
            // the arrow landed or hit something, so release the item
            let item_id = VarModule::get_int(weapon.battle_object, vars::link_bowarrow::instance::FUSED_ITEM_ID) as u32;
            if sv_battle_object::is_active(item_id) {
                let item_boma = sv_battle_object::module_accessor(item_id);
                if fuse_detach_item(item_boma) {
                    let status = VarModule::get_int(weapon.battle_object, vars::link_bowarrow::instance::FUSED_ITEM_POST_STATUS);
                    StatusModule::change_status_request(item_boma, status, false);
                }
            }
        }
        else {
            // the arrow vanished without hitting anything
            fuse_remove_item(weapon.battle_object);
        }
    }
    EffectModule::detach_all(weapon.module_accessor, 5);
    0.into()
}

// WN_LINK_BOWARROW_STATUS_KIND_STICK / HIT_STICK //

// restores the arrow's team after a reflected fused arrow sticks
unsafe extern "C" fn stick_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if VarModule::is_flag(weapon.battle_object, vars::link_bowarrow::instance::IS_REFLECT_FIX) {
        let (owner_boma, _, _) = get_owner(weapon.module_accessor);
        TeamModule::set_team(weapon.module_accessor, fuse_get_owner_team_no(weapon.module_accessor), true);
        TeamModule::set_team_owner_id(weapon.module_accessor, (*owner_boma).battle_object_id);
        VarModule::off_flag(weapon.battle_object, vars::link_bowarrow::instance::IS_REFLECT_FIX);
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *WN_LINK_BOWARROW_STATUS_KIND_HAVED, haved_main);
    agent.status(End, *WN_LINK_BOWARROW_STATUS_KIND_HAVED, haved_end);
    agent.status(Init, *WN_LINK_BOWARROW_STATUS_KIND_FLY, fly_init);
    agent.status(End, *WN_LINK_BOWARROW_STATUS_KIND_FLY, fly_end);
    agent.status(End, *WN_LINK_BOWARROW_STATUS_KIND_STICK, stick_end);
    agent.status(End, *WN_LINK_BOWARROW_STATUS_KIND_HIT_STICK, stick_end);
}
