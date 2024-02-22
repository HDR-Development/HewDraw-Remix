use super::*;
use globals::*;

 

// FIGHTER_BAYONETTA_STATUS_KIND_BATWITHIN //

unsafe extern "C" fn batwithin_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let batwithin_status_kind = WorkModule::get_int(fighter.module_accessor, *FIGHTER_BAYONETTA_STATUS_WORK_ID_BATWITHIN_INT_STATUS_KIND);
    if [*FIGHTER_STATUS_KIND_ESCAPE_F, *FIGHTER_STATUS_KIND_ESCAPE_B].contains(&batwithin_status_kind) {
        fighter.sub_status_end_EscaleFB();
    }
    if batwithin_status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
        fighter.status_end_EscapeAir();
    }
    HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    VisibilityModule::set_whole(fighter.module_accessor, true);
    ItemModule::set_have_item_visibility(fighter.module_accessor, true, 0);
    ItemModule::set_attach_item_visibility(fighter.module_accessor, true, 255);
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_BAYONETTA_GENERATE_ARTICLE_BAT, ArticleOperationTarget(0));
    if fighter.global_table[STATUS_KIND] == FIGHTER_STATUS_KIND_FINAL_VISUAL_ATTACK_OTHER {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("fall"), 0.0, 1.0, false, 0.0, false, false);
    }
    0.into()
}pub fn install() {
    smashline::Agent::new("bayonetta")
        .status(End, *FIGHTER_BAYONETTA_STATUS_KIND_BATWITHIN, batwithin_end)
        .install();
}
