use super::*;
use smash_rs::app::{WorkId, work_ids, transition_groups, transition_terms};

extern "C" {
    #[link_name = "rockman_do_leafshield_things_disable_inner"]
    fn rockman_do_leafshield_things_disable_inner(ctx: &mut skyline::hooks::InlineCtx);

    #[link_name = "rockman_do_leafshield_things_enable_inner"]
    fn rockman_do_leafshield_things_enable_inner(ctx: &mut skyline::hooks::InlineCtx);
}

#[skyline::hook(offset = 0x1083bec, inline)]
unsafe fn rockman_do_leafshield_things_disable(ctx: &mut skyline::hooks::InlineCtx) {
    rockman_do_leafshield_things_disable_inner(ctx);
}

#[skyline::hook(offset = 0x10838e0, inline)]
unsafe fn rockman_do_leafshield_things_enable(ctx: &mut skyline::hooks::InlineCtx) {
    rockman_do_leafshield_things_enable_inner(ctx);
}

const LEAFSHIELD_DISABLE_GROUPS: [WorkId; 3] = [
    // transition_groups::CHECK_GROUND_SPECIAL,
    // transition_groups::CHECK_AIR_SPECIAL,
    transition_groups::CHECK_GROUND_ESCAPE,
    // transition_groups::CHECK_AIR_ESCAPE,
    // transition_groups::CHECK_GROUND_GUARD,
    // transition_groups::CHECK_GROUND_ATTACK,
    transition_groups::CHECK_GROUND_CATCH,
    transition_groups::CHECK_AIR_ATTACK,
    // transition_groups::CHECK_AIR_CLIFF
];

const LEAFSHIELD_DISABLE_INDIVI: [WorkId; 26] = [
    // transition_terms::CONT_DASH,
    // transition_terms::CONT_TURN_DASH,
    transition_terms::CONT_ATTACK,
    transition_terms::CONT_ATTACK_100,
    transition_terms::CONT_ATTACK_S3,
    transition_terms::CONT_ATTACK_HI3,
    transition_terms::CONT_ATTACK_LW3,
    transition_terms::CONT_ATTACK_S4_START,
    transition_terms::CONT_ATTACK_HI4_START,
    transition_terms::CONT_ATTACK_LW4_START,
    transition_terms::CONT_ATTACK_COMMAND1,
    transition_terms::CONT_ITEM_SWING_4,
    transition_terms::CONT_ITEM_SWING_3,
    transition_terms::CONT_ITEM_SWING,
    transition_terms::CONT_ITEM_SHOOT,
    transition_terms::CONT_ITEM_SHOOT_S3,
    transition_terms::CONT_ITEM_SHOOT_S4,
    transition_terms::CONT_COMMAND_623NB,
    transition_terms::CONT_ATTACK_STAND,
    transition_terms::CONT_ATTACK_SQUAT,
    transition_terms::CONT_ATTACK_DASH,
    transition_terms::CONT_CATCH_DASH,
    transition_terms::CONT_CATCH_TURN,
    // transition_terms::CONT_ATTACK_HI4_START,
    // transition_terms::CONT_ATTACK_LW4_START,
    transition_terms::CONT_SPECIAL_N,
    transition_terms::CONT_SPECIAL_S,
    transition_terms::CONT_SPECIAL_HI,
    transition_terms::CONT_CLIFF_ATTACK,
    transition_terms::CONT_CLIFF_ESCAPE
];

#[skyline::hook(replace = FighterSpecializer_Rockman::set_leafshield)]
unsafe extern "C" fn set_leafshield(module_accessor: *mut smash_rs::app::BattleObjectModuleAccessor, set_shield: bool) {
    let work = (*module_accessor).work();
    work.set_flag(set_shield, work_ids::fighter::rockman::instance::SPECIAL_LW_LEAFSHIELD);
    work.set_flag(set_shield, work_ids::fighter::rockman::instance::SPECIAL_LW_ENABLE_SHOOT);
    if !set_shield {
        work.set_int(0, work_ids::fighter::rockman::instance::SPECIAL_LW_HOLD_FRAME);
        for x in LEAFSHIELD_DISABLE_GROUPS.iter() {
            work.unable_transition_term_forbid_group_indivi(*x);
        }
        for x in LEAFSHIELD_DISABLE_INDIVI.iter() {
            work.unable_transition_term_forbid_indivi(*x);
        }
        // work.enable_transition_term_forbid(transition_terms::CONT_SPECIAL_LW);
        // if (*module_accessor).status().status_kind() < 0x27 {
        //     for x in LEAFSHIELD_DISABLE_GROUPS.iter() {
        //         work.enable_transition_term_group(*x);
        //     }
        //     for x in LEAFSHIELD_DISABLE_INDIVI.iter() {
        //         work.enable_transition_term(*x);
        //     }
        // }
    }
    else {
        let hold_frame = work.get_param_int(smash_rs::phx::Hash40::new("param_special_lw"), smash_rs::phx::Hash40::new("hold_frame"));
        work.set_int(hold_frame, work_ids::fighter::rockman::instance::SPECIAL_LW_HOLD_FRAME);
        for x in LEAFSHIELD_DISABLE_GROUPS.iter() {
            work.enable_transition_term_forbid_group_indivi(*x);
        }
        for x in LEAFSHIELD_DISABLE_INDIVI.iter() {
            work.enable_transition_term_forbid_indivi(*x);
        }
    }
}

// #[skyline::hook(offset = 0x1085d60)]
// pub unsafe extern "C" fn rockman_airshooter_init(article: &mut smash::app::Article, fighter: &mut smash::app::FighterModuleAccessor) -> u64 {
//     let ret = original!()(article, fighter);
//     println!("Initializing Air Shooter");
//     let object_id = fighter.battle_object_module_accessor.battle_object_id;
//     let object = MiscModule::get_battle_object_from_id(object_id);
//     let num = VarModule::get_int(object, vars::rockman::status::AIR_SHOOTER_NUM);
//     VarModule::set_int(&mut article.battle_object as *mut BattleObject, rockman_airshooter::instance::int::NUM, num);
//     ret
// }

pub fn install() {
    skyline::install_hooks!(
        rockman_do_leafshield_things_disable,
        rockman_do_leafshield_things_enable,
        set_leafshield,
        // rockman_airshooter_init
    );
}
