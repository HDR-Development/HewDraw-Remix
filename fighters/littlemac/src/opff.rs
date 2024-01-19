// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn handle_ko_meter_decrement(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if !sv_information::is_ready_go()
    || boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_REBIRTH,
        *FIGHTER_STATUS_KIND_ENTRY]) {
        VarModule::set_float(boma.object(), vars::littlemac::instance::CURRENT_DAMAGE, 0.0);
        VarModule::off_flag(boma.object(), vars::littlemac::instance::HIT_INIT);
    }
    let damage_statuses = [
        *FIGHTER_STATUS_KIND_DAMAGE,
        *FIGHTER_STATUS_KIND_DAMAGE_AIR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR];
    if damage_statuses.contains(&status_kind) {
        if StatusModule::is_changing(boma) && !VarModule::is_flag(boma.object(), vars::littlemac::instance::HIT_INIT) {
            //println!();
            VarModule::on_flag(boma.object(), vars::littlemac::instance::HIT_INIT);
            //println!("HIT!");
            let damage = DamageModule::damage(boma, 0);
            let dec = damage - VarModule::get_float(boma.object(), vars::littlemac::instance::CURRENT_DAMAGE);
            let meter = WorkModule::get_float(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE);
            //println!("old meter: {}", meter);
            //println!("dec: {}", dec);
            if meter == 100.0 {
                WorkModule::set_int(boma, 0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_KO_GAGE_MAX_KEEP_FRAME);
                WorkModule::off_flag(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_REQUEST_KO_GAUGE_MAX_EFFECT);
            }
            WorkModule::set_float(boma, (meter - dec).clamp(0.0, 100.0), *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE);
            VarModule::set_float(boma.object(), vars::littlemac::instance::CURRENT_DAMAGE, damage);
            //println!("new damage: {}", VarModule::get_float(boma.object(), vars::littlemac::instance::CURRENT_DAMAGE));
            //println!("new meter: {}", WorkModule::get_float(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE));
        }
        // also handle side special once-per-airtime
        if WorkModule::is_flag(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_S) {
            WorkModule::off_flag(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_S);
        }
    }
    else {
        if boma.is_prev_status_one_of(&damage_statuses)
        && !boma.is_status_one_of(&damage_statuses)
        && StatusModule::is_changing(boma) {
            VarModule::off_flag(boma.object(), vars::littlemac::instance::HIT_INIT);
            //println!("HITNT!");
        }
    }
}

// Tech Roll distance help
unsafe fn tech_roll_help(boma: &mut BattleObjectModuleAccessor, motion_kind: u64, facing: f32, frame: f32) {
    if frame < 6.0 {
        let mut motion_vec = Vector3f{x: 1.75, y: 0.0, z: 0.0};
        if motion_kind == hash40("passive_stand_f") {
            motion_vec.x *= facing;
        } else if motion_kind == hash40("passive_stand_b") {
            motion_vec.x *= -facing;
        } else {
            return; // Break out if not passive_stand_x
        }
        KineticModule::add_speed_outside(boma, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
    }
}

// Fixes weird vanilla behavior where touching ground during upB puts you into special fall for 1f before landing
unsafe fn up_special_proper_landing(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_HI_JUMP)
    && fighter.is_situation(*SITUATION_KIND_GROUND)
    && MotionModule::frame(fighter.module_accessor) >= 28.0 {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, false);
        KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
}

unsafe fn dreamland_express(fighter: &mut L2CFighterCommon) {
    if fighter.is_motion(Hash40::new("attack_12"))
    && (18.0..20.0).contains(&fighter.motion_frame())
    && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
        VarModule::on_flag(fighter.battle_object, vars::littlemac::instance::IS_DREAMLAND_EXPRESS);
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N2,
        *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_S_BLOW_END,
        *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_S_JUMP_END,
        *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_HI_JUMP,
        *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_LW_HIT
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

unsafe fn training_mode_meter(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    if app::smashball::is_training_mode()
    && boma.is_status(*FIGHTER_STATUS_KIND_APPEAL)
    && boma.is_button_trigger(Buttons::Guard) {
        let meter = WorkModule::get_float(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE);
        let meter_inc = (meter + 40.0).clamp(0.0, 100.0);
        WorkModule::set_float(boma, meter_inc, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE);
        EffectModule::req_on_joint(boma, Hash40::new("sys_flash"), Hash40::new("top"), &Vector3f::new(6.0, 15.0, 0.0), &Vector3f::zero(), 0.4, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0);
        //println!("meter_inc: {}", meter_inc);
        //println!("meter: {}", meter);
        //vtable_hook::update_ko_ui(meter_inc, meter, fighter.global_table[0x4].get_ptr() as *mut Fighter);
    }
}

pub unsafe fn moveset(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    handle_ko_meter_decrement(boma, status_kind);
    tech_roll_help(boma, motion_kind, facing, frame);
    up_special_proper_landing(fighter);
    dreamland_express(fighter);
    fastfall_specials(fighter);
    training_mode_meter(fighter, boma);
}

#[utils::macros::opff(FIGHTER_KIND_LITTLEMAC )]
pub fn littlemac_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		littlemac_frame(fighter)
    }
}

pub unsafe fn littlemac_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}