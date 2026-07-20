use super::*;

#[skyline::hook(offset = 0x6c0df0)]
unsafe extern "C" fn c_323_catch(
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
            if data.front_down(lr) {
                class.state = 1;
                class.lr = lr as i8;
            }
            else if data.back_down(lr) {
                class.state = 1;
                class.lr = -lr as i8;
            }
            false
        }
        1 => {
            if data.down() {
                class.state = 2;
                class.command_timer = 0;
            }
            false
        }
        2 => {
            if data.front_down(class.lr as f32) {
                class.state = 3;
                class.command_timer = 0;
            }

            false
        }
        3 => {
            if data.intersects(CommandInputFlags::GRAB_EDGE) {
                println!("grab");
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
        c_323_catch
    );
}
