use utils::{
    *,
    ext::*,
    consts::*
};
use smash::app::BattleObjectModuleAccessor;
use smash::phx::{Vector2f, Vector3f};
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::hash40;

pub unsafe fn reset_all_magic_flags(boma: &mut BattleObjectModuleAccessor) {
    
    VarModule::off_flag(boma.object(), vars::common::instance::JAB_DA_CHECKS);
    VarModule::off_flag(boma.object(), vars::common::instance::TILT_CHECKS);
    VarModule::off_flag(boma.object(), vars::common::instance::SMASH_CHECKS);
    VarModule::off_flag(boma.object(), vars::common::instance::AERIAL_CHECKS);
}

pub unsafe fn set_all_magic_flags(boma: &mut BattleObjectModuleAccessor) {
    
    VarModule::on_flag(boma.object(), vars::common::instance::JAB_DA_CHECKS);
    VarModule::on_flag(boma.object(), vars::common::instance::TILT_CHECKS);
    VarModule::on_flag(boma.object(), vars::common::instance::SMASH_CHECKS);
    VarModule::on_flag(boma.object(), vars::common::instance::AERIAL_CHECKS);
}

pub unsafe fn reset_magic_flag(boma: &mut BattleObjectModuleAccessor, magic_level: i32) {
    
    match magic_level {
        1 => {VarModule::off_flag(boma.object(), vars::common::instance::JAB_DA_CHECKS);}
        2 => {VarModule::off_flag(boma.object(), vars::common::instance::TILT_CHECKS);}
        3 => {VarModule::off_flag(boma.object(), vars::common::instance::SMASH_CHECKS);}
        4 => {VarModule::off_flag(boma.object(), vars::common::instance::AERIAL_CHECKS);}
        _ => {reset_all_magic_flags(boma);}
    };
}

pub unsafe fn set_magic_flag(boma: &mut BattleObjectModuleAccessor, magic_level: i32) {
    
    match magic_level {
        1 => {VarModule::on_flag(boma.object(), vars::common::instance::JAB_DA_CHECKS);}
        2 => {VarModule::on_flag(boma.object(), vars::common::instance::TILT_CHECKS);}
        3 => {VarModule::on_flag(boma.object(), vars::common::instance::SMASH_CHECKS);}
        4 => {VarModule::on_flag(boma.object(), vars::common::instance::AERIAL_CHECKS);}
        _ => {set_all_magic_flags(boma);}
    };
}

pub unsafe fn run(boma: &mut BattleObjectModuleAccessor, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, fighter_kind: i32, stick_x: f32, stick_y: f32, facing: f32) {

}
