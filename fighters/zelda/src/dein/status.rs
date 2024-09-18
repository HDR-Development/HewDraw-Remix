use super::*;

unsafe extern "C" fn tame_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    //let ret = smashline::original_status(Main, weapon, *WEAPON_ZELDA_DEIN_STATUS_KIND_TAME)(weapon);
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
	if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_ZELDA {
		let zelda = utils::util::get_battle_object_from_id(owner_id);
		let dein = VarModule::get_int(zelda, vars::zelda::instance::DEIN_OBJECT_ID);
        let dein2 = VarModule::get_int(zelda, vars::zelda::instance::DEIN_OBJECT_ID_2);
		//if !VarModule::is_flag(weapon.battle_object, vars::zelda::instance::DEIN_EXPLODE) {
            //run normal checks if dein isn't set to detonate
            dein_remove(weapon, dein, dein2);
        //} else {
            //notify_event_msc_cmd!(weapon, Hash40::new_raw(0x27936db96d));
        //}
	}
    //ret
    //fighter.global_table[0x32].assign(&L2CValue::Ptr(air_jump_uniq as *const () as _));
    weapon.global_table[0x14].assign(&L2CValue::Ptr(dins_refresh as *const () as _));
    smashline::original_status(Main, weapon, *WEAPON_ZELDA_DEIN_STATUS_KIND_TAME)(weapon)
}

unsafe extern "C" fn tame_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let zelda = utils::util::get_battle_object_from_id(owner_id);
    let dein = VarModule::get_int(zelda, vars::zelda::instance::DEIN_OBJECT_ID);
    let dein2 = VarModule::get_int(zelda, vars::zelda::instance::DEIN_OBJECT_ID_2);
    let thisdins: i32 = weapon.battle_object_id as i32;
    //if status is changing when not due to explode
    //if weapon.get_float(*WEAPON_ZELDA_DEIN_STATUS_WORK_FLOAT_LIFE) <= 0.0 {
    //    //if dins1 slot not overwritten
    //    if dein == thisdins {
    //        if dein2 == 0 { //if second slot is not taken
    //            VarModule::set_int(zelda, vars::zelda::instance::DEIN_OBJECT_ID, 0); 
    //        } else {
    //            //if second slot is taken, refresh
//
    //        }
    //    } else {//if dins overwritten, explode
    //        //set power to uncharged?
    //        //weapon.set_float(0.0, *WEAPON_ZELDA_DEIN_STATUS_WORK_FLOAT_COUNT);
    //    }
    //} else {
    //    //if dins explodes naturally
        if dein == thisdins {VarModule::set_int(zelda, vars::zelda::instance::DEIN_OBJECT_ID, 0); }
        if dein2 == thisdins {VarModule::set_int(zelda, vars::zelda::instance::DEIN_OBJECT_ID_2, 0); }
    //}

    //if WorkModule::get_float(weapon.module_accessor, *WEAPON_ZELDA_DEIN_STATUS_WORK_FLOAT_LIFE) > 0.0 {dein_remove(weapon, dins1, dins2, 0, true); } //if killed before prime run clear func
    0.into()
}

pub unsafe extern "C" fn dein_remove(weapon: &mut smash::lua2cpp::L2CFighterBase, dein: i32, dein2: i32) {
    let article_boma = sv_battle_object::module_accessor(dein as u32);
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let zelda = utils::util::get_battle_object_from_id(owner_id);
    let zelda_boma = &mut *(*zelda).module_accessor;
    let thisdins: i32 = weapon.battle_object_id as i32;
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
    //get 2 dins info
    let dein_battle_object = utils::util::get_battle_object_from_id(dein as u32);
    let dein_boma = &mut *(*dein_battle_object).module_accessor;
    let dein2_battle_object = utils::util::get_battle_object_from_id(dein2 as u32);
    let dein2_boma = &mut *(*dein2_battle_object).module_accessor;
    if sv_battle_object::is_active(dein as u32) {
        
        if sv_battle_object::is_active(dein2 as u32) { 
            //if both dins slots are full, shuffle slots and explode first dins
            VarModule::set_int(zelda, vars::zelda::instance::DEIN_OBJECT_ID, dein2);
            VarModule::set_int(zelda, vars::zelda::instance::DEIN_OBJECT_ID_2, thisdins);
            sv_battle_object::end_inhaled(dein as u32, true);
            if dein2 != thisdins {
                VarModule::on_flag(dein2_battle_object, vars::zelda::status::DINS_REFRESH);
            }
        } else {
            //if first slot full but second empty
            //assign dins to empty slot, refresh first dins
            VarModule::on_flag(dein_battle_object, vars::zelda::status::DINS_REFRESH);
            VarModule::set_int(zelda, vars::zelda::instance::DEIN_OBJECT_ID_2, thisdins);
        }
    } else {
        //if 1st slot empty and second full
        if sv_battle_object::is_active(dein2 as u32) {
            VarModule::set_int(zelda, vars::zelda::instance::DEIN_OBJECT_ID, dein2);
            VarModule::set_int(zelda, vars::zelda::instance::DEIN_OBJECT_ID_2, 0);
            if dein2 != thisdins {
                //if 2nd slot isnt this dins, refresh 2nd dins
                VarModule::on_flag(dein2_battle_object, vars::zelda::status::DINS_REFRESH);
                VarModule::set_int(zelda, vars::zelda::instance::DEIN_OBJECT_ID_2, thisdins);
            }
        } else {
        //if both dins empty
        VarModule::set_int(zelda, vars::zelda::instance::DEIN_OBJECT_ID, thisdins);
        }
    }
}

unsafe extern "C" fn dins_refresh(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let zelda = utils::util::get_battle_object_from_id(owner_id);
    let zelda_boma = &mut *(*zelda).module_accessor;
    let life = zelda_boma.get_param_float("param_dein", "bang_time");
    if VarModule::is_flag(weapon.battle_object, vars::zelda::status::DINS_REFRESH) {
        EFFECT_OFF_KIND(weapon, Hash40::new("sys_flash"), true, true);
        MotionModule::change_motion_force_inherit_frame(weapon.module_accessor, Hash40::new("tame"), 0.0, 1.0, 1.0);
        weapon.set_float(160.0, *WEAPON_ZELDA_DEIN_STATUS_WORK_FLOAT_LIFE);
        VarModule::off_flag(weapon.battle_object, vars::zelda::status::DINS_REFRESH);
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *WEAPON_ZELDA_DEIN_STATUS_KIND_TAME, tame_main);
    agent.status(End, *WEAPON_ZELDA_DEIN_STATUS_KIND_TAME, tame_end);
}
