// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

#[fighter_frame( agent = FIGHTER_KIND_LUCARIO )]
pub fn lucario_meter(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        MeterModule::update(fighter.object(), false);
        MeterModule::set_meter_cap(fighter.object(), 6);
        MeterModule::set_meter_per_level(fighter.object(), 50.0);
        utils::ui::UiManager::set_aura_meter_enable(fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32, true);
        utils::ui::UiManager::set_aura_meter_info(
            fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32,
            MeterModule::meter(fighter.object()),
            (MeterModule::meter_cap(fighter.object()) as f32 * MeterModule::meter_per_level(fighter.object())),
            MeterModule::meter_per_level(fighter.object()),
            VarModule::is_flag(fighter.object(), vars::lucario::instance::METER_IS_BURNOUT)
        );
    }
}

#[utils::macros::opff(FIGHTER_KIND_LUCARIO )]
pub fn lucario_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		lucario_frame(fighter)
    }
}

pub unsafe fn lucario_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    fastfall_specials(fighter);
    nspecial(fighter, boma, status_kind, situation_kind, cat[1], frame);
    sspecial(fighter, boma, status_kind, situation_kind, cat[1], frame);
    dspecial(fighter, boma, status_kind, situation_kind, cat[1], frame);
    meter_module(fighter, boma, status_kind);
    magic_series(fighter, boma, id, cat, status_kind, situation_kind, motion_kind, stick_x, stick_y, facing, frame);
    training_mode_max_meter(fighter, boma, status_kind);
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_HOLD,
        *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_CANCEL,
        *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_SHOOT,
        *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_MAX,
        *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH_END,
        *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_BOUND,
        *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_LW_END
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
        if fighter.is_flag(*FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
            if [*FIGHTER_KINETIC_TYPE_MOTION_AIR, *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE].contains(&KineticModule::get_kinetic_type(fighter.module_accessor)) {
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
                let speed_y = app::sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);

                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y, 0.0, 0.0, 0.0);
                app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
                
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                app::sv_kinetic_energy::enable(fighter.lua_state_agent);

                KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
            }
        }
    }
}

unsafe fn pause_meter_regen(fighter: &mut L2CFighterCommon, frames: i32) {
    let frames = frames.max(VarModule::get_int(fighter.object(), vars::lucario::instance::METER_PAUSE_REGEN_FRAME));
    VarModule::set_int(fighter.object(), vars::lucario::instance::METER_PAUSE_REGEN_FRAME, frames);
}

unsafe fn training_mode_max_meter(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if app::smashball::is_training_mode()
    && boma.is_status(*FIGHTER_STATUS_KIND_APPEAL)
    && boma.is_button_on(Buttons::Guard)
    {
        let meter_max = (MeterModule::meter_cap(fighter.object()) as f32 * MeterModule::meter_per_level(fighter.object()));
        MeterModule::add(boma.object(), meter_max);
    }
}

unsafe fn nspecial(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32, frame: f32) {

    // aura bomb activation
    // meter is drained in ACMD so that it only happens when projectile is shot
    if status_kind == *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_SHOOT 
    && frame == 8.0
    && fighter.is_flag(*FIGHTER_LUCARIO_SPECIAL_N_STATUS_WORK_ID_FLAG_CHARGE_MAX)
    && fighter.is_button_on(Buttons::SpecialRaw)
    && !VarModule::is_flag(fighter.battle_object, vars::lucario::instance::METER_IS_BURNOUT) {
        if situation_kind == *SITUATION_KIND_GROUND {
            MotionModule::change_motion_inherit_frame(boma, Hash40::new("special_n_bomb"), -1.0, 1.0, 0.0, false, false);
        } else {
            MotionModule::change_motion_inherit_frame(boma, Hash40::new("special_air_n_bomb"), -1.0, 1.0, 0.0, false, false);
        }
        VarModule::on_flag(fighter.battle_object, vars::lucario::instance::IS_POWERED_UP);
        let bonus_aurapower = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "aura.bonus_aurapower");
        VarModule::set_float(fighter.battle_object, vars::lucario::status::AURA_OVERRIDE, bonus_aurapower);
    }

    // float during air aura bomb
    if status_kind == *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_SHOOT 
    && MotionModule::motion_kind(boma) == hash40("special_air_n_bomb")
    && frame < 37.0 {
        KineticModule::mul_speed(boma, &Vector3f::new(0.0, 0.0, 1.0), *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
}

unsafe fn sspecial(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32, frame: f32) {
    // critical hit activation
    if ((MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_s_throw") && frame == 21.0)
    || (MotionModule::motion_kind(fighter.module_accessor) == hash40("special_s_throw") && frame == 26.0))
    && fighter.is_button_on(Buttons::SpecialRaw)
    && !VarModule::is_flag(fighter.battle_object, vars::lucario::instance::METER_IS_BURNOUT) {
        let bonus_aurapower = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "aura.bonus_aurapower");
        VarModule::set_float(fighter.battle_object, vars::lucario::status::AURA_OVERRIDE, bonus_aurapower);
        MeterModule::drain_direct(fighter.battle_object, 2.0 * MeterModule::meter_per_level(fighter.battle_object));
        pause_meter_regen(fighter, 120);
    }
}

unsafe fn dspecial(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32, frame: f32) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_LW)
    && fighter.is_situation(*SITUATION_KIND_GROUND)
    && fighter.is_prev_situation(*SITUATION_KIND_AIR) {
        if CancelModule::is_enable_cancel(fighter.module_accessor)
        || frame > 25.0 {
            fighter.change_status_req(*FIGHTER_STATUS_KIND_LANDING, false);
        } else {
            fighter.set_float(10.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
            fighter.change_status_req(*FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, false);
        }
    }
    if (fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_LW) || (fighter.is_status(*FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR) && fighter.is_prev_status(*FIGHTER_STATUS_KIND_SPECIAL_LW))) 
    && !CancelModule::is_enable_cancel(boma)
    && fighter.is_button_on(Buttons::Attack)
    && !VarModule::is_flag(fighter.object(), vars::lucario::instance::METER_IS_BURNOUT) {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_ATTACK_AIR, false);
        KineticModule::mul_speed(boma, &Vector3f{x: 0.5, y: 0.5, z: 0.5}, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        MeterModule::drain_direct(fighter.object(), MeterModule::meter_per_level(fighter.object()) * 2.0);
        pause_meter_regen(fighter, 120);
    }
}

