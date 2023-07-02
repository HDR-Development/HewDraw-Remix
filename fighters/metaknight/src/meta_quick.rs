// opff import
use super::*;
use globals::*;

/// pre status for metaquick summon
/// handles initialization
pub unsafe extern "C" fn metaquick_summon_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_NONE as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        false,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        0,
        0,
        0,
        0
    );

    0.into()
}

/// main status loop for metaquick summon
unsafe extern "C" fn metaquick_summon_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    // exit if the animation is not done yet
    if !MotionModule::is_end(fighter.module_accessor) {
        if MotionModule::frame(fighter.module_accessor) >= 30.0 && !CancelModule::is_enable_cancel(fighter.module_accessor) {
            CancelModule::enable_cancel(fighter.module_accessor);
        }
        if fighter.is_situation(*SITUATION_KIND_AIR) && fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        } else if fighter.is_situation(*SITUATION_KIND_GROUND) && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
        return 0.into();
    }

    // if the animation is over, transition to fall or wait
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    } else {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }

    1.into()
}

/// main status for metaquick summon
pub unsafe extern "C" fn metaquick_summon_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // change summon anim depending on LR
    let motion = if PostureModule::lr(fighter.module_accessor) < 0.0 {
        Hash40::new("metaquick_summon_l")
    } else {
        Hash40::new("metaquick_summon_r")
    };

    VarModule::on_flag(fighter.battle_object, vars::metaknight::instance::META_QUICK_PLAY_VC);
    MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.5, false, 0.0, false, false);

    // CancelModule::enable_cancel(fighter.module_accessor);
    // request the screenwide effect
    EffectModule::req_screen(fighter.module_accessor, Hash40::new("bg_metaknight_final"), false, false, false) as u32;
    // clear the speed and unable energies
    KineticModule::clear_speed_all(fighter.module_accessor);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);

    start_meta_quick(fighter, 8 * 60);

    fighter.main_shift(metaquick_summon_main_loop)
}

/// end status for metaquick summon
pub unsafe extern "C" fn metaquick_summon_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    // re-enable energies and remove the screenwide effect
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    EffectModule::remove_screen(fighter.module_accessor, Hash40::new("bg_metaknight_final"), 20);
    0.into()
}

/// handles starting metaquick
unsafe fn handle_start_metaquick(fighter: &mut L2CFighterCommon) {
    if fighter.is_situation(*SITUATION_KIND_GROUND) && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_LW) {
        return;
    }
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        if fighter.is_status_one_of(&[
            *FIGHTER_STATUS_KIND_FALL,
            *FIGHTER_STATUS_KIND_JUMP,
            *FIGHTER_STATUS_KIND_JUMP_AERIAL,
            *FIGHTER_STATUS_KIND_FLY])
        {
            fighter.change_to_custom_status(statuses::metaknight::METAQUICK_SUMMON, false, false);
            MeterModule::drain(fighter.battle_object, 10);
        }
    }
}

