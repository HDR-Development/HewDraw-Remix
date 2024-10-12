use super::*;
use interpolation::Lerp;

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

const HUE_EMPTY: f32 = 0.0;
const HUE_FULL: f32 = 1.0 / 3.0;
const HUE_BUFFER: f32 = 0.125;

fn hsl_to_rgba(h: f32, s: f32, l: f32) -> [f32; 4] {
    let h = h.clamp(0.0, 1.0);
    let s = s.clamp(0.0, 1.0);
    let l = l.clamp(0.0, 1.0);
    if (s == 0.0) {
        return [l, l, l, 1.0];
    }

    let q = if l < 0.5 { l * (1.0 + s) } else { l + s - l * s};
    let p = 2.0 * l - q;
    let r = hue_to_rgb(p, q, h + 1.0 / 3.0);
    let g = hue_to_rgb(p, q, h);
    let b = hue_to_rgb(p, q, h - 1.0 / 3.0);
    return [r, g, b, 1.0];
}

fn hue_to_rgb(p: f32, q: f32, t: f32) -> f32 {
    let mut t = t;
    if (t < 0.0) { t += 1.0 };
    if (t > 1.0) { t -= 1.0 };
    if (t < 1.0 / 6.0) { return p + (q - p) * 6.0 * t };
    if (t < 0.5) { return q };
    if (t < 2.0 / 3.0) { return p + (q - p) * (2.0 / 3.0 - t) * 6.0 };
    return p;
}

#[derive(Default, Copy, Clone)]
pub struct RobotMeter {
    // Panes
    pub meter_base: u64,
    pub meter_bar_bg: u64,
    pub meter_bar: u64,
    pub meter_arrow: u64,

    // Initial state
    pub meter_bar_bg_width_height: (f32, f32),
    pub meter_bar_width_height: (f32, f32),
    pub meter_arrow_width_height: (f32, f32),

    // Progress tracking
    pub actual_percentage: f32,
    pub visual_percentage: f32,

    is_enabled: bool,
}

impl RobotMeter {
    pub fn new(layout_data: u64) -> Self {
        let meter_base = get_pane_from_layout(layout_data, "fuel_meter_base\0")
            .expect("Couldn't find fuel_meter_base");
        let meter_bar_bg = get_pane_from_layout(layout_data, "fuel_meter_bar_bg\0")
            .expect("Couldn't find fuel_meter_bar_bg");
        let meter_bar = get_pane_from_layout(layout_data, "fuel_meter_bar\0")
            .expect("Couldn't find fuel_meter_bar");
        let meter_arrow = get_pane_from_layout(layout_data, "fuel_arrow\0")
            .expect("Couldn't find fuel_arrow");

        return Self {
            meter_base,
            meter_bar_bg,
            meter_bar,
            meter_arrow,

            meter_bar_bg_width_height: (-1.0, -1.0),
            meter_bar_width_height: (-1.0, -1.0),
            meter_arrow_width_height: (-1.0, -1.0),

            actual_percentage: -1.0,
            visual_percentage: -1.0,

            is_enabled: false
        };
    }

    pub fn reset(&mut self) {
        set_pane_visible(self.meter_base, true);
        set_pane_visible(self.meter_bar_bg, true);
        set_pane_visible(self.meter_bar, true);
        set_pane_visible(self.meter_arrow, true);

        self.meter_bar_bg_width_height = get_width_height(self.meter_bar_bg);
        self.meter_bar_width_height = get_width_height(self.meter_bar);
        self.meter_arrow_width_height = get_width_height(self.meter_arrow);

        self.actual_percentage = 0.0;
        self.visual_percentage = 0.0;
    }

    pub fn set_meter_info(&mut self, current: f32, _max: f32, per_level: f32) {
        let percent = current / _max;
        self.actual_percentage = percent;
        self.visual_percentage = self.actual_percentage;
    }

    pub fn update_meter_progress(&mut self) {
        // meter_bar
        set_tex_coords(
            self.meter_bar,
            [
                0.0, 0.0,
                1.0, 0.0,
                0.0, self.visual_percentage,
                1.0, self.visual_percentage
            ]
        );
        let hue = (HUE_EMPTY - HUE_BUFFER).lerp(&(HUE_FULL + HUE_BUFFER), &self.visual_percentage).clamp(HUE_EMPTY, HUE_FULL);
        let rgba = hsl_to_rgba(hue, 1.0, 0.5);
        set_pane_colors(self.meter_bar, rgba, rgba);
        set_width_height(self.meter_bar, self.meter_bar_width_height.0, self.meter_bar_width_height.1 * self.visual_percentage);
        set_pane_visible(self.meter_bar, true);

        // meter_arrow
        let new_meter_arrow_height = Lerp::lerp(&(self.meter_bar_width_height.1 / 24.0), &self.meter_arrow_width_height.1, &self.visual_percentage);
        let ratio = new_meter_arrow_height / self.meter_arrow_width_height.1;
        set_tex_coords(
            self.meter_arrow,
            [
                0.0, 0.0,
                1.0, 0.0,
                0.0, ratio,
                1.0, ratio
            ]
        );
        set_width_height(self.meter_arrow, self.meter_arrow_width_height.0, new_meter_arrow_height);
        set_pane_visible(self.meter_arrow, true);

        // meter_bar_bg
        set_tex_coords(
            self.meter_bar_bg,
            FULL_TEXCOORDS
        );
        set_width_height(self.meter_bar_bg, self.meter_bar_bg_width_height.0, self.meter_bar_bg_width_height.1);
        set_pane_visible(self.meter_bar_bg, true);
    }

    pub fn update_percentages(&mut self) {
        let speed = 0.04;
        if self.visual_percentage < self.actual_percentage {
            self.visual_percentage = f32::min(self.visual_percentage + speed, self.actual_percentage);
        } else {
            self.visual_percentage = f32::max(self.visual_percentage - speed, self.actual_percentage);
        }
    }
}

impl UiObject for RobotMeter {
    fn update(&mut self) {
        self.update_meter_progress();
        self.update_percentages();
    }

    fn is_valid(&self) -> bool {
        return is_pane_valid(self.meter_base)
            && is_pane_valid(self.meter_bar_bg)
            && is_pane_valid(self.meter_bar)
            && is_pane_valid(self.meter_arrow);
    }

    fn set_enable(&mut self, enable: bool) {
        if !enable {
            set_pane_visible(self.meter_base, false);
            set_pane_visible(self.meter_bar_bg, false);
            set_pane_visible(self.meter_bar, false);
            set_pane_visible(self.meter_arrow, false);
        } else if !self.is_enabled {
            self.reset();
        }
        self.is_enabled = enable;
    }

    fn is_enabled(&self) -> bool {
        return self.is_enabled;
    }
}