// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;
 
// symbol-based call for the pikachu/pichu characters' common opff
extern "Rust" {
    fn electric_rats_common(fighter: &mut smash::lua2cpp::L2CFighterCommon);
    fn gimmick_flash(boma: &mut BattleObjectModuleAccessor);
}

// handles pichu's charge increase
unsafe fn charge_state_increase(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    if VarModule::get_int(boma.object(), vars::pichu::instance::CHARGE_LEVEL) == 0 {
        if MeterModule::level(boma.object()) == 2 {
            let charge_state_time = ParamModule::get_int(boma.object(), ParamType::Agent, "charge_state_time");
            VarModule::set_int(boma.object(), vars::common::instance::GIMMICK_TIMER, charge_state_time);
            VarModule::set_int(boma.object(), vars::pichu::instance::CHARGE_LEVEL, 1);
            //gimmick_flash(boma);
        }
    }
}

// handles pichu's charge decrease once at full charge
unsafe fn charge_state_decrease(boma: &mut BattleObjectModuleAccessor) {
    if VarModule::get_int(boma.object(), vars::pichu::instance::CHARGE_LEVEL) == 1 {
        if VarModule::get_int(boma.object(), vars::common::instance::GIMMICK_TIMER) > 0 
        && !boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW]) {
            let charge_state_time = ParamModule::get_int(boma.object(), ParamType::Agent, "charge_state_time");
            VarModule::dec_int(boma.object(), vars::common::instance::GIMMICK_TIMER);
            MeterModule::drain_direct(boma.object(), 50.0/(charge_state_time as f32));
            if VarModule::get_int(boma.object(), vars::common::instance::GIMMICK_TIMER) == charge_state_time - 45 {
                let handle = VarModule::get_int(boma.object(), vars::pichu::instance::CHARGE_EFFECT_HANDLER);
                EffectModule::set_scale(boma, handle as u32, &Vector3f{ x: 0.8, y: 0.8, z: 0.8 });
            }
            if VarModule::get_int(boma.object(), vars::common::instance::GIMMICK_TIMER) == charge_state_time - 60 {
                let handle = VarModule::get_int(boma.object(), vars::pichu::instance::CHARGE_EFFECT_HANDLER);
                EffectModule::set_scale(boma, handle as u32, &Vector3f{ x: 0.7, y: 0.7, z: 0.7 });
            }
            if VarModule::get_int(boma.object(), vars::common::instance::GIMMICK_TIMER) == charge_state_time - 75 {
                let handle = VarModule::get_int(boma.object(), vars::pichu::instance::CHARGE_EFFECT_HANDLER);
                EffectModule::set_scale(boma, handle as u32, &Vector3f{ x: 0.6, y: 0.6, z: 0.6 });
            }
            if VarModule::get_int(boma.object(), vars::common::instance::GIMMICK_TIMER) == charge_state_time - 90 {
                let handle = VarModule::get_int(boma.object(), vars::pichu::instance::CHARGE_EFFECT_HANDLER);
                EffectModule::set_scale(boma, handle as u32, &Vector3f{ x: 0.5, y: 0.5, z: 0.5 });
            }
            if VarModule::get_int(boma.object(), vars::common::instance::GIMMICK_TIMER) == charge_state_time - 72 {
                STOP_SE(get_fighter_common_from_accessor(boma), Hash40::new("vc_pichu_final01"));
            }
        }
        if VarModule::get_int(boma.object(), vars::common::instance::GIMMICK_TIMER) <= 0 {
            VarModule::set_int(boma.object(), vars::pichu::instance::CHARGE_LEVEL, 0);
            EffectModule::req_on_joint(
                boma,
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
}

// handles the damage multipliers
unsafe fn charge_state_damage_multipliers(boma: &mut BattleObjectModuleAccessor) {
    if VarModule::get_int(boma.object(), vars::pichu::instance::CHARGE_LEVEL) == 0 {
        VarModule::set_float(boma.object(), vars::pichu::instance::CHARGE_DAMAGE_MUL, 1.0);
        VarModule::set_float(boma.object(), vars::pichu::instance::CHARGE_RECOIL_MUL, 1.0);
    }
    else if VarModule::get_int(boma.object(), vars::pichu::instance::CHARGE_LEVEL) == 1 {
        VarModule::set_float(boma.object(), vars::pichu::instance::CHARGE_DAMAGE_MUL, 1.2);
        VarModule::set_float(boma.object(), vars::pichu::instance::CHARGE_RECOIL_MUL, 1.25);
    }
}

// charge status resets on death and game end
unsafe fn charge_state_reset(boma: &mut BattleObjectModuleAccessor) {
    if lua_bind::FighterManager::is_result_mode(utils::singletons::FighterManager()) {
        VarModule::set_int(boma.object(), vars::pichu::instance::CHARGE_LEVEL, 0);
        VarModule::set_int(boma.object(), vars::common::instance::GIMMICK_TIMER, 0);
        VarModule::set_int(boma.object(), vars::pichu::instance::CHARGE_EFFECT_HANDLER, -1);
        MeterModule::reset(boma.object());
    }
    
    if boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_WIN,
        *FIGHTER_STATUS_KIND_LOSE,
        *FIGHTER_STATUS_KIND_ENTRY,]) || !sv_information::is_ready_go() {
        VarModule::set_int(boma.object(), vars::common::instance::GIMMICK_TIMER, 0);
        VarModule::set_int(boma.object(), vars::pichu::instance::CHARGE_LEVEL, 0);
        VarModule::set_int(boma.object(), vars::pichu::instance::CHARGE_EFFECT_HANDLER, -1);
        MeterModule::reset(boma.object());
    }

    if boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_REBIRTH]) {
            VarModule::set_int(boma.object(), vars::common::instance::GIMMICK_TIMER, 0);
            if VarModule::get_int(boma.object(), vars::pichu::instance::CHARGE_LEVEL) == 1 {
                VarModule::set_int(boma.object(), vars::pichu::instance::CHARGE_LEVEL, 0);
                MeterModule::drain_direct(boma.object(), (MeterModule::meter(boma.object())/4.0)*3.0);
            }
        }
}

