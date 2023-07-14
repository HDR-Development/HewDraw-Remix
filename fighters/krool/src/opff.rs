// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn jetpack_cancel(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, cat1: i32) {
    if status_kind == *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI {
        let fuel_burn_rate = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "param_special_hi.fuel_burn_rate");
        let fuel = VarModule::get_int(fighter.battle_object, vars::krool::instance::SPECIAL_HI_FUEL);
        VarModule::set_int(
            fighter.battle_object,
            vars::krool::instance::SPECIAL_HI_FUEL,
            fuel - fuel_burn_rate,
        );
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) || fuel <= 0 {
            StatusModule::change_status_request_from_script(
                boma,
                *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_AIR_END,
                true,
            );
        }
    } else if fighter.is_situation(*SITUATION_KIND_GROUND) {
        let fuel_max = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "param_special_hi.fuel_max");
        if VarModule::get_int(fighter.battle_object, vars::krool::instance::SPECIAL_HI_FUEL) < fuel_max {
            let fuel_recharge_rate = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "param_special_hi.fuel_recharge_rate");
            VarModule::add_int(fighter.battle_object, vars::krool::instance::SPECIAL_HI_FUEL, fuel_recharge_rate);
        }
    }
}

unsafe fn fuel_reset(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_WIN,
        *FIGHTER_STATUS_KIND_LOSE,
        *FIGHTER_STATUS_KIND_ENTRY,
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_REBIRTH]) {
        let fuel_max = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "param_special_hi.fuel_max");
        VarModule::set_int(fighter.battle_object, vars::krool::instance::SPECIAL_HI_FUEL, fuel_max);
    }
}

// K. Rool Side B Crown Item Grab
unsafe fn crownerang_item_grab(boma: &mut BattleObjectModuleAccessor, status_kind: i32, cat1: i32) {
    if [
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_KROOL_STATUS_KIND_SPECIAL_S_THROW,
    ]
    .contains(&status_kind)
    {
        //println!("K. Rool side B");
        if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
            //println!("K. Rool crown grab");
            if !ItemModule::is_have_item(boma, 0) {
                ItemModule::have_item(
                    boma,
                    app::ItemKind(*ITEM_KIND_KROOLCROWN),
                    0,
                    0,
                    false,
                    false,
                );
            }
        }
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_KROOL_STATUS_KIND_SPECIAL_N_LOOP,
        *FIGHTER_KROOL_STATUS_KIND_SPECIAL_N_END,
        *FIGHTER_KROOL_STATUS_KIND_SPECIAL_N_SUCTION,
        *FIGHTER_KROOL_STATUS_KIND_SPECIAL_N_SPIT,
        *FIGHTER_KROOL_STATUS_KIND_SPECIAL_N_SWALLOW,
        *FIGHTER_KROOL_STATUS_KIND_SPECIAL_S_GET,
        *FIGHTER_KROOL_STATUS_KIND_SPECIAL_S_CATCH,
        *FIGHTER_KROOL_STATUS_KIND_SPECIAL_S_THROW,
        *FIGHTER_KROOL_STATUS_KIND_SPECIAL_S_FAILURE,
        *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_FALL,
        *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_AIR_END,
        *FIGHTER_KROOL_STATUS_KIND_SPECIAL_LW_TURN,
        *FIGHTER_KROOL_STATUS_KIND_SPECIAL_LW_HIT
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

pub unsafe fn moveset(
    fighter: &mut L2CFighterCommon,
    boma: &mut BattleObjectModuleAccessor,
    id: usize,
    cat: [i32; 4],
    status_kind: i32,
    situation_kind: i32,
    motion_kind: u64,
    stick_x: f32,
    stick_y: f32,
    facing: f32,
    frame: f32,
) {
    jetpack_cancel(fighter, boma, status_kind, cat[0]);
    fuel_reset(fighter);
    //crownerang_item_grab(boma, status_kind, cat[0]);
    fastfall_specials(fighter);
}

#[utils::macros::opff(FIGHTER_KIND_KROOL)]
pub fn krool_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
        krool_frame(fighter)
    }
}

pub unsafe fn krool_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(
            fighter,
            &mut *info.boma,
            info.id,
            info.cat,
            info.status_kind,
            info.situation_kind,
            info.motion_kind.hash,
            info.stick_x,
            info.stick_y,
            info.facing,
            info.frame,
        );
    }
}

#[smashline::weapon_frame( agent = WEAPON_KIND_KROOL_BACKPACK, main)]
pub fn krool_backpack_frame(weapon: &mut smash::lua2cpp::L2CFighterBase) {
    unsafe {
        let boma = weapon.boma();
        let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        
        // upB low fuel indicator
        let fuel_max = ParamModule::get_int(owner_boma.object(), ParamType::Agent, "param_special_hi.fuel_max") as f32;
        let low_fuel_threshold = fuel_max * 0.33;
        if VarModule::get_int(owner_boma.object(), vars::krool::instance::SPECIAL_HI_FUEL) as f32 <= low_fuel_threshold
        && VarModule::get_int(owner_boma.object(), vars::krool::instance::FUEL_EFFECT_HANDLER) == -1 {
            let handle = EffectModule::req_follow(weapon.module_accessor, Hash40::new("krool_buckpack"), Hash40::new("backpack"), &Vector3f{x: -12.0, y: -1.5, z: -6.0}, &Vector3f::zero(), 1.5, true, 0, 0, 0, 0, 0, false, false) as u32;
            EffectModule::set_rgb(weapon.module_accessor, handle, 0.15, 0.15, 0.15);
            EffectModule::enable_sync_init_pos_last(weapon.module_accessor);
            VarModule::set_int(owner_boma.object(), vars::krool::instance::FUEL_EFFECT_HANDLER, handle as i32);
        }
    }
}