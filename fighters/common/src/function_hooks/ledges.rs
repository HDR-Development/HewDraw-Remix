use super::*;
use globals::*;

//=================================================================
//== GroundModule::entry_cliff
//== Note: Ledge occupancy handled by sys_line::ledges::occupy_ledge()
//=================================================================
#[skyline::hook(replace=GroundModule::entry_cliff)]
unsafe fn entry_cliff_hook(boma: &mut BattleObjectModuleAccessor) -> u64{
    //println!("Grabbing ledge... player number: {}", WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;);
    //println!("Current status kind: {}", StatusModule::status_kind(boma));
    //println!("Previous status kind: {}", StatusModule::prev_status_kind(boma, 0));
    original!()(boma)
}

//=================================================================
//== GroundModule::can_entry_cliff
//== Note: Handles ledgehogging, fighter-specific ledge entry,
//         and disallows rising ledge grabs (for non-specials)
//=================================================================
#[skyline::hook(replace=GroundModule::can_entry_cliff)]
unsafe fn can_entry_cliff_hook(boma: &mut BattleObjectModuleAccessor) -> u64 {
    let situation_kind = StatusModule::situation_kind(boma);
    let status_kind = StatusModule::status_kind(boma);
    let fighter_kind = boma.kind();

    let rising: f32 = KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN); // Rising while jumping/airdodging

    let tether_zair = boma.is_fighter()
                        && [*FIGHTER_KIND_LUCAS, *FIGHTER_KIND_YOUNGLINK, *FIGHTER_KIND_TOONLINK, *FIGHTER_KIND_SAMUS, *FIGHTER_KIND_SAMUSD, *FIGHTER_KIND_SZEROSUIT].contains(&fighter_kind)
                        && [*FIGHTER_STATUS_KIND_AIR_LASSO, *FIGHTER_STATUS_KIND_AIR_LASSO_REACH, *FIGHTER_STATUS_KIND_AIR_LASSO_HANG, *FIGHTER_STATUS_KIND_AIR_LASSO_REWIND].contains(&status_kind);

    let tether_special = boma.is_fighter()
                        && ( (fighter_kind == *FIGHTER_KIND_SZEROSUIT   && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S)
                          || (fighter_kind == *FIGHTER_KIND_SHIZUE      && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S)
                          || (fighter_kind == *FIGHTER_KIND_TANTAN      && (status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI || status_kind == *FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_AIR))
                          || (fighter_kind == *FIGHTER_KIND_MASTER      && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI)
                          || (fighter_kind == *FIGHTER_KIND_JACK        && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI)
                          || (fighter_kind == *FIGHTER_KIND_PFUSHIGISOU && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI) );

    let tether_aerial = boma.is_fighter()
                        && ( (fighter_kind == *FIGHTER_KIND_SIMON   && status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR)
                          || (fighter_kind == *FIGHTER_KIND_RICHTER && status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR)
                          || (fighter_kind == *FIGHTER_KIND_MASTER  && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI) );

    // Ledgehog code
    let cliff_id = GroundModule::get_cliff_id_uint32(boma);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    for object_id in util::get_all_active_battle_object_ids() {
        let object = ::utils::util::get_battle_object_from_id(object_id);
        if !object.is_null() {
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) == WorkModule::get_int(&mut *(*object).module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) {
                continue;
            }

            if VarModule::get_int(object, vars::common::instance::LEDGE_ID) == cliff_id as i32 {
                if !((tether_zair || tether_special || tether_aerial) && WorkModule::is_flag(boma, *FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK)) {
                    return 0;
                }
            }
        }
    }

    if boma.is_fighter() {
        // Character specific ledge grabbing rules
        // If check_cliff_entry_specializer returns 0, disable ledge grabbing
        // If check_cliff_entry_specializer returns 1, run the rising check
        // Otherwise (-1 returned) skip the rise check and let the vanilla behavior take place
        if check_cliff_entry_specializer(boma) == 0 {
            return 0;
        }
        else if check_cliff_entry_specializer(boma) == 1 {
            // Disable grabbing ledge while rising during an airborne state (not specials)
            if situation_kind == *SITUATION_KIND_AIR {
                if rising >= 0.0 && !((tether_zair || tether_special || tether_aerial) && WorkModule::is_flag(boma, *FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK)) {
                    return 0;
                }
            }
        }

        // Unable to grab ledge during runfall/walkfall (the first few frames after you run off an edge)
        if boma.is_motion_one_of(&[Hash40::new("run_fall_l"), Hash40::new("run_fall_r"), Hash40::new("walk_fall_l"), Hash40::new("walk_fall_r")]) {
            return 0;
        }
    }

    original!()(boma)
}

