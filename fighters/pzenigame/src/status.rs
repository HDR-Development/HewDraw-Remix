use super::*;
use globals::*;
// status script import
 
pub unsafe fn end_run(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_Run();
    MotionAnimcmdModule::enable_skip_delay_update(fighter.module_accessor);
    0.into()
}

pub fn install() {
    install_status_scripts!(
        end_run
    );
}