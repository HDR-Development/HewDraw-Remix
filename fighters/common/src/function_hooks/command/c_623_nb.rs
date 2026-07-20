use super::*;

#[skyline::hook(offset = 0x6c0120)]
unsafe extern "C" fn c_623_nb(
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
        3 => data.front(class.lr as f32) || data.front_up(class.lr as f32) || data.front_down(class.lr as f32),
        _ => {
            unreachable!()
        }
    }
}

pub fn install() {
    skyline::install_hooks!(
        c_623_nb
    );
}
