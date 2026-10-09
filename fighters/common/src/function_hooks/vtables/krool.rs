use super::*;
use utils::ext::*;

extern "C" {
    #[link_name = "krool_belly_damage_hook_impl"]
    fn krool_belly_damage_hook_impl(damage: f32, fighter: *mut Fighter, unk: bool);
}

// #[skyline::hook(offset = 0xc050d8, inline)]
// pub unsafe fn krool_belly_toggle_hook(ctx: &mut skyline::hooks::InlineCtx) {
//     krool_belly_toggle_hook_impl(ctx);
// }

#[skyline::hook(offset = 0xc055f0)]
pub unsafe fn krool_belly_damage_hook(damage: f32, fighter: *mut Fighter, unk: bool) {
    krool_belly_damage_hook_impl(damage, fighter, unk);
}

#[skyline::hook(offset = 0xc077e0)]
pub unsafe extern "C" fn hook_on_damage(vtable: u64, battle_object: *mut Fighter, log: u64) -> u64 {
    let boma = (&mut *(battle_object)).battle_object.boma();
    let opponent_id = *(*(log as *mut u64).add(0x10 / 0x8) as *mut u32).add(0x44 / 0x4);
    let opponent_boma = &mut *(sv_battle_object::module_accessor(opponent_id));

    let ret = call_original!(vtable, battle_object, log);

    // a crown fused to one of link's arrows, whether the crown or the arrow made contact
    let fuse_arrow_id = if opponent_boma.is_item() && LinkModule::is_link(opponent_boma, *ITEM_LINK_NO_HAVE) {
        LinkModule::get_parent_id(opponent_boma, *ITEM_LINK_NO_HAVE, true) as u32
    }
    else {
        opponent_id
    };
    let mut fuse_arrow = std::ptr::null_mut();
    if sv_battle_object::is_active(fuse_arrow_id) {
        let arrow_boma = &mut *sv_battle_object::module_accessor(fuse_arrow_id);
        if arrow_boma.is_weapon() && arrow_boma.kind() == *WEAPON_KIND_LINK_BOWARROW {
            fuse_arrow = get_battle_object_from_id(fuse_arrow_id);
        }
    }
    let is_crown_fuse_arrow = !fuse_arrow.is_null()
        && VarModule::is_flag(fuse_arrow, vars::link_bowarrow::instance::IS_ARROW_FUSE)
        && VarModule::get_int(fuse_arrow, vars::link_bowarrow::instance::FUSED_ITEM_KIND) == *ITEM_KIND_KROOLCROWN;

    if is_crown_fuse_arrow
    && boma.is_flag(*FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_DROP_CROWN) {
        crate::opff::fuse::fuse_remove_item(fuse_arrow);
        EffectModule::req_follow(boma, Hash40::new("sys_item_get"), Hash40::new("crown1"), &Vector3f::zero(), &Vector3f::zero(), 1.0, false, 0, 0, 0, 0, 0, false, false);
        boma.off_flag(*FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_DROP_CROWN);
    }
    else if opponent_boma.is_item() && opponent_boma.kind() == *ITEM_KIND_KROOLCROWN
    && boma.is_flag(*FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_DROP_CROWN) {
        StatusModule::change_status_request(opponent_boma, *ITEM_STATUS_KIND_DEAD, false);
        EffectModule::req_follow(boma, Hash40::new("sys_item_get"), Hash40::new("crown1"), &Vector3f::zero(), &Vector3f::zero(), 1.0, false, 0, 0, 0, 0, 0, false, false);
        boma.off_flag(*FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_DROP_CROWN);
    }

    ret
}

pub fn install() {
    skyline::install_hooks!(
        krool_belly_damage_hook,
        hook_on_damage
    );
}