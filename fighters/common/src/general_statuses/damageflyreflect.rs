use super::*;
use globals::*;

// This file contains code for ceiling/wall/ground bounces

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            end_damage_reflect_stop
        );
    }
}

// Runs at the end of stage spike hitlag (wall splat)
#[skyline::hook(replace = L2CFighterCommon_end_damage_reflect_stop)]
pub unsafe fn end_damage_reflect_stop(fighter: &mut L2CFighterCommon) {
    let damage_speed_x = fighter.get_speed_x(*FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
    let damage_speed_y = fighter.get_speed_y(*FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
    // damage_fly_reflect_reaction_frame_mul is also applied to hitstun on stage spikes,
    // handled in a separate location
    let damage_fly_reflect_reaction_frame_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("damage_fly_reflect_reaction_frame_mul"));
    let new_speed = Vector2f::new(
        damage_speed_x * damage_fly_reflect_reaction_frame_mul,
        damage_speed_y * damage_fly_reflect_reaction_frame_mul,
    );

    fighter.set_speed(new_speed, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);

    original!()(fighter)
}