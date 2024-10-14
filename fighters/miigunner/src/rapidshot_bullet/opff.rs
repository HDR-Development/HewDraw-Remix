utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;

pub extern "C" fn rapidshot_frame(weapon: &mut smash::lua2cpp::L2CFighterBase) {
    unsafe { 
        println!("weapon motion: {}", weapon.is_motion(Hash40::new("fly")));
        println!("weapon frame: {}", weapon.motion_frame());
        if weapon.is_motion(Hash40::new("fly")) && weapon.motion_frame() >= 7.0 {
            println!("scaling");
            ModelModule::set_scale(weapon.module_accessor, 0.4);
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, rapidshot_frame);
}