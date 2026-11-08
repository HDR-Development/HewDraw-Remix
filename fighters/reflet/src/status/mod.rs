use super::*;
use globals::*;
// status script import

mod appeal;
mod attack_air;
mod attack_100;
mod float;
mod special_n;
mod special_hi;
mod special_lw;
mod landing;

extern "Rust" {
    #[link_name = "float_check_air_jump"]
    fn float_check_air_jump(fighter: &mut L2CFighterCommon, float_status: L2CValue) -> L2CValue;
    #[link_name = "float_check_air_jump_aerial"]
    fn float_check_air_jump_aerial(fighter: &mut L2CFighterCommon, float_status: L2CValue) -> L2CValue;
}

unsafe extern "C" fn reflet_air_jump_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT) <= 0 {
        return false.into();
    }
    float_check_air_jump(fighter, statuses::reflet::FLOAT.into())
}

unsafe extern "C" fn reflet_air_jump_aerial_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT) <= 0 {
        return false.into();
    }
    float_check_air_jump_aerial(fighter, statuses::reflet::FLOAT.into())
}

unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_GROUND) || fighter.is_situation(*SITUATION_KIND_CLIFF)
    || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_GIMMICK_SPRING_JUMP]) {
        VarModule::off_flag(fighter.battle_object, vars::reflet::instance::SPECIAL_HI_ENABLE_FREEFALL);
        VarModule::off_flag(fighter.battle_object, vars::common::instance::STALL_PREVENTION);
    }
    if fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_FALL, statuses::reflet::FLOAT]) {
        spawn_items(fighter);
    }
    true.into()
}

unsafe extern "C" fn reflet_on_start(fighter: &mut L2CFighterCommon) {
    fighter.global_table[0x32].assign(&L2CValue::Ptr(reflet_air_jump_uniq as *const () as _));
    fighter.global_table[0x33].assign(&L2CValue::Ptr(reflet_air_jump_aerial_uniq as *const () as _));
    fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));
    VarModule::set_int(fighter.battle_object, vars::common::instance::FLOAT_STATUS_KIND, statuses::reflet::FLOAT);
    VarModule::set_int(fighter.battle_object, vars::reflet::instance::DISCARD_TYPE, -1);//0 is thunder
}

unsafe fn spawn_items(fighter: &mut L2CFighterCommon) {
    let discard_type = VarModule::get_int(fighter.battle_object, vars::reflet::instance::DISCARD_TYPE);
    if discard_type > -1 
    && !VarModule::is_flag(fighter.battle_object, vars::reflet::instance::DISCARD_SKIP_STATUS) {
        if ItemModule::is_have_item(fighter.module_accessor, 0) {
            ItemModule::throw_item(fighter.module_accessor, 110.0, 1.6, 1.0, 0, true, fighter.get_float(*ITEM_FIGHTER_VAR_FLOAT_ITEM_THROW_POWER));//should have no hitbox maybe?
            //ItemModule::drop_item(fighter.module_accessor, 90.0, 0.0, 0);
            //could copy vanilla and just make new item be the one to toss/drop
        }
        if discard_type != *FIGHTER_REFLET_MAGIC_KIND_SWORD {
            ItemModule::have_item(fighter.module_accessor, app::ItemKind(*ITEM_KIND_BOOK), 0, 0, false, false);
            let item_id = ItemModule::get_have_item_id(fighter.module_accessor, 0);
            let item_boma = sv_battle_object::module_accessor(item_id as u32);
            MotionModule::set_rate_material(item_boma, 0.0, MaterialAnimeKind{_address: 0});
            MotionModule::set_frame_material(item_boma, discard_type as f32, MaterialAnimeKind{_address: 0});
        } else {
            ItemModule::have_item(fighter.module_accessor, app::ItemKind(*ITEM_KIND_THUNDERSWORD), 0, 0, false, false);
        }
        VarModule::set_int(fighter.battle_object, vars::reflet::instance::DISCARD_TYPE, -1);
    }
}

pub unsafe extern "C" fn mot_handler(fighter: &mut L2CFighterCommon) -> L2CValue {//van handling
    let mot_gr = fighter.get_int64(*FIGHTER_REFLET_STATUS_COMMON_INT_MOTION_KIND_GROUND);
    let mot_air = fighter.get_int64(*FIGHTER_REFLET_STATUS_COMMON_INT_MOTION_KIND_AIR);
    let kin_gr = fighter.get_int(*FIGHTER_REFLET_STATUS_COMMON_INT_KINETIC_GROUND);
    let kin_air = fighter.get_int(*FIGHTER_REFLET_STATUS_COMMON_INT_KINETIC_AIR);
    let cor_gr = fighter.get_int(*FIGHTER_REFLET_STATUS_COMMON_INT_CORRECT_GROUND);
    let cor_air = fighter.get_int(*FIGHTER_REFLET_STATUS_COMMON_INT_CORRECT_AIR);
    if StatusModule::is_changing(fighter.module_accessor) {
        fighter.ground_correct_by_situation(cor_gr, cor_air);
        fighter.change_kinetic_by_situation(kin_gr, kin_air);
        fighter.sub_change_motion_by_situation(Hash40::new_raw(mot_gr).into(), Hash40::new_raw(mot_air).into(), false.into());
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        fighter.ground_correct_by_situation(cor_gr, cor_air);
        fighter.change_kinetic_by_situation(kin_gr, kin_air);
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(mot_gr), -1.0, 1.0, 0.0, false, false);
        } else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(mot_air), -1.0, 1.0, 0.0, false, false);
        }
    }
    0.into()
}

pub unsafe fn CHECK_MAGIC(fighter: &mut L2CAgentBase) -> bool {
    let magic = fighter.get_int(*FIGHTER_REFLET_INSTANCE_WORK_ID_INT_LAST_USED_MAGIC_KIND);
    let tome_level = match magic {
        _ if magic == *FIGHTER_REFLET_MAGIC_KIND_GIGA_FIRE => fighter.get_float(*FIGHTER_REFLET_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_CURRENT_POINT).ceil() as i32,
        _ if magic == *FIGHTER_REFLET_MAGIC_KIND_EL_WIND => fighter.get_int(*FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT),
        _ if magic == *FIGHTER_REFLET_MAGIC_KIND_RIZAIA => fighter.get_int(*FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_LW_CURRENT_POINT),
        _ => fighter.get_int(*FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_CURRENT_POINT),
    };
    if tome_level > 0 {
        return true.into();
    }
    FighterSpecializer_Reflet::change_grimoire(fighter.module_accessor as *mut FighterModuleAccessor, -1);
    false.into()
}

pub fn install(agent: &mut Agent) {
    agent.on_start(reflet_on_start);

    appeal::install(agent);
    attack_air::install(agent);
    attack_100::install(agent);
    float::install(agent);
    special_n::install(agent);
    special_hi::install(agent);
    special_lw::install(agent);
    landing::install(agent);
}
