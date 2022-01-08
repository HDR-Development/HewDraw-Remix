use smash::app::{self, lua_bind::*};
use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::phx::*;
use acmd::*;
use smash::app::utility::*;
use crate::utils::hdr;
use crate::vars::*;


pub fn install() {
    acmd::add_hooks!(
        koopag_jump_squat_game
    );
}


#[acmd::acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_KOOPAG,
animation = "jump_squat",
animcmd = "game_jumpsquat"
)]
fn koopag_jump_squat_game(fighter: &mut L2CFighterCommon) {
acmd!({
});
}


