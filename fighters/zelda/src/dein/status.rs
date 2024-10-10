use super::*;

unsafe extern "C" fn tame_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
	if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_ZELDA {
		let zelda = utils::util::get_battle_object_from_id(owner_id);
		let dein = VarModule::get_int(zelda, vars::zelda::instance::SPECIAL_S_DEIN_OBJECT_ID) as u32;
		dein_remove(weapon, dein);
	}
    smashline::original_status(Main, weapon, *WEAPON_ZELDA_DEIN_STATUS_KIND_TAME)(weapon)
}

pub unsafe extern "C" fn dein_remove(weapon: &mut smash::lua2cpp::L2CFighterBase, dein: u32) {
    let article_boma = sv_battle_object::module_accessor(dein as u32);
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let zelda = utils::util::get_battle_object_from_id(owner_id);
    // Fire
    let handle1 = EffectModule::req_on_joint(article_boma, Hash40::new("zelda_appeal_s_fire"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 1.5, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0);
    EffectModule::set_rate_last(article_boma, 2.5);
    EffectModule::set_rgb(article_boma, handle1 as u32, 0.65, 0.3, 0.3);
    // Smoke Dark
    let handle = EffectModule::req_on_joint(article_boma, Hash40::new("sys_steam"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 1.5, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0);
    EffectModule::set_rgb(article_boma, handle as u32, 0.0, 0.0, 0.0);
    EffectModule::set_alpha(article_boma, handle as u32, 3.0);
    // Smoke Light
    let handle2 = EffectModule::req_on_joint(article_boma, Hash40::new("sys_steam"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 1.5, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0);
    EffectModule::set_rgb(article_boma, handle2 as u32, 0.1, 0.1, 0.1);
    EffectModule::set_alpha(article_boma, handle2 as u32, 3.0);
    //despawn
    if sv_battle_object::is_active(dein) && dein != weapon.battle_object_id {sv_battle_object::end_inhaled(dein, true); }
	//set id
    VarModule::set_int(zelda, vars::zelda::instance::SPECIAL_S_DEIN_OBJECT_ID, weapon.battle_object_id as i32);
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *WEAPON_ZELDA_DEIN_STATUS_KIND_TAME, tame_main);
}
