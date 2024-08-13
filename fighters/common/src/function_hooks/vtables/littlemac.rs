use super::*;
use utils::ext::*;

#[skyline::hook(offset = 0xc456a0)]
pub unsafe extern "C" fn hook_ko_meter_gain(vtable: u64, battle_object: *mut BattleObject, collisionLog: CollisionLog, damage: f32) -> u64 {
    let boma = (&mut *(battle_object)).boma();
    let opponent_boma = &mut *(sv_battle_object::module_accessor(collisionLog.opponent_battle_object_id));
    //if HitModule::get_status(opponent_boma, collisionLog.receiver_part_id as i32, 0) != 0 { return call_original!(vtable, battle_object, collisionLog, 0.0); }
    let mut meter_gain = 5.0;

    // this effectively stubs the logic handling critical zoom unless at full meter
    if boma.is_status(*FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N2) {
        let meter = WorkModule::get_float(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE);
        if meter != 100.0 {
            let size = if meter < 40.0 { 0.5 } else { 0.7 };
            EffectModule::req_on_joint(boma, Hash40::new("sys_hit_normal_l"), Hash40::new("handr"), &Vector3f::zero(), &Vector3f::zero(), 0.8, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0);
            return 1
        }
    }

    // modify meter gain based on opponent action
    if opponent_boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_DAMAGE,
        *FIGHTER_STATUS_KIND_DAMAGE_AIR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
        *FIGHTER_STATUS_KIND_DAMAGE_FALL]) {
        meter_gain = 10.0;
    }
    if opponent_boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD,
        *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD,
        *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD]) {
        EffectModule::req_on_joint(boma, Hash40::new("sys_sp_flash"), Hash40::new("top"), &Vector3f::new(0.0, 15.0, 4.0), &Vector3f::new(0.0, 0.0, 0.0), 1.0, &Vector3f::zero(), &Vector3f::zero(), true, 0, 0, 0);
        meter_gain = 30.0;
    }
    if opponent_boma.is_status(*FIGHTER_STATUS_KIND_APPEAL) {
        if TeamModule::hit_team_no(boma) != TeamModule::hit_team_no(opponent_boma) {
            SlowModule::set_whole(boma, 4, 60);
            EffectModule::req_on_joint(boma, Hash40::new("sys_hit_dead"), Hash40::new("bust"), &Vector3f::new(0.0, 0.0, 0.0), &Vector3f::new(0.0, 0.0, 0.0), 0.6, &Vector3f::zero(), &Vector3f::zero(), true, 0, 0, 0);
            SoundModule::play_se_no3d(boma, Hash40::new("vc_littlemac_appeal05"), true, true);
            SoundModule::play_se_no3d(boma, Hash40::new("vc_littlemac_missfoot02"), true, true);
            meter_gain = 100.0;
        }
    }

    // modify meter based on the attack used
    if boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_ATTACK_100,
        *FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_HI_JUMP,
        *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_HI_START,
        *FIGHTER_STATUS_KIND_ATTACK]) {
        if !VarModule::is_flag(boma.object(), vars::littlemac::status::LIMIT_METER_GAIN) {
            VarModule::on_flag(boma.object(), vars::littlemac::status::LIMIT_METER_GAIN);
        }
        else {
            meter_gain *= if boma.is_status(*FIGHTER_STATUS_KIND_ATTACK_100) { 0.1 }
            else if boma.is_status_one_of(&[
                *FIGHTER_STATUS_KIND_SPECIAL_HI,
                *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_HI_JUMP,
                *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_HI_START
            ]) { 0.2 } else { 0.5 };
        }
        if boma.is_motion(Hash40::new("attack_13"))
        && VarModule::is_flag(boma.object(), vars::littlemac::instance::IS_DREAMLAND_EXPRESS) {
            EffectModule::req_on_joint(boma, Hash40::new("sys_hit_magic"), Hash40::new("handl"), &Vector3f::zero(), &Vector3f::zero(), 0.4, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0);
            meter_gain += 10.0;
        }
    }
    if boma.is_status(*FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_S_JUMP) {
        meter_gain = 0.0;
    }
    if boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_ATTACK_S4,
        *FIGHTER_STATUS_KIND_ATTACK_HI4,
        *FIGHTER_STATUS_KIND_ATTACK_LW4]) {
        meter_gain *= 2.0;
    }

    // Example on how to call update_littlemac_ui

    // let meter = WorkModule::get_float(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE);
    // println!("Current Meter: {}", meter);
    // println!("Gained Meter: {}", meter_gain);
    // println!();

    // let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    // update_littlemac_ui(entry_id, meter + meter_gain);

    call_original!(vtable, battle_object, collisionLog, meter_gain)
}

pub fn install() {
    skyline::install_hooks!(
        hook_ko_meter_gain,
    );
}