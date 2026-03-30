// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn pin_drop_waveland(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status(*FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_END)
    && !boma.is_in_hitlag() && boma.status_frame() >= 13 {
        boma.check_airdodge_cancel();
    }
}

unsafe fn bair_charge(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_motion(Hash40::new("attack_air_b")) {
        let is_hold = ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK);
        let charge = VarModule::get_float(boma.object(), vars::kamui::status::ATTACK_AIR_B_CHARGE);
        let mut charge_start_frame = 6.0;
        let mut charge_end_frame = 11.0;

        if (charge_start_frame..charge_end_frame).contains(&boma.motion_frame()) {
            if is_hold {
                MotionModule::set_rate(boma, 0.2);
                VarModule::set_float(boma.object(), vars::kamui::status::ATTACK_AIR_B_CHARGE, charge + 1.0);
            }
            else {
                MotionModule::set_rate(boma, 1.0);
            }
        }
        else if boma.motion_frame() > charge_end_frame {
            VarModule::set_float(boma.object(), vars::kamui::status::ATTACK_AIR_B_CHARGE, (charge/13.0) * 10.0);
            MotionModule::set_rate(boma, 1.0);
        }
    }
}

unsafe fn up_special_early_landing(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI) {
        if fighter.is_situation(*SITUATION_KIND_GROUND)
        && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS) {
            fighter.change_status_req(*FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, false);
        }
    }
}

unsafe fn chain_hit(fighter: &mut L2CFighterCommon) {
    if fighter.is_motion_one_of(&[
        Hash40::new("attack_s3_s"),
        Hash40::new("attack_hi3"),
        Hash40::new("attack_lw3"),
        Hash40::new("attack_lw4"),
        Hash40::new("attack_air_n"),
        Hash40::new("attack_air_f"),
        Hash40::new("attack_air_hi"),
    ]) {
        if fighter.status_frame() <= 1 {
            VarModule::set_int(fighter.object(), vars::common::instance::LAST_ATTACK_HITBOX_ID, -1);
        }
        let last_id = VarModule::get_int(fighter.object(), vars::common::instance::LAST_ATTACK_HITBOX_ID);
        if fighter.is_motion(Hash40::new("attack_lw3")) {
            if last_id != 2 { return; }
        }
        else if fighter.is_motion(Hash40::new("attack_hi3")) {
            if !(last_id == 2 || last_id == 3) { return; }
        }
        else {
            if !(last_id == 1 || last_id == 2) { return; }
        }
        ATTACK(fighter, 4, 1, Hash40::new("haver"), 2.0, 367, 0, 0, 0, 4.0, 0.0, 0.0, -1.25, Some(0.0), Some(4.0), Some(-1.25), 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_SWORD);
        VarModule::set_int(fighter.object(), vars::common::instance::LAST_ATTACK_HITBOX_ID, -1);
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_HOLD,
        *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_SHOOT,
        *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_BITE,
        *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_JUMP_END,
        *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_JUMP,
        *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_ATTACK_END,
        *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_B,
        *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_F,
        *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_ATTACK,
        *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_LW_HIT
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    bair_charge(boma);
    pin_drop_waveland(boma);
    up_special_early_landing(fighter);
    chain_hit(fighter);
    fastfall_specials(fighter);
}

pub extern "C" fn kamui_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		kamui_frame(fighter)
    }
}

pub unsafe fn kamui_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, kamui_frame_wrapper);
}