unsafe fn meter_module(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    let damage_gain_mul = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "aura.damage_meter_gain_mul");
    MeterModule::set_damage_gain_mul(fighter.object(), damage_gain_mul);
    if [ // list of statuses that should pause passive meter regen
        // wallcling
	    *FIGHTER_STATUS_KIND_ATTACH_WALL,
	    *FIGHTER_STATUS_KIND_ATTACH_WALL_WAIT,
        // grabbed
        *FIGHTER_STATUS_KIND_CAPTURE_PULLED,
        *FIGHTER_STATUS_KIND_CAPTURE_PULLED_FISHINGROD,
        *FIGHTER_STATUS_KIND_CAPTURE_PULLED_OCTOPUS,
        *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL,
        *FIGHTER_STATUS_KIND_CAPTURE_PULLED_YOSHI,
        // ledge stuff
        *FIGHTER_STATUS_KIND_CLIFF_ATTACK,
        *FIGHTER_STATUS_KIND_CLIFF_CATCH,
        *FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE,
        *FIGHTER_STATUS_KIND_CLIFF_CLIMB,
        *FIGHTER_STATUS_KIND_CLIFF_ESCAPE,
        *FIGHTER_STATUS_KIND_CLIFF_JUMP1,
        *FIGHTER_STATUS_KIND_CLIFF_JUMP2,
        *FIGHTER_STATUS_KIND_CLIFF_JUMP3,
        *FIGHTER_STATUS_KIND_CLIFF_ROBBED,
        *FIGHTER_STATUS_KIND_CLIFF_WAIT,
        // hit
        *FIGHTER_STATUS_KIND_DAMAGE,
        *FIGHTER_STATUS_KIND_DAMAGE_AIR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
        *FIGHTER_STATUS_KIND_DAMAGE_FALL,
        // dead
        *FIGHTER_STATUS_KIND_DEAD,
        // knockdown
        *FIGHTER_STATUS_KIND_DOWN,
	    *FIGHTER_STATUS_KIND_DOWN_CONTINUE,
	    *FIGHTER_STATUS_KIND_DOWN_DAMAGE,
	    *FIGHTER_STATUS_KIND_DOWN_EAT,
	    *FIGHTER_STATUS_KIND_DOWN_REFLECT_LR,
	    *FIGHTER_STATUS_KIND_DOWN_SPOT,
	    *FIGHTER_STATUS_KIND_DOWN_STAND,
	    *FIGHTER_STATUS_KIND_DOWN_STAND_ATTACK,
	    *FIGHTER_STATUS_KIND_DOWN_STAND_FB,
	    *FIGHTER_STATUS_KIND_DOWN_WAIT,
	    *FIGHTER_STATUS_KIND_DOWN_WAIT_CONTINUE,
        // defensive options
        *FIGHTER_STATUS_KIND_ESCAPE,
        *FIGHTER_STATUS_KIND_ESCAPE_F,
        *FIGHTER_STATUS_KIND_ESCAPE_B,
        *FIGHTER_STATUS_KIND_GUARD, 
        *FIGHTER_STATUS_KIND_GUARD_OFF, 
        *FIGHTER_STATUS_KIND_GUARD_ON,
        *FIGHTER_STATUS_KIND_REBIRTH,
        // shieldbreak
        *FIGHTER_STATUS_KIND_SHIELD_BREAK_DOWN,
        *FIGHTER_STATUS_KIND_SHIELD_BREAK_FALL,
        *FIGHTER_STATUS_KIND_SHIELD_BREAK_FLY,
        *FIGHTER_STATUS_KIND_FURAFURA,
        *FIGHTER_STATUS_KIND_FURAFURA_END,
        *FIGHTER_STATUS_KIND_FURAFURA_STAND,
        // lay down
        *FIGHTER_STATUS_KIND_LAY_DOWN,
        // sleeping
        *FIGHTER_STATUS_KIND_SLEEP,
        *FIGHTER_STATUS_KIND_SLEEP_END,
        *FIGHTER_STATUS_KIND_SLEEP_FALL,
        *FIGHTER_STATUS_KIND_SLEEP_START,
        // slip
        *FIGHTER_STATUS_KIND_SLIP,
        *FIGHTER_STATUS_KIND_SLIP_DAMAGE,
        *FIGHTER_STATUS_KIND_SLIP_STAND,
        *FIGHTER_STATUS_KIND_SLIP_STAND_ATTACK,
        *FIGHTER_STATUS_KIND_SLIP_STAND_B,
        *FIGHTER_STATUS_KIND_SLIP_STAND_F,
        *FIGHTER_STATUS_KIND_SLIP_WAIT,

    ].contains(&status_kind) {
        pause_meter_regen(fighter, 180);
    }

    let meter = MeterModule::meter(fighter.object());
    let meter_per_level = MeterModule::meter_per_level(fighter.object());
    let meter_max = (MeterModule::meter_cap(fighter.object()) as f32) * meter_per_level;
    if (meter <= 0.0) {
        VarModule::on_flag(fighter.battle_object, vars::lucario::instance::METER_IS_BURNOUT);
    } else if (meter >= meter_max) {
        VarModule::off_flag(fighter.battle_object, vars::lucario::instance::METER_IS_BURNOUT);
    }

    if !boma.is_in_hitlag() {
        // Set faster passive regeneration rate in burnout
        let is_burnout = VarModule::is_flag(fighter.battle_object, vars::lucario::instance::METER_IS_BURNOUT);
        if !is_burnout {
            // regular rate
            let rate = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "aura.regen_rate");
            VarModule::set_float(fighter.battle_object, vars::lucario::instance::METER_PASSIVE_RATE, rate);
        } else {
            // burnout rate
            let rate = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "aura.regen_rate_burnout");
            VarModule::set_float(fighter.battle_object, vars::lucario::instance::METER_PASSIVE_RATE, rate);
        }

        let passive_rate = VarModule::get_float(fighter.battle_object, vars::lucario::instance::METER_PASSIVE_RATE);
        let lockout_frame = VarModule::get_int(fighter.battle_object, vars::lucario::instance::METER_PAUSE_REGEN_FRAME);
        if lockout_frame > 0 {
            // decrement passive regen lockout frame
            VarModule::set_int(fighter.battle_object, vars::lucario::instance::METER_PAUSE_REGEN_FRAME, (lockout_frame - 1).max(0));
        } else {
            // add passive regen
            MeterModule::add(boma.object(), passive_rate);
        }
    }
}

