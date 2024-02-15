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


//#[skyline::from_offset(0xc45530)]
//pub unsafe fn update_ko_ui(arg1: f32, arg2: f32, arg3: *mut Fighter);

// static mut num: u32 = 0;


// pub fn offset_to_addr(offset: usize) -> *const () {
//     unsafe { (getRegionAddress(Region::Text) as *const u8).add(offset) as _ }
// }

// #[skyline::from_offset(0x68cd80)]
// pub fn update_battle_ui(x: *const u64, y: u32);


// pub fn update_little_mac_meter(player_id: u8, val: u32){
//     unsafe {
//         let player_id = player_id as u64;
//         let off = **(offset_to_addr(0x52b74f8) as *const *const u64);
//         let ptr = (off + (player_id * 8) + 0x20) as *const u64;
//         let res = (*ptr + 0x41e4) as *const u64;
//         update_battle_ui(res, val);
//     }
// }

// #[fighter_frame( agent = FIGHTER_KIND_LITTLEMAC )]
// fn mac_frame(fighter: &mut L2CFighterCommon) {
//     unsafe {
//         num += 1;
//         update_little_mac_meter(0, num);
//         if num > 100 {
//             num = 0;
//         }
//     }
// }

// #[skyline::main(name = "little_mac_test")]
// pub fn main() {
//     smashline::install_agent_frames!(
//         mac_frame
//     );
// }


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
    );
}