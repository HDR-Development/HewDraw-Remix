// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

pub unsafe fn armored_charge(fighter: &mut L2CFighterCommon, motion_kind: u64) {
    if fighter.is_motion_one_of(&[
        Hash40::new("attack_s3_s"),
        Hash40::new("attack_s3_hi"),
        Hash40::new("attack_s3_lw"),
        Hash40::new("attack_hi3"),
        Hash40::new("attack_lw3") ]) {
        let is_hold = ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK);
        let charge = VarModule::get_int(fighter.battle_object, vars::krool::status::ATTACK_CHARGE);
        let mut charge_start_frame = 0.0;
        let mut charge_end_frame = 0.0;
        // due to what I presume is internal rounding error, the current amount of 20.0 equates to 18 frames
        let max_charge_frames = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_waist.max_charge_frames");

        match MotionModule::motion_kind(fighter.module_accessor) {
            _ if [hash40("attack_s3_s"), hash40("attack_s3_hi"), hash40("attack_s3_lw")].contains(&motion_kind) => {
                charge_start_frame = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_waist.attack_s3_charge_start");
                charge_end_frame = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_waist.attack_s3_charge_end");
            },
            _ if motion_kind == hash40("attack_hi3") => {
                charge_start_frame = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_waist.attack_hi3_charge_start");
                charge_end_frame = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_waist.attack_hi3_charge_end");
            },
            _ if motion_kind == hash40("attack_lw3") => {
                charge_start_frame = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_waist.attack_lw3_charge_start");
                charge_end_frame = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "param_waist.attack_lw3_charge_end");
            },
            _ => {}
        }

        if (charge_start_frame..charge_end_frame).contains(&fighter.motion_frame()) && charge < (max_charge_frames as i32) && is_hold {
            if fighter.motion_frame() == charge_start_frame {
                let facing = 0.0 * PostureModule::lr(fighter.module_accessor);
                EFFECT_FOLLOW(fighter, Hash40::new("sys_level_up"), Hash40::new("waistswells"), 3.0, 0.0, facing, 0, 0, 0, 0.5, true);
                PLAY_SEQUENCE(fighter, Hash40::new("seq_krool_rnd_attack"));
            }
            let motion_rate = (charge_end_frame - charge_start_frame)/max_charge_frames;
            MotionModule::set_rate(fighter.module_accessor, motion_rate);
            VarModule::set_int(fighter.battle_object, vars::krool::status::ATTACK_CHARGE, charge + 1);
        } else {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
        }
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_KROOL_STATUS_KIND_SPECIAL_S_GET,
        *FIGHTER_KROOL_STATUS_KIND_SPECIAL_S_CATCH,
        *FIGHTER_KROOL_STATUS_KIND_SPECIAL_S_THROW,
        *FIGHTER_KROOL_STATUS_KIND_SPECIAL_S_FAILURE,
        *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_FALL,
        *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_AIR_END,
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    armored_charge(fighter, motion_kind);
    fastfall_specials(fighter);
}

pub extern "C" fn krool_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
        krool_frame(fighter)
    }
}

pub unsafe fn krool_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, krool_frame_wrapper);
}