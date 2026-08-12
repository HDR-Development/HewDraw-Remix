use super::*;

#[skyline::hook(offset = 0x6bf210)]
unsafe extern "C" fn c_623_a(
    class: &mut CommandInputState,
    args: *const CommandInputFlags,
    lr: f32
) -> bool {
    let data = *args.add(2);
    if !data.intersects(CommandInputFlags::ANY_DIRECTION) {
        if class.state != 0 {
            if class.unk2 != 0 {
                class.command_timer = 0;
                class.state = 0;
                return false;
            }
        }
    }

    match class.state {
        0 => {
            if data.front_down(lr) || data.front(lr) {
                class.state = 2;
                class.lr = lr as i8;
            }
            else if data.back_down(lr) || data.back(lr) {
                class.state = 2;
                class.lr = -lr as i8;
            }
            false
        }
        1 => {
            if data.down() {
                class.state = 3;
                class.command_timer = 0;
                return false;
            }
            if data.front_down(class.lr as f32) || data.front(class.lr as f32) {
                class.state = 2;
                class.command_timer = 0;
            }
            false
        }
        2 => {
            if data.down() {
                class.state = 3;
                class.command_timer = 0;
                return false;
            }

            // impossible check?
            if data.back_down(class.lr as f32) {
                class.state = 3;
                class.command_timer = 0;
                return false;
            }

            if !data.intersects(CommandInputFlags::ANY_DIRECTION) {
                class.state = 1;
            }

            false
        }
        3 => {
            if data.front(class.lr as f32) || data.front_down(class.lr as f32) || data.front_up(class.lr as f32) {
                class.command_timer = 0;
                class.state = 4;
            }

            false
        }
        4 => {
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
        c_623_a
    );
}
