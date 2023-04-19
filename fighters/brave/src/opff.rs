use super::*;

utils::import_noreturn!(common::opff::fighter_common_opff);

#[smashline::fighter_init]
fn brave_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        // init roll history
        VarModule::set_int(fighter.battle_object, vars::brave::instance::SPELL_SLOT_USED_1_1, -1);
        VarModule::set_int(fighter.battle_object, vars::brave::instance::SPELL_SLOT_USED_1_2, -1);
        VarModule::set_int(fighter.battle_object, vars::brave::instance::SPELL_SLOT_USED_1_3, -1);
        VarModule::set_int(fighter.battle_object, vars::brave::instance::SPELL_SLOT_USED_1_4, -1);
        VarModule::set_int(fighter.battle_object, vars::brave::instance::SPELL_SLOT_USED_2_1, -1);
        VarModule::set_int(fighter.battle_object, vars::brave::instance::SPELL_SLOT_USED_2_2, -1);
        VarModule::set_int(fighter.battle_object, vars::brave::instance::SPELL_SLOT_USED_2_3, -1);
        VarModule::set_int(fighter.battle_object, vars::brave::instance::SPELL_SLOT_USED_2_4, -1);
    
        // roll to get one set of fresh values
        let mut vals = vec![];
        status::roll_spells(fighter, &mut vals);
    }
}

unsafe fn nspecial_cancels(fighter: &mut L2CFighterCommon) {
    //PM-like neutral-b canceling
    if fighter.is_status(*FIGHTER_BRAVE_STATUS_KIND_SPECIAL_N_CANCEL)
    && fighter.is_situation(*SITUATION_KIND_AIR)
    && WorkModule::get_int(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_N_HOLD_INT_NEXT_STATUS) == *FIGHTER_STATUS_KIND_ESCAPE_AIR
    {
        WorkModule::set_int(fighter.module_accessor, *STATUS_KIND_NONE, *FIGHTER_BRAVE_STATUS_SPECIAL_N_HOLD_INT_NEXT_STATUS);
        ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
    }
}

unsafe fn dspecial_cancels(fighter: &mut L2CFighterCommon) {
    //PM-like down-b canceling
    if fighter.is_status(*FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_CANCEL)
    && fighter.is_situation(*SITUATION_KIND_AIR)
    && WorkModule::get_int(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_HOLD_INT_NEXT_STATUS) == *FIGHTER_STATUS_KIND_ESCAPE_AIR
    {
        WorkModule::set_int(fighter.module_accessor, *STATUS_KIND_NONE, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_HOLD_INT_NEXT_STATUS);
        ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
    }
}

unsafe fn persist_rng(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_SELECT) {
        let index = fighter.get_int(*FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_SELECT_INDEX);
        VarModule::set_int(fighter.battle_object, vars::brave::instance::CURSOR_SLOT, index);
    }
    if fighter.is_status(*FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_START)
    || fighter.is_status(*FIGHTER_STATUS_KIND_DEAD) {
        VarModule::off_flag(fighter.battle_object, vars::brave::instance::PERSIST_RNG);
    }
}

// Hero dash cancel Frizz
unsafe fn dash_cancel_frizz(fighter: &mut L2CFighterCommon) {
    let mut brave_fighter = app::Fighter{battle_object: *(fighter.battle_object)};
    if fighter.is_status(*FIGHTER_BRAVE_STATUS_KIND_SPECIAL_N_SHOOT)
    && fighter.is_situation(*SITUATION_KIND_GROUND)
    && fighter.is_motion(Hash40::new("special_n1"))
    && fighter.motion_frame() > 20.0 && fighter.motion_frame() < 44.0 // after F20 and before the FAF
    && (WorkModule::get_float(fighter.module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLOAT_SP) > 12.0)
    {
        if fighter.check_dash_cancel() {
            FighterSpecializer_Brave::add_sp(&mut brave_fighter, -12.0);
            EFFECT(fighter, Hash40::new("sys_flash"), Hash40::new("top"), 0, 15, -2, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

// Hero woosh cancel
unsafe fn woosh_cancel(fighter: &mut L2CFighterCommon) {
    if fighter.is_motion_one_of(&[Hash40::new("special_hi1"), Hash40::new("special_air_hi1"), Hash40::new("special_hi_empty"), Hash40::new("special_air_hi_empty")]){
        if MotionModule::frame(fighter.module_accessor) >= 41.0 {
            VarModule::on_flag(fighter.battle_object, vars::common::instance::UP_SPECIAL_CANCEL);
            fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL, true);
        }
    }

}

#[utils::macros::opff(FIGHTER_KIND_BRAVE )]
pub unsafe fn brave_frame_wrapper(fighter: &mut L2CFighterCommon) {
    smashline::install_agent_init_callbacks!(brave_init);
    common::opff::fighter_common_opff(fighter);
    persist_rng(fighter);
    nspecial_cancels(fighter);
    dspecial_cancels(fighter);
    dash_cancel_frizz(fighter);
    woosh_cancel(fighter);

    // Extend sword length
    ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("sword1"), &Vector3f::new(1.1, 1.05, 1.045));
}