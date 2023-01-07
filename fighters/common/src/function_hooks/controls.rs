use super::*;
use utils::ext::*;
use rand::prelude::SliceRandom;
use rand::Rng;

#[skyline::hook(offset = 0x16d948c, inline)]
unsafe fn packed_packet_creation(ctx: &mut skyline::hooks::InlineCtx) {
    // *ctx.registers[8].x.as_mut() |= 0xFC_00000000;
    *ctx.registers[22].x.as_mut() = 0x2;
    // println!("{:#x} | {:#x}", *ctx.registers[8].x.as_ref(), *ctx.registers[22].x.as_ref());
}

#[skyline::hook(offset = 0x16d94c0, inline)]
unsafe fn write_packet(ctx: &mut skyline::hooks::InlineCtx) {
    let raw = *ctx.registers[19].x.as_ref();

    let mapped_inputs = *((raw + 0x49508) as *const MappedInputs);
    let mut packet = 0u64;

    *(&mut packet as *mut u64 as *mut i8) = mapped_inputs.lstick_x;
    *(&mut packet as *mut u64 as *mut i8).add(1) = mapped_inputs.lstick_y;

    let buttons = (mapped_inputs.buttons.bits() as u64) << 16;
    packet |= buttons;

    *(&mut packet as *mut u64 as *mut i8).add(6) = mapped_inputs.rstick_x;
    *(&mut packet as *mut u64 as *mut i8).add(7) = mapped_inputs.rstick_y;

    *ctx.registers[8].x.as_mut() = packet;
}

#[repr(C)]
struct SomeControllerStruct {
    padding: [u8; 0x10],
    controller: &'static mut Controller
}

unsafe fn get_player_idx_from_boma(boma: u64) -> i32 {
    let control_module = *((boma + 0x48) as *const u64);
    let next = *((control_module + 0x118) as *const u64);
    let next = *((next + 0x58) as *const u64);
    let next = *((next + 0x8) as *const u64);
    *((next + 0x8) as *const i32)
}

macro_rules! apply_button_mappings {
    ($controller:ident, $mappings:ident, $(($button:ident, $mapped:ident, $kind:ident, $output:expr))*) => {{
        let mut buttons = Buttons::empty();
        $(
                if $controller.current_buttons.$button() && (*$mappings).$mapped == InputKind::$kind {
                    buttons |= $output;
                }
        )*
        buttons
    }}
}

