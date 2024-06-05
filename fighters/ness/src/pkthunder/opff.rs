// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe extern "C" fn pk_thunder_angle(weapon: &mut L2CWeaponCommon) {
    if AttackModule::is_attack(weapon.module_accessor, 0, false) {
        let mut thunder_angle = WorkModule::get_float(weapon.module_accessor, *WEAPON_NESS_PK_THUNDER_STATUS_WORK_ID_FLOAT_ANGLE).to_degrees();
        let mut launch_angle = (thunder_angle + 180.0) % 360.0;
        while thunder_angle < 0.0 {
            thunder_angle += 360.0;
          }
        if (90.0..270.0).contains(&thunder_angle) {
            launch_angle = 180.0 - launch_angle;
        }
        AttackModule::set_vector(weapon.module_accessor, 0, launch_angle as i32, false);
    }
}

pub unsafe extern "C" fn pkthunder_callback(weapon: &mut smash::lua2cpp::L2CFighterBase) {
    WorkModule::on_flag(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_FLAG_NO_DEAD);
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, pkthunder_callback);
    agent.on_line(Main, pk_thunder_angle);
}