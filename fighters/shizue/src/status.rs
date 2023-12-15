use super::*;

mod special_hi;
mod clayrocket;


#[smashline::fighter_init]
fn shizue_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        // set the callbacks on fighter init
        if fighter.kind() == *FIGHTER_KIND_SHIZUE {
            // empty
        }
    }
}


pub fn install() {
    smashline::install_agent_init_callbacks!(shizue_init);
    special_hi::install();
    clayrocket::install();
}