use super::*;

// when a fused arrow is reflected, the fused item joins the reflector's team
// the arrow's team is restored to its owner once it sticks (see stick end)
unsafe fn fuse_reflect_fix(weapon: &mut L2CFighterBase) {
    if !weapon.is_status(*WN_LINK_BOWARROW_STATUS_KIND_FLY)
    || !AttackModule::is_infliction(weapon.module_accessor, *COLLISION_KIND_MASK_REFLECTOR)
    || !VarModule::is_flag(weapon.battle_object, vars::link_bowarrow::instance::IS_ARROW_FUSE) {
        return;
    }
    VarModule::on_flag(weapon.battle_object, vars::link_bowarrow::instance::IS_REFLECT_FIX);
    let item_id = VarModule::get_int(weapon.battle_object, vars::link_bowarrow::instance::FUSED_ITEM_ID) as u32;
    if !sv_battle_object::is_active(item_id) {
        return;
    }
    let item_boma = sv_battle_object::module_accessor(item_id);
    TeamModule::set_team(item_boma, TeamModule::team_no(weapon.module_accessor) as i32, true);
    TeamModule::set_team_owner_id(item_boma, TeamModule::team_owner_id(weapon.module_accessor) as u32);
}

pub unsafe extern "C" fn link_bowarrow_frame(weapon: &mut L2CFighterBase) {
    fuse_reflect_fix(weapon);
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, link_bowarrow_frame);
}
