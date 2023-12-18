use super::*;

const BACKGROUND_WHITE: [f32; 4] = [244.0 / 255.0, 177.0 / 255.0, 31.0 / 255.0, 1.0];
const BACKGROUND_BLACK: [f32; 4] = [88.0 / 255.0, 61.0 / 255.0, 4.0 / 255.0, 1.0];

const FOREGROUND_CHARGE_WHITE: [f32; 4] = [11.0 / 255.0, 115.0 / 255.0, 234.0 / 255.0, 1.0];
const FOREGROUND_CHARGE_BLACK: [f32; 4] = [6.0 / 255.0, 67.0 / 255.0, 136.0 / 255.0, 1.0];

const FULL_TEXCOORDS: [f32; 8] = [
    0.0, 0.0,
    1.0, 0.0,
    0.0, 1.0,
    1.0, 1.0
];

const EMPTY_TEXCOORDS: [f32; 8] = [
    0.0, 0.0,
    0.0, 0.0,
    0.0, 1.0,
    0.0, 1.0
];

#[derive(Default, Copy, Clone)]
#[repr(C)]
pub struct FfMeter {
    // Panes
    pub base_bar: u64,
    pub bars: [u64; 2],
    pub bars_background: [u64; 2],
    pub number: u64,

    // Initial state
    pub original_bar_width: f32,
    pub original_bar_height: f32,

    pub original_bar2_width: f32,
    pub original_bar2_height: f32,

    // Progress tracking
    pub actual_percentage: f32,
    pub visual_percentage: f32,

    // Number tracking
    pub current_number: usize,
    pub current_max: usize,

    enabled: bool,
}

impl FfMeter {
    pub fn new(layout_data: u64) -> Self {
        let base_bar = super::get_pane_from_layout(layout_data, "ff_meter_base\0")
            .expect("Could not find the base FF meter!");

        let bar1 = super::get_pane_from_layout(layout_data, "ff_meter_bar1\0")
            .expect("Could not find first bar for FF meter!");
        
        let bar2 = super::get_pane_from_layout(layout_data, "ff_meter_bar2\0")
            .expect("Could not find second bar for FF meter!");

        let bar1_bg = super::get_pane_from_layout(layout_data, "ff_meter_bar1_bg\0")
            .expect("Could not find first bg bar for FF meter!");

        let bar2_bg = super::get_pane_from_layout(layout_data, "ff_meter_bar2_bg\0")
            .expect("Could not find second bg bar for FF meter!");

        let number = super::get_pane_from_layout(layout_data, "ff_meter_num\0")
            .expect("Could no find number for FF meter!");

        Self {
            base_bar,
            bars: [bar1, bar2],
            bars_background: [bar1_bg, bar2_bg],
            number,

            original_bar_width: -1.0,
            original_bar_height: -1.0,

            original_bar2_width: -1.0,
            original_bar2_height: -1.0,

            actual_percentage: -1.0,
            visual_percentage: -1.0,

            current_number: 0,
            current_max: 2,

            enabled: false,
        }
    }

    pub fn reset(&mut self) {
        set_pane_visible(self.base_bar, true);
        set_pane_visible(self.bars[0], false);
        set_pane_visible(self.bars[1], false);
        set_pane_visible(self.bars_background[0], false);
        set_pane_visible(self.bars_background[1], false);
        set_pane_visible(self.number, true);

        if self.original_bar_height < 0.0 {
            self.original_bar_width = get_width_height(self.bars[0]).0;
            self.original_bar_height = get_width_height(self.bars[0]).1;

            self.original_bar2_width = get_width_height(self.bars[1]).0;
            self.original_bar2_height = get_width_height(self.bars[1]).1;
        }

        self.current_number = 0;
        self.current_max = 2;
        self.actual_percentage = 0.0;
        self.visual_percentage = 0.0;
    }

    pub fn update_number(&mut self) {
        set_pane_visible(self.number, true);

        let left_x = self.current_number as f32 / 6.0;
        let right_x = (self.current_number + 1) as f32 / 6.0;

        set_tex_coords(
            self.number,
            [
                left_x, 0.0,
                right_x, 0.0,
                left_x, 1.0,
                right_x, 1.0
            ]
        );
    }

    pub fn set_meter_info(&mut self, current: f32, max: f32, per_level: f32) {
        let bar_total = per_level * 2.0;
        self.current_max = (max / bar_total) as usize;

        let number = current / bar_total;
        let number = number.clamp(0.0, 5.0) as usize;

        let percent = if number == self.current_max {
            1.0
        } else {
            (current % bar_total) / bar_total
        };

        if number != self.current_number && number != 5 {
            self.actual_percentage = percent;
            self.visual_percentage = 0.0;
        } else {
            self.actual_percentage = percent;
        }

        self.current_number = number;
    }

