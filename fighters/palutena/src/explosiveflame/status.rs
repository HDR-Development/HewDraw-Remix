use super::*;
use globals::*;

unsafe extern "C" fn check_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_RESET,
        *GROUND_CORRECT_KIND_AIR as u32,
        app::GroundCliffCheckKind(*GROUND_CORRECT_KIND_AIR),
        false,
        0,
        0,
        0,
        0
    );

    return 0.into();
}

unsafe extern "C" fn check_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    weapon.set_int(29, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("check"), 0.0, 1.0, false, 0.0, false, false);

    let mut pos = *PostureModule::pos(weapon.get_owner_boma());
    pos.y += 11.0; // set height
    PostureModule::set_pos(weapon.module_accessor, &pos);
    PostureModule::init_pos(weapon.module_accessor, &pos, true, true);
    weapon.shift(L2CValue::Ptr(check_main_loop as *const () as _))
}

unsafe extern "C" fn check_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    weapon.dec_int(*WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let life = weapon.get_int(*WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if LinkModule::is_parent_damage_reaction(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT) {
        weapon.on_flag(*WEAPON_PALUTENA_EXPLOSIVEFLAME_INSTANCE_WORK_ID_FLAG_RESERVE_MISS);
    }
    if life == 19 {
        // use f0 pos, effective 9f to release special
        let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
        let palutena = utils::util::get_battle_object_from_id(owner_id);
        let palutena_boma: &mut BattleObjectModuleAccessor = &mut *(*palutena).module_accessor;
        let palu_lr = palutena_boma.lr();
        let kirby = palutena_boma.kind() == *FIGHTER_KIND_KIRBY;
        let base_range = if kirby {45.0} else {palutena_boma.get_param_float("param_special_s", "special_s_generate_range")};
        let held_range = if kirby {75.0} else {palutena_boma.get_param_float("param_special_s", "special_s_flick_generate_range")};
        // hold check, filter out mashed b inputs
        let buffer = ControlModule::get_command_life_count_max(palutena_boma) as usize;
        let hold_frames = InputModule::get_trigger_count(palutena, Buttons::Special);
        let offset_x = if palutena_boma.is_button_on(Buttons::Special) && hold_frames >= buffer {held_range} else {base_range};
        // check where to put it
        let pos = *PostureModule::pos(weapon.module_accessor);
        let mut new_pos = Vector3f::new(pos.x + (offset_x * palu_lr), pos.y, pos.z);
        let ground_pos_stage = &mut Vector2f::zero();
        // forces it to not clip as much thru stage (nerf)
        // sideways clip
        let start_pos_x = &Vector2f::new(pos.x, new_pos.y);
        let end_pos_x = &Vector2f::new(new_pos.x+(palu_lr+5.25), new_pos.y);
        let is_touch_side = GroundModule::line_segment_check(weapon.module_accessor, start_pos_x, end_pos_x, &Vector2f::zero(), ground_pos_stage, false);
        if is_touch_side != 0 as *const *const u64 {new_pos.x = ground_pos_stage.x-(5.25*palu_lr)};
        // floor clip
        let center_y = &Vector2f::new(new_pos.x, new_pos.y);
        let bottom_y = &Vector2f::new(new_pos.x, new_pos.y - 10.5);
        let is_touch_down = GroundModule::line_segment_check(weapon.module_accessor, center_y, bottom_y, &Vector2f::zero(), ground_pos_stage, false);
        if is_touch_down != 0 as *const *const u64 {new_pos.y = ground_pos_stage.y+10.5};
        PostureModule::set_pos(weapon.module_accessor, &new_pos);
        PostureModule::init_pos(weapon.module_accessor, &new_pos, true, true);
        // if somehow spawns inside ground
        //if GroundModule::is_touch(weapon.module_accessor, (*GROUND_TOUCH_FLAG_LEFT | *GROUND_TOUCH_FLAG_UP | *GROUND_TOUCH_FLAG_RIGHT | *GROUND_TOUCH_FLAG_UP_LEFT
        //| *GROUND_TOUCH_FLAG_UP_RIGHT) as u32) {
        //    weapon.on_flag(*WEAPON_PALUTENA_EXPLOSIVEFLAME_INSTANCE_WORK_ID_FLAG_RESERVE_MISS);
        //}
        //if !weapon.is_flag(*WEAPON_PALUTENA_EXPLOSIVEFLAME_INSTANCE_WORK_ID_FLAG_RESERVE_MISS)
        //&& WeaponSpecializer_PalutenaExplosiveflame::is_touch_down(weapon.battle_object as *mut smash::app::Weapon) {
        //    weapon.on_flag(*WEAPON_PALUTENA_EXPLOSIVEFLAME_INSTANCE_WORK_ID_FLAG_RESERVE_MISS);
        //}
    }
    if weapon.is_flag(*WEAPON_PALUTENA_EXPLOSIVEFLAME_INSTANCE_WORK_ID_FLAG_RESERVE_MISS) {
        weapon.change_status(WEAPON_PALUTENA_EXPLOSIVEFLAME_STATUS_KIND_MISS.into(), false.into());
    }
    if life <= 0 {
        weapon.change_status(WEAPON_PALUTENA_EXPLOSIVEFLAME_STATUS_KIND_EXPLODE.into(), false.into());
    }

    return 0.into();
}

unsafe extern "C" fn explode_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_RESET,
        0,
        app::GroundCliffCheckKind(*GROUND_CORRECT_KIND_AIR),
        false,
        0,
        0,
        0,
        0
    );

    return 0.into();
}

unsafe extern "C" fn explode_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    weapon.set_int(36, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("explode"), 0.0, 1.0, false, 0.0, false, false);
    if StopModule::is_stop(weapon.module_accessor) {
        explode_main_substatus(weapon);
    }
    
    weapon.global_table[SUB_STATUS2].assign(&L2CValue::Ptr(explode_main_substatus as *const () as _));
    weapon.fastshift(L2CValue::Ptr(explode_main_loop as *const () as _))
}

unsafe extern "C" fn explode_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    return 0.into();
}

unsafe extern "C" fn explode_main_substatus(weapon: &mut L2CWeaponCommon) -> L2CValue {
    weapon.dec_int(*WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if weapon.get_int(*WEAPON_INSTANCE_WORK_ID_INT_LIFE) <= 0 {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
    
    return 0.into();
}

unsafe extern "C" fn explode_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *WEAPON_PALUTENA_EXPLOSIVEFLAME_STATUS_KIND_CHECK, check_pre);
    agent.status(Main, *WEAPON_PALUTENA_EXPLOSIVEFLAME_STATUS_KIND_CHECK, check_main);

    agent.status(Pre, *WEAPON_PALUTENA_EXPLOSIVEFLAME_STATUS_KIND_EXPLODE, explode_pre);
    agent.status(Main, *WEAPON_PALUTENA_EXPLOSIVEFLAME_STATUS_KIND_EXPLODE, explode_main);
    agent.status(End, *WEAPON_PALUTENA_EXPLOSIVEFLAME_STATUS_KIND_EXPLODE, explode_end);
}