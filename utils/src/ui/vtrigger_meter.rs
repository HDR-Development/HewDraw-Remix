use crate::offsets;

use super::*;

const VTRIGGER_PULSE: [f32; 4] = [138.0 / 255.0, 190.0 / 255.0, 249.0 / 255.0, 1.0];

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
pub struct VTriggerMeter {
    // Panes
    pub meter_base: u64,
    pub vtrigger_txt: u64,
    pub meter_bar_progress: u64,
    pub meter_bar: u64,
    pub meter_bar_base: u64,
    pub meter_bar_full_1: u64,
    pub meter_bar_full_2: u64,

    // Initial State
    pub meter_bar_progress_size: (f32, f32),
    pub meter_bar_size: (f32, f32),

    // Progress tracking
    pub actual_percentage: f32,
    pub visual_percentage: f32,

    // Pulsing
    pub current_pulse_frame: f32,

    // Meter Sizing
    pub level: i32,
    pub level_max: i32,

    pub is_vtrigger: bool,
    enabled: bool,
}

impl VTriggerMeter {
    pub fn new(layout_data: u64) -> Self {
        let meter_base = super::get_pane_from_layout(layout_data, "v_trigger_meter_base\0")
            .expect("Could not find v_trigger_meter_base!");
        let vtrigger_txt = super::get_pane_from_layout(layout_data, "v_trigger_txt\0")
            .expect("Could not find v_trigger_txt!");
        let meter_bar_progress = super::get_pane_from_layout(layout_data, "v_trigger_meter_bar_prog\0")
            .expect("Could not find v_trigger_meter_bar_prog!");
        let meter_bar = super::get_pane_from_layout(layout_data, "v_trigger_meter_bar\0")
            .expect("Could not find v_trigger_meter_bar!");
        let meter_bar_base = super::get_pane_from_layout(layout_data, "v_trigger_meter_bar_base\0")
            .expect("Could not find v_trigger_meter_bar_base!");
        let meter_bar_full_1 = super::get_pane_from_layout(layout_data, "meter_bar_full_one\0")
            .expect("Could not find meter_bar_full_one!");
        let meter_bar_full_2 = super::get_pane_from_layout(layout_data, "meter_bar_full_two\0")
            .expect("Could not find meter_bar_full_two!");

        Self {
            meter_base,
            vtrigger_txt,
            meter_bar_progress,
            meter_bar,
            meter_bar_base,
            meter_bar_full_1,
            meter_bar_full_2,

            meter_bar_progress_size: (-1.0, -1.0),
            meter_bar_size: (-1.0, -1.0),

            actual_percentage: -1.0,
            visual_percentage: -1.0,

            current_pulse_frame: 0.0,

            level: 0,
            level_max: 4,

            is_vtrigger: false,
            enabled: false,
        }
    }

    pub fn reset(&mut self) {
        set_pane_visible(self.meter_base, true);
        set_pane_visible(self.vtrigger_txt, false);
        set_pane_visible(self.meter_bar_progress, false);
        set_pane_visible(self.meter_bar, false);
        set_pane_visible(self.meter_bar_base, true);
        set_pane_visible(self.meter_bar_full_1, false);
        set_pane_visible(self.meter_bar_full_2, false);

        self.meter_bar_progress_size = get_width_height(self.meter_bar_progress);
        self.meter_bar_size = get_width_height(self.meter_bar);

        self.actual_percentage = 0.0;
        self.visual_percentage = 0.0;

        self.level = 0;
        self.level_max = 4;

        self.is_vtrigger = false;
    }

    pub fn set_meter_info(&mut self, current: f32, level_max: i32, per_level: f32, is_vtrigger: bool) {
        self.actual_percentage = current / (per_level * level_max as f32);
        self.level = (current / per_level).floor() as i32;
        self.level_max = level_max;
        self.is_vtrigger = is_vtrigger;
        if self.level < self.level_max || self.is_vtrigger {
            self.current_pulse_frame = 0.0;
        }
    }

    pub fn update_percentages(&mut self) {
        let speed = 0.04;
        if self.visual_percentage < self.actual_percentage {
            self.visual_percentage = f32::min(self.visual_percentage + speed, self.actual_percentage);
        } else {
            self.visual_percentage = f32::max(self.visual_percentage - speed, self.actual_percentage);
        }
    }

