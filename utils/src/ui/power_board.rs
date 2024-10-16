use super::*;
use bitflags::bitflags;

const ORB_RED: [f32; 4] = [7000.0 / 255.0, 0.0, 0.0, 1.0];
const ORB_YELLOW: [f32; 4] = [8000.0 / 255.0, 6000.0 / 255.0, 0.0, 1.0];
const ORB_BLUE: [f32; 4] = [5.0 / 255.0, 100.0 / 255.0, 3000.0 / 255.0, 1.0];
const ORB_NONE: [f32; 4] = [0.0, 0.0, 0.0, 0.0];

bitflags! {
    #[derive(Default, Copy, Clone, PartialEq, Eq)]
    pub struct PowerColor: i32 {
        const NONE = 0b000;
        const RED = 0b001;
        const ORANGE = 0b011;
        const YELLOW = 0b010;
        const GREEN = 0b110;
        const BLUE = 0b100;
        const PURPLE = 0b101;
        const INVALID = 0b111;
    }
}

#[derive(Default, Copy, Clone)]
pub struct PowerBoard {
    // Panes
    pub meter_base: u64,
    pub icon: u64,
    pub orb_right: u64,
    pub orb_left: u64,

    pub color_1: PowerColor,
    pub color_2: PowerColor,
    pub color_icon: PowerColor,

    is_enabled: bool,
}

impl PowerBoard {
    pub fn new(layout_data: u64) -> Self {
        let meter_base = get_pane_from_layout(layout_data, "palu_meter_base\0")
            .expect("Couldn't find palu_meter_base");
        let icon = get_pane_from_layout(layout_data, "palu_icon\0")
            .expect("Couldn't find palu_icon");
        let orb_left = get_pane_from_layout(layout_data, "palu_orb_1\0")
            .expect("Couldn't find palu_orb_1");
        let orb_right = get_pane_from_layout(layout_data, "palu_orb_2\0")
            .expect("Couldn't find palu_orb_2");

        return Self {
            meter_base,
            icon,
            orb_right,
            orb_left,

            color_1: PowerColor::NONE,
            color_2: PowerColor::NONE,
            color_icon: PowerColor::NONE,

            is_enabled: false
        };
    }

    pub fn reset(&mut self) {
        set_pane_visible(self.meter_base, true);
        set_pane_visible(self.icon, true);
        set_pane_visible(self.orb_right, false);
        set_pane_visible(self.orb_left, false);
        self.color_1 = PowerColor::NONE;
        self.color_2 = PowerColor::NONE;
    }

    pub fn get_power_color_from_i32(i: i32) -> PowerColor {
        return match i {
            1 => PowerColor::RED,
            2 => PowerColor::BLUE,
            3 => PowerColor::YELLOW,
            _ => PowerColor::NONE
        }
    }

    pub fn set_meter_info(&mut self, color_1: i32, color_2: i32) {
        self.color_1 = Self::get_power_color_from_i32(color_1);
        self.color_2 = Self::get_power_color_from_i32(color_2);
        self.color_icon = self.color_1 | self.color_2;
    }

    pub fn update_orbs(&mut self) {
        // orb_1
        let color = match (self.color_1) {
            PowerColor::RED => ORB_RED,
            PowerColor::YELLOW => ORB_YELLOW,
            PowerColor::BLUE => ORB_BLUE,
            _ => ORB_NONE
        };
        if color == ORB_NONE {
            set_pane_visible(self.orb_right, false);
            set_pane_visible(self.orb_left, false);
            return;
        }
        set_pane_visible(self.orb_right, true);
        set_pane_colors(self.orb_right, color, color);

        // orb_2
        let color = match (self.color_2) {
            PowerColor::RED => ORB_RED,
            PowerColor::YELLOW => ORB_YELLOW,
            PowerColor::BLUE => ORB_BLUE,
            _ => ORB_NONE
        };
        if color == ORB_NONE {
            set_pane_visible(self.orb_left, false);
            return;
        }
        set_pane_visible(self.orb_left, true);
        set_pane_colors(self.orb_left, color, color);
    }

    pub fn update_icon(&mut self) {
        let offset = match (self.color_icon) {
            PowerColor::RED => 0.0,
            PowerColor::ORANGE => 1.0,
            PowerColor::YELLOW => 2.0,
            PowerColor::GREEN => 3.0,
            PowerColor::BLUE => 4.0,
            PowerColor::PURPLE => 5.0,
            _ => 6.0
        };
        if offset == 6.0 {
            set_pane_visible(self.icon, false);
            return;
        }
        let offset = offset / 7.0;
        let len = 1.0 / 7.0;
        set_pane_visible(self.icon, true);
        set_tex_coords(
            self.icon, 
            [
                offset, 0.0,
                offset + len, 0.0,
                offset, 1.0,
                offset + len, 1.0
            ]
        );
    }
}

impl UiObject for PowerBoard {
    fn update(&mut self) {
        self.update_icon();
        self.update_orbs();
    }

    fn is_valid(&self) -> bool {
        return is_pane_valid(self.meter_base)
            && is_pane_valid(self.icon)
            && is_pane_valid(self.orb_right)
            && is_pane_valid(self.orb_left);
    }

    fn set_enable(&mut self, enable: bool) {
        if !enable {
            set_pane_visible(self.meter_base, false);
            set_pane_visible(self.icon, false);
            set_pane_visible(self.orb_right, false);
            set_pane_visible(self.orb_left, false);
        } else if !self.is_enabled {
            self.reset();
        }
        self.is_enabled = enable;
    }

    fn is_enabled(&self) -> bool {
        return self.is_enabled;
    }
}