/// handles all of the meta quick logic
pub unsafe fn run(fighter: &mut smash::lua2cpp::L2CFighterCommon) {

    if lua_bind::FighterManager::is_result_mode(utils::singletons::FighterManager()) {
        VarModule::set_int(fighter.battle_object, vars::metaknight::instance::META_QUICK_CHARGE_EFFECT_HANDLE, -1);
        VarModule::set_int(fighter.battle_object, vars::metaknight::instance::META_QUICK_EFFECT_HANDLE, -1);
        VarModule::set_int(fighter.battle_object, vars::metaknight::instance::META_QUICK_EFFECT_HANDLE2, -1);
        VarModule::set_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER, 0);
        MeterModule::reset(fighter.battle_object);
    }
    // update MeterModule
    MeterModule::update(fighter.object(), false);
    MeterModule::watch_damage(fighter.object(), true);
    MeterModule::set_damage_gain_mul(fighter.object(), 6.0);
    
    // logic for starting metaquick:
    // if our meter is >= 10, then we want to check for taunt
    // if we are pressing down taunt, then we want to enter metaquick
    // if we are in the air and in an actionable state, we want to enter the metaquick summon status
    // if we are grounded and in a state where we would normally enter down taunt, we want to enter
    // the metaquick summon status.
    // this should be achievable by not draining the meter if we are about to enter down taunt

    if MeterModule::level(fighter.object()) >= 10 {
        if fighter.is_cat_flag(Cat2::AppealLw) {
            handle_start_metaquick(fighter);
        } else if VarModule::get_int(fighter.battle_object, vars::metaknight::instance::META_QUICK_CHARGE_EFFECT_HANDLE) == -1 {
            VarModule::set_int(
                fighter.battle_object,
                vars::metaknight::instance::META_QUICK_CHARGE_EFFECT_HANDLE,
                EffectModule::req_common(fighter.module_accessor, Hash40::new("charge_max"), 0.0) as i32
            );
        }
    }

    // during meta quick, if you deal damage, it should extend meta quick
    if is_meta_quick(fighter) && MeterModule::level(fighter.object()) >= 1 {
        MeterModule::drain(fighter.object(), 1);
        // an additional 0.5 seconds of quick per 5 damage dealt
        VarModule::add_int(fighter.object(), vars::common::instance::GIMMICK_TIMER, 30);
    }
    
    check_apply_speeds(fighter);
    update_meta_quick_timer(fighter);

    // handle the main meta quick logic
    if is_meta_quick(fighter) {
        check_reset(fighter);
        // set the increased jump speed max multiplier for momentum transfer
        VarModule::set_float(fighter.object(), vars::common::instance::JUMP_SPEED_MAX_MUL, 1.5);
    } else {
        kill_quick_effect(fighter);
    
        // set the regular jump speed max multiplier for momentum transfer
        VarModule::set_float(fighter.object(), vars::common::instance::JUMP_SPEED_MAX_MUL, 0.75);

        // if you are initial dash, slow them down slightly
        if fighter.is_status(*FIGHTER_STATUS_KIND_DASH) {
            let motion_vec = Vector3f {
                x: -0.25 * PostureModule::lr(fighter.boma()) * (1.0 - (MotionModule::frame(fighter.boma()) / MotionModule::end_frame(fighter.boma()))),
                y: 0.0, 
                z: 0.0
            };
            KineticModule::add_speed_outside(fighter.boma(), *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
        }
        
    }
}

/// decrement meta quick timer
unsafe fn update_meta_quick_timer(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    let timer = VarModule::get_int(fighter.object(), vars::common::instance::GIMMICK_TIMER);
    if timer > 0 {
        VarModule::dec_int(fighter.object(), vars::common::instance::GIMMICK_TIMER);
    }
    // do nothing
}

/// handle speed application
unsafe fn check_apply_speeds(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    
    // handle speed application once
    if VarModule::is_flag(fighter.object(), vars::metaknight::instance::META_QUICK_NEED_SET_SPEEDS) {
        if VarModule::get_int(fighter.object(), vars::common::instance::GIMMICK_TIMER) > 0 {
            apply_status_speed_mul(fighter, 1.25);
        } else {
            apply_status_speed_mul(fighter, 0.95);
        }
        VarModule::off_flag(fighter.object(), vars::metaknight::instance::META_QUICK_NEED_SET_SPEEDS);
    }

    //println!("current status: {}", fighter.status());
    //println!("meta quick status: {}", VarModule::get_int(fighter.object(), vars::metaknight::instance::META_QUICK_STATUS));
    // if our status is changing, then we need to indicate that next frame we will need to set new speeds
    if fighter.status() != VarModule::get_int(fighter.object(), vars::metaknight::instance::META_QUICK_STATUS) {
        //println!("Status is changing!");
        VarModule::on_flag(fighter.object(), vars::metaknight::instance::META_QUICK_NEED_SET_SPEEDS);
        VarModule::set_int(fighter.object(), vars::metaknight::instance::META_QUICK_STATUS, fighter.status());
        //println!("new meta quick status: {}", VarModule::get_int(fighter.object(), vars::metaknight::instance::META_QUICK_STATUS));
    }
}

