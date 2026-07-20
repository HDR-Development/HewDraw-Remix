use super::*;

#[skyline::hook(offset = 0x6c0480)]
unsafe extern "C" fn c_623_ab_long(
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
            if data.front_down(class.lr as f32) || data.front_up(class.lr as f32) || data.front(class.lr as f32) {
                class.state = 4;
                class.command_timer = 0;
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
                class.state = 5;
                class.command_timer = 0;
                class.enable_timer = 1;
                class.lr = (data.bits() >> 10 & 1) as i8; // it just reuses this???
            }

            false
        }
        5 => {
            let check = if class.lr == 0 {
                0xc
            }
            else {
                0xd
            };
            if data.bits() >> check & 1 == 0 {
                class.state = 0;
                class.command_timer = 0;
                return false;
            }

            let count = class.enable_timer;
            class.enable_timer = count + 1;
            class.command_timer = 0;

            if class.unk2 <= count {
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
        c_623_ab_long
    );
}
