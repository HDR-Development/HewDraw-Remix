use super::*;

pub struct CollisionLog {
    next: *mut CollisionLog,
    end: *mut CollisionLog,
    location: Vector3f,
    padding_0: u32,
    padding_1: u32,
    opponent_battle_object_id: u32,
    padding_2: [u8;7],
    collision_kind: u8,
    receiver_part_id: u8,
    collider_part_id: u8,
    receiver_id: u8,
    collider_id: u8,
    padding_3: [u8;10]
}

#[skyline::hook(offset = 0xc45680)]
pub unsafe extern "C" fn hook_ko_meter_gain(vtable: u64, battle_object: *mut BattleObject, collisionLog: CollisionLog, damage: f32) -> u64 {
    let boma = (&mut *(battle_object)).boma();
    let opponent_boma = &mut *(sv_battle_object::module_accessor(collisionLog.opponent_battle_object_id));
    let mut meter_gain = 5.0;

    // this effectively stubs the logic handling critical zoom unless at full meter
    if boma.is_status(*FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N2) {
        let meter = WorkModule::get_float(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE);
        if meter != 100.0 {
            let size = if meter < 30.0 { 0.5 } else { 0.7 };
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
        //let eff_handle = EffectModule::req_follow(boma, Hash40::new("sys_starrod_bullet"), Hash40::new("top"), &Vector3f::new(0.0, 15.0, 0.0), &Vector3f::new(0.0, 0.0, 90.0), 0.8, false, 0, 0, 0, 0, 0, false, false);
        //EffectModule::set_rate(boma, eff_handle, 0.0);
        EffectModule::req_follow(boma, Hash40::new("sys_flash"), Hash40::new("top"), &Vector3f::new(0.0, 15.0, 6.0), &Vector3f::zero(), 0.8, false, 0, 0, 0, 0, 0, false, false);
        meter_gain = 30.0;
    }
    if opponent_boma.is_status(*FIGHTER_STATUS_KIND_APPEAL) {
        meter_gain = 100.0;
    }

    // modify meter based on the attack used
    if boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_ATTACK_100,
        *FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_STATUS_KIND_ATTACK]) {
        if !VarModule::is_flag(boma.object(), vars::littlemac::status::LIMIT_METER_GAIN) {
            VarModule::on_flag(boma.object(), vars::littlemac::status::LIMIT_METER_GAIN);
        }
        else {
            meter_gain *= if boma.is_status(*FIGHTER_STATUS_KIND_ATTACK_100) { 0.1 }
            else if boma.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI) { 0.2 } else { 0.5 };
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

    //println!("Current Meter: {}", WorkModule::get_float(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE));
    //println!("Gained Meter: {}", meter_gain);
    //println!();

    call_original!(vtable, battle_object, collisionLog, meter_gain)
}

pub fn install() {
    skyline::install_hooks!(
        hook_ko_meter_gain,
    );
}