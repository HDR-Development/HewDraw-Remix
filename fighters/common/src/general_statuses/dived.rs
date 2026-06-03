// status imports
use super::*;
use globals::*;

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_KoopaDived
        );
    }
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_KoopaDived)]
pub unsafe fn status_KoopaDived(fighter: &mut L2CFighterCommon) -> L2CValue {
    AreaModule::set_whole(fighter.module_accessor, false);
    ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    let motion_share = fighter.get_param_int("param_motion", "motion_share");
    if motion_share == *FIGHTER_MOTION_SHARE_TYPE_TARO || motion_share == *FIGHTER_MOTION_SHARE_TYPE_GIRL {
        FighterMotionModuleImpl::add_body_type_hash(fighter.module_accessor, Hash40::new("koopa_dived"), *BODY_TYPE_MOTION_DX);
    }
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("koopa_dived"), 0.0, 1.0, false, 0.0, false, false);
    
    // stub the linked camera movement if not doing Flying Slam

    let pos_y = PostureModule::pos_y(fighter.module_accessor);
    fighter.set_float(pos_y, *FIGHTER_STATUS_KOOPA_DIVED_WORK_FLOAT_INIT_Y);

    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_ClungCaptain_Main as *const () as _))
}