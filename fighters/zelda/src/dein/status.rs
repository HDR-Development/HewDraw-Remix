use super::*;

unsafe extern "C" fn move_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let zelda = utils::util::get_battle_object_from_id(owner_id);
    let life = weapon.get_param_float("param_dein", "life");
    VarModule::set_int(zelda, vars::zelda::instance::SPECIAL_S_CURRENT_DEIN_MOVE_OBJECT_ID, weapon.battle_object_id as i32);
    weapon.set_float(life, *WEAPON_ZELDA_DEIN_STATUS_WORK_FLOAT_LIFE);
    weapon.set_float(0.0, *WEAPON_ZELDA_DEIN_STATUS_WORK_FLOAT_COUNT);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("move"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(move_loop as *const () as _))
}

unsafe extern "C" fn move_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let zelda = utils::util::get_battle_object_from_id(owner_id);
    let zelda_boma = &mut *(*zelda).module_accessor;
    if (weapon.get_float(*WEAPON_ZELDA_DEIN_STATUS_WORK_FLOAT_LIFE) < 0.0 && !zelda_boma.is_status(*FIGHTER_ZELDA_STATUS_KIND_SPECIAL_S_LOOP)) //dins-fly away tech
    || (zelda_boma.is_button_trigger(Buttons::Attack) && !zelda_boma.is_button_trigger(Buttons::CStickOn))
    {
        weapon.change_status(WEAPON_ZELDA_DEIN_STATUS_KIND_TAME.into(), false.into())
    } 
    //hold to keep dins out w/o moving
    if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32)
    || weapon.get_float(*WEAPON_ZELDA_DEIN_STATUS_WORK_FLOAT_LIFE) < 0.0 {
        sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    }
    //ticks frame counter, skips 2nd substatus
    WorkModule::add_float(weapon.module_accessor, -1.0, *WEAPON_ZELDA_DEIN_STATUS_WORK_FLOAT_LIFE);
    let count = weapon.get_param_float("param_dein", "count");
    if count > weapon.get_float(*WEAPON_ZELDA_DEIN_STATUS_WORK_FLOAT_COUNT) {
        WorkModule::add_float(weapon.module_accessor, 1.0, *WEAPON_ZELDA_DEIN_STATUS_WORK_FLOAT_COUNT);
    }
    0.into()
}

unsafe extern "C" fn tame_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let zelda = utils::util::get_battle_object_from_id(owner_id);
	if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_ZELDA {
		let zelda = utils::util::get_battle_object_from_id(owner_id);
		let dein = VarModule::get_int(zelda, vars::zelda::instance::SPECIAL_S_DEIN_OBJECT_ID);
        let dein2 = VarModule::get_int(zelda, vars::zelda::instance::SPECIAL_S_DEIN_OBJECT_ID_2);
        dein_remove(weapon, dein, dein2);
	}
    weapon.global_table[0x14].assign(&L2CValue::Ptr(dins_refresh as *const () as _));
    smashline::original_status(Main, weapon, *WEAPON_ZELDA_DEIN_STATUS_KIND_TAME)(weapon)
}

