use super::*;

unsafe extern "C" fn attack_s4_start_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    //println!("s4 start exit");
    //println!("start next: {:x}", StatusModule::status_kind_next(fighter.module_accessor));
    let ret = smashline::original_status(Exit, fighter, *FIGHTER_STATUS_KIND_ATTACK_S4_START)(fighter);
    if ![*FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD].contains(&StatusModule::status_kind_next(fighter.module_accessor)) {
        ArticleModule::remove_exist(fighter.module_accessor, articles::plizardon::ROCK, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }

    ret
}

unsafe extern "C" fn attack_s4_hold_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    //println!("s4 hold exit");
    //println!("hold next: {:x}", StatusModule::status_kind_next(fighter.module_accessor));
    if StatusModule::status_kind_next(fighter.module_accessor) != *FIGHTER_STATUS_KIND_ATTACK_S4 {
        ArticleModule::remove_exist(fighter.module_accessor, articles::plizardon::ROCK, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    let ret = smashline::original_status(Exit, fighter, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD)(fighter);
    //println!("original called");

    ret
}

unsafe extern "C" fn attack_s4_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    //println!("s4 exit");
    //println!("s4 next: {:x}", StatusModule::status_kind_next(fighter.module_accessor));
    if StatusModule::status_kind_next(fighter.module_accessor) != *FIGHTER_STATUS_KIND_ATTACK_S4 {
        ArticleModule::remove_exist(fighter.module_accessor, articles::plizardon::ROCK, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Exit, *FIGHTER_STATUS_KIND_ATTACK_S4_START, attack_s4_start_exit);
    agent.status(Exit, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, attack_s4_hold_exit);
    agent.status(Exit, *FIGHTER_STATUS_KIND_ATTACK_S4, attack_s4_exit);
}