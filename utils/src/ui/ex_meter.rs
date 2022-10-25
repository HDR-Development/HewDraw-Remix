use super::*;

const BACKGROUND_WHITE: [f32; 4] = [244.0 / 255.0, 177.0 / 255.0, 31.0 / 255.0, 1.0];
const BACKGROUND_BLACK: [f32; 4] = [88.0 / 255.0, 61.0 / 255.0, 4.0 / 255.0, 1.0];

const FOREGROUND_CHARGE_WHITE: [f32; 4] = [11.0 / 255.0, 115.0 / 255.0, 234.0 / 255.0, 1.0];
const FOREGROUND_CHARGE_BLACK: [f32; 4] = [6.0 / 255.0, 67.0 / 255.0, 136.0 / 255.0, 1.0];

const FOREGROUND_CHARGED_WHITE: [f32; 4] = [133.0 / 255.0, 187.0 / 255.0, 249.0 / 255.0, 1.0];
const FOREGROUND_CHARGED_BLACK: [f32; 4] = [9.0 / 255.0, 91.0 / 255.0, 185.0 / 255.0, 1.0];

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
pub struct ExMeter {
    // Panes
    pub base_bar: u64,
    pub bars: [u64; 2],
    pub bars_background: [u64; 2],
    pub ex_icon: u64,
    pub number: u64,

    // Initial state
    pub original_bar_width: f32,
    pub original_bar_height: f32,

    // Progress tracking
    pub actual_percentage: f32,
    pub visual_percentage: f32,

    // Number tracking
    pub current_number: usize,

    // Pulsing
    pub current_pulse_frame: f32,

    enabled: bool,
}

impl ExMeter {
    pub fn new(layout_data: u64) -> Self {
        let base_bar = super::get_pane_from_layout(layout_data, "ex_meter_base\0")
            .expect("Could not find the base EX meter!");

        let bar1 = super::get_pane_from_layout(layout_data, "ex_meter_bar1\0")
            .expect("Could not find first bar for EX meter!");
        
        let bar2 = super::get_pane_from_layout(layout_data, "ex_meter_bar2\0")
            .expect("Could not find second bar for EX meter!");

        let bar1_bg = super::get_pane_from_layout(layout_data, "ex_meter_bar1_bg\0")
            .expect("Could not find first bg bar for EX meter!");

        let bar2_bg = super::get_pane_from_layout(layout_data, "ex_meter_bar2_bg\0")
            .expect("Could not find second bg bar for EX meter!");

        let ex_icon = super::get_pane_from_layout(layout_data, "ex_meter_ex\0")
            .expect("Could not find ex icon for EX meter!");

        let number = super::get_pane_from_layout(layout_data, "ex_meter_number\0")
            .expect("Could no find number for EX meter!");

        Self {
            base_bar,
            bars: [bar1, bar2],
            bars_background: [bar1_bg, bar2_bg],
            ex_icon,
            number,

            original_bar_width: -1.0,
            original_bar_height: -1.0,

            actual_percentage: -1.0,
            visual_percentage: -1.0,

            current_number: 0,

            current_pulse_frame: 0.0,
            enabled: false,
        }
    }

    pub fn reset(&mut self) {
        set_pane_visible(self.base_bar, true);
        set_pane_visible(self.ex_icon, false);
        set_pane_visible(self.bars[0], false);
        set_pane_visible(self.bars[1], false);
        set_pane_visible(self.bars_background[0], false);
        set_pane_visible(self.bars_background[1], false);
        set_pane_visible(self.number, true);

        if self.original_bar_height < 0.0 {
            self.original_bar_width = get_width_height(self.bars[0]).0;
            self.original_bar_height = get_width_height(self.bars[0]).1;
        }

        self.current_number = 0;
        self.actual_percentage = 0.0;
        self.visual_percentage = 0.0;
    }

