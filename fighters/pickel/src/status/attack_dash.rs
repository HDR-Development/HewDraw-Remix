use super::*;
 
// FIGHTER_STATUS_KIND_ATTACK_DASH

pub unsafe extern "C" fn attack_dash_pre(fighter: &mut L2CFighterCommon) -> L2CValue{
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_TROLLEY) {
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_S_FAILED);
        return 1.into();
    } 

    let pickel = fighter.global_table[0x5].get_ptr() as *mut BattleObjectModuleAccessor;
    if !FighterSpecializer_Pickel::check_material_special_s_generate_trolley(pickel) {
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_S_FAILED);
        return 1.into();
    }

    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_ATTACK_DASH | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_DISABLE_INTERRUPT_WARP as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_DASH as u32,
        0
    );
    
    return 0.into();
}

const FIGHTER_TEAM_2ND_PICKEL_TROLLEY: i32 = 0x1f;

pub unsafe extern "C" fn attack_dash_main(fighter: &mut L2CFighterCommon) -> L2CValue{
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_RESET);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_dash"), 0.0, 1.0, false, 0.0, false, false);

    let team_id = fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) + FIGHTER_TEAM_2ND_PICKEL_TROLLEY;
    TeamModule::set_team_second(fighter.module_accessor, team_id);
    TeamModule::set_hit_team_second(fighter.module_accessor, team_id);

    fighter.sub_shift_status_main(L2CValue::Ptr(attack_dash_main_loop as *const () as _))
}

pub unsafe extern "C" fn attack_dash_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue{
    fighter.change_status(FIGHTER_PICKEL_STATUS_KIND_SPECIAL_S_RIDE.into(), false.into());

    return 0.into();
}

pub unsafe extern "C" fn attack_dash_end(fighter: &mut L2CFighterCommon) -> L2CValue{
    KineticModule::clear_speed_all(fighter.module_accessor);
    
    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_ATTACK_DASH, attack_dash_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_DASH, attack_dash_main);
    agent.status(End, *FIGHTER_STATUS_KIND_ATTACK_DASH, attack_dash_end);
}