unsafe fn magic_series(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    // Dont use magic series if we're already in cancel frames, if we're in hitlag, or if we didn't connect
    if CancelModule::is_enable_cancel(boma) 
    || boma.is_in_hitlag() 
    || !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) {
        return;
    }
    
    // Tilt cancels
    if [
        *FIGHTER_STATUS_KIND_ATTACK, 
        *FIGHTER_STATUS_KIND_ATTACK_DASH,
    ].contains(&status_kind) {
        if boma.is_cat_flag(Cat1::AttackS3) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3,false);
        }
        if boma.is_cat_flag(Cat1::AttackHi3) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI3,false);
        }
        if boma.is_cat_flag(Cat1::AttackLw3) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW3,false);
        }
    }

    // Smash cancels
    if [
        *FIGHTER_STATUS_KIND_ATTACK, 
        *FIGHTER_STATUS_KIND_ATTACK_DASH, 
        *FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
    ].contains(&status_kind) {
        if boma.is_cat_flag(Cat1::AttackS4) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S4_START,true);
        }
        if boma.is_cat_flag(Cat1::AttackHi4) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START,true);
        }
        if boma.is_cat_flag(Cat1::AttackLw4) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START,true);
        }
    }

    // Special cancels
    if [
        *FIGHTER_STATUS_KIND_ATTACK, 
        *FIGHTER_STATUS_KIND_ATTACK_DASH, 
        *FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
        *FIGHTER_STATUS_KIND_ATTACK_S4,
        *FIGHTER_STATUS_KIND_ATTACK_HI4,
        *FIGHTER_STATUS_KIND_ATTACK_LW4,
        *FIGHTER_STATUS_KIND_ATTACK_AIR
    ].contains(&status_kind) {
        if boma.is_cat_flag(Cat1::SpecialN) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N,false);
        }
        if boma.is_cat_flag(Cat1::SpecialS) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S,false);
        }
        if boma.is_cat_flag(Cat1::SpecialHi) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI,false);
        }
        if boma.is_cat_flag(Cat1::SpecialLw) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW,false);
        }
    }
}
