use super::*;
use globals::*;
mod special_hi_h;
mod gaogaen_special_n;
mod luigi_special_n;
mod mario_special_n;
mod mariod_special_n;
mod ridley_special_n;
mod ganon_special_n;
mod ganon_special_n_float;
mod koopa_special_n;
mod littlemac_special_n_cancel;
mod diddy_special_n_cancel;
mod lucas_special_n;
mod sonic;
 
extern "C" fn kirby_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        // set the callbacks on fighter init
        if fighter.kind() == *FIGHTER_KIND_KIRBY {
            fighter.global_table[globals::USE_SPECIAL_HI_CALLBACK].assign(&L2CValue::Ptr(should_use_special_hi_callback as *const () as _));
            fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));
            fighter.global_table[globals::USE_SPECIAL_N_CALLBACK].assign(&L2CValue::Ptr(ganon_should_use_special_n_callback as *const () as _));
            fighter.global_table[globals::CHECK_SPECIAL_COMMAND].assign(&L2CValue::Ptr(shoto_check_special_command as *const () as _));

            if is_training_mode() {
                VarModule::set_int(fighter.battle_object, vars::koopa::instance::FIREBALL_COOLDOWN_FRAME,0);
            }
            else {
                VarModule::set_int(fighter.battle_object, vars::koopa::instance::FIREBALL_COOLDOWN_FRAME,KOOPA_MAX_COOLDOWN);
            }
        }
    }
}


extern "C" fn kirby_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        if fighter.kind() == *FIGHTER_KIND_KIRBY {
            //let charge_time = ParamModule::get_int(fighter.object(), ParamType::Agent, "attack_up_charge_time");
            VarModule::set_int(fighter.object(), LUCAS_CHARGE_TIME, vars::lucas::instance::SPECIAL_N_OFFENSE_UP_CHARGE_LEVEL);
            VarModule::off_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_ACTIVE);
            VarModule::off_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_INIT);
            VarModule::off_flag(fighter.object(), vars::lucas::instance::SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF);
        }
    }
}

unsafe extern "C" fn should_use_special_hi_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) && VarModule::is_flag(fighter.battle_object, vars::kirby::instance::DISABLE_SPECIAL_HI) {
        false.into()
    } else {
        true.into()
    }
}

unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if (fighter.is_situation(*SITUATION_KIND_GROUND) || fighter.is_situation(*SITUATION_KIND_CLIFF))
    || fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_DAMAGE,
        *FIGHTER_STATUS_KIND_DAMAGE_AIR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
        *FIGHTER_STATUS_KIND_DAMAGE_FALL]) {
            VarModule::off_flag(fighter.battle_object, vars::kirby::instance::DISABLE_SPECIAL_HI);
    }

    /// Ganon: Re-enables the ability to use aerial specials when connecting to ground or cliff
    if fighter.is_situation(*SITUATION_KIND_GROUND) || fighter.is_situation(*SITUATION_KIND_CLIFF)
    || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD]) {
        VarModule::off_flag(fighter.battle_object, vars::ganon::instance::DISABLE_SPECIAL_N);
    }

    /// Bowser: Remove fireball ready effect
    if fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_ENTRY,*FIGHTER_STATUS_KIND_DEAD,*FIGHTER_STATUS_KIND_REBIRTH,
        *FIGHTER_STATUS_KIND_WIN,*FIGHTER_STATUS_KIND_LOSE]) || !sv_information::is_ready_go() {
        EFFECT_OFF_KIND(fighter,Hash40::new("koopa_breath_m_fire"),false,false);
        VarModule::set_int(fighter.battle_object, vars::koopa::instance::FIREBALL_EFFECT_ID,0);
        VarModule::set_int(fighter.battle_object, vars::koopa::instance::FIREBALL_COOLDOWN_FRAME,KOOPA_MAX_COOLDOWN);
    }

    return true.into();
}

// FIGHTER_STATUS_KIND_JUMP //


