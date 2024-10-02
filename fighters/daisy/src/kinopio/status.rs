use super::*;
use vars::daisy_kinopio::status::*;

unsafe extern "C" fn yap_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor, 
        app::SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NONE,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        0,
        0,
        0,
        0
    );

    0.into()
}

unsafe extern "C" fn yap_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    VarModule::off_flag(weapon.object(), YAP_ON);
    VarModule::off_flag(weapon.object(), YAP_OFF);
    VarModule::set_int(weapon.object(), YAPPING_TIMER, 999);
    VisibilityModule::set_whole(weapon.module_accessor, false);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("yapnt"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(yap_main_loop as *const () as _))
}

unsafe extern "C" fn yap_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let yap_timer = VarModule::get_int(weapon.object(), YAPPING_TIMER);
    if yap_timer > 0 { VarModule::dec_int(weapon.object(), YAPPING_TIMER); }
    let idle_frames = 40;   // how long the flower sticks around after it's done talking

    if weapon.status_frame() == 3 {
        VisibilityModule::set_whole(weapon.module_accessor, true);
    }
    let start_frame = if VarModule::is_flag(weapon.object(), PARRY_YAP) { 10 } else { 30 };
    if weapon.status_frame() == start_frame {
        let mut quote_data: [&str;2] = ["dummy", "dummy"];
        let mut yapping_frames = 0; // approximate amount of frames each line takes to complete
        let rng = app::sv_math::rand(hash40("fighter"), 25);
        match (rng as i32) { // assign data for what the flower will say based on rng
            0 => { quote_data = ["se_daisy_appeal_x01_onward", "daisy_flower_bubble_01"]; yapping_frames = 84; },
            1 => { quote_data = ["se_daisy_appeal_x02_company", "daisy_flower_bubble_02"]; yapping_frames = 82; },
            2 => { quote_data = ["se_daisy_appeal_x03_great", "daisy_flower_bubble_03"]; yapping_frames = 76; },
            3 => { quote_data = ["se_daisy_appeal_x04_everyday", "daisy_flower_bubble_04"]; yapping_frames = 95; },
            4 => { quote_data = ["se_daisy_appeal_x05_feelsoff", "daisy_flower_bubble_05"]; yapping_frames = 173; },
            5 => { quote_data = ["se_daisy_appeal_x06_focus", "daisy_flower_bubble_06"]; yapping_frames = 125; },
            6 => { quote_data = ["se_daisy_appeal_x07_howyadoin", "daisy_flower_bubble_07"]; yapping_frames = 60; },
            7 => { quote_data = ["se_daisy_appeal_x08_something", "daisy_flower_bubble_08"]; yapping_frames = 80; },
            8 => { quote_data = ["se_daisy_appeal_x09_keeptrying", "daisy_flower_bubble_09"]; yapping_frames = 124; },
            9 => { quote_data = ["se_daisy_appeal_x10_almost", "daisy_flower_bubble_10"]; yapping_frames = 94; },
            10 => { quote_data = ["se_daisy_appeal_x11_goodday", "daisy_flower_bubble_11"]; yapping_frames = 122; },
            11 => { quote_data = ["se_daisy_appeal_x12_newspecies", "daisy_flower_bubble_12"]; yapping_frames = 92; },
            12 => { quote_data = ["se_daisy_appeal_x13_taste", "daisy_flower_bubble_13"]; yapping_frames = 95; },
            13 => { quote_data = ["se_daisy_appeal_x14_nexttime", "daisy_flower_bubble_14"]; yapping_frames = 66; },
            14 => { quote_data = ["se_daisy_appeal_x15_ohhey", "daisy_flower_bubble_15"]; yapping_frames = 60; },
            15 => { quote_data = ["se_daisy_appeal_x16_party", "daisy_flower_bubble_16"]; yapping_frames = 125; },
            16 => { quote_data = ["se_daisy_appeal_x17_peaceful", "daisy_flower_bubble_17"]; yapping_frames = 94; },
            17 => { quote_data = ["se_daisy_appeal_x18_rooting", "daisy_flower_bubble_18"]; yapping_frames = 84; },
            18 => { quote_data = ["se_daisy_appeal_x19_sogood", "daisy_flower_bubble_19"]; yapping_frames = 92; },
            19 => { quote_data = ["se_daisy_appeal_x20_summoned", "daisy_flower_bubble_20"]; yapping_frames = 98; },
            20 => { quote_data = ["se_daisy_appeal_x21_surprise", "daisy_flower_bubble_21"]; yapping_frames = 62; },
            21 => { quote_data = ["se_daisy_appeal_x22_wellthen", "daisy_flower_bubble_22"]; yapping_frames = 40; },
            22 => { quote_data = ["se_daisy_appeal_x23_what", "daisy_flower_bubble_23"]; yapping_frames = 88; },
            23 => { quote_data = ["se_daisy_appeal_x24_where", "daisy_flower_bubble_24"]; yapping_frames = 65; },
            _ => { quote_data = ["se_daisy_appeal_x25_yikes", "daisy_flower_bubble_25"]; yapping_frames = 36; }
        };
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("yap"), 0.0, 1.0, false, 0.0, false, false);
        SoundModule::play_se(weapon.module_accessor, Hash40::new(quote_data[0]), true, false, false, false, app::enSEType(0));
        let effect_id = EffectModule::req_on_joint(weapon.module_accessor, Hash40::new(quote_data[1]), Hash40::new("top"), &Vector3f::new(0.5, 11.25, 0.0), &Vector3f::zero(), 0.6, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0);
        VarModule::set_int(weapon.object(), FLOWER_EFFECT_ID, effect_id as i32);
        VarModule::set_int(weapon.object(), YAPPING_TIMER, (idle_frames + yapping_frames));
        VarModule::set_int(weapon.object(), FLOWER_EFFECT_FRAMES, 5);
        VarModule::on_flag(weapon.object(), YAP_ON);
    }

    // handles the animation of the speech bubble effect
    let effect = VarModule::get_int(weapon.object(), FLOWER_EFFECT_ID);
    let effect_frame = VarModule::get_int(weapon.object(), FLOWER_EFFECT_FRAMES) as f32;
    if effect_frame as i32 > 0 { VarModule::dec_int(weapon.object(), FLOWER_EFFECT_FRAMES); }
    // speech bubble entry
    if VarModule::is_flag(weapon.object(), YAP_ON) {
        if effect_frame > 0.0 {
            let mul = 1.0 - (effect_frame * 0.05);
            EffectModule::set_scale(weapon.module_accessor, effect as u32, &Vector3f::new(1.4 * mul, 1.6 * mul, 1.4 * mul));
            EffectModule::set_alpha(weapon.module_accessor, effect as u32, 1.0 - (0.2 * effect_frame));
        } else {
            EffectModule::set_scale(weapon.module_accessor, effect as u32, &Vector3f::new(1.4, 1.6, 1.4));
            EffectModule::set_alpha(weapon.module_accessor, effect as u32, 1.0);
            VarModule::off_flag(weapon.object(), YAP_ON);
        }
    }
    // speech bubble exit
    if VarModule::is_flag(weapon.object(), YAP_OFF) {
        if effect_frame > 0.0 {
            let mul = 0.75 + (effect_frame * 0.05);
            EffectModule::set_scale(weapon.module_accessor, effect as u32, &Vector3f::new(1.4 * mul, 1.6 * mul, 1.4 * mul));
            EffectModule::set_alpha(weapon.module_accessor, effect as u32, (0.1 * effect_frame));
        } else {
            EffectModule::kill(weapon.module_accessor, effect as u32, true, true);
            VarModule::off_flag(weapon.object(), YAP_OFF);
        }
    }

    // when the dialogue is over, return to idle and remove the effect
    if yap_timer == idle_frames {
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("yapnt"), 0.0, 1.0, false, 0.0, false, false);
        VarModule::set_int(weapon.object(), FLOWER_EFFECT_FRAMES, 5);
        VarModule::on_flag(weapon.object(), YAP_OFF);
    }

    // removes the flower once all is said and done
    if yap_timer <= 0 {
        let exit_effect = EffectModule::req_on_joint(weapon.module_accessor, Hash40::new("sys_erace_smoke"), Hash40::new("top"), &Vector3f::new(0.2, 4.5, 0.0), &Vector3f::zero(), 0.6, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0);
        EffectModule::set_rate(weapon.module_accessor, exit_effect as u32, 1.0);
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        return 1.into();
    }

    return 0.into();
}

unsafe extern "C" fn yap_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, statuses::daisy_kinopio::YAP, yap_pre);
    agent.status(Main, statuses::daisy_kinopio::YAP, yap_main);
    agent.status(End, statuses::daisy_kinopio::YAP, yap_end);
}