//=================================================================
//== GroundModule::leave_cliff
//== Note: Resets ledge occupancy
//=================================================================
#[skyline::hook(replace=GroundModule::leave_cliff)]
unsafe fn leave_cliff_hook(boma: &mut BattleObjectModuleAccessor) -> u64 {
    VarModule::set_int(boma.object(), vars::common::instance::LEDGE_ID, -1);
    original!()(boma)
}

pub fn install() {
    //skyline::install_hook!(entry_cliff_hook);
    skyline::install_hook!(can_entry_cliff_hook);
    //skyline::install_hook!(leave_cliff_hook);
}

//=================================================================
//== GroundModule::can_entry_cliff specializer
//=================================================================
unsafe fn check_cliff_entry_specializer(boma: &mut BattleObjectModuleAccessor) -> i32 {
    let fighter_kind = boma.kind();
    let status_kind = StatusModule::status_kind(boma);
    let motion_kind = MotionModule::motion_kind(boma);
    let prev_status_kind = StatusModule::prev_status_kind(boma, 0);
    let prev_status_kind_2 = StatusModule::prev_status_kind(boma, 1);
    let frame = MotionModule::frame(boma);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if fighter_kind == *FIGHTER_KIND_MARIO {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            if frame < 25.0 {
                return 0;
            }
        }
    }

    if fighter_kind == *FIGHTER_KIND_DONKEY {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_FALL {
                if frame < 65.0 {
                    return 0;
                }
            }
            else {
                return 1;
            }
        }
    }

    if fighter_kind == *FIGHTER_KIND_LINK {
        if boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END]) {
            if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_FALL {
                if frame < 42.0 {
                    return 0;
                }
            }
            else {
                return 1;
            }
        }
    }

    if fighter_kind == *FIGHTER_KIND_SAMUS || fighter_kind == *FIGHTER_KIND_SAMUSD {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            if frame < 35.0 {
                return 0;
            }
        }
    }

    // Yoshi: Unchanged

    // Kirby: Unchanged

    if fighter_kind == *FIGHTER_KIND_FOX {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            if frame < 5.0 {
                return 0;
            }
            else {
                return 1;
            }
        }
        if status_kind == *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_RUSH {
            return 1;
        }

        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
            //if frame < 8.0 {
                return 1;
            //}
        }
    }

    if fighter_kind == *FIGHTER_KIND_PIKACHU {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI || status_kind == *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_WARP {
            return 1;
        }
        /*
        if status_kind == *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_END {
            if frame < 6.0 {
                return 0;
            }
        }
        */

        if status_kind == *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_ATTACK {
            if frame < 39.0 {
                return 1;
            }
        }
    }

    if fighter_kind == *FIGHTER_KIND_LUIGI {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            if frame < 31.0 {
                return 0;
            }
        }
    }

    if fighter_kind == *FIGHTER_KIND_NESS {
        if status_kind == *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_ATTACK {
            /*
            if frame > 5.0 && frame < 15.0 {
                return 1;
            }
            else{
                return -1;
            }
            */
            return 1;
        }
    }

    if fighter_kind == *FIGHTER_KIND_CAPTAIN {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            if frame < 41.0 {
                return 0;
            }
        }

        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
            if frame < 34.0 {
                return 0;
            }
        }
    }

    if fighter_kind == *FIGHTER_KIND_PEACH || fighter_kind == *FIGHTER_KIND_DAISY {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            if frame < 24.0 {
                return 0;
            }
        }

        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S || status_kind == *FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_JUMP {
            if frame < 20.0 {
                return 0;
            }
        }
    }

    if fighter_kind == *FIGHTER_KIND_KOOPA {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI || status_kind == *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_A {
            if prev_status_kind != *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_G {
                if frame < 37.0 {
                    return 1;
                }
            }
            else {
                return 1;
            }
        }
    }

    // Ice Climbers
    if [*FIGHTER_KIND_ICE_CLIMBER, *FIGHTER_KIND_POPO, *FIGHTER_KIND_NANA].contains(&fighter_kind) {
        if [*FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_JUMP_PRE, *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_PARTNER, *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_JUMP_PARTNER, *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_JUMP, *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_CLIFF_COMP_PARTNER, *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_CLIFF_PULL_PARTNER, *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_CLIFF_COMP].contains(&status_kind) {
            return -1;
        }
        if status_kind == *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_FAIL {
            return 1;
        }
    }

    if fighter_kind == *FIGHTER_KIND_SHEIK {
        if status_kind == *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_HI_MOVE {
            return 1;
        }

        if status_kind == *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_HI_END {
            if frame < 6.0 {
                return 1;
            }
        }

        if status_kind == *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_ATTACK || status_kind == *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_RETURN {
            if frame < 17.0 {
                return 0;
            }
        }
    }

    if fighter_kind == *FIGHTER_KIND_ZELDA {
        if status_kind == *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_2 {
            return 1;
        }
    }

    if fighter_kind == *FIGHTER_KIND_MARIOD {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            if frame < 25.0 {
                return 0;
            }
        }
    }

    if fighter_kind == *FIGHTER_KIND_PICHU {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI || status_kind == *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_WARP {
            return 1;
        }
        /*
        if status_kind == *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_END {
            if frame < 6.0 {
                return 0;
            }
        }
        */

        if status_kind == *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_ATTACK {
            if frame < 39.0 {
                return 1;
            }
        }
    }

    if fighter_kind == *FIGHTER_KIND_FALCO {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            if frame < 5.0 {
                return 0;
            }
            else {
                return 1;
            }
        }
        if status_kind == *FIGHTER_FALCO_STATUS_KIND_SPECIAL_HI_RUSH {
            return 1;
        }

        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
            //if frame < 8.0 {
                return 1;
            //}
        }
    }

    if fighter_kind == *FIGHTER_KIND_MARTH || fighter_kind == *FIGHTER_KIND_LUCINA {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            if frame < 20.0 {
                return 0;
            }
        }
    }

    if fighter_kind == *FIGHTER_KIND_YOUNGLINK {
        if status_kind == *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END {
            if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_FALL {
                if frame < 42.0 {
                    return 0;
                }
            }
            else {
                return 1;
            }
        }
    }

    if fighter_kind == *FIGHTER_KIND_GANON {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            if frame < 39.0 {
                return 0;
            }
        }

        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
            //if frame < 42.0 {
                return 1;
            //}
        }
    }

    if fighter_kind == *FIGHTER_KIND_MEWTWO {
        if status_kind == *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_2 {
            return 1;
        }

        if status_kind == *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_3 {
            if frame < 6.0 {
                return 1;
            }
        }
    }

    if fighter_kind == *FIGHTER_KIND_ROY {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI || status_kind == *FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_2 || status_kind == *FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_3 || status_kind == *FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_4 {
            if frame < 35.0 {
                return 0;
            }
        }
    }

    // Chrom: Unchanged

    if fighter_kind == *FIGHTER_KIND_GAMEWATCH {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            if frame < 22.0 {
                return 0;
            }
        }
    }

    if fighter_kind == *FIGHTER_KIND_METAKNIGHT {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI || status_kind == *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_HI_LOOP {
            if frame < 28.0 {
                return 0;
            }
        }

        if status_kind == *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_S_RUSH {
            if frame < 50.0 {
                return 0;
            }
        }
    }

    if fighter_kind == *FIGHTER_KIND_PIT || fighter_kind == *FIGHTER_KIND_PITB {
        if status_kind == *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH {
            return 0;
        }
        if status_kind == *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH_END {
            if frame < 10.0 {
                return 0;
            }
        }
    }

    if fighter_kind == *FIGHTER_KIND_SZEROSUIT {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            if frame < 30.0 {
                return 0;
            }
        }
    }

    if fighter_kind == *FIGHTER_KIND_WARIO {
        if status_kind == *FIGHTER_WARIO_STATUS_KIND_SPECIAL_HI_JUMP {
            if frame < 35.0 {
                return 0;
            }
        }
    }

    if fighter_kind == *FIGHTER_KIND_SNAKE {
        if status_kind == *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_HI_HANG {
            //if frame < 60.0 {
                return 0;
            //}
        }
    }

    if fighter_kind == *FIGHTER_KIND_IKE {
        if status_kind == *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_DASH {
            return 0;
        }
    }

    if fighter_kind == *FIGHTER_KIND_PZENIGAME {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            return 0;
        }
    }

    if fighter_kind == *FIGHTER_KIND_PLIZARDON {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            if frame < 38.0 {
                return 0;
            }
        }
    }

    if fighter_kind == *FIGHTER_KIND_DIDDY {
        /*
        if status_kind == *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_UPPER {
            if frame < 28.0 {
                return 0;
            }
        }
        */
    }

    if fighter_kind == *FIGHTER_KIND_LUCAS {
        if status_kind == *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_ATTACK {
            /*
            if frame > 5.0 && frame < 15.0 {
                return 1;
            }
            else{
                return -1;
            }
            */
            return 1;
        }
    }

    if fighter_kind == *FIGHTER_KIND_SONIC {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            return 0;
        }
        if status_kind == *FIGHTER_SONIC_STATUS_KIND_SPECIAL_HI_JUMP {
            if frame < 23.0 {
                return 0;
            }
        }
    }

    if fighter_kind == *FIGHTER_KIND_DEDEDE {
        if status_kind == *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_HI_JUMP {
            if frame < 25.0 {
                return 0;
            }
        }
    }

    if fighter_kind == *FIGHTER_KIND_PIKMIN {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI || status_kind == *FIGHTER_PIKMIN_STATUS_KIND_SPECIAL_HI_WAIT {
            return 0;
        }
    }

    if fighter_kind == *FIGHTER_KIND_LUCARIO {
        if status_kind == *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH {
            if frame < 17.0 {
                return 1;
            }
        }
    }

    if fighter_kind == *FIGHTER_KIND_ROBOT {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            //if frame < 60.0 {
                return 1;
            //}
        }
    }

    if fighter_kind == *FIGHTER_KIND_TOONLINK {
        if status_kind == *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END {
            if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_FALL {
                if frame < 42.0 {
                    return 0;
                }
            }
            else {
                return 1;
            }
        }
    }

    if fighter_kind == *FIGHTER_KIND_WOLF {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI || status_kind == *FIGHTER_WOLF_STATUS_KIND_SPECIAL_HI_RUSH {
            return 1;
        }
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
            //if frame < 11.0 {
                return 1;
            //}
        }
    }

    if fighter_kind == *FIGHTER_KIND_MURABITO {
        if status_kind == *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_FLAP {
            return 0;
        }

        if status_kind == *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_WAIT || status_kind == *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_TURN {
            if frame < 10.0 {
                return 0;
            }
        }
    }

    if fighter_kind == *FIGHTER_KIND_ROCKMAN {
        if status_kind == *FIGHTER_ROCKMAN_STATUS_KIND_SPECIAL_HI_JUMP {
            if frame < 28.0 {
                return 0;
            }
        }
    }

    if fighter_kind == *FIGHTER_KIND_WIIFIT {
        if status_kind == *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_HI_JUMP {
            return 0;
        }
        if prev_status_kind_2 == *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_HI_JUMP && status_kind == *FIGHTER_STATUS_KIND_FALL_SPECIAL {
            if frame < 7.0 {
                return 1;
            }
        }
    }

    if fighter_kind == *FIGHTER_KIND_ROSETTA {
        if status_kind == *FIGHTER_ROSETTA_STATUS_KIND_SPECIAL_HI_JUMP {
            return 0;
        }
        if status_kind == *FIGHTER_ROSETTA_STATUS_KIND_SPECIAL_HI_END {
            if frame < 10.0 {
                return 0;
            }
        }
    }

    if fighter_kind == *FIGHTER_KIND_LITTLEMAC {
        if status_kind == *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_HI_JUMP {
            if frame < 30.0 {
                return 0;
            }
        }

        if status_kind == *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_S_BLOW {
            if frame < 11.0 {
                return 1;
            }
        }
    }

    if fighter_kind == *FIGHTER_KIND_GEKKOUGA {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI || status_kind == *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_HI_LOOP {
            return 1;
        }
        /*
        if status_kind == *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_HI_END {
            if frame < 6.0 {
                return 0;
            }
        }
        */
    }

    if fighter_kind == *FIGHTER_KIND_PALUTENA {
        if status_kind == *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_2 {
            return 1;
        }
        if status_kind == *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_3 {
            if frame < 6.0 {
                return 1;
            }
        }
    }

    if fighter_kind == *FIGHTER_KIND_PACMAN {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI || status_kind == *FIGHTER_PACMAN_STATUS_KIND_SPECIAL_HI_LOOP {
            if frame < 25.0 {
                return 1;
            }
        }
        if motion_kind == hash40("special_s_move") || motion_kind == hash40("special_s_dash") {
            return 1;
        }
    }

    if fighter_kind == *FIGHTER_KIND_REFLET {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            if frame < 9.0 {
                return 0;
            }
            else {
                return 1;
            }
        }
        if status_kind == *FIGHTER_REFLET_STATUS_KIND_SPECIAL_HI_2 {
            if frame < 40.0 {
                return 1;
            }
        }
    }

    if fighter_kind == *FIGHTER_KIND_SHULK {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            if frame < 28.0 {
                return 1;
            }
        }
    }

    if fighter_kind == *FIGHTER_KIND_KOOPAJR {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI || status_kind == *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_SHOOT {
            return 1;
        }
    }

    if fighter_kind == *FIGHTER_KIND_DUCKHUNT {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI || status_kind == *FIGHTER_DUCKHUNT_STATUS_KIND_SPECIAL_HI_FLY {
            //if frame < 60.0 {
                return 1;
            //}
        }
    }

    if fighter_kind == *FIGHTER_KIND_RYU || fighter_kind == *FIGHTER_KIND_KEN {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI || status_kind == *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_JUMP {
            if frame < 25.0 {
                return 1;
            }
        }
        if [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_END, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND].contains(&status_kind) {
            return 1;
        }
    }

    // Cloud
    if fighter_kind == *FIGHTER_KIND_CLOUD {
        if motion_kind == hash40("special_hi") || motion_kind == hash40("special_air_hi") {
            /*
            if frame > 11.0 && frame < 18.0 {
                return 0;
            }
            */
            return 1;
        }
    }

    if fighter_kind == *FIGHTER_KIND_KAMUI {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            if frame < 45.0 {
                return 1;
            }
        }
    }

    // Bayonetta: Unchanged

    if fighter_kind == *FIGHTER_KIND_INKLING {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI || status_kind == *FIGHTER_INKLING_STATUS_KIND_SPECIAL_HI_JUMP {
                return 1;
        }

        if status_kind == *FIGHTER_INKLING_STATUS_KIND_SPECIAL_HI_ROT {
            if frame < 15.0 {
                return 1;
            }
        }
    }

    if fighter_kind == *FIGHTER_KIND_RIDLEY {
        if status_kind == *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_F || status_kind == *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_B || status_kind == *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_HI /*|| status_kind == *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_LW*/ {
            return 0;
        }
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S || status_kind == *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_FAILURE {
            //if frame < 39.0 {
                return 1;
            //}
        }
    }

    if fighter_kind == *FIGHTER_KIND_SIMON || fighter_kind == *FIGHTER_KIND_RICHTER {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            if frame < 31.0 {
                return 1;
            }
        }
    }

    if fighter_kind == *FIGHTER_KIND_KROOL {
        if status_kind == *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_START || status_kind == *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI {
            //if frame < 60.0 {
                return 0;
            //}
        }

        if status_kind == *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_AIR_END || status_kind == *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_FALL {
            if frame < 14.0 {
                return 1;
            }
        }
    }

    if fighter_kind == *FIGHTER_KIND_SHIZUE {
        if status_kind == *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_FLAP {
            return 1;
        }

        if status_kind == *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_WAIT || status_kind == *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_TURN {
            if frame < 10.0 {
                return 1;
            }
        }
    }

    if fighter_kind == *FIGHTER_KIND_GAOGAEN {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            return 1;
        }

        if status_kind == *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_TURN {
            if frame < 4.0 {
                return 1;
            }
        }

        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
            //if frame < 41.0 {
                return 1;
            //}
        }
    }

    if fighter_kind == *FIGHTER_KIND_PACKUN {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            //if frame < 60.0 {
                return 1;
            //}
        }

        if status_kind == *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_END {
            if frame < 14.0 {
                return 1;
            }
        }
    }

    if fighter_kind == *FIGHTER_KIND_JACK {
        if status_kind == *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI2_RUSH {
            return 1;
        }

        if status_kind == *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI2_END {
            if frame < 7.0 {
                return 1;
            }
        }
    }

    if fighter_kind == *FIGHTER_KIND_BRAVE {
        if status_kind == *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_HI_JUMP {
            if frame < 33.0 {
                return 1;
            }
        }
    }

    if fighter_kind == *FIGHTER_KIND_MIIFIGHTER {
        if status_kind == *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_HI1_2 || status_kind == *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_HI1_3 {
            if frame < 26.0 {
                return 1;
            }
        }

        if status_kind == *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_HI2_ATTACK {
            return 1;
        }

        if status_kind == *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_HI2_END {
            if frame < 47.0 {
                return 1;
            }
        }

        if status_kind == *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_S2_ATTACK || status_kind == *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_S2_WEAK {
            return 1;
        }

        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            if frame < 33.0 {
                return 1;
            }
        }
    }

    if fighter_kind == *FIGHTER_KIND_MIISWORDSMAN {
        if status_kind == *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI1_JUMP {
            if frame < 23.0 {
                return 1;
            }
        }

        if status_kind == *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S1_WEAK || status_kind == *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S1_ATTACK {
            if frame < 42.0 {
                return 1;
            }
        }

        if status_kind == *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI2_RUSH {
            return 1;
        }

        if status_kind == *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_ATTACK || status_kind == *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_DASH {
            return 1;
        }

        if status_kind == *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI3_END {
            if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_FALL {
                if frame < 41.0 {
                    return 0;
                }
            }
            else {
                return 1;
            }
        }
    }

    if fighter_kind == *FIGHTER_KIND_MIIGUNNER {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            return 1;
        }

        if status_kind == *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_HI2_JUMP {
            if frame < 35.0 {
                return 1;
            }
        }

        if status_kind == *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_HI3_RUSH {
            return 1;
        }

        if status_kind == *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_HI3_RUSH_END {
            if frame < 7.0 {
                return 1;
            }
        }
    }

    if fighter_kind == *FIGHTER_KIND_BUDDY {

        if status_kind == *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_HI_JUMP {
            if frame < 32.0 {
                return 1;
            }
        }

        if status_kind == *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH {
            return 1;
        }
    }

    if(fighter_kind == *FIGHTER_KIND_PICKEL){
        if(status_kind == *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_HI_GLIDING){
            if VarModule::get_float(boma.object(), vars::pickel::status::GLIDE_TIMER) < 40.0 /*40 frames of up b travel time*/ {
                return 1;
            }
        }
    }

    if fighter_kind == *FIGHTER_KIND_EFLAME {
        if status_kind == *FIGHTER_EFLAME_STATUS_KIND_SPECIAL_HI_JUMP {
            if frame < 3.0 {
                return 0;
            }
        }
    }

    // General fighter check stuff
    if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR{
        return -1;
    }

    1
}