pub unsafe extern "C" fn pre_jump(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_COPY) {
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_PICKEL {
            if fighter.status_pre_Jump_Common_param(L2CValue::Bool(true)).get_bool() {
                return 1.into()
            }
            else {
                if kirby_pickel_jump_status_check(fighter).get_bool() {
                    fighter.status_pre_Jump_sub_param(
                        L2CValue::I32(-1),
                        L2CValue::I32(-1),
                        L2CValue::I32(-1),
                        L2CValue::I32(*KINETIC_TYPE_NONE),
                        L2CValue::I32(*FS_SUCCEEDS_KEEP_EFFECT | *FS_SUCCEEDS_KEEP_SOUND | *FS_SUCCEEDS_KEEP_TRANSITION | *FS_SUCCEEDS_KEEP_CANCEL)
                    );
                }
                else {
                    fighter.status_pre_Jump_sub_param(
                        L2CValue::I32(*FIGHTER_STATUS_WORK_KEEP_FLAG_JUMP_FLAG),
                        L2CValue::I32(*FIGHTER_STATUS_WORK_KEEP_FLAG_JUMP_INT),
                        L2CValue::I32(*FIGHTER_STATUS_WORK_KEEP_FLAG_JUMP_FLOAT),
                        L2CValue::I32(*FIGHTER_KINETIC_TYPE_JUMP),
                        L2CValue::I32(0)
                    );
                }
                return 0.into()
            }
        }
    }
    fighter.status_pre_Jump()
}

unsafe extern "C" fn kirby_pickel_jump_status_check(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_STATUS_KIND] != FIGHTER_KIRBY_STATUS_KIND_PICKEL_SPECIAL_N1_JUMP && fighter.global_table[PREV_STATUS_KIND] != FIGHTER_KIRBY_STATUS_KIND_PICKEL_SPECIAL_N3_JUMP {
        return L2CValue::Bool(false);
    }
    else {
        return L2CValue::Bool(true);
    }
}

pub unsafe extern "C" fn shoto_check_special_command(fighter: &mut L2CFighterCommon) -> L2CValue {
    if (WorkModule::get_int(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_RYU)  {
        let cat4 = fighter.global_table[CMD_CAT4].get_i32();
        let is_special = fighter.is_cat_flag(Cat1::SpecialAny);

        // shakenetsu
        if is_special
        && cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N2_COMMAND != 0
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND)
        && fighter.sub_transition_term_id_cont_disguise(fighter.global_table[USE_SPECIAL_N_CALLBACK].clone()).get_bool() {
            fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_RYU_SPECIAL_N2_COMMAND.into(), true.into());
            return true.into();
        }

        // hado
        if is_special
        && cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N_COMMAND != 0
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND)
        && fighter.sub_transition_term_id_cont_disguise(fighter.global_table[USE_SPECIAL_N_CALLBACK].clone()).get_bool() {
            fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_RYU_SPECIAL_N_COMMAND.into(), true.into());
            return true.into();
        }

    } else if (WorkModule::get_int(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_KEN)  {
        let cat4 = fighter.global_table[CMD_CAT4].get_i32();
        let is_special = fighter.is_cat_flag(Cat1::SpecialAny);

        // hado
        if is_special
        && cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N_COMMAND != 0
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND)
        && fighter.sub_transition_term_id_cont_disguise(fighter.global_table[USE_SPECIAL_N_CALLBACK].clone()).get_bool() {
            fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_KEN_SPECIAL_N_COMMAND.into(), true.into());
            return true.into();
        }

    }

    false.into()
}

// FIGHTER_STATUS_KIND_THROW_KIRBY //


