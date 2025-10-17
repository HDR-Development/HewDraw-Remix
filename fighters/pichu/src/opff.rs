// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

// symbol-based call for the pikachu/pichu characters' common opff
extern "Rust" {
    fn gimmick_flash(boma: &mut BattleObjectModuleAccessor);
}

// handles pichu's charge increase
unsafe fn charge_state_increase(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if !VarModule::is_flag(fighter.battle_object, vars::pichu::instance::CHARGE_STATE_ENABLED)
    && MeterModule::level(fighter.battle_object) >= MeterModule::meter_cap(fighter.battle_object) {
        let charge_state_time = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "charge_state_time");
        VarModule::set_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER, charge_state_time);
        VarModule::on_flag(fighter.battle_object, vars::pichu::instance::CHARGE_STATE_ENABLED)
        //gimmick_flash(fighter.module_accessor);
    }
}

// handles pichu's charge decrease once at full charge
unsafe fn charge_state_decrease(fighter: &mut L2CFighterCommon) {
    if !VarModule::is_flag(fighter.battle_object, vars::pichu::instance::CHARGE_STATE_ENABLED) {
        return;
    }
    let gimmick_timer = VarModule::get_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER);
    if gimmick_timer > 0 
    && !fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW]) {
        let charge_state_time = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "charge_state_time");
        VarModule::dec_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER);
        let meter_max = (MeterModule::meter_cap(fighter.battle_object) as f32 * MeterModule::meter_per_level(fighter.battle_object));
        MeterModule::drain_direct(fighter.battle_object, meter_max / (charge_state_time as f32));
        let handle = VarModule::get_int(fighter.battle_object, vars::pichu::instance::CHARGE_EFFECT_HANDLER);
        if gimmick_timer == charge_state_time - 45 {
            EffectModule::set_scale(fighter.module_accessor, handle as u32, &Vector3f{ x: 0.8, y: 0.8, z: 0.8 });
        }
        if gimmick_timer == charge_state_time - 60 {
            EffectModule::set_scale(fighter.module_accessor, handle as u32, &Vector3f{ x: 0.7, y: 0.7, z: 0.7 });
        }
        if gimmick_timer == charge_state_time - 75 {
            EffectModule::set_scale(fighter.module_accessor, handle as u32, &Vector3f{ x: 0.6, y: 0.6, z: 0.6 });
        }
        if gimmick_timer == charge_state_time - 90 {
            EffectModule::set_scale(fighter.module_accessor, handle as u32, &Vector3f{ x: 0.5, y: 0.5, z: 0.5 });
        }
        if gimmick_timer == charge_state_time - 72 {
            STOP_SE(fighter, Hash40::new("vc_pichu_final01"));
        }
    }
    if gimmick_timer <= 0 {
        VarModule::off_flag(fighter.battle_object, vars::pichu::instance::CHARGE_STATE_ENABLED);
        EffectModule::req_on_joint(
            fighter.module_accessor,
            Hash40::new("sys_smash_flash"),
            Hash40::new("head"),
            &Vector3f::zero(),
            &Vector3f::zero(),
            1.5,
            &Vector3f::zero(),
            &Vector3f::zero(),
            false,
            0,
            0,
            0
        );
    }
}

// handles the damage multipliers
unsafe fn charge_state_damage_multipliers(fighter: &mut L2CFighterCommon) {
    if !VarModule::is_flag(fighter.battle_object, vars::pichu::instance::CHARGE_STATE_ENABLED) {
        VarModule::set_float(fighter.battle_object, vars::pichu::instance::CHARGE_STATE_DAMAGE_MUL, 1.0);
        VarModule::set_float(fighter.battle_object, vars::pichu::instance::CHARGE_STATE_RECOIL_MUL, 1.0);
        MeterModule::set_damage_gain_mul(fighter.battle_object, 1.0);
    } else {
        VarModule::set_float(fighter.battle_object, vars::pichu::instance::CHARGE_STATE_DAMAGE_MUL, 1.2);
        VarModule::set_float(fighter.battle_object, vars::pichu::instance::CHARGE_STATE_RECOIL_MUL, 1.25);
        MeterModule::set_damage_gain_mul(fighter.battle_object, 0.0);
    }
}

// charge status resets on death and game end
unsafe fn charge_state_reset(fighter: &mut L2CFighterCommon) {
    if !sv_information::is_ready_go()
    || lua_bind::FighterManager::is_result_mode(utils::singletons::FighterManager())
    || fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_WIN,
        *FIGHTER_STATUS_KIND_LOSE,
        *FIGHTER_STATUS_KIND_ENTRY,
    ]) {
        VarModule::set_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER, 0);
        VarModule::off_flag(fighter.battle_object, vars::pichu::instance::CHARGE_STATE_ENABLED);
        VarModule::set_int(fighter.battle_object, vars::pichu::instance::CHARGE_EFFECT_HANDLER, -1);
        MeterModule::reset(fighter.battle_object);
    }

    if fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_REBIRTH
    ]) {
        VarModule::set_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER, 0);
        if VarModule::is_flag(fighter.battle_object, vars::pichu::instance::CHARGE_STATE_ENABLED) {
            VarModule::off_flag(fighter.battle_object, vars::pichu::instance::CHARGE_STATE_ENABLED);
            let meter_lost = MeterModule::meter(fighter.battle_object) * 2.0 / 3.0;
            MeterModule::drain_direct(fighter.battle_object, meter_lost);
        }
    }
}

