use super::*;
use utils::ext::*;

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
    let ret = original!()(mappings, player_idx, out, controller_struct, arg);
    let controller = &mut controller_struct.controller;

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

pub fn install() {
    skyline::install_hooks!(map_controls_hook, analog_trigger_l, analog_trigger_r);
}