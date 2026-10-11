use super::*;

#[skyline::hook(offset = 0x6bef70)]
unsafe extern "C" fn c_236(
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
        else if !data.down() && !data.front_down(class.lr as f32)
        && !data.front(class.lr as f32) && !data.front_up(class.lr as f32) {
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
            false
        }
        1 => {
            if data.front_down(class.lr as f32) {
                class.state = 2;
                class.command_timer = 0;
            }
            false
        }
        2 | 3 => {
            if class.state == 2 {
                if !data.front(class.lr as f32) {
                    return false;
                }
                class.command_timer = 0;
                class.state = 3;
            }

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
        c_236
    );
}