pub unsafe extern "C" fn throw_kirby_map_correction(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
    let frame = MotionModule::frame(fighter.module_accessor);
    let prev_frame = MotionModule::prev_frame(fighter.module_accessor);
    let fall_start_frame = if motion_kind == hash40("throw_b") { 27.0 } else { 31.0 };
    let fall_stop_frame = if motion_kind == hash40("throw_b") { 29.0 } else { 34.0 };
    let landing_frame = if motion_kind == hash40("throw_b") { 30.0 } else { 35.0 };
    let return_air_frame = if motion_kind == hash40("throw_b") { 39.0 } else { 43.0 };

    if (motion_kind != hash40("throw_b") && motion_kind != hash40("throw_f"))
    || frame <= fall_start_frame
    {
        return 0.into()
    }
    if prev_frame < return_air_frame && frame >= return_air_frame {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_THROW_FLAG_START_AIR);
    }
    if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
        if prev_frame < fall_stop_frame && frame >= fall_stop_frame {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -10.0);
            app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
            MotionModule::set_frame(fighter.module_accessor, fall_stop_frame, true);
            MotionModule::set_rate(fighter.module_accessor, 0.0);
            LinkModule::send_event_nodes_throw(fighter.module_accessor, Hash40::new("throw_sync_motion"), Hash40::new("invalid"), true, 0, 0, 0);
        }
    }
    else {
        if frame < landing_frame {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            MotionModule::set_frame(fighter.module_accessor, landing_frame, true);
            MotionModule::set_rate(fighter.module_accessor, 1.0);
            LinkModule::send_event_nodes_throw(fighter.module_accessor, Hash40::new("throw_sync_motion"), Hash40::new("invalid"), true, 0, 0, 0);
        }
    }
    0.into()
}


/// Prevents side b from being used again in air when it has been disabled by up-b fall
unsafe extern "C" fn ganon_should_use_special_n_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) && VarModule::is_flag(fighter.battle_object, vars::ganon::instance::DISABLE_SPECIAL_N) {
        false.into()
    } else {
        true.into()
    }
}

// FIGHTER_STATUS_KIND_SPECIAL_N //


unsafe extern "C" fn special_n_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let copy_chara = WorkModule::get_int(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA);
    if copy_chara == *FIGHTER_KIND_TRAIL { // swaps the cycle order of thundaga and blizzaga when copying sora
        let prev_status = fighter.global_table[0x10].get_i32();
        WorkModule::set_int(fighter.module_accessor, prev_status, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_STATUS_KIND_ATTACK_PREV);
        let mut magic_kind = WorkModule::get_int(fighter.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_SPECIAL_N_MAGIC_KIND);
        let fire = *FIGHTER_TRAIL_SPECIAL_N_MAGIC_KIND_FIRE;
        let blizzard = *FIGHTER_TRAIL_SPECIAL_N_MAGIC_KIND_BLIZZARD;
        let thunder = *FIGHTER_TRAIL_SPECIAL_N_MAGIC_KIND_THUNDER;
        if magic_kind == blizzard { // change to fire
            magic_kind = fire;
        } else if magic_kind == fire { // change to blizzard
            magic_kind = blizzard;
        }
        if ![fire, blizzard, thunder].contains(&magic_kind) {
            StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_KIND_TRAIL_SPECIAL_N1);
        } else if magic_kind == fire {
            StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_KIND_TRAIL_SPECIAL_N1);
        } else if magic_kind == blizzard { 
            StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_KIND_TRAIL_SPECIAL_N2);
        } else if magic_kind == thunder {
            StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_KIND_TRAIL_SPECIAL_N3); 
        }
    
        return 1.into();
    } else {
        return smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_SPECIAL_N)(fighter);
    }
}
pub fn install() {
    special_hi_h::install();
    ganon_special_n_float::install();
    littlemac_special_n_cancel::install();
    diddy_special_n_cancel::install();

    smashline::Agent::new("kirby")
        .on_init(kirby_init)
        .on_start(kirby_reset)
        .status(Pre, *FIGHTER_STATUS_KIND_JUMP, pre_jump)
        .status(
            MapCorrection,
            *FIGHTER_STATUS_KIND_THROW_KIRBY,
            throw_kirby_map_correction,
        )
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_pre)
        .install();
}