#[skyline::hook(offset = offsets::map_controls())]
unsafe fn map_controls_hook(
    mappings: *mut ControllerMapping,
    player_idx: i32,
    out: *mut MappedInputs,
    controller_struct: &mut SomeControllerStruct,
    arg: bool
) {
    let entry_count = (*mappings.add(player_idx as usize))._34[0];
    let ret = original!()(mappings, player_idx, out, controller_struct, arg);
    let controller = &mut controller_struct.controller;

    //println!("entry_count vs. current: {} vs. {}", entry_count, (*mappings.add(player_idx as usize))._34[0]);

    if (*out).buttons.contains(Buttons::CStickOn) && (*mappings.add(player_idx as usize))._34[0] != entry_count {
        (*out).rstick_x = (controller.left_stick_x * (i8::MAX as f32)) as i8;
        (*out).rstick_y = (controller.left_stick_y * (i8::MAX as f32)) as i8;
        (*out).buttons |= Buttons::CStickOverride;
    } else {
        (*out).rstick_x = (controller.right_stick_x * (i8::MAX as f32)) as i8;
        (*out).rstick_y = (controller.right_stick_y * (i8::MAX as f32)) as i8;
    }

    let mappings = mappings.add(player_idx as usize);

    if controller.style == ControllerStyle::GCController {
        (*out).buttons |= apply_button_mappings!(
            controller,
            mappings,
                (l, gc_l, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (r, gc_r, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (zl, gc_z, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (zr, gc_z, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (a, gc_a, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (b, gc_b, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (x, gc_x, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (y, gc_y, JumpMini, Buttons::JumpMini | Buttons::Jump)
        );
        (*out).buttons |= apply_button_mappings!(
            controller,
            mappings,
                (l, gc_l, SmashAttack, Buttons::AttackAll)
                (r, gc_r, SmashAttack, Buttons::AttackAll)
                (zl, gc_z, SmashAttack, Buttons::AttackAll)
                (zr, gc_z, SmashAttack, Buttons::AttackAll)
                (a, gc_a, SmashAttack, Buttons::AttackAll)
                (b, gc_b, SmashAttack, Buttons::AttackAll)
                (x, gc_x, SmashAttack, Buttons::AttackAll)
                (y, gc_y, SmashAttack, Buttons::AttackAll)
        );
        (*out).buttons |= apply_button_mappings!(
            controller,
            mappings,
                (l, gc_l,   AppealHi, Buttons::AppealHi)
                (r, gc_r,   AppealHi, Buttons::AppealHi)
                (zl, gc_z,  AppealHi, Buttons::AppealHi)
                (zr, gc_z,  AppealHi, Buttons::AppealHi)
                (a, gc_a,   AppealHi, Buttons::AppealHi)
                (b, gc_b,   AppealHi, Buttons::AppealHi)
                (x, gc_x,   AppealHi, Buttons::AppealHi)
                (y, gc_y,   AppealHi, Buttons::AppealHi)
        );
        (*out).buttons |= apply_button_mappings!(
            controller,
            mappings,
                (l, gc_l,   TiltAttack, Buttons::TiltAttack | Buttons::AttackAll)
                (r, gc_r,   TiltAttack, Buttons::TiltAttack | Buttons::AttackAll)
                (zl, gc_z,  TiltAttack, Buttons::TiltAttack | Buttons::AttackAll)
                (zr, gc_z,  TiltAttack, Buttons::TiltAttack | Buttons::AttackAll)
                (a, gc_a,   TiltAttack, Buttons::TiltAttack | Buttons::AttackAll)
                (b, gc_b,   TiltAttack, Buttons::TiltAttack | Buttons::AttackAll)
                (x, gc_x,   TiltAttack, Buttons::TiltAttack | Buttons::AttackAll)
                (y, gc_y,   TiltAttack, Buttons::TiltAttack | Buttons::AttackAll)
        );
        if (*mappings.add(player_idx as usize)).gc_absmash {
            if (*out).buttons.contains(Buttons::Attack | Buttons::Special) {
                (*out).buttons &= !(Buttons::Special | Buttons::TiltAttack);
                (*out).buttons |= Buttons::Smash;
                (*mappings.add(player_idx as usize)).is_absmash = true;
            } else if !(*out).buttons.intersects(Buttons::Attack | Buttons::Special) {
                (*mappings.add(player_idx as usize)).is_absmash = false;
            } else if (*mappings.add(player_idx as usize)).is_absmash {
                (*out).buttons &= !(Buttons::Special | Buttons::TiltAttack);
            }
        }
    } else if controller.style == ControllerStyle::LeftJoycon || controller.style == ControllerStyle::RightJoycon {
        (*out).buttons |= apply_button_mappings!(
            controller,
            mappings,
                (l, joy_shoulder, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (r, joy_shoulder, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (zl, joy_zshoulder, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (zr, joy_zshoulder, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (left_sl, joy_sl, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (left_sr, joy_sr, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (right_sl, joy_sl, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (right_sr, joy_sr, JumpMini, Buttons::JumpMini | Buttons::Jump)
        );
        (*out).buttons |= apply_button_mappings!(
            controller,
            mappings,
                (l, joy_shoulder,   SmashAttack, Buttons::AttackAll)
                (r, joy_shoulder,   SmashAttack, Buttons::AttackAll)
                (zl, joy_zshoulder, SmashAttack, Buttons::AttackAll)
                (zr, joy_zshoulder, SmashAttack, Buttons::AttackAll)
                (left_sl, joy_sl,   SmashAttack, Buttons::AttackAll)
                (left_sr, joy_sr,   SmashAttack, Buttons::AttackAll)
                (right_sl, joy_sl,  SmashAttack, Buttons::AttackAll)
                (right_sr, joy_sr,  SmashAttack, Buttons::AttackAll)
        );
        (*out).buttons |= apply_button_mappings!(
            controller,
            mappings,
                (l, joy_shoulder,   AppealHi, Buttons::AppealHi)
                (r, joy_shoulder,   AppealHi, Buttons::AppealHi)
                (zl, joy_zshoulder, AppealHi, Buttons::AppealHi)
                (zr, joy_zshoulder, AppealHi, Buttons::AppealHi)
                (left_sl, joy_sl,   AppealHi, Buttons::AppealHi)
                (left_sr, joy_sr,   AppealHi, Buttons::AppealHi)
                (right_sl, joy_sl,  AppealHi, Buttons::AppealHi)
                (right_sr, joy_sr,  AppealHi, Buttons::AppealHi)
        );
        (*out).buttons |= apply_button_mappings!(
            controller,
            mappings,
                (l, joy_shoulder,   TiltAttack, Buttons::TiltAttack | Buttons::AttackAll)
                (r, joy_shoulder,   TiltAttack, Buttons::TiltAttack | Buttons::AttackAll)
                (zl, joy_zshoulder, TiltAttack, Buttons::TiltAttack | Buttons::AttackAll)
                (zr, joy_zshoulder, TiltAttack, Buttons::TiltAttack | Buttons::AttackAll)
                (left_sl, joy_sl,   TiltAttack, Buttons::TiltAttack | Buttons::AttackAll)
                (left_sr, joy_sr,   TiltAttack, Buttons::TiltAttack | Buttons::AttackAll)
                (right_sl, joy_sl,  TiltAttack, Buttons::TiltAttack | Buttons::AttackAll)
                (right_sr, joy_sr,  TiltAttack, Buttons::TiltAttack | Buttons::AttackAll)
        );

        if controller.style == ControllerStyle::LeftJoycon {
            (*out).buttons |= apply_button_mappings!(
                controller,
                mappings,
                    (dpad_left, joy_down, JumpMini, Buttons::JumpMini | Buttons::Jump)
                    (dpad_right, joy_up, JumpMini, Buttons::JumpMini | Buttons::Jump)
                    (dpad_up, joy_left, JumpMini, Buttons::JumpMini | Buttons::Jump)
                    (dpad_down, joy_right, JumpMini, Buttons::JumpMini | Buttons::Jump)
            );
            (*out).buttons |= apply_button_mappings!(
                controller,
                mappings,
                    (dpad_left, joy_down, SmashAttack, Buttons::AttackAll)
                    (dpad_right, joy_up, SmashAttack, Buttons::AttackAll)
                    (dpad_up, joy_left, SmashAttack, Buttons::AttackAll)
                    (dpad_down, joy_right, SmashAttack, Buttons::AttackAll)
            );
            (*out).buttons |= apply_button_mappings!(
                controller,
                mappings,
                    (dpad_left, joy_down,   AppealHi, Buttons::AppealHi)
                    (dpad_right, joy_up,    AppealHi, Buttons::AppealHi)
                    (dpad_up, joy_left,     AppealHi, Buttons::AppealHi)
                    (dpad_down, joy_right,  AppealHi, Buttons::AppealHi)
            );
            (*out).buttons |= apply_button_mappings!(
                controller,
                mappings,
                    (dpad_left, joy_down,   TiltAttack, Buttons::TiltAttack | Buttons::AttackAll)
                    (dpad_right, joy_up,    TiltAttack, Buttons::TiltAttack | Buttons::AttackAll)
                    (dpad_up, joy_left,     TiltAttack, Buttons::TiltAttack | Buttons::AttackAll)
                    (dpad_down, joy_right,  TiltAttack, Buttons::TiltAttack | Buttons::AttackAll)
            );
        } else {
            (*out).buttons |= apply_button_mappings!(
                controller,
                mappings,
                    (a, joy_down, JumpMini, Buttons::JumpMini | Buttons::Jump)
                    (y, joy_up, JumpMini, Buttons::JumpMini | Buttons::Jump)
                    (b, joy_left, JumpMini, Buttons::JumpMini | Buttons::Jump)
                    (x, joy_right, JumpMini, Buttons::JumpMini | Buttons::Jump)
            );
            (*out).buttons |= apply_button_mappings!(
                controller,
                mappings,
                    (a, joy_down, SmashAttack, Buttons::AttackAll)
                    (y, joy_up, SmashAttack, Buttons::AttackAll)
                    (b, joy_left, SmashAttack, Buttons::AttackAll)
                    (x, joy_right, SmashAttack, Buttons::AttackAll)
            );
            (*out).buttons |= apply_button_mappings!(
                controller,
                mappings,
                    (a, joy_down,   AppealHi, Buttons::AppealHi)
                    (y, joy_up,     AppealHi, Buttons::AppealHi)
                    (b, joy_left,   AppealHi, Buttons::AppealHi)
                    (x, joy_right,  AppealHi, Buttons::AppealHi)
            );
            (*out).buttons |= apply_button_mappings!(
                controller,
                mappings,
                    (a, joy_down,   TiltAttack, Buttons::TiltAttack | Buttons::AttackAll)
                    (y, joy_up,     TiltAttack, Buttons::TiltAttack | Buttons::AttackAll)
                    (b, joy_left,   TiltAttack, Buttons::TiltAttack | Buttons::AttackAll)
                    (x, joy_right,  TiltAttack, Buttons::TiltAttack | Buttons::AttackAll)
            );
        }
        if (*mappings.add(player_idx as usize)).joy_absmash {
            if (*out).buttons.contains(Buttons::Attack | Buttons::Special) {
                (*out).buttons &= !(Buttons::Special | Buttons::TiltAttack);
                (*out).buttons |= Buttons::Smash;
                (*mappings.add(player_idx as usize)).is_absmash = true;
            } else if !(*out).buttons.intersects(Buttons::Attack | Buttons::Special) {
                (*mappings.add(player_idx as usize)).is_absmash = false;
            } else if (*mappings.add(player_idx as usize)).is_absmash {
                (*out).buttons &= !(Buttons::Special | Buttons::TiltAttack);
            }
        }
    } else {
        (*out).buttons |= apply_button_mappings!(
            controller,
            mappings,
                (l, pro_l, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (r, pro_r, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (zl, pro_zl, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (zr, pro_zr, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (a, pro_a, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (b, pro_b, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (x, pro_x, JumpMini, Buttons::JumpMini | Buttons::Jump)
                (y, pro_y, JumpMini, Buttons::JumpMini | Buttons::Jump)
        );
        (*out).buttons |= apply_button_mappings!(
            controller,
            mappings,
                (l, pro_l, SmashAttack, Buttons::AttackAll)
                (r, pro_r, SmashAttack, Buttons::AttackAll)
                (zl, pro_zl, SmashAttack, Buttons::AttackAll)
                (zr, pro_zr, SmashAttack, Buttons::AttackAll)
                (a, pro_a, SmashAttack, Buttons::AttackAll)
                (b, pro_b, SmashAttack, Buttons::AttackAll)
                (x, pro_x, SmashAttack, Buttons::AttackAll)
                (y, pro_y, SmashAttack, Buttons::AttackAll)
        );
        (*out).buttons |= apply_button_mappings!(
            controller,
            mappings,
                (l, pro_l,      AppealHi, Buttons::AppealHi)
                (r, pro_r,      AppealHi, Buttons::AppealHi)
                (zl, pro_zl,    AppealHi, Buttons::AppealHi)
                (zr, pro_zr,    AppealHi, Buttons::AppealHi)
                (a, pro_a,      AppealHi, Buttons::AppealHi)
                (b, pro_b,      AppealHi, Buttons::AppealHi)
                (x, pro_x,      AppealHi, Buttons::AppealHi)
                (y, pro_y,      AppealHi, Buttons::AppealHi)
        );
        (*out).buttons |= apply_button_mappings!(
            controller,
            mappings,
                (l, pro_l,      TiltAttack, Buttons::TiltAttack | Buttons::AttackAll)
                (r, pro_r,      TiltAttack, Buttons::TiltAttack | Buttons::AttackAll)
                (zl, pro_zl,    TiltAttack, Buttons::TiltAttack | Buttons::AttackAll)
                (zr, pro_zr,    TiltAttack, Buttons::TiltAttack | Buttons::AttackAll)
                (a, pro_a,      TiltAttack, Buttons::TiltAttack | Buttons::AttackAll)
                (b, pro_b,      TiltAttack, Buttons::TiltAttack | Buttons::AttackAll)
                (x, pro_x,      TiltAttack, Buttons::TiltAttack | Buttons::AttackAll)
                (y, pro_y,      TiltAttack, Buttons::TiltAttack | Buttons::AttackAll)
        );

        if (*mappings.add(player_idx as usize)).pro_absmash {
            if (*out).buttons.contains(Buttons::Attack | Buttons::Special) {
                (*out).buttons &= !(Buttons::Special | Buttons::TiltAttack);
                (*out).buttons |= Buttons::Smash;
                (*mappings.add(player_idx as usize)).is_absmash = true;
            } else if !(*out).buttons.intersects(Buttons::Attack | Buttons::Special) {
                (*mappings.add(player_idx as usize)).is_absmash = false;
            } else if (*mappings.add(player_idx as usize)).is_absmash {
                (*out).buttons &= !(Buttons::Special | Buttons::TiltAttack);
            }
        }
    }

    // Check if the button combos are being pressed and then force Stock Share + AttackRaw/SpecialRaw depending on input

    if controller.current_buttons.l()
    && controller.current_buttons.r()
    && controller.current_buttons.a()
    && (controller.current_buttons.minus() || controller.current_buttons.plus())
    {
        controller.current_buttons.set_plus(false);
        controller.current_buttons.set_minus(false);
        controller.just_down.set_plus(false);
        controller.just_down.set_minus(false);

        if controller.current_buttons.y() {
            (*out).buttons = Buttons::StockShare | Buttons::AttackRaw;
        } else if controller.current_buttons.x() {
            (*out).buttons = Buttons::StockShare | Buttons::SpecialRaw;
        } else {
            controller.current_buttons.set_plus(true);
            controller.current_buttons.set_minus(true);
            controller.just_down.set_plus(true);
            controller.just_down.set_minus(true);
        }
    }
}

#[skyline::hook(offset = offsets::analog_trigger_l(), inline)]
unsafe fn analog_trigger_l(ctx: &mut skyline::hooks::InlineCtx) {
    if *ctx.registers[9].x.as_ref() & 0x40 != 0 {
        *ctx.registers[11].x.as_mut() = 0;
    } else {
        *ctx.registers[11].w.as_mut() = 0x27FF;
    }
}

#[skyline::hook(offset = offsets::analog_trigger_r(), inline)]
unsafe fn analog_trigger_r(ctx: &mut skyline::hooks::InlineCtx) {
    if *ctx.registers[8].x.as_ref() & 0x80 != 0 {
        *ctx.registers[11].x.as_mut() = 0;
    } else {
        *ctx.registers[11].w.as_mut() = 0x27FF;
    }
}

#[repr(C)]
struct ControlModuleInternal {
    vtable: *mut u8,
    controller_index: i32,
    buttons: Buttons,
    stick_x: f32,
    stick_y: f32,
    padding: [f32; 2],
    unk: [u32; 8],
    clamped_lstick_x: f32,
    clamped_lstick_y: f32,
    padding2: [f32; 2],
    clamped_rstick_x: f32,
    clamped_rstick_y: f32,
}

unsafe fn get_mapped_controller_inputs(player: usize) -> &'static MappedInputs {
    let base = *((skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *mut u8).add(0x52c30f0) as *const u64);
    &*((base + 0x2b8 + 0x8 * (player as u64)) as *const MappedInputs)
}

static mut LAST_ALT_STICK: [f32; 2] = [0.0, 0.0];

#[skyline::hook(offset = 0x3f7220)]
unsafe fn parse_inputs(this: &mut ControlModuleInternal) {
    const NEUTRAL: f32 = 0.2;
    const CLAMP_MAX: f32 = 120.0;

    // println!("this: {:#x}", this as *mut ControlModuleInternal as u64);

    if this.controller_index == -1 {
        return call_original!(this);
    }

    //println!("this.controller_index: {}", this.controller_index);
    // assert!(this.controller_index <= 7);

    let inputs = get_mapped_controller_inputs(this.controller_index as usize);

    let clamp_mul = 1.0 / CLAMP_MAX;

    // let raw_lstick_x = ((inputs.lstick_x as f32) * clamp_mul).clamp(-1.0, 1.0);
    // let raw_lstick_y = ((inputs.lstick_y as f32) * clamp_mul).clamp(-1.0, 1.0);

    // let raw_lstick_x = if raw_lstick_x.abs() >= NEUTRAL { raw_lstick_x } else { 0.0 };
    // let raw_lstick_y = if raw_lstick_y.abs() >= NEUTRAL { raw_lstick_y } else { 0.0 };

    let raw_rstick_x = ((inputs.rstick_x as f32) * clamp_mul).clamp(-1.0, 1.0);
    let raw_rstick_y = ((inputs.rstick_y as f32) * clamp_mul).clamp(-1.0, 1.0);

    LAST_ALT_STICK[0] = if raw_rstick_x.abs() >= NEUTRAL { raw_rstick_x } else { 0.0 };
    LAST_ALT_STICK[1] = if raw_rstick_y.abs() >= NEUTRAL { raw_rstick_y } else { 0.0 };

    call_original!(this)
}

#[skyline::hook(offset = 0x6b9c5c, inline)]
unsafe fn after_exec(ctx: &skyline::hooks::InlineCtx) {
    let module = *ctx.registers[19].x.as_ref();
    let internal_class = *(module as *const u64).add(0x110 / 0x8);
    *(internal_class as *mut f32).add(0x40 / 0x4) = LAST_ALT_STICK[0];
    *(internal_class as *mut f32).add(0x44 / 0x4) = LAST_ALT_STICK[1];
}

#[skyline::hook(offset = 0x16d7ee4, inline)]
unsafe fn handle_incoming_packet(ctx: &mut skyline::hooks::InlineCtx) {
    let packet = *ctx.registers[15].x.as_ref();

    let mut inputs = MappedInputs {
        buttons: Buttons::empty(),
        lstick_x: 0,
        lstick_y: 0,
        rstick_x: 0,
        rstick_y: 0
    };

    let raw_buttons = ((packet >> 16) & 0xFFFF_FFFF) as u32;
    let lstick_x = (packet & 0xFF) as i8;
    let lstick_y = ((packet & 0xFF00) >> 8) as i8;
    let rstick_x = ((packet >> 0x30) & 0xFF) as i8;
    let rstick_y = ((packet >> 0x38) & 0xFF) as i8;

    inputs.buttons = Buttons::from_bits_unchecked(raw_buttons as _);
    inputs.lstick_x = lstick_x;
    inputs.lstick_y = lstick_y;
    inputs.rstick_x = rstick_x;
    inputs.rstick_y = rstick_y;

    *ctx.registers[13].x.as_mut() = std::mem::transmute(inputs);
}

/// fix throws not respecting the cstick, especially dk cargo throw
#[skyline::hook(replace = L2CFighterCommon_IsThrowStick)]
unsafe extern "C" fn is_throw_stick(fighter: &mut L2CFighterCommon) -> L2CValue {

    let mut out = fighter.local_func__fighter_status_catch_1();
    let stick_x = fighter.stick_x() * PostureModule::lr(fighter.boma());
    let stick_y = fighter.stick_y();
    if stick_x > fighter.get_param_float("common", "attack_lw3_stick_x") {
        out["f"] = true.into();
    } else if stick_x < -fighter.get_param_float("common", "attack_lw3_stick_x") {
        out["b"] = true.into();
    }
    if stick_y > fighter.get_param_float("common", "attack_hi4_stick_y") {
        out["hi"] = true.into();
    } else if stick_y < fighter.get_param_float("common", "attack_lw4_stick_y") {
        out["lw"] = true.into();
    }
    out
}

static mut SHOULD_END_RESULT_SCREEN : bool = false;

// Skip results screen with start button
#[skyline::hook(offset = 0x3664040)]
unsafe fn process_inputs_handheld(controller: &mut Controller) {
    let entry_count = lua_bind::FighterManager::entry_count(utils::singletons::FighterManager());
    if lua_bind::FighterManager::is_result_mode(utils::singletons::FighterManager()) && entry_count > 0 {
        if ninput::any::is_press(ninput::Buttons::PLUS) {
            SHOULD_END_RESULT_SCREEN = true;
        }
        if SHOULD_END_RESULT_SCREEN {
            let mut rng = rand::thread_rng();
            // Need to space apart A-presses so it does not seem like we are holding the button.
            let n: u32 = rng.gen_range(0..3);
            if n == 1 {
                controller.current_buttons.set_a(true);
                controller.just_down.set_a(true);
            }
        }
    }
    if entry_count == 0 {
        SHOULD_END_RESULT_SCREEN = false;
    }
    call_original!(controller);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hook!(is_throw_stick);
    }
}

pub fn install() {
    skyline::install_hooks!(
        map_controls_hook,
        analog_trigger_l,
        analog_trigger_r,
        packed_packet_creation,
        write_packet,
        parse_inputs,
        handle_incoming_packet,
        after_exec,
        process_inputs_handheld
    );
    skyline::nro::add_hook(nro_hook);
}