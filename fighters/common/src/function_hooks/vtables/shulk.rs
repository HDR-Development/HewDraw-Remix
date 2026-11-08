use super::*;

extern "C" {
    #[link_name = "shulk_check_valid_arts_statuses_inner"]
    fn shulk_check_valid_arts_statuses_inner(fighter: &mut Fighter) -> bool;

    #[link_name = "shulk_arts_change_status_inner"]
    fn shulk_arts_change_status_inner(ctx: &mut skyline::hooks::InlineCtx);
}

// disables art wheel during hitstun, and enables it during jab, tilts and aerials

#[skyline::hook(offset = 0x116a3d0)]
pub unsafe extern "C" fn shulk_check_valid_arts_statuses(fighter: &mut Fighter) -> bool {
    shulk_check_valid_arts_statuses_inner(fighter)
}

macro_rules! decl_hooks_shulk_change_arts {
    ($install_fn:ident => $func:expr; $($name:ident($offset:expr));*) => {
        $(
            #[skyline::hook(offset = $offset, inline)]
            unsafe extern "C" fn $name(ctx: &mut skyline::hooks::InlineCtx) {
                $func(ctx);
            }
        )*
        fn $install_fn() {
            skyline::install_hooks!(
                $(
                    $name,
                )*
            );
        }
    }
}

decl_hooks_shulk_change_arts! {
    install_disable_arts_on_beat_hooks => shulk_arts_change_status_inner;
    shulk_arts_change_status_1(0x1167bbc);
    shulk_arts_change_status_2(0x1168d94)
}

pub fn install() {
    skyline::install_hooks!(
        shulk_check_valid_arts_statuses,
    );
    install_disable_arts_on_beat_hooks();
}