/// checks if meta quick should have its state reset due to death or match end
unsafe fn check_reset(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    // we dont want meta quick *or* the ready effect to persist during these states
    if fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_WIN,
        *FIGHTER_STATUS_KIND_LOSE,
        *FIGHTER_STATUS_KIND_ENTRY,]) {
        VarModule::set_int(fighter.object(), vars::common::instance::GIMMICK_TIMER, 0);
        MeterModule::reset(fighter.object());
    }

    // we don't want meta quick to persist during any of these states
    if fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_REBIRTH]) {
        VarModule::set_int(fighter.object(), vars::common::instance::GIMMICK_TIMER, 0);
    }
}

/// check if meta quick is currently running
pub unsafe fn is_meta_quick(fighter: &mut smash::lua2cpp::L2CFighterCommon) -> bool {
    let timer = VarModule::get_int(fighter.object(), vars::common::instance::GIMMICK_TIMER);
    if timer > 0 {
        return true;
    }
    return false;
}

/// start meta quick
/// length: how many frames meta quick should be active
pub unsafe fn start_meta_quick(fighter: &mut smash::lua2cpp::L2CFighterCommon, length: i32) {
    VarModule::set_int(fighter.object(), vars::common::instance::GIMMICK_TIMER, length);

    // indicate that we will need to set the status speeds next frame
    VarModule::on_flag(fighter.object(), vars::metaknight::instance::META_QUICK_NEED_SET_SPEEDS);
}


/// remove the effect indicating that meta quick is currently active, if it exists
unsafe fn kill_quick_effect(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    let mut did_kill = false;
    let handle = VarModule::get_int(fighter.battle_object, vars::metaknight::instance::META_QUICK_EFFECT_HANDLE);
    if handle != -1 {
        EffectModule::kill(fighter.module_accessor, handle as _, false, false);
        VarModule::set_int(fighter.battle_object, vars::metaknight::instance::META_QUICK_EFFECT_HANDLE, -1);
        did_kill = true;
    }

    let handle2 = VarModule::get_int(fighter.battle_object, vars::metaknight::instance::META_QUICK_EFFECT_HANDLE2);
    if handle2 != -1 {
        EffectModule::kill(fighter.module_accessor, handle2 as _, false, false);
        VarModule::set_int(fighter.battle_object, vars::metaknight::instance::META_QUICK_EFFECT_HANDLE2, -1);
        did_kill = true;
    }

    if did_kill {
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

/// applies the given multiplier to various speed stats of the given fighter. 
/// This should only be called once per status, or you will get some multiplicative effects
unsafe fn apply_status_speed_mul(fighter: &mut smash::lua2cpp::L2CFighterCommon, mul: f32) {
    // set the X motion speed multiplier (where movement is baked into an anim)
    lua_bind::FighterKineticEnergyMotion::set_speed_mul(fighter.get_motion_energy(), mul);

    // set the X motion accel multiplier for control energy (used in the air, during walk, fall, etc)
    lua_bind::FighterKineticEnergyController::mul_x_accel_mul( fighter.get_controller_energy(), mul);

    // set the X motion accel multiplier for control energy (used in the air, during walk, fall, etc)
    lua_bind::FighterKineticEnergyController::mul_x_accel_add( fighter.get_controller_energy(), mul);

    // set the X speed max multiplier for control energy (used in the air, during walk, fall, etc)
    lua_bind::FighterKineticEnergyController::mul_x_speed_max(fighter.get_controller_energy(), mul);
}

pub fn install() {
    CustomStatusManager::add_new_agent_status_script(
        Hash40::new("fighter_kind_metaknight"),
        statuses::metaknight::METAQUICK_SUMMON,
        StatusInfo::new()
            .with_pre(metaquick_summon_pre)
            .with_main(metaquick_summon_main)
            .with_end(metaquick_summon_end)
    );
}