    pub fn update_meter_progress(&mut self) {
        // meter_base
        let size = 6.0;
        let bonus_offset = if self.level_max <= 4 { 0.0 } else { 3.0 };
        let offset = if self.is_vtrigger {
            (0.0 + bonus_offset) / size
        } else if self.level >= self.level_max {
            (1.0 + bonus_offset) / size
        } else {
            (2.0 + bonus_offset) /size
        };
        let len = 1.0 / size;
        set_tex_coords(
            self.meter_base, 
            [
                0.0, offset,
                1.0, offset,
                0.0, offset + len,
                1.0, offset + len
            ]
        );
        set_pane_visible(self.meter_base, true);

        // vtrigger_txt
        set_pane_visible(self.vtrigger_txt, self.level >= self.level_max || self.is_vtrigger);

        // meter_bar_progress
        let short_ratio = if self.level_max <= 4 { 0.68 } else { 1.0 };
        set_tex_coords(
            self.meter_bar_progress, 
            [
                0.0, offset,
                self.actual_percentage * short_ratio, offset,
                0.0, offset + len,
                self.actual_percentage * short_ratio, offset + len
            ]
        );
        set_width_height(self.meter_bar_progress, self.meter_bar_progress_size.0 * self.actual_percentage * short_ratio, self.meter_bar_progress_size.1);
        set_pane_visible(self.meter_bar_progress, self.level < self.level_max);

        // meter_bar
        set_tex_coords(
            self.meter_bar, 
            [
                0.0, offset,
                self.visual_percentage * short_ratio, offset,
                0.0, offset + len,
                self.visual_percentage * short_ratio, offset + len
            ]
        );
        set_width_height(self.meter_bar, self.meter_bar_size.0 * self.visual_percentage * short_ratio, self.meter_bar_size.1);
        set_pane_visible(self.meter_bar, self.level < self.level_max);

        // meter_bar_base
        let size = 2.0;
        let offset = if self.level_max <= 4 {
            0.0 / size
        } else {
            1.0 / size
        };
        let len = 1.0 / size;
        set_tex_coords(
            self.meter_bar_base, 
            [
                0.0, offset,
                1.0, offset,
                0.0, offset + len,
                1.0, offset + len
            ]
        );
        set_pane_visible(self.meter_bar_base, self.level < self.level_max && !self.is_vtrigger);

        // meter_bar_full_1
        set_pane_visible(
            self.meter_bar_full_1, 
            !self.is_vtrigger 
                && self.level < self.level_max
                && self.level >= 2
        );

        // meter_bar_full_2
        set_pane_visible(
            self.meter_bar_full_2, 
            !self.is_vtrigger 
                && self.level < self.level_max
                && self.level >= 4
        );
    }

    pub fn update_pulse(&mut self) {
        if self.level < self.level_max || self.is_vtrigger { return; }

        let g = colorgrad::CustomGradient::new()
            .colors(&[
                colorgrad::Color::from_rgba8(255, 255, 255, 255),
                colorgrad::Color::from_rgba8(
                    (VTRIGGER_PULSE[0] * 255.0) as u8,
                    (VTRIGGER_PULSE[1] * 255.0) as u8,
                    (VTRIGGER_PULSE[2] * 255.0) as u8,
                    (VTRIGGER_PULSE[3] * 255.0) as u8
                ),
            ])
            .build()
            .unwrap();

        let distance = ((-(self.current_pulse_frame * 2.0).to_radians().cos() + 1.0) / 2.0).abs().powf(0.5);
        let color = g.at(distance as f64);
        set_pane_colors(
            self.vtrigger_txt,
            [color.r as f32, color.g as f32, color.b as f32, color.a as f32],
            [color.r as f32, color.g as f32, color.b as f32, color.a as f32],
        );
        self.current_pulse_frame += 1.0;
    }

}

impl UiObject for VTriggerMeter {
    fn update(&mut self) {
        self.update_percentages();
        self.update_meter_progress();
        self.update_pulse();
    }

    fn is_valid(&self) -> bool {
        return is_pane_valid(self.meter_base)
            && is_pane_valid(self.vtrigger_txt)
            && is_pane_valid(self.meter_bar_progress)
            && is_pane_valid(self.meter_bar)
            && is_pane_valid(self.meter_base)
            && is_pane_valid(self.meter_bar_full_1)
            && is_pane_valid(self.meter_bar_full_2);
    }

    fn set_enable(&mut self, enable: bool) {
        if enable && !self.enabled {
            self.reset();
        } else if !enable {
            set_pane_visible(self.meter_base, false);
            set_pane_visible(self.vtrigger_txt, false);
            set_pane_visible(self.meter_bar_progress, false);
            set_pane_visible(self.meter_bar, false);
            set_pane_visible(self.meter_bar_base, false);
            set_pane_visible(self.meter_bar_full_1, false);
            set_pane_visible(self.meter_bar_full_2, false);
        }
        self.enabled = enable;
    }

    fn is_enabled(&self) -> bool {
        self.enabled
    }
}