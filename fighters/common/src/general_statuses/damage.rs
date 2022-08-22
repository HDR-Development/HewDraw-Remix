// status imports
use super::*;
use globals::*;

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            FighterStatusUniqProcessDamage_leave_stop_hook,
        );
    }
}

// this runs as you leave hitlag
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_FighterStatusUniqProcessDamage_leave_stop)]
pub unsafe fn FighterStatusUniqProcessDamage_leave_stop_hook(fighter: &mut L2CFighterCommon, arg2: L2CValue, arg3: L2CValue) -> L2CValue {
    original!()(fighter, arg2, arg3);
    let hashmap = fighter.local_func__fighter_status_damage_2();
    let sdi_mul = hashmap["stop_delay_"].get_f32();
    // get stick x/y length
    // uses cstick's value if cstick is on (for Double Stick DI)
    let stick_x = if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) {
        ControlModule::get_sub_stick_x(fighter.module_accessor)
    }
    else {
        ControlModule::get_stick_x(fighter.module_accessor)
    };
    let stick_y = if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) {
        ControlModule::get_sub_stick_y(fighter.module_accessor)
    }
    else {
        ControlModule::get_stick_y(fighter.module_accessor)
    };
    // get base asdi distance
    let base_asdi = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("hit_stop_delay_auto_mul"));
    // mul sdi_mul by hit_stop_delay_auto_mul = total sdi
    let asdi = sdi_mul * base_asdi;
    // mul stick x/y by total sdi
    let asdi_x = asdi * stick_x;
    let asdi_y = asdi * stick_y;
    // get current pos
    let mut pos = Vector3f {
        x: PostureModule::pos_x(fighter.module_accessor),
        y: PostureModule::pos_y(fighter.module_accessor),
        z: PostureModule::pos_z(fighter.module_accessor)
    };
    // add asdi x/y to pos
    pos.x += asdi_x;
    pos.y += asdi_y;
    PostureModule::set_pos(fighter.module_accessor, &Vector3f{x: pos.x, y: pos.y, z: pos.z});
    // make sure we can enter tech/missed tech on f1 of damage fly statuses (vanilla only allows them starting on f3)
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_ENABLE_DOWN);
    return 0.into()
}