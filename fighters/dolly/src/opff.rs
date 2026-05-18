// opff import
utils::import_noreturn!(common::opff::{fighter_common_opff});
use super::*;
use globals::*;

// TRAINING MODE
// Full Meter Gain via shield during taunt
unsafe fn training_mode_full_meter(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if app::smashball::is_training_mode()
    && boma.is_status(*FIGHTER_STATUS_KIND_APPEAL)
    && boma.is_button_on(Buttons::Guard)
    {
        VarModule::set_int(fighter.battle_object, vars::dolly::instance::ADDED_METER_LEVELS, 8);
        let meter_cap = (VarModule::get_int(fighter.battle_object, vars::dolly::instance::ADDED_METER_LEVELS) + 2).clamp(2, 10);
        MeterModule::set_meter_cap(fighter.object(), meter_cap);
        let meter_max = (meter_cap as f32) * MeterModule::meter_per_level(fighter.object());
        MeterModule::add(boma.object(), meter_max);
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_END,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_FALL,
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

unsafe fn specials_ledgegrab_fix(fighter: &mut L2CFighterCommon) {
    if fighter.is_status_one_of(&[
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_END,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_ATTACK,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_ATTACK,
    ]) {
        fighter.sub_transition_group_check_air_cliff();
    }
}

unsafe fn disable_special_cancels_on_parry(fighter: &mut L2CFighterCommon) {
    if (
        fighter.is_flag(*FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL) ||
        fighter.is_flag(*FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL) ||
        VarModule::is_flag(fighter.battle_object, vars::dolly::status::INHERIT_FINAL_CANCEL_ON_END)
    )
    && (
        AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_PARRY) ||
        AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_PARRY)
    ) {
        fighter.off_flag(*FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
        fighter.off_flag(*FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        VarModule::off_flag(fighter.battle_object, vars::dolly::status::INHERIT_FINAL_CANCEL_ON_END);
    }
}

unsafe fn inherit_final_cancel(fighter: &mut L2CFighterCommon) {
    if fighter.is_status_one_of(&[
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_JUMP,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_ATTACK,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_ATTACK,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_ATTACK
    ])
    && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD)
    && !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_PARRY) {
        VarModule::on_flag(fighter.battle_object, vars::dolly::status::INHERIT_FINAL_CANCEL_ON_END);
    }
}

unsafe fn super_special_cancels(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, motion_kind: u64, frame: f32) {
    // Dont use cancels if we're already in cancel frames, if we're in hitlag, or if we didn't connect
    if CancelModule::is_enable_cancel(fighter.module_accessor) 
    || fighter.is_in_hitlag() {
        return;
    }

    let is_landing_cancel = {
        fighter.is_status_one_of(&[
            *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_LANDING,
            *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_END,
            *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_LANDING,
            *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_LANDING,
        ]) && VarModule::is_flag(fighter.battle_object, vars::dolly::status::INHERIT_FINAL_CANCEL_ON_END)
    };

    let is_nspecial_cancel = {
        fighter.is_flag(*FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL)
        && fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_N)
    };

    let is_other_special_cancel = {
        fighter.is_flag(*FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL)
        && fighter.is_status_one_of(&[
            *FIGHTER_STATUS_KIND_SPECIAL_HI,
            *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_COMMAND,
            *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_ATTACK,
            *FIGHTER_STATUS_KIND_SPECIAL_LW,
            *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_COMMAND,
            statuses::dolly::SPECIAL_LW_BREAKING,
            statuses::dolly::ATTACK_COMMAND_4
        ])
        && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) 
        && !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_PARRY) 
    };

    if is_landing_cancel || is_nspecial_cancel || is_other_special_cancel {
        status::dolly_check_super_special_command_wrapper(fighter);
    }
}

unsafe fn hit_cancel_timer(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    let hit_cancel_timer = VarModule::get_int(fighter.battle_object, vars::dolly::status::HIT_CANCEL_TIMER);
    if hit_cancel_timer > 0
    && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD)
    && !fighter.is_in_hitlag() {
        VarModule::dec_int(fighter.battle_object, vars::dolly::status::HIT_CANCEL_TIMER);
        if hit_cancel_timer - 1 == 0 {
            fighter.off_flag(*FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
            fighter.off_flag(*FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        }
    }
}

unsafe fn burn_knuckle_end_on_shield(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_ATTACK) {
        // Skip to end on shield
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD | *COLLISION_KIND_MASK_PARRY)
        && !fighter.is_in_hitlag() {
            fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_END.into(), false.into());
        }
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    inherit_final_cancel(fighter);
    disable_special_cancels_on_parry(fighter);
    training_mode_full_meter(fighter, boma, status_kind);
    fastfall_specials(fighter);
    specials_ledgegrab_fix(fighter);
    super_special_cancels(fighter, boma, status_kind, situation_kind, motion_kind, frame);
    hit_cancel_timer(fighter, boma);
    burn_knuckle_end_on_shield(fighter);
}

pub extern "C" fn dolly_meter(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        if !sv_information::is_ready_go() && fighter.status_frame() < 1 {
            return;
        }
        MeterModule::update(fighter.battle_object, false);
        let meter_cap = (VarModule::get_int(fighter.battle_object, vars::dolly::instance::ADDED_METER_LEVELS) + 2).clamp(2, 10);
        MeterModule::set_meter_cap(fighter.object(), meter_cap);
        MeterModule::set_meter_per_level(fighter.object(), 30.0);
        utils::ui::UiManager::set_ff_meter_enable(fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32, true);
        utils::ui::UiManager::set_ff_meter_info(
            fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32,
            MeterModule::meter(fighter.object()),
            (MeterModule::meter_cap(fighter.object()) as f32 * MeterModule::meter_per_level(fighter.object())),
            MeterModule::meter_per_level(fighter.object())
        );
    }
}

pub extern "C" fn dolly_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		dolly_frame(fighter)
    }
}

pub unsafe fn dolly_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, dolly_frame_wrapper);
    agent.on_line(Main, dolly_meter);
}