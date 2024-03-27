use super::*;
use globals::*;
// status script import

mod special_hi;
mod special_lw; 
mod attack_lw4;

// handle damage to belly
#[no_mangle]
pub unsafe extern "C" fn krool_belly_damage_hook_impl(damage: f32, fighter: *mut Fighter, unk: bool) {
    let mut battle_object = &mut (*fighter).battle_object;
    let boma = battle_object.module_accessor;
    let mut waist = WorkModule::get_float(boma, 0x4d);  // WAIST_LIFE

    // play belly flash
    WorkModule::on_flag(boma, 0x200000e3);              // *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_WAIST_HIT_FLASH
    WorkModule::set_int(boma, 0x1e, 0x100000c1);        // *FIGHTER_KROOL_INSTANCE_WORK_ID_INT_WAIST_HIT_FLASH_COUNT

    // store incoming damage
    let stored_damage = VarModule::get_float(battle_object, vars::krool::instance::STORED_DAMAGE);
    VarModule::set_float(battle_object, vars::krool::instance::STORED_DAMAGE, f32::min(stored_damage + damage, 45.0));

    if damage > ParamModule::get_float(battle_object, ParamType::Agent, "param_waist.deplete_damage_min") {
        // decrease belly health
        waist -= 1.0;
        WorkModule::set_float(boma, waist, 0x4d);

        // critical zoom if out of health
        if WorkModule::get_float(boma, 0x4d) <= 0.0 {
            MotionAnimcmdModule::call_script_single(boma, *FIGHTER_ANIMCMD_SOUND, Hash40::new("sound_waistbreak"), -1);
        }
        else {
            let krool = utils::util::get_fighter_common_from_accessor(&mut (*boma));
            PLAY_SE(krool, Hash40::new("se_krool_damage_clack"));
        }
    }
    else {
        let krool = utils::util::get_fighter_common_from_accessor(&mut (*boma));
        PLAY_SE(krool, Hash40::new("se_krool_special_n11"));    //s07 l01, l02 l05
    }

    // disable belly for the rest of the move
    WorkModule::on_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_OFF);
}

// handle toggling belly on/off
// #[no_mangle]
// pub unsafe extern "C" fn krool_belly_toggle_hook(ctx: &mut skyline::hooks::InlineCtx) {
//     *ctx.registers[0].x.as_mut() = 0;    // bool for toggle
//     // ...as_mut() &= logic
// }

pub fn install(agent: &mut Agent) {
    special_hi::install(agent);
    special_lw::install(agent);
    attack_lw4::install(agent);
}