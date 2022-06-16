use super::*;
use globals::*;

pub trait Vector3fExt {
    fn mag(&self) -> f32;
    fn normalize(&self) -> Self;
  }
  
  impl Vector3fExt for Vector3f {
    fn mag(&self) -> f32 {
      (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
    fn normalize(&self) -> Self {
      let mag = self.mag();
      Self {
        x: self.x / mag,
        y: self.y / mag,
        z: self.z / mag
      }
    }
  }

//=================================================================
//== StatusModule::init_settings
//=================================================================
#[skyline::hook(replace=StatusModule::init_settings)]
unsafe fn init_settings_hook(boma: &mut BattleObjectModuleAccessor, situation: smash::app::SituationKind, arg3: i32, arg4: u32,
                             ground_cliff_check_kind: smash::app::GroundCliffCheckKind, jostle: bool,
                             keep_flag: i32, keep_int: i32, keep_float: i32, arg10: i32) -> u64 {
    let id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let fighter_kind = boma.kind();
    let status_kind = StatusModule::status_kind(boma);
    let situation_kind = StatusModule::situation_kind(boma);
                                
    // Call edge_slippoffs init_settings
    let fix = super::edge_slipoffs::init_settings_edges(boma, situation, arg3, arg4, ground_cliff_check_kind, jostle, keep_flag, keep_int, keep_float, arg10);

    if boma.is_fighter() {

        // Disable wiggle out of tumble flag during damage_fly states
        if [*FIGHTER_STATUS_KIND_DAMAGE_FLY,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL].contains(&status_kind) {
            VarModule::off_flag(boma.object(), vars::common::CAN_ESCAPE_TUMBLE);
        }

        // ken and ryu airdash effect
        if [*FIGHTER_KIND_KEN, *FIGHTER_KIND_RYU].contains(&fighter_kind) && status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
            if StatusModule::prev_status_kind(boma, 0) != *FIGHTER_STATUS_KIND_JUMP_SQUAT {
                let mut stick_value_x = ControlModule::get_stick_x(boma);
                let stick_value_y = ControlModule::get_stick_y(boma);
                let lr =  PostureModule::lr(boma);
                if (stick_value_x.abs() > 0.66 || stick_value_y.abs() > 0.66) {
                    if (stick_value_x == 0.0) {
                        stick_value_x = 0.001; // to avoid a divide by zero on the next instruction below
                    }
                    let angle = (stick_value_y/stick_value_x).atan();
                    let norm_stick_pos = Vector3f { x: stick_value_x * -1.0 * lr, y: stick_value_y * -1.0, z: 0.0}.normalize();
                    let pos1 = Vector3f { x: norm_stick_pos.x * 4.0, y: 8.0 + norm_stick_pos.y * 8.0, z: 0.};
                    let pos2 = Vector3f { x: norm_stick_pos.x * 9.0, y: 8.0 + norm_stick_pos.y * 12.0, z: 0.};
                    let rot = Vector3f { x:5.0, y:0.0, z: ((stick_value_x.signum() * 90.0) + 180. * angle/3.14159)};
                    let mut effect_hash = Hash40::new("sys_whirlwind_l");
                    if stick_value_x >= 0. {
                        effect_hash = Hash40::new("sys_whirlwind_r");
                    }

                    EffectModule::req_on_joint(boma, effect_hash, Hash40::new("top"),
                    &pos1, &rot, 0.75, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, false, 0, 0, 0);
                    EffectModule::req_on_joint(boma, effect_hash, Hash40::new("top"),
                    &pos2, &rot, 0.40, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, false, 0, 0, 0);
                }
            }
        }


        VarModule::off_flag(boma.object(), vars::common::B_REVERSED);

        // Walk through other fighters
        JostleModule::set_team(boma, 0);

        // clear platform drop input when entering airdodge (to avoid buffering waveland platdrop with the same down input as the actual waveland)
        if [*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE, *FIGHTER_STATUS_KIND_JUMP_SQUAT].contains(&status_kind) {
            VarModule::off_flag(boma.object(), vars::common::ENABLE_WAVELAND_PLATDROP);
        }

        // Repeated tilt scaling; UNUSED
        /*
        if [*FIGHTER_KIND_RYU, *FIGHTER_KIND_KEN, *FIGHTER_KIND_DOLLY].contains(&fighter_kind) {
            VarModule::off_flag(boma.object(), vars::common::REPEAT_INCREMENTED);
            if status_kind != *FIGHTER_STATUS_KIND_ATTACK_HI3 {
                if VarModule::get_int(boma.object(), vars::common::REPEAT_NUM_HI) > 0 {
                    VarModule::set_int(boma.object(), vars::common::REPEAT_NUM_HI, 0);
                }
            }
            if ![*FIGHTER_STATUS_KIND_ATTACK_LW3,
                *FIGHTER_STATUS_KIND_SQUAT,
                *FIGHTER_STATUS_KIND_SQUAT_WAIT].contains(&status_kind) {
                    if VarModule::get_int(boma.object(), vars::common::REPEAT_NUM_LW) > 0 {
                        VarModule::set_int(boma.object(), vars::common::REPEAT_NUM_LW, 0);
                    }
            }
        }
        */

        //Sword trails
        if (boma.kind() == *FIGHTER_KIND_ROY || boma.kind() == *FIGHTER_KIND_CHROM) 
        && VarModule::is_flag(boma.object(), vars::roy::TRAIL_EFFECT) {
            EffectModule::kill_joint_id(boma, Hash40::new("sword1"), false, false);
            if fighter_kind == *FIGHTER_KIND_ROY {
                EffectModule::req_follow(boma, Hash40::new("roy_fire_small"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 1.0, false, 0, 0, 0, 0, 0, false, false);
            }
            else if fighter_kind == *FIGHTER_KIND_CHROM {
                EffectModule::req_follow(boma, Hash40::new("chrom_sword"), Hash40::new("sword1"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 1.0, false, 0, 0, 0, 0, 0, false, false);
            }

            if [*FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_ENTRY].contains(&status_kind) {
                VarModule::off_flag(boma.object(), vars::roy::TRAIL_EFFECT);
                EffectModule::kill_joint_id(boma, Hash40::new("sword1"), false, false);
            }
        }

    }

    // VarModule Status Variable reset checks
    // This makes the assumption that if the KEEP_FLAG is not NONE, you want to clear the
    // status variable array for that data type. Because Smash shares its space between
    // INT and INT64, I have included both of them under a single check.
    let mut mask = 0;
    if keep_flag == *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG {
        mask += VarModule::RESET_STATUS_FLAG;
    }
    if keep_int == *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT {
        mask += VarModule::RESET_STATUS_INT;
        mask += VarModule::RESET_STATUS_INT64;
    }
    if keep_float == *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT {
        mask += VarModule::RESET_STATUS_FLOAT;
    }
    let object = boma.object();
    VarModule::reset(object, mask);

    original!()(boma, situation, arg3, fix, ground_cliff_check_kind, jostle, keep_flag, keep_int, keep_float, arg10)
}

pub fn install() {
    skyline::install_hooks!(
        init_settings_hook,
    );
}
