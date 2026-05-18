// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

extern "Rust" {
    fn gimmick_flash(boma: &mut BattleObjectModuleAccessor);
}

// Power Board Death Reset
unsafe fn var_reset(fighter: &mut L2CFighterCommon) {
    if fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_REBIRTH]) {
        VarModule::on_flag(fighter.object(), vars::palutena::instance::SPECIAL_N_FLUSH_BOARD);
    }
    if fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_WIN, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_ENTRY])
    || !sv_information::is_ready_go() {
        VarModule::on_flag(fighter.object(), vars::palutena::instance::SPECIAL_N_FLUSH_BOARD);
    }
}

// sets set_color var, controlling when a color is charged
unsafe fn color_charge(fighter: &mut L2CFighterCommon) {
    if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD)
    && !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_PARRY)
    && VarModule::is_flag(fighter.object(), vars::palutena::status::ENABLE_COLOR_INCREMENT) {
        VarModule::off_flag(fighter.object(), vars::palutena::status::ENABLE_COLOR_INCREMENT);
        // yellow moves: side
        if fighter.is_motion(Hash40::new("attack_s3_s"))
        || fighter.is_motion(Hash40::new("attack_s4_s"))
        || fighter.is_motion(Hash40::new("attack_air_f"))
        || fighter.is_motion(Hash40::new("attack_air_b")) {
            VarModule::set_int(fighter.object(), vars::palutena::instance::SPECIAL_N_GAINED_COLOR, 3);
        }

        // blue moves: up
        else if fighter.is_motion(Hash40::new("attack_hi3"))
        || fighter.is_motion(Hash40::new("attack_hi4"))
        || fighter.is_motion(Hash40::new("attack_air_hi")) {
            VarModule::set_int(fighter.object(), vars::palutena::instance::SPECIAL_N_GAINED_COLOR, 2);
        }

        // red moves: down
        else if fighter.is_motion(Hash40::new("attack_lw3"))
        || fighter.is_motion(Hash40::new("attack_lw4"))
        || fighter.is_motion(Hash40::new("attack_air_lw")) {
            VarModule::set_int(fighter.object(), vars::palutena::instance::SPECIAL_N_GAINED_COLOR, 1);
        }
    }
}

// handles the color charges
unsafe fn power_board(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    // check if we should gain a color
    if VarModule::get_int(fighter.object(), vars::palutena::instance::SPECIAL_N_GAINED_COLOR) != 0 {
        // set slot 2 to old slot 1, slot 1 becomes new color; fill up 1 stock if possible
        VarModule::set_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_2, VarModule::get_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_1));
        VarModule::set_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_1, VarModule::get_int(fighter.object(), vars::palutena::instance::SPECIAL_N_GAINED_COLOR));
        VarModule::set_int(boma.object(), vars::palutena::instance::SPECIAL_N_GAINED_COLOR, 0);
        utils::ui::UiManager::change_power_board_color(
            fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32,
            VarModule::get_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_1),
            VarModule::get_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_2)
        );
    }

    // check if we should flush our power board
    if VarModule::is_flag(fighter.object(), vars::palutena::instance::SPECIAL_N_FLUSH_BOARD) {
        // set each slot to 0
        VarModule::set_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_2, 0);
        VarModule::set_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_1, 0);
        VarModule::off_flag(fighter.object(), vars::palutena::instance::SPECIAL_N_FLUSH_BOARD);
        utils::ui::UiManager::change_power_board_color(
            fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32,
            VarModule::get_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_1),
            VarModule::get_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_2)
        );

        VarModule::on_flag(fighter.object(), vars::palutena::status::POWER_BOARD_FLUSHED);
    }
    
    utils::ui::UiManager::change_power_board_color(
        fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32,
        VarModule::get_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_1),
        VarModule::get_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_2)
    );
    
}

pub extern "C" fn palu_power_board(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        if !sv_information::is_ready_go() && fighter.status_frame() < 1 {
            return;
        }
        utils::ui::UiManager::set_power_board_enable(fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32, true);
        utils::ui::UiManager::set_power_board_info(
            fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32,
            VarModule::get_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_1),
            VarModule::get_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_2)
        );
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    var_reset(fighter);
    power_board(fighter, boma);
    color_charge(fighter);
}

pub extern "C" fn palutena_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		palutena_frame(fighter)
    }
}

pub unsafe fn palutena_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, palutena_frame_wrapper);
    agent.on_line(Main, palu_power_board);
}