#![allow(non_upper_case_globals)]
pub mod offsets;
pub mod util;
pub mod singletons;
pub mod consts;
pub mod ext;
mod modules;
pub mod frame_info;
pub mod game_modes;
pub mod ui;

#[macro_use]
extern crate modular_bitfield;

pub use hdr_macros::{
    export,
    import,
    import_noreturn,
    hash40
};

pub use hdr_macros as macros;

pub use modules::*;
pub use frame_info::*;

/// Transitions an effect from one color alpha into another over a defined amount of frames
/// 
/// # Arguments
/// * `boma` - The fighter boma
/// * `fighter_kind` - The FIGHTER_KIND LuaConst for the character
/// * `effect_handler` - A VarModule constant to store the effect handle. Should be an instance int and needs an effect stored in it beforehand
/// * `start_frame` - The starting frame to apply the blend
/// * `end_frame` - The frame at which the starting color fully transitions to the ending one
/// * `start_color` - The RGB alpha value of the first color
/// * `end_color` - The RGB alpha value of the final color
/// * `is_article` - Set to true if being applied to an article's effect acmd, false if it is in the fighter's effect acmd
#[macro_export]
macro_rules! blend_effect_rgb {
    ($fighter:expr, $fighter_kind:expr, $effect_handler:expr, $start_frame:expr, $end_frame:expr, ($r1:expr, $g1:expr, $b1:expr), ($r2:expr, $g2:expr, $g3:expr), $is_article:expr) => {
        let start_color = Vector3f::new($r1 as f32, $g1 as f32, $b1 as f32);
        let end_color = Vector3f::new($r1 as f32, $g1 as f32, $b1 as f32);
        let eff_handle = if $is_article {
            let owner_id = WorkModule::get_int($fighter.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
            let owner_object = get_battle_object_from_id(owner_id);
            VarModule::get_int(owner_object, $effect_handler)
        } else {
            VarModule::get_int($fighter.battle_object, $effect_handler)
        };

        for _ in 0..9999999999 {
            let i = MotionModule::frame($fighter.module_accessor);
            if i > $end_frame { break; }
            // Smoothly interpolate from starting to ending color based on current frame
            let blend_vector = Vector3f {
                x: start_color.x + ((end_color.x - start_color.x) * ((i - $start_frame) / ($end_frame - $start_frame))),
                y: start_color.y + ((end_color.y - start_color.y) * ((i - $start_frame) / ($end_frame - $start_frame))),
                z: start_color.z + ((end_color.z - start_color.z) * ((i - $start_frame) / ($end_frame - $start_frame)))
            };
            // Apply color shift
            EffectModule::set_rgb($fighter.module_accessor, eff_handle as u32, blend_vector.x, blend_vector.y, blend_vector.z);
            wait($fighter.lua_state_agent, MotionModule::rate($fighter.module_accessor));
        }
    };
}