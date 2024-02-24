use super::*;

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

