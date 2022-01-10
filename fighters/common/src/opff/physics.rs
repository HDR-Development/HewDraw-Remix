use crate::opff_import::*;
use smash::app::BattleObjectModuleAccessor;
use smash::phx::{Vector2f, Vector3f};
use smash::lib::{lua_const::*, L2CValue, L2CAgent};
use smash::app::{self, lua_bind::*, sv_kinetic_energy, sv_animcmd};
use smash::lua2cpp::L2CFighterCommon;
use smash::hash40;
use smash::phx::Hash40;
use smash_script::{self, *, macros::*};


//=================================================================
//== ECB ADJUSTMENTS
//== Note: Resetting while a fighter is in air in training mode
//         causes said fighters to drop through the floor
//=================================================================
unsafe fn ecb_shifts(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, fighter_kind: i32) {
    let mut offset: Vector2f = Vector2f {x: 0., y: 0.};

    let mut max_offset: f32 = 0.;

    let prev_status_kind: i32 = StatusModule::prev_status_kind(boma, 0);

    // Statuses for regular ECB
    let vanilla_ecb = ([
                        *FIGHTER_STATUS_KIND_CAPTURE_PULLED,
                        *FIGHTER_STATUS_KIND_CAPTURE_WAIT,
                        *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE,
                        *FIGHTER_STATUS_KIND_CAPTURE_CUT,
                        *FIGHTER_STATUS_KIND_THROWN
                       ].contains(&prev_status_kind))
                    || ([
                        *FIGHTER_STATUS_KIND_CAPTURE_PULLED,
                        *FIGHTER_STATUS_KIND_CAPTURE_WAIT,
                        *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE,
                        *FIGHTER_STATUS_KIND_CAPTURE_CUT,
                        *FIGHTER_STATUS_KIND_ENTRY,
                        *FIGHTER_STATUS_KIND_THROWN,
                        *FIGHTER_STATUS_KIND_DAMAGE_FLY,
                        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
                        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
                        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
                        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
                        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
                        *FIGHTER_STATUS_KIND_DAMAGE_FALL,
                        *FIGHTER_STATUS_KIND_TREAD_DAMAGE_AIR,
                        *FIGHTER_STATUS_KIND_BURY,
                        *FIGHTER_STATUS_KIND_BURY_WAIT
                       ].contains(&status_kind))
                    || (WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_AIR))
                    || (WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_GROUND));

    if !hdr::is_ready_go() {
        GroundModule::set_rhombus_offset(boma, &offset);
        return;
    }

    let air_trans: bool = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR) < 10;

    // Small
    let group1: &[i32] = &[
        *FIGHTER_KIND_KIRBY,
        *FIGHTER_KIND_PIKACHU,
        *FIGHTER_KIND_NESS,
        *FIGHTER_KIND_PURIN,
        *FIGHTER_KIND_GAMEWATCH,
        *FIGHTER_KIND_POPO,
        *FIGHTER_KIND_NANA,
        *FIGHTER_KIND_PICHU,
        *FIGHTER_KIND_METAKNIGHT,
        *FIGHTER_KIND_WARIO,
        *FIGHTER_KIND_PZENIGAME,
        *FIGHTER_KIND_PFUSHIGISOU,
        *FIGHTER_KIND_LUCAS,
        *FIGHTER_KIND_PIKMIN,
        *FIGHTER_KIND_TOONLINK,
        *FIGHTER_KIND_DUCKHUNT,
        *FIGHTER_KIND_MURABITO,
        *FIGHTER_KIND_INKLING,
        *FIGHTER_KIND_SHIZUE
    ];

    // Medium
    let group2: &[i32] = &[
        *FIGHTER_KIND_MARIO,
        *FIGHTER_KIND_YOSHI,
        *FIGHTER_KIND_LUIGI,
        *FIGHTER_KIND_MARIOD,
        *FIGHTER_KIND_YOUNGLINK,
        *FIGHTER_KIND_PLIZARDON,
        *FIGHTER_KIND_DIDDY,
        *FIGHTER_KIND_DEDEDE,
        *FIGHTER_KIND_ROCKMAN,
        *FIGHTER_KIND_GEKKOUGA,
        *FIGHTER_KIND_PACMAN,
        *FIGHTER_KIND_KOOPAJR,
        *FIGHTER_KIND_PACKUN,
        *FIGHTER_KIND_MIIFIGHTER,
        *FIGHTER_KIND_MIISWORDSMAN,
        *FIGHTER_KIND_MIIGUNNER,
        *FIGHTER_KIND_PACKUN,
        *FIGHTER_KIND_BUDDY,
        *FIGHTER_KIND_PICKEL
    ];

    // Large
    let group3: &[i32] = &[
        *FIGHTER_KIND_FOX,
        *FIGHTER_KIND_FALCO,
        *FIGHTER_KIND_DAISY,
        *FIGHTER_KIND_MEWTWO,
        *FIGHTER_KIND_PIT,
        *FIGHTER_KIND_PITB,
        *FIGHTER_KIND_SONIC,
        *FIGHTER_KIND_LUCARIO,
        *FIGHTER_KIND_ROBOT,
        *FIGHTER_KIND_WOLF,
        *FIGHTER_KIND_LITTLEMAC,
        *FIGHTER_KIND_KROOL,
        *FIGHTER_KIND_GAOGAEN
    ];

    // X-Large
    let group4: &[i32] = &[
        *FIGHTER_KIND_DONKEY,
        *FIGHTER_KIND_LINK,
        *FIGHTER_KIND_SAMUS,
        *FIGHTER_KIND_SAMUSD,
        *FIGHTER_KIND_CAPTAIN,
        *FIGHTER_KIND_PEACH,
        *FIGHTER_KIND_KOOPA,
        *FIGHTER_KIND_SHEIK,
        *FIGHTER_KIND_ZELDA,
        *FIGHTER_KIND_MARTH,
        *FIGHTER_KIND_LUCINA,
        *FIGHTER_KIND_GANON,
        *FIGHTER_KIND_ROY,
        *FIGHTER_KIND_CHROM,
        *FIGHTER_KIND_SZEROSUIT,
        *FIGHTER_KIND_SNAKE,
        *FIGHTER_KIND_IKE,
        *FIGHTER_KIND_WIIFIT,
        *FIGHTER_KIND_ROSETTA,
        *FIGHTER_KIND_PALUTENA,
        *FIGHTER_KIND_REFLET,
        *FIGHTER_KIND_SHULK,
        *FIGHTER_KIND_RYU,
        *FIGHTER_KIND_KEN,
        *FIGHTER_KIND_CLOUD,
        *FIGHTER_KIND_KAMUI,
        *FIGHTER_KIND_BAYONETTA,
        *FIGHTER_KIND_RIDLEY,
        *FIGHTER_KIND_SIMON,
        *FIGHTER_KIND_RICHTER,
        *FIGHTER_KIND_JACK,
        *FIGHTER_KIND_BRAVE,
        *FIGHTER_KIND_DOLLY,
        *FIGHTER_KIND_MASTER,
        *FIGHTER_KIND_TANTAN,
		*FIGHTER_KIND_EFLAME,
		*FIGHTER_KIND_ELIGHT,
        *FIGHTER_KIND_DEMON,
        *FIGHTER_KIND_DEMON + 1
    ];

    // XX-Large
    let group5: &[i32] = &[
        *FIGHTER_KIND_SAMUS,
        *FIGHTER_KIND_SAMUSD,
        *FIGHTER_KIND_EDGE
    ];

    // There *must* be a better "Rust" way to do this :)
    max_offset = match fighter_kind {
        y if group1.contains(&y) => 2.,
        y if group2.contains(&y) => 3.5,
        y if group3.contains(&y) => 4.,
        y if group4.contains(&y) => 5.,
        y if group5.contains(&y) => 6.,
        _ => max_offset,
    };

    if status_kind == *FIGHTER_STATUS_KIND_ENTRY {
        max_offset = 0.;
    }

    let player_number = hdr::get_player_number(boma);

    let mut correct_ecb: bool = !vanilla_ecb;

    if situation_kind == *SITUATION_KIND_AIR {
        /*
        if ecb_y_offsets[player_number] < max_offset) {
            ecb_y_offsets[player_number] += 0.05;
        } else {
            ecy_y_offsets[player_number] = max_offset;
        }
        */

        ecb_y_offsets[player_number] = max_offset;

        // Make wavelands a bit smoother
        if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
            ecb_y_offsets[player_number] = max_offset - 0.2;
        }

        offset.x = 0.;
        offset.y = ecb_y_offsets[player_number];

        correct_ecb = correct_ecb && !air_trans;
    } else if situation_kind == *SITUATION_KIND_GROUND {
        max_offset = 0.;
        offset.x = 0.;
        offset.y = 0.;
    } else {
        ecb_y_offsets[player_number] = 0.;
        offset.x = 0.;
        offset.y = ecb_y_offsets[player_number];
    }

    if correct_ecb {
        GroundModule::set_rhombus_offset(boma, &offset);
    }
}

