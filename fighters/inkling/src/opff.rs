// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;
use skyline::hooks::InlineCtx;

unsafe fn splatter_vfx(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_motion(Hash40::new("attack_s3_s")) {
        if AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT)
        && (1..=2).contains(&VarModule::get_int(boma.object(), vars::common::instance::LAST_ATTACK_HITBOX_ID)) {
            let pos = Vector3f{ x: 6.0, y: 0.0, z: 0.5 };
            let rot = Vector3f{ x: 0.0, y: 90.0, z: 0.0 };
            let handle = EffectModule::req_on_joint(boma, Hash40::new("inkling_blaster_muzzle"), Hash40::new("handr"), &pos, &rot, 0.8, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0) as u32;
            let r = boma.get_float(*FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_R);
            let g = boma.get_float(*FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_G);
            let b = boma.get_float(*FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_B);
            EffectModule::set_rgb(boma, handle, r, g, b);
            EffectModule::set_rate_last(boma, 0.5);
        }
    }
    else if boma.is_motion(Hash40::new("attack_air_b")) {
        if AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT)
        && (3..=4).contains(&VarModule::get_int(boma.object(), vars::common::instance::LAST_ATTACK_HITBOX_ID)) {
            let pos = Vector3f{ x: -18.0, y: 2.5, z: 0.0 };
            let rot = Vector3f{ x: 0.0, y: 90.0, z: 0.0 };
            let handle = EffectModule::req_on_joint(boma, Hash40::new("inkling_blaster_muzzle"), Hash40::new("top"), &pos, &rot, 0.8, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0) as u32;
            let r = boma.get_float(*FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_R);
            let g = boma.get_float(*FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_G);
            let b = boma.get_float(*FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_B);
            EffectModule::set_rgb(boma, handle, r, g, b);
            EffectModule::set_rate_last(boma, 0.5);
        }
    }
    else if boma.is_motion(Hash40::new("attack_air_lw")) {
        if AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT)
        && (1..=2).contains(&VarModule::get_int(boma.object(), vars::common::instance::LAST_ATTACK_HITBOX_ID)) {
            let pos = Vector3f{ x: 0.0, y: -6.5, z: 0.0 };
            let rot = Vector3f{ x: 0.0, y: 90.0, z: 0.0 };
            let handle = EffectModule::req_on_joint(boma, Hash40::new("inkling_blaster_muzzle"), Hash40::new("top"), &pos, &rot, 1.0, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0) as u32;
            let r = boma.get_float(*FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_R);
            let g = boma.get_float(*FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_G);
            let b = boma.get_float(*FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_B);
            EffectModule::set_rgb(boma, handle, r, g, b);
            EffectModule::set_rate_last(boma, 0.5);
        }
    }
}

unsafe fn roller_jump_cancel(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status(*FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_END)
        && boma.is_situation(*SITUATION_KIND_GROUND)
        && boma.status_frame() > 10 {
        boma.check_jump_cancel(true, false, true);
    }
    if boma.is_status(*FIGHTER_STATUS_KIND_SPECIAL_S)
    && boma.is_situation(*SITUATION_KIND_AIR)
    && boma.status_frame() <= 5
    && boma.is_cat_flag(Cat1::AirEscape) {
        ControlModule::reset_trigger(boma);
        StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_FALL, true);
        ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
    }
    if boma.is_motion(Hash40::new("special_air_s_jump_end")) {
        if MotionModule::frame(boma) > 6.0 {
            CancelModule::enable_cancel(boma);
        }
    }
}

unsafe fn ink_charge_cancel(boma: &mut BattleObjectModuleAccessor) {
    if (boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_INKLING_STATUS_KIND_SPECIAL_N_SHOOT])
    && boma.is_button_on(Buttons::Guard))
    && boma.is_situation(*SITUATION_KIND_GROUND)
    {
        boma.change_status_req(*FIGHTER_INKLING_STATUS_KIND_CHARGE_INK_START, false);
    }
}

unsafe fn up_special_startup_ledgegrab(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI) {
        // allows ledgegrab during upB startup
        fighter.sub_transition_group_check_air_cliff();
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_INKLING_STATUS_KIND_SPECIAL_N_END,
        *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_JUMP_END,
        *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_END,
        *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_STOP_WALL,
        *FIGHTER_INKLING_STATUS_KIND_SPECIAL_HI_JUMP,
        *FIGHTER_INKLING_STATUS_KIND_SPECIAL_HI_FALL,
        *FIGHTER_INKLING_STATUS_KIND_SPECIAL_HI_STOP_CEIL,
        *FIGHTER_INKLING_STATUS_KIND_SPECIAL_LW_EMPTY,
        *FIGHTER_INKLING_STATUS_KIND_SPECIAL_LW_THROW
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    splatter_vfx(boma);
    roller_jump_cancel(boma);
    ink_charge_cancel(boma);
    up_special_startup_ledgegrab(fighter);
    fastfall_specials(fighter);
}

pub extern "C" fn inkling_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		inkling_frame(fighter);
    }
}

pub unsafe fn inkling_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, inkling_frame_wrapper);
}