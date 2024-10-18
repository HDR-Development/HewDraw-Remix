use super::*;
use interpolation::Lerp;

const COLOR_LEVEL1: [f32; 4] = [20.0 / 255.0, 19.0 / 255.0, 15.0 / 255.0, 1.0];
const COLOR_LEVEL2: [f32; 4] = [70.0 / 255.0, 67.0 / 255.0, 57.0 / 255.0, 1.0];
const COLOR_LEVEL3: [f32; 4] = [255.0 / 255.0, 240.0 / 255.0, 189.0 / 255.0, 1.0];

const FULL_TEXCOORDS: [f32; 8] = [
    0.0, 0.0,
    1.0, 0.0,
    0.0, 1.0,
    1.0, 1.0
];

const EMPTY_TEXCOORDS: [f32; 8] = [
    0.0, 0.0,
    1.0, 0.0,
    0.0, 0.0,
    1.0, 0.0
];

#[derive(Default, Copy, Clone)]
pub struct GarlicMeter {
    // Panes
    pub meter_base_1: u64,
    pub meter_base_2: u64,
    pub meter_bar_full: u64,
    pub meter_bar: u64,
    pub meter_bar_2: u64,

    // Initial state
    pub meter_bar_width_height: (f32, f32),
    pub meter_bar_2_width_height: (f32, f32),

    // Progress tracking
    pub actual_percentage: f32,
    pub level: i32,

    is_enabled: bool,
}

impl GarlicMeter {
    pub fn new(layout_data: u64) -> Self {
        let meter_base_1 = get_pane_from_layout(layout_data, "garlic_meter_base_1\0")
            .expect("Couldn't find garlic_meter_base_1");
        let meter_base_2 = get_pane_from_layout(layout_data, "garlic_meter_base_2\0")
            .expect("Couldn't find garlic_meter_base_2");
        let meter_bar_full = get_pane_from_layout(layout_data, "garlic_meter_bar_full\0")
            .expect("Couldn't find garlic_meter_bar_full");
        let meter_bar = get_pane_from_layout(layout_data, "garlic_meter_bar\0")
            .expect("Couldn't find garlic_meter_bar");
        let meter_bar_2 = get_pane_from_layout(layout_data, "garlic_meter_bar_2\0")
            .expect("Couldn't find garlic_meter_bar2");

        return Self {
            meter_base_1,
            meter_base_2,
            meter_bar_full,
            meter_bar,
            meter_bar_2,

            meter_bar_width_height: (-1.0, -1.0),
            meter_bar_2_width_height: (-1.0, -1.0),

            actual_percentage: -1.0,
            level: 0,

            is_enabled: false
        };
    }

    pub fn reset(&mut self) {
        set_pane_visible(self.meter_base_1, true);
        set_pane_visible(self.meter_base_2, true);
        set_pane_visible(self.meter_bar_full, false);
        set_pane_visible(self.meter_bar, false);
        set_pane_visible(self.meter_bar_2, false);

        self.meter_bar_width_height = get_width_height(self.meter_bar);
        self.meter_bar_2_width_height = get_width_height(self.meter_bar_2);

        self.actual_percentage = 0.0;
        self.level = 0;
    }

    pub fn set_meter_info(&mut self, current: f32, level1: f32, level2: f32, level3: f32) {
        if current % 1.0 == 0.0 {
            dbg!(current);
        }
        if current < level1 {
            self.actual_percentage = current / level1;
            self.level = 0;
        } else if current < level2 {
            self.actual_percentage = (current - level1) / (level2 - level1);
            self.level = 1;
        } else if current < level3 {
            self.actual_percentage = (current - level2) / (level3 - level2);
            self.level = 2;
        } else {
            self.actual_percentage = 1.0;
            self.level = 3;
        }
    }

    pub fn update_meter_progress(&mut self) {

        // meter_base_1
        set_tex_coords(
            self.meter_base_1,
            FULL_TEXCOORDS
        );
        set_pane_visible(self.meter_base_1, true);

        // meter_base_2
        set_tex_coords(
            self.meter_base_2,
            FULL_TEXCOORDS
        );
        set_pane_visible(self.meter_base_2, true);

        if self.level > 2 {
            // meter_bar_full
            set_tex_coords(
                self.meter_bar_full,
                FULL_TEXCOORDS
            );
            set_pane_visible(self.meter_bar_full, true);
            set_pane_visible(self.meter_bar, false);
            set_pane_visible(self.meter_bar_2, false);
            return;
        }
        set_pane_visible(self.meter_bar_full, false);
        set_pane_visible(self.meter_bar, true);
        set_pane_visible(self.meter_bar_2, true);

        // handle bar colors
        if self.level > 1 {
            set_pane_colors(self.meter_bar, COLOR_LEVEL2, COLOR_LEVEL2);
            set_pane_colors(self.meter_bar_2, COLOR_LEVEL3, COLOR_LEVEL3);
        } else if self.level > 0 {
            set_pane_colors(self.meter_bar, COLOR_LEVEL1, COLOR_LEVEL1);
            set_pane_colors(self.meter_bar_2, COLOR_LEVEL2, COLOR_LEVEL2);
        } else {
            set_pane_visible(self.meter_bar, false);
            set_pane_colors(self.meter_bar_2, COLOR_LEVEL1, COLOR_LEVEL1);
        }

        // meter_bar
        set_tex_coords(
            self.meter_bar,
            FULL_TEXCOORDS
        );
        set_width_height(self.meter_bar, self.meter_bar_width_height.0, self.meter_bar_width_height.1);

        // meter_bar2
        set_tex_coords(
            self.meter_bar_2,
            [
                0.0, 1.0 - self.actual_percentage,
                1.0, 1.0 - self.actual_percentage,
                0.0, 1.0,
                1.0, 1.0
            ]
        );
        set_width_height(self.meter_bar_2, self.meter_bar_2_width_height.0, self.meter_bar_2_width_height.1 * self.actual_percentage);
    }
}

impl UiObject for GarlicMeter {
    fn update(&mut self) {
        self.update_meter_progress();
    }

    fn is_valid(&self) -> bool {
        return is_pane_valid(self.meter_base_1)
            && is_pane_valid(self.meter_base_2)
            && is_pane_valid(self.meter_bar_full)
            && is_pane_valid(self.meter_bar)
            && is_pane_valid(self.meter_bar_2);
    }

    fn set_enable(&mut self, enable: bool) {
        if !enable {
            set_pane_visible(self.meter_base_1, false);
            set_pane_visible(self.meter_base_2, false);
            set_pane_visible(self.meter_bar_full, false);
            set_pane_visible(self.meter_bar, false);
            set_pane_visible(self.meter_bar_2, false);
        } else if !self.is_enabled {
            self.reset();
        }
        self.is_enabled = enable;
    }

    fn is_enabled(&self) -> bool {
        return self.is_enabled;
    }
}