//=================================================================
//== EXTRA TRACTION
//=================================================================
unsafe fn extra_traction(fighter: &mut L2CFighterCommon, lua_state: u64, l2c_agent: &mut L2CAgent, boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, facing: f32) {
    let max_walk: f32 = WorkModule::get_param_float(boma, hash40("walk_speed_max"), 0);
    let traction: f32 = WorkModule::get_param_float(boma, hash40("ground_brake"), 0);

    let cur_speed = Vector2f {x: KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN),
                              y: KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN)};

    let travel_dir: f32 = if cur_speed.x >= 0.0 {1.0} else {-1.0};
    let kinetic_type: i32 = KineticModule::get_kinetic_type(boma);

    let mut set_brake = Vector3f {x: 0.0, y: 0.0, z: 0.0};

    //println!("Current Kinetic Type: {}", KineticModule::get_kinetic_type(boma));

    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
    let test_speed = app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
    fighter.clear_lua_stack();

// Extra traction when above max walk speed
    if cur_speed.x.abs() > max_walk && situation_kind == *SITUATION_KIND_GROUND {
        if [*FIGHTER_STATUS_KIND_WAIT,
            *FIGHTER_STATUS_KIND_LANDING_LIGHT,
            *FIGHTER_STATUS_KIND_LANDING,
            *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR,
            *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL,
            //*FIGHTER_STATUS_KIND_CATCH,
            *FIGHTER_STATUS_KIND_CATCH_PULL,
			*FIGHTER_STATUS_KIND_JUMP_SQUAT,
            *FIGHTER_STATUS_KIND_WAIT,
            *FIGHTER_STATUS_KIND_SQUAT,
            *FIGHTER_STATUS_KIND_SQUAT_RV,
            *FIGHTER_STATUS_KIND_ATTACK,
            *FIGHTER_STATUS_KIND_ATTACK_S3,
            *FIGHTER_STATUS_KIND_ATTACK_HI3,
            *FIGHTER_STATUS_KIND_ATTACK_LW3,
            *FIGHTER_STATUS_KIND_ATTACK_S4_START,
            *FIGHTER_STATUS_KIND_ATTACK_S4,
            *FIGHTER_STATUS_KIND_ATTACK_HI4_START,
            *FIGHTER_STATUS_KIND_ATTACK_HI4,
            *FIGHTER_STATUS_KIND_ATTACK_LW4_START,
            *FIGHTER_STATUS_KIND_ATTACK_LW4].contains(&status_kind) {
                //double_traction_check[hdr::get_player_number(boma)] = true;
                VarModule::on_flag(boma.object(), common::ENABLE_DOUBLE_TRACTION);
            }
        else{
            //double_traction_check[hdr::get_player_number(boma)] = false;
            VarModule::off_flag(boma.object(), common::ENABLE_DOUBLE_TRACTION);
        }
    }
    else{
        //double_traction_check[hdr::get_player_number(boma)] = false;
        VarModule::off_flag(boma.object(), common::ENABLE_DOUBLE_TRACTION);
    }

    //println!("Current traction: {}", WorkModule::get_param_float(boma, hash40("ground_brake"), 0));

    /*
    // Extra traction when above max walk speed
    if test_speed.abs() > max_walk {
        if [*FIGHTER_STATUS_KIND_WAIT,
            *FIGHTER_STATUS_KIND_LANDING_LIGHT,
            *FIGHTER_STATUS_KIND_LANDING,
            *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR,
            *FIGHTER_STATUS_KIND_CATCH,
            *FIGHTER_STATUS_KIND_CATCH_PULL,
			*FIGHTER_STATUS_KIND_JUMP_SQUAT,
            *FIGHTER_STATUS_KIND_WAIT,
            *FIGHTER_STATUS_KIND_SQUAT,
            *FIGHTER_STATUS_KIND_SQUAT_RV,
            *FIGHTER_STATUS_KIND_ATTACK,
            *FIGHTER_STATUS_KIND_ATTACK_S3,
            *FIGHTER_STATUS_KIND_ATTACK_HI3,
            *FIGHTER_STATUS_KIND_ATTACK_LW3,
            *FIGHTER_STATUS_KIND_ATTACK_S4_START,
            *FIGHTER_STATUS_KIND_ATTACK_S4,
            *FIGHTER_STATUS_KIND_ATTACK_HI4_START,
            *FIGHTER_STATUS_KIND_ATTACK_HI4,
            *FIGHTER_STATUS_KIND_ATTACK_LW4_START,
            *FIGHTER_STATUS_KIND_ATTACK_LW4,
            *FIGHTER_STATUS_KIND_SPECIAL_N].contains(&status_kind) 
            && situation_kind == *SITUATION_KIND_GROUND {
            //set_brake.x = -traction * 1.0 * facing * travel_dir;
            //KineticModule::add_speed(boma, &set_brake);
            double_traction_check[hdr::get_player_number(boma)] = true;
        }
        else{
            double_traction_check[hdr::get_player_number(boma)] = false;
        }
    }
    else{
        double_traction_check[hdr::get_player_number(boma)] = false;
    }
    */
    
    
    /*
    // Extra traction when above max walk speed
    if cur_speed.x.abs() > max_walk {
        if [*FIGHTER_STATUS_KIND_WAIT,
            *FIGHTER_STATUS_KIND_LANDING_LIGHT,
            *FIGHTER_STATUS_KIND_LANDING,
            *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR,
            *FIGHTER_STATUS_KIND_CATCH,
            *FIGHTER_STATUS_KIND_CATCH_PULL,
			*FIGHTER_STATUS_KIND_JUMP_SQUAT,
            *FIGHTER_STATUS_KIND_WAIT,
            *FIGHTER_STATUS_KIND_SQUAT,
            *FIGHTER_STATUS_KIND_SQUAT_RV,
            *FIGHTER_STATUS_KIND_ATTACK,
            *FIGHTER_STATUS_KIND_ATTACK_S3,
            *FIGHTER_STATUS_KIND_ATTACK_HI3,
            *FIGHTER_STATUS_KIND_ATTACK_LW3,
            *FIGHTER_STATUS_KIND_ATTACK_S4_START,
            *FIGHTER_STATUS_KIND_ATTACK_S4,
            *FIGHTER_STATUS_KIND_ATTACK_HI4_START,
            *FIGHTER_STATUS_KIND_ATTACK_HI4,
            *FIGHTER_STATUS_KIND_ATTACK_LW4_START,
            *FIGHTER_STATUS_KIND_ATTACK_LW4,
            *FIGHTER_STATUS_KIND_SPECIAL_N].contains(&status_kind) 
            && situation_kind == *SITUATION_KIND_GROUND {
            set_brake.x = -traction * 1.0 * facing * travel_dir;
            KineticModule::add_speed(boma, &set_brake);
        }
    }
    */

    /*
    // Extra traction during DACDS until under 50% of walk speed
    if cur_speed.x.abs() > (max_walk * 0.5) {
        if (status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW4_START && StatusModule::prev_status_kind(boma, 0) == *FIGHTER_STATUS_KIND_ATTACK_DASH)
            || (status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW4 && StatusModule::prev_status_kind(boma, 1) == *FIGHTER_STATUS_KIND_ATTACK_DASH) {
            set_brake.x = -traction * 0.9 * facing * travel_dir;
            KineticModule::add_speed(boma, &set_brake);
        }
    }
    */

    /*
    // Extra traction during damage until under 25% of walk speed
    if cur_speed.x.abs() > (max_walk * 0.25) {
        if((status_kind == *FIGHTER_STATUS_KIND_DAMAGE || status_kind == *FIGHTER_STATUS_KIND_LANDING_DAMAGE_LIGHT) && situation_kind == *SITUATION_KIND_GROUND){
            set_brake.x = -traction * 0.75 * facing * travel_dir;
            KineticModule::add_speed(boma, &set_brake);
        }
    }
    */
    
}

