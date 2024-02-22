use super::*;

const BACKGROUND_WHITE: [f32; 4] = [90.0 / 255.0, 90.0 / 255.0, 90.0 / 255.0, 1.0];
const BACKGROUND_BLACK: [f32; 4] = [24.0 / 255.0, 24.0 / 255.0, 24.0 / 255.0, 1.0];

const FOREGROUND_COLORLESS_WHITE: [f32; 4] = [90.0 / 255.0, 90.0 / 255.0, 90.0 / 255.0, 1.0];
const FOREGROUND_COLORLESS_BLACK: [f32; 4] = [24.0 / 255.0, 24.0 / 255.0, 24.0 / 255.0, 1.0];

const FOREGROUND_RED: [f32; 4] = [197.0 / 255.0, 3.0 / 255.0, 16.0 / 255.0, 1.0];

const FOREGROUND_BLUE: [f32; 4] = [7.0 / 255.0, 33.0 / 255.0, 194.0 / 255.0, 1.0];

const FOREGROUND_YELLOW: [f32; 4] = [244.0 / 255.0, 177.0 / 255.0, 31.0 / 255.0, 1.0];

const FOREGROUND_PURPLE: [f32; 4] = [135.0 / 255.0, 29.0 / 255.0, 186.0 / 255.0, 1.0];

const FOREGROUND_ORANGE: [f32; 4] = [200.0 / 255.0, 63.0 / 255.0, 27.0 / 255.0, 1.0];

const FOREGROUND_GREEN: [f32; 4] = [26.0 / 255.0, 170.0 / 255.0, 12.0 / 255.0, 1.0];

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
pub struct PowerBoard {
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
    pub color_1: i32,
    pub color_2: i32,
    pub current_color: [f32; 4],

    enabled: bool,
}

impl PowerBoard {
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
            color_1: 0,
            color_2: 0,
            current_color: [0.0, 0.0, 0.0, 0.0],

            enabled: false,
        }
    }

    pub fn reset(&mut self) {
        set_pane_visible(self.base_bar, true);
        set_pane_visible(self.bars[0], false);
        set_pane_visible(self.bars[1], false);
        set_pane_visible(self.bars_background[0], false);
        set_pane_visible(self.bars_background[1], false);
        set_pane_visible(self.number, false);

        if self.original_bar_height < 0.0 {
            self.original_bar_width = get_width_height(self.bars[0]).0;
            self.original_bar_height = get_width_height(self.bars[0]).1;

            self.original_bar2_width = get_width_height(self.bars[1]).0;
            self.original_bar2_height = get_width_height(self.bars[1]).1;
        }

        self.current_number = 0;
        self.color_1 = 0;
        self.color_2 = 0;
        self.current_color= [0.0, 0.0, 0.0, 0.0];
        self.actual_percentage = 0.0;
        self.visual_percentage = 0.0;
    }

    pub fn set_meter_info(&mut self, current: f32, max: f32, per_level: f32, color_1: i32, color_2: i32) {
        let bar_total = per_level * 2.0;

        let number = current / bar_total;
        let number = number.clamp(0.0, 2.0) as usize;

        let percent = if number == 1 {
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
        self.color_1 = color_1;
        self.color_2 = color_2;
    }

    pub fn change_color(&mut self, color_1: i32, color_2: i32) {
        self.color_1 = color_1;
        self.color_2 = color_2;
    }

    pub fn set_tex_coords(&mut self) {
        set_pane_colors(self.bars_background[0], BACKGROUND_WHITE, BACKGROUND_BLACK);
        set_pane_colors(self.bars_background[1], BACKGROUND_WHITE, BACKGROUND_BLACK);
        set_pane_colors(self.bars[0], FOREGROUND_COLORLESS_WHITE, FOREGROUND_COLORLESS_BLACK);
        set_pane_colors(self.bars[1], FOREGROUND_COLORLESS_WHITE, FOREGROUND_COLORLESS_BLACK);

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

    pub fn update_color(&mut self) {
        
        if self.color_1 == 1 {
            if self.color_2 == 2 {
                self.current_color = FOREGROUND_PURPLE;
                //println!("and why he ourple");

            }
            else if self.color_2 == 3 {
                self.current_color = FOREGROUND_ORANGE;
                //println!("bornana");
            }
            else {
                self.current_color = FOREGROUND_RED;
                //println!("red");
            }
        }
        else if self.color_1 == 2 {
            if self.color_2 == 1 {
                self.current_color = FOREGROUND_PURPLE;
                //println!("and why he ourple");
            }
            else if self.color_2 == 3 {
                self.current_color = FOREGROUND_GREEN;
                //println!("i like cash from my hair to my ass");
            }
            else {
                self.current_color = FOREGROUND_BLUE;
                //println!("blud");
            }
        }
        else if self.color_1 == 3 {
            if self.color_2 == 1 {
                self.current_color = FOREGROUND_ORANGE;
                //println!("bornana");
            }
            else if self.color_2 == 2 {
                self.current_color = FOREGROUND_GREEN;
                //println!("i like cash from my hair to my ass");
            }
            else {
                self.current_color = FOREGROUND_YELLOW;
                //println!("ielo");
            }
        }
        else {
            self.current_color = FOREGROUND_COLORLESS_BLACK;
                //println!("fuck");
        }
        let g = colorgrad::CustomGradient::new()
            .colors(&[
                colorgrad::Color::from_rgba8(255, 255, 255, 255),
                colorgrad::Color::from_rgba8(
                    (self.current_color[0] * 255.0) as u8,
                    (self.current_color[1] * 255.0) as u8,
                    (self.current_color[2] * 255.0) as u8,
                    (self.current_color[3] * 255.0) as u8
                ),
            ])
            .build()
            .unwrap();

        let color = g.at(1.0);

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
    }

    pub fn update_percentages(&mut self) {
        self.visual_percentage = self.actual_percentage;
    }
}

impl UiObject for PowerBoard {
    fn update(&mut self) {
        self.set_tex_coords();
        self.update_percentages();
        self.update_color();
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