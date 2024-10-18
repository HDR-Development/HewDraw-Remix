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
        EffectModule::req_on_joint(opponent_boma, Hash40::new("sys_hit_magic"), Hash40::new("hip"), &Vector3f::new(0.0, 2.0, 0.0), &Vector3f::zero(), 0.45, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0);
        SoundModule::play_se_no3d(boma, Hash40::new("se_ingame_reach"), true, true);
        // se_ingame_reach, se_gohoubi_icon_spirits
        let slow_frame = SlowModule::frame(opponent_boma, 0);
        if slow_frame < 10 {
            SlowModule::set(boma, 0, 2, 10, false, 0x50000000);
            SlowModule::set(opponent_boma, 0, 2, 10, false, 0x50000000);
        }
        meter_gain = 30.0;
    }
    if opponent_boma.is_status(*FIGHTER_STATUS_KIND_APPEAL) {
        if FighterUtil::get_team_color(boma) != FighterUtil::get_team_color(opponent_boma) {
            let slow_frame = SlowModule::frame(opponent_boma, 0);
            if slow_frame < 60 {
                SlowModule::set(boma, 0, 4, 60, false, 0x50000000);
                SlowModule::set(opponent_boma, 0, 4, 60, false, 0x50000000);
            }
            let team_color = FighterUtil::get_team_color(boma);
            let mut effect_team_color = FighterUtil::get_effect_team_color(EColorKind(team_color as i32), Hash40::new("direction_effect_color"));
            let effect = EffectModule::req_on_joint(opponent_boma, Hash40::new("sys_hit_dead"), Hash40::new("hip"), &Vector3f::new(0.0, 2.0, 0.0), &Vector3f::new(0.0, 0.0, 90.0), 0.6, &Vector3f::zero(), &Vector3f::zero(), true, 0, 0, 0);
            EffectModule::set_rgb(boma, effect as u32, effect_team_color.value[0], effect_team_color.value[1], effect_team_color.value[2]);
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
        && VarModule::is_flag(boma.object(), vars::littlemac::instance::ATTACK_13_DREAMLAND_EXPRESS) {
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

#[skyline::hook(offset = 0xc463d0)]
pub unsafe extern "C" fn littlemac_on_search(vtable: u64, fighter: &mut Fighter, log: u64, some_float: f32) -> u64 {
    // some_float seems to always be 1
    let object = &mut fighter.battle_object;
    let boma = &mut *(*object).module_accessor;
    if boma.is_status(*FIGHTER_STATUS_KIND_SPECIAL_LW) {
        // let collision_log = *(log as *const u64).add(0x10 / 0x8);
        // let collision_log = collision_log as *const CollisionLog;
        // let opponent_object = utils::util::get_battle_object_from_id((*collision_log).opponent_battle_object_id);
        // let opponent_boma = (*opponent_object).module_accessor;
        // let slow_frame = SlowModule::frame(opponent_boma, 0);
        // if slow_frame < 20 {
        //     SlowModule::set(opponent_boma, 0, 2, 20, false, 0x50000000);
        // }
        let meter = WorkModule::get_float(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE);
        let counter_meter_gain = ParamModule::get_float(boma.object(), ParamType::Agent, "power_meter.counter_meter_gain");
        let meter_inc = (meter + counter_meter_gain).clamp(0.0, 100.0);
        let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        update_littlemac_ui(entry_id, meter_inc);
        WorkModule::set_float(boma, meter_inc, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE);
    }

    return original!()(vtable, fighter, log, some_float);
}

pub unsafe extern "C" fn update_littlemac_ui(entry_id: i32, total_gauge: f32) {
    let manager = singletons::FighterManager() as *mut u64;
    let offset = (*manager + (entry_id as u64 * 8) + 0x20) as *mut u64;
    update_littlemac_ui_internal((*offset + 0x41e4) as *mut u32, total_gauge as i32);
}

#[skyline::from_offset(0x68cda0)]
fn update_littlemac_ui_internal(manager_offset: *mut u32, total_gauge: i32);

pub fn install() {
    skyline::install_hooks!(
        hook_ko_meter_gain,
        littlemac_on_search
    );
}