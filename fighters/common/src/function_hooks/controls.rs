use super::*;
use utils::ext::*;

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

    println!("entry_count vs. current: {} vs. {}", entry_count, (*mappings.add(player_idx as usize))._34[0]);

    if (*out).buttons.contains(Buttons::CStickOn) && (*mappings.add(player_idx as usize))._34[0] != entry_count {
        (*out).rstick_x = (controller.left_stick_x * (i8::MAX as f32)) as i8;
        (*out).rstick_y = (controller.left_stick_y * (i8::MAX as f32)) as i8;
    } else {
        (*out).rstick_x = (controller.right_stick_x * (i8::MAX as f32)) as i8;
        (*out).rstick_y = (controller.right_stick_y * (i8::MAX as f32)) as i8;
    }

    let mut sh_footstool_input = Buttons::empty();
    for x in 0..8 {
        if !smash::app::sv_information::is_ready_go() {
            if *(skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *const u32).add(0x52e6b44 / 0x4) < 1200 {
                break;
            }
        }
        let addr = *(skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *const u64).add(0x52b6548 / 0x8);

        if addr == 0 || *(addr as *const u64) == 0 {
            break;
        }

        
        let src_object = util::get_battle_object_from_entry_id(x);
        if src_object.is_none() {
            continue;
        }
        let src_object = src_object.unwrap();

        let active_object = util::get_active_battle_object_id_from_entry_id(x);
        if active_object.is_none() {
            continue;
        }

        let active_object = util::get_battle_object_from_id(active_object.unwrap());
        if get_player_idx_from_boma((*src_object).module_accessor as u64) == player_idx {
            let boma = (*active_object).boma();
            if boma.is_status(*FIGHTER_STATUS_KIND_JUMP_SQUAT) {
                sh_footstool_input = Buttons::empty();
            } else if boma.is_situation(*SITUATION_KIND_GROUND) {
                sh_footstool_input = Buttons::Jump | Buttons::JumpMini;
            } else {
                if boma.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR) >= 5 {
                    sh_footstool_input = Buttons::AppealAll;
                }
            }
            break;
        }
    }

    let mappings = mappings.add(player_idx as usize);

    if controller.style == ControllerStyle::GCController {
        if controller.just_down.l() && (*mappings).gc_l == InputKind::JumpMini {
            (*out).buttons |= sh_footstool_input;
        } else if controller.just_down.r() && (*mappings).gc_r == InputKind::JumpMini {
            (*out).buttons |= sh_footstool_input;
        } else if (controller.just_down.zl() || controller.just_down.zr()) && (*mappings).gc_z == InputKind::JumpMini {
            (*out).buttons |= sh_footstool_input;
        } else if controller.just_down.a() && (*mappings).gc_a == InputKind::JumpMini {
            (*out).buttons |= sh_footstool_input;
        } else if controller.just_down.b() && (*mappings).gc_b == InputKind::JumpMini {
            (*out).buttons |= sh_footstool_input;
        } else if controller.just_down.x() && (*mappings).gc_x == InputKind::JumpMini {
            (*out).buttons |= sh_footstool_input;
        } else if controller.just_down.y() && (*mappings).gc_y == InputKind::JumpMini {
            (*out).buttons |= sh_footstool_input;
        }
    } else if controller.style == ControllerStyle::LeftJoycon || controller.style == ControllerStyle::RightJoycon {
        if (controller.just_down.l() || controller.just_down.r()) && (*mappings).joy_shoulder == InputKind::JumpMini {
            (*out).buttons |= sh_footstool_input;
        } else if (controller.just_down.zl() || controller.just_down.zr()) && (*mappings).joy_zshoulder == InputKind::JumpMini {
            (*out).buttons |= sh_footstool_input;
        } else if (controller.just_down.left_sl() || controller.just_down.right_sl()) && (*mappings).joy_sl == InputKind::JumpMini {
            (*out).buttons |= sh_footstool_input;
        } else if (controller.just_down.left_sr() || controller.just_down.right_sr()) && (*mappings).joy_sl == InputKind::JumpMini {
            (*out).buttons |= sh_footstool_input;
        } else if controller.style == ControllerStyle::LeftJoycon {
            if controller.just_down.dpad_left() && (*mappings).joy_down == InputKind::JumpMini {
                (*out).buttons |= sh_footstool_input;
            } else if controller.just_down.dpad_right() && (*mappings).joy_up == InputKind::JumpMini {
                (*out).buttons |= sh_footstool_input;
            } else if controller.just_down.dpad_up() && (*mappings).joy_left == InputKind::JumpMini {
                (*out).buttons |= sh_footstool_input;
            } else if controller.just_down.dpad_down() && (*mappings).joy_right == InputKind::JumpMini {
                (*out).buttons |= sh_footstool_input;
            }
        } else {
            if controller.just_down.a() && (*mappings).joy_down == InputKind::JumpMini {
                (*out).buttons |= sh_footstool_input;
            } else if controller.just_down.y() && (*mappings).joy_up == InputKind::JumpMini {
                (*out).buttons |= sh_footstool_input;
            } else if controller.just_down.b() && (*mappings).joy_left == InputKind::JumpMini {
                (*out).buttons |= sh_footstool_input;
            } else if controller.just_down.x() && (*mappings).joy_right == InputKind::JumpMini {
                (*out).buttons |= sh_footstool_input;
            }
        }
    } else {
        if controller.just_down.l() && (*mappings).pro_l == InputKind::JumpMini {
            (*out).buttons |= sh_footstool_input;
        } else if controller.just_down.r() && (*mappings).pro_r == InputKind::JumpMini {
            (*out).buttons |= sh_footstool_input;
        } else if controller.just_down.zl() && (*mappings).pro_zl == InputKind::JumpMini {
            (*out).buttons |= sh_footstool_input;
        } else if controller.just_down.zr() && (*mappings).pro_zr == InputKind::JumpMini {
            (*out).buttons |= sh_footstool_input;
        } else if controller.just_down.a() && (*mappings).pro_a == InputKind::JumpMini {
            (*out).buttons |= sh_footstool_input;
        } else if controller.just_down.b() && (*mappings).pro_b == InputKind::JumpMini {
            (*out).buttons |= sh_footstool_input;
        } else if controller.just_down.x() && (*mappings).pro_x == InputKind::JumpMini {
            (*out).buttons |= sh_footstool_input;
        } else if controller.just_down.y() && (*mappings).pro_y == InputKind::JumpMini {
            (*out).buttons |= sh_footstool_input;
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

    println!("this.controller_index: {}", this.controller_index);
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

pub fn install() {
    skyline::install_hooks!(
        map_controls_hook,
        analog_trigger_l,
        analog_trigger_r,
        packed_packet_creation,
        write_packet,
        parse_inputs,
        handle_incoming_packet,
        after_exec
    );
}