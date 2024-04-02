use super::*;
use globals::*;

// WEAPON_LUCARIO_AURABALL_STATUS_KIND_SHOOT

pub unsafe extern "C" fn shoot_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(fighter.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    VarModule::set_flag(fighter.battle_object, vars::lucario::instance::IS_POWERED_UP, VarModule::is_flag(owner_module_accessor.object(), vars::lucario::instance::IS_POWERED_UP));
    //println!("lucario_auraball is_powered_up: {}", VarModule::is_flag(fighter.battle_object, vars::lucario::instance::IS_POWERED_UP));
    smashline::original_status(Pre, fighter, *WEAPON_LUCARIO_AURABALL_STATUS_KIND_SHOOT)(fighter)
}

// WEAPON_LUCARIO_AURABALL_STATUS_KIND_START

// #[status_script(agent = "lucario_auraball", status = WEAPON_LUCARIO_AURABALL_STATUS_KIND_START, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
// pub unsafe fn auraball_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
//     MotionModule::change_motion(fighter.module_accessor, Hash40::new("start"), 0.0, 1.0, false, 0.0, false, false);
//     auraball_set_scale(fighter);
//     fighter.fastshift(L2CValue::Ptr(auraball_start_main_loop as *const () as _))
// }

// unsafe extern "C" fn auraball_start_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
//     return 0.into();
// }

// WEAPON_LUCARIO_AURABALL_STATUS_KIND_CHARGE

// #[status_script(agent = "lucario_auraball", status = WEAPON_LUCARIO_AURABALL_STATUS_KIND_CHARGE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
// pub unsafe fn auraball_charge_main(fighter: &mut L2CFighterCommon) -> L2CValue {
//     let charge_frame = fighter.get_int(*WEAPON_LUCARIO_AURABALL_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
//     let max_charge_frame = fighter.get_int(*WEAPON_LUCARIO_AURABALL_INSTANCE_WORK_ID_INT_PARAM_MAX_CHARGE_FRAME);
//     let motion_kind = if charge_frame >= max_charge_frame {Hash40::new("charge_max")} else {Hash40::new("charge")};
//     MotionModule::change_motion(fighter.module_accessor, Hash40::new("charge"), 0.0, 1.0, false, 0.0, false, false);
//     auraball_set_scale(fighter);
//     fighter.fastshift(L2CValue::Ptr(auraball_charge_main_loop as *const () as _))
// }

// unsafe extern "C" fn auraball_charge_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
//     let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
//     if motion_kind != hash40("charge") {
//         return 0.into();
//     }
//     let charge_frame = fighter.get_int(*WEAPON_LUCARIO_AURABALL_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
//     let max_charge_frame = fighter.get_int(*WEAPON_LUCARIO_AURABALL_INSTANCE_WORK_ID_INT_PARAM_MAX_CHARGE_FRAME);
//     if charge_frame >= max_charge_frame {
//         MotionModule::change_motion(fighter.module_accessor, Hash40::new("charge_max"), 0.0, 1.0, false, 0.0, false, false);
//     }
//     return 0.into();
// }

// unsafe extern "C" fn auraball_set_scale(fighter: &mut L2CFighterCommon) {
//     let original_size = fighter.get_param_float("param_auraball", "original_size");
//     let max_charge_frame = fighter.get_int(*WEAPON_LUCARIO_AURABALL_INSTANCE_WORK_ID_INT_PARAM_MAX_CHARGE_FRAME);
//     let charge_frame = fighter.get_int(*WEAPON_LUCARIO_AURABALL_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
//     let charge_ratio = (charge_frame as f32) / (max_charge_frame as f32);
//     println!("charge_ratio: {}", charge_ratio);

//     let mut hvar4 = "";
//     let mut min_scale = 0.0;
//     let mut max_scale = 0.0;
//     if StatusModule::status_kind(fighter.module_accessor) == *WEAPON_LUCARIO_AURABALL_STATUS_KIND_SHOOT {
//         min_scale = fighter.get_param_float("param_auraball", "charge_min_scale_shoot");
//         hvar4 = "charge_max_scale_shoot";
//     } else {
//         hvar4 = "charge_max_scale_mid";
//         min_scale = fighter.get_param_float("param_auraball", "charge_min_scale");
//         if 1.0 <= charge_ratio {
//             println!("if conditional");
//             max_scale = fighter.get_param_float("param_auraball", "charge_max_scale");
//             let fvar9 = fighter.get_param_float("param_auraball", "charge_max_scale_mid");
//             fighter.set_float(fvar9 / max_scale, *WEAPON_LUCARIO_AURABALL_INSTANCE_WORK_ID_FLOAT_EFFECT_JOINT_SCALE);
//             hvar4 = "charge_max_scale";
//         }
//     }
//     max_scale = fighter.get_param_float("param_auraball", hvar4);

//     let work_scale = fighter.get_float(*WEAPON_LUCARIO_AURABALL_INSTANCE_WORK_ID_FLOAT_SCALE);
//     let work_aurapower = fighter.get_float(*WEAPON_LUCARIO_AURABALL_INSTANCE_WORK_ID_FLOAT_AURAPOWER);
//     let scale_mul = fighter.get_param_float("param_auraball", "scale_mul");
//     let scale_add = fighter.get_param_float("param_auraball", "scale_add");

//     let scale = work_scale * (1.0 / original_size) * 
//         ((charge_ratio * max_scale + (1.0 - charge_ratio) * min_scale) * work_aurapower * scale_mul + scale_add);
//     println!("scale: {}, work_scale: {}, original_size: {}, charge_ratio: {}, max_scale: {}, min_scale: {}, work_aurapower: {}, scale_mul: {}, scale_add: {}", 
//         scale, work_scale, original_size, charge_ratio, max_scale, min_scale, work_aurapower, scale_mul, scale_add);
//     PostureModule::set_scale(fighter.module_accessor, scale, false);

// }

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *WEAPON_LUCARIO_AURABALL_STATUS_KIND_SHOOT, shoot_pre);
}