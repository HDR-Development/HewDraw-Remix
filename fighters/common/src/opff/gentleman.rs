use utils::{
    *,
    ext::*,
    consts::*
};
use smash::app::BattleObjectModuleAccessor;
use smash::phx::{Vector2f, Vector3f};
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::hash40;
use smash::phx::Hash40;

unsafe fn gentleman(boma: &mut BattleObjectModuleAccessor, status_kind: i32, fighter_kind: i32) {
    let mut jab_finisher_start = 1.0; // Start of window for rapid jab finisher transition, determined by character
    let mut jab_finisher_end = 1.0;   // End of window for rapid jab finisher transition, determined by character

    if [*FIGHTER_KIND_KIRBY, *FIGHTER_KIND_FOX, *FIGHTER_KIND_SHEIK, *FIGHTER_KIND_FALCO, *FIGHTER_KIND_MURABITO, *FIGHTER_KIND_KOOPAJR, *FIGHTER_KIND_SIMON, *FIGHTER_KIND_MIIFIGHTER].contains(&fighter_kind) {
        jab_finisher_start = 6.0;
        jab_finisher_end = 14.0;
    } else if fighter_kind == *FIGHTER_KIND_ZELDA {
        jab_finisher_start = 9.0;
        jab_finisher_end = 19.0;
    } else if fighter_kind == *FIGHTER_KIND_MEWTWO {
        jab_finisher_start = 9.0;
        jab_finisher_end = 17.0;
    } else if fighter_kind == *FIGHTER_KIND_PFUSHIGISOU {
        jab_finisher_start = 7.0;
        jab_finisher_end = 15.0;
    } else if fighter_kind == *FIGHTER_KIND_GAMEWATCH {
        jab_finisher_start = 7.0;
        jab_finisher_end = 14.0;
    } else if fighter_kind == *FIGHTER_KIND_DEDEDE {
        jab_finisher_start = 17.0;
        jab_finisher_end = 25.0;
    } else if fighter_kind == *FIGHTER_KIND_PALUTENA {
        jab_finisher_start = 14.0;
        jab_finisher_end = 22.0;
    }
    if status_kind == FIGHTER_STATUS_KIND_ATTACK {
        if [*FIGHTER_KIND_ZELDA, *FIGHTER_KIND_MEWTWO, *FIGHTER_KIND_PALUTENA].contains(&fighter_kind) {
            if MotionModule::motion_kind(boma) == hash40("attack_11") {
                if MotionModule::frame(boma) > jab_finisher_start && MotionModule::frame(boma) < jab_finisher_end {
                    if boma.is_pad_flag(PadFlag::SpecialTrigger) {
                        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
                        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
                        MotionModule::change_motion(boma, Hash40::new("attack_100_end"), 0.0, 1.0, false, 0.0, false, false);
                        MotionModule::set_rate(boma, 1.25);
                    }
                }
            }
        } else if [*FIGHTER_KIND_KIRBY, *FIGHTER_KIND_FOX, *FIGHTER_KIND_SHEIK, *FIGHTER_KIND_FALCO, *FIGHTER_KIND_KOOPAJR, *FIGHTER_KIND_SIMON, *FIGHTER_KIND_MIIFIGHTER, *FIGHTER_KIND_DEDEDE, *FIGHTER_KIND_PFUSHIGISOU, *FIGHTER_KIND_MURABITO].contains(&fighter_kind) {
            if MotionModule::motion_kind(boma) == hash40("attack_12") {
                if MotionModule::frame(boma) > jab_finisher_start && MotionModule::frame(boma) < jab_finisher_end {
                    if boma.is_pad_flag(PadFlag::SpecialTrigger) {
                        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
                        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
                        if fighter_kind == *FIGHTER_KIND_MURABITO {
                            MotionModule::change_motion(boma, Hash40::new("attack_12_end"), 0.0, 1.0, false, 0.0, false, false);
                        }
                        else{
                            MotionModule::change_motion(boma, Hash40::new("attack_100_end"), 0.0, 1.0, false, 0.0, false, false);
                        }
                        MotionModule::set_rate(boma, 1.25);
                    }
                }
            }
        } else if fighter_kind == *FIGHTER_KIND_GAMEWATCH {
            if MotionModule::motion_kind(boma) == hash40("attack_11") {
                if MotionModule::frame(boma) > jab_finisher_start && MotionModule::frame(boma) < jab_finisher_end {
                    if boma.is_pad_flag(PadFlag::AttackTrigger) {
                        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
                        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
                        MotionModule::change_motion(boma, Hash40::new("attack_100_end"), 0.0, 1.0, false, 0.0, false, false);
                        MotionModule::set_rate(boma, 1.25);
                    }
                }
            }
        }
    }
}

pub unsafe fn run(boma: &mut BattleObjectModuleAccessor, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, fighter_kind: i32, stick_x: f32, stick_y: f32, facing: f32) {
    gentleman(boma, status_kind, fighter_kind);
}