pub unsafe extern "C" fn dein_remove(weapon: &mut smash::lua2cpp::L2CFighterBase, dein: i32, dein2: i32) {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let zelda = utils::util::get_battle_object_from_id(owner_id);
    let zelda_boma = &mut *(*zelda).module_accessor;
    let thisdins: i32 = weapon.battle_object_id as i32;
    //Multi-dins code and checks
    //2 Dins IDS
    let dein_battle_object = utils::util::get_battle_object_from_id(dein as u32);
    let dein_boma = &mut *(*dein_battle_object).module_accessor;
    let dein2_battle_object = utils::util::get_battle_object_from_id(dein2 as u32);
    let dein2_boma = &mut *(*dein2_battle_object).module_accessor;
    //Dins existence checks, applicable refreshes and variable settings
    if sv_battle_object::is_active(dein as u32) {
        if sv_battle_object::is_active(dein2 as u32) { 
            //if both dins slots are full, shuffle slots and explode first dins
            VarModule::set_int(zelda, vars::zelda::instance::SPECIAL_S_DEIN_OBJECT_ID, dein2);
            VarModule::set_int(zelda, vars::zelda::instance::SPECIAL_S_DEIN_OBJECT_ID_2, thisdins);
            delete_effects(weapon, dein);
            sv_battle_object::end_inhaled(dein as u32, true);
            VarModule::on_flag(dein2_battle_object, vars::zelda::status::SPECIAL_S_DINS_REFRESH);
        } else {
            //if first slot full but second empty
            //assign dins to empty slot, refresh first dins
            VarModule::set_int(zelda, vars::zelda::instance::SPECIAL_S_DEIN_OBJECT_ID_2, thisdins);
            VarModule::on_flag(dein_battle_object, vars::zelda::status::SPECIAL_S_DINS_REFRESH);
        }
    } else {
        //if 1st slot empty and second full
        if sv_battle_object::is_active(dein2 as u32) {
            VarModule::set_int(zelda, vars::zelda::instance::SPECIAL_S_DEIN_OBJECT_ID, dein2);
            VarModule::set_int(zelda, vars::zelda::instance::SPECIAL_S_DEIN_OBJECT_ID_2, thisdins);
            VarModule::on_flag(dein2_battle_object, vars::zelda::status::SPECIAL_S_DINS_REFRESH);
        } else {
            //if both dins empty
            VarModule::set_int(zelda, vars::zelda::instance::SPECIAL_S_DEIN_OBJECT_ID, thisdins);
        }
    }
    //Dins player arrow effect
    let effect = EffectModule::req_follow(weapon.module_accessor, Hash40::new("sys_direction"), Hash40::new("top"), &Vector3f::new(0.0, 7.5, 0.0), &Vector3f::new(0.0, 90.0, 180.0), 0.67, true, 0, 0, 0, 0, 0, false, false);
    VarModule::set_int(weapon.battle_object, vars::zelda::instance::SPECIAL_S_COOLDOWN_EFFECT_HANDLE, effect as i32);
    LAST_EFFECT_SET_SCALE_W(weapon, 0.67, 0.4, 0.67);
    let team_color = FighterUtil::get_team_color(zelda_boma);
    let effect_team_color = FighterUtil::get_effect_team_color(EColorKind(team_color as i32), Hash40::new("direction_effect_color"));
    EffectModule::set_rgb_partial_last(weapon.module_accessor, effect_team_color.value[0], effect_team_color.value[1], effect_team_color.value[2]);
}

unsafe extern "C" fn dins_refresh(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if VarModule::is_flag(weapon.battle_object, vars::zelda::status::SPECIAL_S_DINS_REFRESH) {
        EFFECT_OFF_KIND(weapon, Hash40::new("sys_flash"), true, true);
        MotionModule::change_motion_force_inherit_frame(weapon.module_accessor, Hash40::new("tame"), 0.0, 1.0, 1.0);
        weapon.set_float(160.0, *WEAPON_ZELDA_DEIN_STATUS_WORK_FLOAT_LIFE);
        VarModule::off_flag(weapon.battle_object, vars::zelda::status::SPECIAL_S_DINS_REFRESH);
    }
    0.into()
}

pub unsafe extern "C" fn delete_effects(weapon: &mut smash::lua2cpp::L2CFighterBase, dein: i32) {
    let article_boma = sv_battle_object::module_accessor(dein as u32);
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    //Fizzle effects upon deletion of 1st dins
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
}

unsafe extern "C" fn tame_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let zelda = utils::util::get_battle_object_from_id(owner_id);
    let dein = VarModule::get_int(zelda, vars::zelda::instance::SPECIAL_S_DEIN_OBJECT_ID);
    let dein2 = VarModule::get_int(zelda, vars::zelda::instance::SPECIAL_S_DEIN_OBJECT_ID_2);
    let thisdins: i32 = weapon.battle_object_id as i32;
    if dein == thisdins {VarModule::set_int(zelda, vars::zelda::instance::SPECIAL_S_DEIN_OBJECT_ID, 0); }
    if dein2 == thisdins {VarModule::set_int(zelda, vars::zelda::instance::SPECIAL_S_DEIN_OBJECT_ID_2, 0); }
    let effect = VarModule::get_int(weapon.battle_object, vars::zelda::instance::SPECIAL_S_COOLDOWN_EFFECT_HANDLE);
    EffectModule::kill(weapon.module_accessor, effect as u32, true, true);
    VarModule::set_int(weapon.battle_object, vars::zelda::instance::SPECIAL_S_COOLDOWN_EFFECT_HANDLE, 0);
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *WEAPON_ZELDA_DEIN_STATUS_KIND_MOVE, move_main);
    agent.status(Main, *WEAPON_ZELDA_DEIN_STATUS_KIND_TAME, tame_main);
    agent.status(End, *WEAPON_ZELDA_DEIN_STATUS_KIND_TAME, tame_end);
}