//=================================================================
//== GRAB JUMP REFRESH
//=================================================================
unsafe fn grab_jump_refresh(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if [*FIGHTER_STATUS_KIND_CAPTURE_PULLED,
        *FIGHTER_STATUS_KIND_CAPTURE_WAIT,
        *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE,
        *FIGHTER_STATUS_KIND_CAPTURE_CUT,
        *FIGHTER_STATUS_KIND_CAPTURE_JUMP].contains(&status_kind) {
        // Grants 1 extra jump if all jumps used up
        if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) == WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
            WorkModule::set_int(boma, WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) - 1, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        }
    }
}

//=================================================================
//== BUFFERABLE FULL HOP AERIALS
//== Note: UNUSED
//=================================================================
unsafe fn full_hop_aerial(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    // Last two frames of jump squat
    if MotionModule::motion_kind(boma) == hash40("jump_squat") && (MotionModule::frame(boma) >= MotionModule::end_frame(boma) - 1.0) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) {
            WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_MINI_JUMP);
            WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK);
        }
    }
}



pub unsafe fn run(fighter: &mut L2CFighterCommon, lua_state: u64, l2c_agent: &mut L2CAgent, boma: &mut BattleObjectModuleAccessor, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, fighter_kind: i32, stick_x: f32, stick_y: f32, facing: f32) {
    ecb_shifts(boma, status_kind, situation_kind, fighter_kind);
    extra_traction(fighter, lua_state, l2c_agent, boma, status_kind, situation_kind, facing);
    grab_jump_refresh(boma, status_kind);
    //full_hop_aerial(boma, status_kind);

    //WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FLY_REFLECT_D); //Melee style spike knockdown (courtesey of zabimaru), leaving it commented here just to have it saved somewhere
}