// handles the effects of pichu's charged state
unsafe fn charge_state_effects(fighter: &mut L2CFighterCommon) {
    if VarModule::is_flag(fighter.battle_object, vars::pichu::instance::CHARGE_STATE_ENABLED)
    && VarModule::get_int(fighter.battle_object, vars::pichu::instance::CHARGE_EFFECT_HANDLER) == -1 {
        app::FighterUtil::flash_eye_info(fighter.module_accessor);
        let handle = EffectModule::req_follow(fighter.module_accessor, Hash40::new("pichu_final_hold"), Hash40::new("waist"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &Vector3f::zero(), 0.9, true, 0, 0, 0, 0, 0, true, true) as u32;
        VarModule::set_int(fighter.battle_object, vars::pichu::instance::CHARGE_EFFECT_HANDLER, handle as i32);
        PLAY_SE(fighter, Hash40::new("vc_pichu_final01"));
        PLAY_SE(fighter, Hash40::new("se_pichu_final02"));
    }
    else if !VarModule::is_flag(fighter.battle_object, vars::pichu::instance::CHARGE_STATE_ENABLED) 
    && VarModule::get_int(fighter.battle_object, vars::pichu::instance::CHARGE_EFFECT_HANDLER) != -1 {
        let handle = VarModule::get_int(fighter.battle_object, vars::pichu::instance::CHARGE_EFFECT_HANDLER) as u32;
        EffectModule::kill(fighter.module_accessor, handle, false, false);
        VarModule::set_int(fighter.battle_object, vars::pichu::instance::CHARGE_EFFECT_HANDLER, -1);
    }
}

unsafe fn zippy_zap_attack_cancels(fighter: &mut L2CFighterCommon) {
    // set cancel flag
    if [
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_WARP,
        *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_END
    ].contains(&fighter.status())
    && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD)
    && !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_PARRY)  {
        VarModule::on_flag(fighter.battle_object, vars::pichu::status::SPECIAL_HI_QUICK_ATTACK_CANCEL);
    }

    // Immediate attack cancels
    if [
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_WARP,
    ].contains(&fighter.status())
    && StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_GROUND
    && VarModule::is_flag(fighter.battle_object, vars::pichu::status::SPECIAL_HI_QUICK_ATTACK_CANCEL)
    && !fighter.is_in_hitlag()
    && fighter.get_aerial() != None { // Aerial cancels
        KineticModule::mul_speed(fighter.module_accessor, &Vector3f::new(0.25, 0.25, 0.25), *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        PostureModule::add_pos(fighter.module_accessor, &Vector3f::new(0.0, 5.0, 0.0)); // to prevent landing instantly
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_AIR.into(), false.into());
        return
    }
}

// TRAINING MODE
// Full Meter Gain/Drain via shield during up/down taunt
unsafe fn charge_training_taunt(fighter: &mut L2CFighterCommon) {
    let mut agent_base = fighter.fighter_base.agent_base;
    if is_training_mode()
    && fighter.status() == *FIGHTER_STATUS_KIND_APPEAL
    && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
    && !VarModule::is_flag(fighter.battle_object, vars::pichu::instance::CHARGE_STATE_ENABLED) { 
        let meter_max = (MeterModule::meter_cap(fighter.object()) as f32 * MeterModule::meter_per_level(fighter.object()));
        MeterModule::add(fighter.battle_object, meter_max);
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_HOLD,
        *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_END,
        *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_LW_HIT
    ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

unsafe fn skull_bash_edge_cancel(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_END) 
    && fighter.global_table[PREV_SITUATION_KIND] == SITUATION_KIND_GROUND
    && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL, false);
    }
}

pub unsafe extern "C" fn pichu_meter(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if !sv_information::is_ready_go() && fighter.status_frame() < 1 {
        return;
    }
    MeterModule::update(fighter.object(), false);
    MeterModule::set_meter_cap(fighter.object(), 1);
    MeterModule::set_meter_per_level(fighter.object(), 70.0);
    utils::ui::UiManager::set_pichu_meter_enable(fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32, true);
    utils::ui::UiManager::set_pichu_meter_info(
        fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32,
        MeterModule::meter(fighter.object()),
        (MeterModule::meter_cap(fighter.object()) as f32 * MeterModule::meter_per_level(fighter.object())),
        MeterModule::meter_per_level(fighter.object()),
        VarModule::is_flag(fighter.battle_object, vars::pichu::instance::CHARGE_STATE_ENABLED)
    );
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    // charge state
    charge_state_increase(fighter);
    charge_state_decrease(fighter);
    charge_state_damage_multipliers(fighter);
    charge_state_reset(fighter);
    charge_state_effects(fighter);

    // tech
    zippy_zap_attack_cancels(fighter);
    skull_bash_edge_cancel(fighter);
    fastfall_specials(fighter);

    // training mode
    charge_training_taunt(fighter);
}

pub unsafe extern "C" fn pichu_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    common::opff::fighter_common_opff(fighter);
	pichu_frame(fighter);
}

pub unsafe fn pichu_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, pichu_frame_wrapper);
    agent.on_line(Main, pichu_meter);
}