    pub fn update_number(&mut self) {
        if self.current_number == 5 {
            set_pane_visible(self.number, false);
            set_pane_visible(self.ex_icon, true);
        } else {
            set_pane_visible(self.number, true);
            set_pane_visible(self.ex_icon, false);
        }

        let left_x = self.current_number as f32 / 5.0;
        let right_x = (self.current_number + 1) as f32 / 5.0;

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

        let number = current / bar_total;
        let number = number.clamp(0.0, 5.0) as usize;

        let percent = if number == 5 {
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

        if number != 5 {
            self.current_pulse_frame = 0.0;
        }

        self.current_number = number;
    }

    pub fn set_tex_coords(&mut self) {
        // Handle the first bar
        set_pane_colors(self.bars_background[0], BACKGROUND_BLACK, BACKGROUND_WHITE);
        set_pane_colors(self.bars_background[1], BACKGROUND_BLACK, BACKGROUND_WHITE);
        set_pane_colors(self.bars[0], FOREGROUND_CHARGED_BLACK, FOREGROUND_CHARGED_WHITE);
        set_pane_colors(self.bars[1], FOREGROUND_CHARGED_BLACK, FOREGROUND_CHARGED_WHITE);

        if self.actual_percentage >= 0.5 {
            set_tex_coords(self.bars_background[0], FULL_TEXCOORDS);
            set_width_height(self.bars_background[0], self.original_bar_width, self.original_bar_height);
            set_pane_visible(self.bars_background[0], true);

            if self.actual_percentage >= 1.0 {
                set_tex_coords(self.bars_background[1], FULL_TEXCOORDS);
                set_width_height(self.bars_background[1], self.original_bar_width, self.original_bar_height);
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
                set_width_height(self.bars_background[1], self.original_bar_width * percentage, self.original_bar_height);
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
            set_pane_colors(self.bars[0], FOREGROUND_CHARGED_BLACK, FOREGROUND_CHARGED_WHITE);

            if self.visual_percentage >= 1.0 {
                set_tex_coords(self.bars[1], FULL_TEXCOORDS);
                set_width_height(self.bars[1], self.original_bar_width, self.original_bar_height);
                set_pane_visible(self.bars[1], true);
                set_pane_colors(self.bars[1], FOREGROUND_CHARGED_BLACK, FOREGROUND_CHARGED_WHITE);
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
                set_width_height(self.bars[1], self.original_bar_width * percentage, self.original_bar_height);
                set_pane_colors(self.bars[1], FOREGROUND_CHARGE_BLACK, FOREGROUND_CHARGE_WHITE);
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
            set_pane_colors(self.bars[0], FOREGROUND_CHARGE_BLACK, FOREGROUND_CHARGE_WHITE);
        }
    }

    pub fn update_pulse(&mut self) {
        if self.current_number != 5 || self.actual_percentage > self.visual_percentage {
            return;
        }

        let g = colorgrad::CustomGradient::new()
            .colors(&[
                colorgrad::Color::from_rgba8(255, 255, 255, 255),
                colorgrad::Color::from_rgba8(
                    (FOREGROUND_CHARGED_BLACK[0] * 255.0) as u8,
                    (FOREGROUND_CHARGED_BLACK[1] * 255.0) as u8,
                    (FOREGROUND_CHARGED_BLACK[2] * 255.0) as u8,
                    (FOREGROUND_CHARGED_BLACK[3] * 255.0) as u8
                ),
            ])
            .build()
            .unwrap();

        let distance = ((-(self.current_pulse_frame * 2.0).to_radians().cos() + 1.0) / 2.0).abs().powf(0.5);

        let color = g.at(distance as f64);

        set_pane_colors(
            self.ex_icon,
            [color.r as f32, color.g as f32, color.b as f32, color.a as f32],
            [color.r as f32, color.g as f32, color.b as f32, color.a as f32],
        );

        set_pane_colors(
            self.bars[0],
            [color.r as f32, color.g as f32, color.b as f32, color.a as f32],
            [color.r as f32, color.g as f32, color.b as f32, color.a as f32],
        );

        set_pane_colors(
            self.bars[1],
            [color.r as f32, color.g as f32, color.b as f32, color.a as f32],
            [color.r as f32, color.g as f32, color.b as f32, color.a as f32],
        );
        self.current_pulse_frame += 1.0;

    }

    pub fn update_percentages(&mut self) {
        self.visual_percentage = f32::min(self.visual_percentage + 0.01, self.actual_percentage);
    }
}

impl UiObject for ExMeter {
    fn update(&mut self) {
        self.update_number();
        self.set_tex_coords();
        self.update_percentages();
        self.update_pulse();
    }

    fn is_valid(&self) -> bool {
        is_pane_valid(self.base_bar)
            && is_pane_valid(self.ex_icon)
            && is_pane_valid(self.number)
    }

    fn set_enable(&mut self, enable: bool) {
        if enable && !self.enabled {
            self.reset();
        } else if !enable {
            set_pane_visible(self.base_bar, false);
            set_pane_visible(self.ex_icon, false);
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