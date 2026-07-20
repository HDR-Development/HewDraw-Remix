use super::*;

#[skyline::hook(offset = 0x6c0270)]
unsafe extern "C" fn c_623_strict(
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

            if data.bits() & 1 == 0 {
                if !data.intersects(CommandInputFlags::ANY_DIRECTION) {
                    class.state = 1;
                }
            }
            false
        }
        3 => {
            if data.front_down(class.lr as f32) {
                let mut check_flag = CommandInputFlags::SPECIAL_EDGE;
                if class.input_allow.intersects(InputAllow::ATTACK) {
                    check_flag = CommandInputFlags::ATTACK_EDGE;
                }
                let mut check_flag2 = CommandInputFlags::ATTACK_EDGE | CommandInputFlags::SPECIAL_EDGE;
                if !class.input_allow.bits() & 3 != 0 {
                    check_flag2 = check_flag;
                }
                if !data.intersects(check_flag2) {
                    class.state = 4;
                    class.command_timer = 0;
                    return false;
                }
                return true;
            }

            if !data.intersects(CommandInputFlags::ANY_DIRECTION) {
                return false;
            }

            if data.bits() >> 2 & 1 != 0 {
                return false;
            }

            if data.back_down(class.lr as f32) {
                return false;
            }

            class.command_timer = 0;
            class.state = 0;
            false
        }
        4 => {
            if data.front_down(class.lr as f32) {
                let mut check_flag = CommandInputFlags::SPECIAL_EDGE;
                if class.input_allow.intersects(InputAllow::ATTACK) {
                    check_flag = CommandInputFlags::ATTACK_EDGE;
                }
                let mut check_flag2 = CommandInputFlags::ATTACK_EDGE | CommandInputFlags::SPECIAL_EDGE;
                if !class.input_allow.bits() & 3 != 0 {
                    check_flag2 = check_flag;
                }
                if !data.intersects(check_flag2) {
                    class.state = 4;
                    class.command_timer = 0;
                    return false;
                }
                return true;
            }

            class.command_timer = 0;
            class.state = 0;
            false
        }
        _ => {
            unreachable!()
        }
    }
}

pub fn install() {
    skyline::install_hooks!(
        c_623_strict
    );
}
