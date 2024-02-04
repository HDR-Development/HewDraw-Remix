use super::*;
use globals::*;

// This file contains code for ceiling/wall/ground bounces


pub fn install() {
    Agent::new("fighter")
        .status(End, *FIGHTER_STATUS_KIND_DAMAGE_FLY, damage_fly_end)
        .status(End, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D, damage_fly_reflect_d_end)
        .status(End, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_JUMP_BOARD, damage_fly_reflect_jump_board_end)
        .status(End, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR, damage_fly_reflect_lr_end)
        .status(End, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, damage_fly_roll_end)
        .status(End, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, damage_fly_meteor_end)
        .install();
    skyline::nro::add_hook(nro_hook);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_ftStatusUniqProcessDamageFly_initReflect,
            damage_fly_end,
            damage_fly_reflect_d_end,
            damage_fly_reflect_jump_board_end,
            damage_fly_reflect_lr_end,
            damage_fly_reflect_u_end,
            damage_fly_roll_end,
        );
    }
}

extern "C" {
    #[link_name = "_ZN7lua2cpp16L2CFighterCommon36status_end_DamageFlyReflectJumpBoardEv"]
    fn L2CFighterCommon_status_end_DamageFlyReflectJumpBoard();

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon29status_end_DamageFlyReflectLREv"]
    fn L2CFigterCommon_status_end_DamageFlyReflectLR();
}

#[skyline::hook(replace = L2CFighterCommon_status_end_DamageFly)]
pub unsafe fn damage_fly_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::set_command_life_extend(fighter.module_accessor, 0);
    original!()(fighter)
}

#[skyline::hook(replace = L2CFighterCommon_status_end_DamageFlyReflectD)]
pub unsafe fn damage_fly_reflect_d_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::set_command_life_extend(fighter.module_accessor, 0);
    original!()(fighter)
}

#[skyline::hook(replace = L2CFighterCommon_status_end_DamageFlyReflectJumpBoard)]
pub unsafe fn damage_fly_reflect_jump_board_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::set_command_life_extend(fighter.module_accessor, 0);
    original!()(fighter)
}

#[skyline::hook(replace = L2CFigterCommon_status_end_DamageFlyReflectLR)]
pub unsafe fn damage_fly_reflect_lr_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::set_command_life_extend(fighter.module_accessor, 0);
    original!()(fighter)
}

#[skyline::hook(replace = L2CFighterCommon_status_end_DamageFlyReflectU)]
pub unsafe fn damage_fly_reflect_u_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::set_command_life_extend(fighter.module_accessor, 0);
    original!()(fighter)
}

#[skyline::hook(replace = L2CFighterCommon_status_end_DamageFlyRoll)]
pub unsafe fn damage_fly_roll_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::set_command_life_extend(fighter.module_accessor, 0);
    original!()(fighter)
}

pub unsafe extern "C" fn damage_fly_meteor_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::set_command_life_extend(fighter.module_accessor, 0);
    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_ftStatusUniqProcessDamageFly_initReflect)]
unsafe fn sub_ftStatusUniqProcessDamageFly_initReflect(fighter: &mut L2CFighterCommon) {
    if fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR]) {
        let reaction_frame = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME);
        let reflect_reaction_frame_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("damage_fly_reflect_reaction_frame_mul"));
        WorkModule::set_float(fighter.module_accessor, reaction_frame * reflect_reaction_frame_mul, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME);
    }
    fighter.start_damage_reflect_stop();
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGE_FLY_AIR);
    WorkModule::set_int64(fighter.module_accessor, hash40("invalid") as i64, *FIGHTER_STATUS_DAMAGE_WORK_INT_DAMAGE_MOTION_KIND);
    let reflect_disable_escape_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("damage_fly_reflect_disable_escape_frame"));
    WorkModule::set_int(fighter.module_accessor, reflect_disable_escape_frame, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_DAMAGE_REFLECT_ESCAPE_DISABLE_FRAME);
    WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);

    fighter.clear_lua_stack();
	lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
	let speed_x = app::sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);

    fighter.clear_lua_stack();
	lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
	let speed_y = app::sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);

    let reflect_speed_threshold = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("damage_fly_reflect_speed"));
    if speed_x.abs() <= reflect_speed_threshold || speed_y.abs() <= reflect_speed_threshold {
        GroundModule::set_test_coll_stop_status(fighter.module_accessor, true);
        let length = fighter.sub_FighterStatusDamage_get_coll_stop_slidable_length().get_f32();
        GroundModule::set_coll_stop_slidable_length(fighter.module_accessor, length);
    }
}