    pub fn change_cap(&mut self, cap: f32) {
        self.current_max = cap as usize;
    }

    pub fn set_tex_coords(&mut self) {
        // Handle the first bar
        set_pane_colors(self.bars_background[0], BACKGROUND_WHITE, BACKGROUND_BLACK);
        set_pane_colors(self.bars_background[1], BACKGROUND_WHITE, BACKGROUND_BLACK);
        set_pane_colors(self.bars[0], FOREGROUND_CHARGE_WHITE, FOREGROUND_CHARGE_BLACK);
        set_pane_colors(self.bars[1], FOREGROUND_CHARGE_WHITE, FOREGROUND_CHARGE_BLACK);

        if self.actual_percentage >= 0.5 {
            set_tex_coords(self.bars_background[0], FULL_TEXCOORDS);
            set_width_height(self.bars_background[0], self.original_bar_width, self.original_bar_height);
            set_pane_visible(self.bars_background[0], true);

            if self.actual_percentage >= 1.0 {
                set_tex_coords(self.bars_background[1], FULL_TEXCOORDS);
                set_width_height(self.bars_background[1], self.original_bar2_width, self.original_bar2_height);
                set_pane_visible(self.bars_background[1], true);
            } else {
                let percentage = (self.actual_percentage - 0.5) / 0.5;
                set_pane_visible(self.bars_background[1], true);
                set_tex_coords(
                    self.bars_background[1],
                    [
                        0.0, 0.0,
                        percentage, 0.0,
                        0.0, 1.0,
                        percentage, 1.0
                    ]
                );
                set_width_height(self.bars_background[1], self.original_bar2_width * percentage, self.original_bar2_height);
            }
        } else {
            set_tex_coords(self.bars_background[1], EMPTY_TEXCOORDS);
            set_width_height(self.bars_background[1], 0.0, 0.0);
            set_pane_visible(self.bars_background[1], false);

            let percentage = self.actual_percentage / 0.5;

            set_tex_coords(
                self.bars_background[0],
                [
                    0.0, 0.0,
                    percentage, 0.0,
                    0.0, 1.0,
                    percentage, 1.0
                ]
            );

            set_width_height(self.bars_background[0], self.original_bar_width * percentage, self.original_bar_height);
            set_pane_visible(self.bars_background[0], true);
        }
    
        if self.visual_percentage >= 0.5 {
            set_tex_coords(self.bars[0], FULL_TEXCOORDS);
            set_width_height(self.bars[0], self.original_bar_width, self.original_bar_height);
            set_pane_visible(self.bars[0], true);

            if self.visual_percentage >= 1.0 {
                set_tex_coords(self.bars[1], FULL_TEXCOORDS);
                set_width_height(self.bars[1], self.original_bar2_width, self.original_bar2_height);
                set_pane_visible(self.bars[1], true);
            } else {
                let percentage = (self.visual_percentage - 0.5) / 0.5;
                set_pane_visible(self.bars[1], true);
                set_tex_coords(
                    self.bars[1],
                    [
                        0.0, 0.0,
                        percentage, 0.0,
                        0.0, 1.0,
                        percentage, 1.0
                    ]
                );
                set_width_height(self.bars[1], self.original_bar2_width * percentage, self.original_bar2_height);
            }
        } else {
            set_tex_coords(self.bars[1], EMPTY_TEXCOORDS);
            set_width_height(self.bars[1], 0.0, 0.0);
            set_pane_visible(self.bars[1], false);

            let percentage = self.visual_percentage / 0.5;

            set_tex_coords(
                self.bars[0],
                [
                    0.0, 0.0,
                    percentage, 0.0,
                    0.0, 1.0,
                    percentage, 1.0
                ]
            );

            set_width_height(self.bars[0], self.original_bar_width * percentage, self.original_bar_height);
            set_pane_visible(self.bars[0], true);
        }
    }

    pub fn update_percentages(&mut self) {
        self.visual_percentage = f32::min(self.visual_percentage + 0.01, self.actual_percentage);
    }
}

impl UiObject for FfMeter {
    fn update(&mut self) {
        self.update_number();
        self.set_tex_coords();
        self.update_percentages();
    }

    fn is_valid(&self) -> bool {
        is_pane_valid(self.base_bar)
            && is_pane_valid(self.number)
    }

    fn set_enable(&mut self, enable: bool) {
        if enable && !self.enabled {
            self.reset();
        } else if !enable {
            set_pane_visible(self.base_bar, false);
            set_pane_visible(self.number, false);
            set_pane_visible(self.bars[0], false);
            set_pane_visible(self.bars[1], false);
            set_pane_visible(self.bars_background[0], false);
            set_pane_visible(self.bars_background[1], false);
        }
        self.enabled = enable;
    }

    fn is_enabled(&self) -> bool {
        self.enabled
    }
}