// handles the effects of pichu's charged state
unsafe fn charge_state_effects(boma: &mut BattleObjectModuleAccessor) {
    if VarModule::get_int(boma.object(), vars::pichu::instance::CHARGE_LEVEL) == 1
    && VarModule::get_int(boma.object(), vars::pichu::instance::CHARGE_EFFECT_HANDLER) == -1 {
        app::FighterUtil::flash_eye_info(boma);
        let handle = EffectModule::req_follow(boma, Hash40::new("pichu_final_hold"), Hash40::new("waist"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &Vector3f::zero(), 0.9, true, 0, 0, 0, 0, 0, true, true) as u32;
        VarModule::set_int(boma.object(), vars::pichu::instance::CHARGE_EFFECT_HANDLER, handle as i32);
        PLAY_SE(get_fighter_common_from_accessor(boma), Hash40::new("vc_pichu_final01"));
        PLAY_SE(get_fighter_common_from_accessor(boma), Hash40::new("se_pichu_final02"));
    }
    else if VarModule::get_int(boma.object(), vars::pichu::instance::CHARGE_LEVEL) == 0 && VarModule::get_int(boma.object(), vars::pichu::instance::CHARGE_EFFECT_HANDLER) != -1 {
        let handle = VarModule::get_int(boma.object(), vars::pichu::instance::CHARGE_EFFECT_HANDLER) as u32;
        EffectModule::kill(boma, handle, false, false);
        VarModule::set_int(boma.object(), vars::pichu::instance::CHARGE_EFFECT_HANDLER, -1);
    }
}

unsafe fn discharge_momentum(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_LW_HIT) && VarModule::is_flag(fighter.battle_object, vars::pichu::instance::IS_CHARGE_ATTACK) {
        let air_brake_x = WorkModule::get_param_float(fighter.module_accessor, smash::hash40("air_brake_x"), 0);
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, air_brake_x, 0.0);
        app::sv_kinetic_energy::set_brake(fighter.lua_state_agent);
    }
}

unsafe fn zippy_zap_jump_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32) {
    if [*FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_WARP, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_END].contains(&status_kind) && VarModule::is_flag(boma.object(), vars::pichu::instance::IS_CHARGE_ATTACK) {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) && !boma.is_in_hitlag() {
            boma.check_jump_cancel(false, false);
        }
    }
}

// TRAINING MODE
// Full Meter Gain/Drain via shield during up/down taunt
unsafe fn charge_training_taunt(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    let mut agent_base = fighter.fighter_base.agent_base;
    if is_training_mode() {
        if status_kind == *FIGHTER_STATUS_KIND_APPEAL {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
                if VarModule::get_int(boma.object(), vars::pichu::instance::CHARGE_LEVEL) == 0 { 
                    let meter_max = (MeterModule::meter_cap(fighter.object()) as f32 * MeterModule::meter_per_level(fighter.object()));
                    MeterModule::add(boma.object(), meter_max);
                }
            }
        }         
    }
}

pub extern "C" fn pichu_meter(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        if !sv_information::is_ready_go() && fighter.status_frame() < 1 {
            return;
        }
        MeterModule::update(fighter.object(), false);
        MeterModule::set_meter_cap(fighter.object(), 2);
        MeterModule::set_meter_per_level(fighter.object(), 25.0);
        utils::ui::UiManager::set_pichu_meter_enable(fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32, true);
        utils::ui::UiManager::set_pichu_meter_info(
            fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32,
            MeterModule::meter(fighter.object()),
            (MeterModule::meter_cap(fighter.object()) as f32 * MeterModule::meter_per_level(fighter.object())),
            MeterModule::meter_per_level(fighter.object()),
            VarModule::get_int(fighter.battle_object, vars::pichu::instance::CHARGE_LEVEL) == 1
        );
    }
}
    
// JC Agility
unsafe fn jc_agility(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status(*FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL)
    && boma.status_frame() > 3
    && boma.is_situation(*SITUATION_KIND_GROUND)
    && boma.is_prev_status(*FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_END)
    && !VarModule::is_flag(boma.object(), vars::pikachu::instance::DISABLE_QA_JC) {
        boma.check_jump_cancel(true, false);
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    charge_state_increase(fighter, boma);
    charge_state_decrease(boma);
    charge_state_damage_multipliers(boma);
    charge_state_reset(boma);
    charge_state_effects(boma);
    discharge_momentum(fighter);
    zippy_zap_jump_cancel(boma, status_kind, situation_kind, cat[0]);
    charge_training_taunt(fighter, boma, status_kind);
    jc_agility(boma);
}

pub extern "C" fn pichu_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		pichu_frame(fighter);
        electric_rats_common(fighter);
    }
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
