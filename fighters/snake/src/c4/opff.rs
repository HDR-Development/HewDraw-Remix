// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe extern "C" fn c4_callback(weapon: &mut smash::lua2cpp::L2CFighterBase) {
    if weapon.is_status(*WEAPON_SNAKE_C4_STATUS_KIND_STICK_TARGET) {
        let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        let snake = utils::util::get_battle_object_from_id(owner_id);
        let snake_boma = &mut *(*snake).module_accessor;
        let snake_status_kind = StatusModule::status_kind(snake_boma);
        // Despawn sticky when snake dies
        if snake_status_kind == *FIGHTER_STATUS_KIND_DEAD {
            ArticleModule::remove_exist(snake_boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, c4_callback);
}