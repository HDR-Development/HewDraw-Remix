use super::*;

#[skyline::hook(offset = 0x6bff10)]
unsafe extern "C" fn c_21416_r(
    class: &mut CommandInputState,
    args: *const CommandInputFlags,
    lr: f32
) -> bool {
    let data = *args.add(2);
    if class.state != 0 {
        if !data.intersects(CommandInputFlags::ANY_DIRECTION) {
            if class.unk2 != 0 {
                class.command_timer = 0;
                class.state = 0;
                return false;
            }
        }
        else if data.up() {
            class.command_timer = 0;
            class.state = 0;
        }
    }

    match class.state {
        0 => {
            if data.down() {
                class.state = 1;
                class.lr = lr as i8;
            }
            // if data.front_down(lr) {
            //     class.state = 2;
            //     class.lr = lr as i8;
            // }
            false
        }
        1 | 2 => {
            if class.state == 1 {
                if data.front_down(class.lr as f32) {
                    class.state = 3;
                    class.command_timer = 0;
                    *(class as *mut CommandInputState as *mut u8).add(0xF) = 1;
                    return false;
                }
            }

            if data.front(class.lr as f32) || data.front_up(class.lr as f32) {
                class.state = 3;
                class.command_timer = 0;
                *(class as *mut CommandInputState as *mut u8).add(0xF) = 4;
            }
            false
        }
        3 => {
            if *(class as *mut CommandInputState as *mut u8).add(0xF) != 1
            && data.front_down(class.lr as f32) {
                class.state = 4;
                class.command_timer = 0;
                *(class as *mut CommandInputState as *mut u8).add(0xF) = 1;
                return false;
            }

            if data.down() {
                class.state = 4;
                class.command_timer = 0;
                *(class as *mut CommandInputState as *mut u8).add(0xF) = 2;
                return false;
            }

            if data.front_down(class.lr as f32) {
                class.state = 4;
                class.command_timer = 0;
                *(class as *mut CommandInputState as *mut u8).add(0xF) = 3;
                return false;
            }

            if data.back(class.lr as f32) {
                class.state = 5;
                class.command_timer = 0;
                *(class as *mut CommandInputState as *mut u8).add(0xF) = 1;
                return false;
            }

            false
        }
        4 => {
            if data.back(class.lr as f32) {
                class.state = 5;
                class.command_timer = 0;
                *(class as *mut CommandInputState as *mut u8).add(0xF) = 1;
                return false;
            }

            if *(class as *mut CommandInputState as *mut u8).add(0xF) != 3
            && data.back_down(class.lr as f32) {
                class.state = 5;
                class.command_timer = 0;
                *(class as *mut CommandInputState as *mut u8).add(0xF) = 1;
            }

            false
        }
        5 => {
            let check_flag = if !class.input_allow.bits() & 3 == 0 {
                CommandInputFlags::ATTACK_EDGE | CommandInputFlags::SPECIAL_EDGE
            }
            else {
                if class.input_allow.intersects(InputAllow::ATTACK) {
                    CommandInputFlags::ATTACK_EDGE
                }
                else {
                    CommandInputFlags::SPECIAL_EDGE
                }
            };
            if data.intersects(check_flag) {
                return true;
            }

            false
        }
        _ => {
            unreachable!()
        }
    }
}

pub fn install() {
    skyline::install_hooks!(
        c_21416_r
    );
}
