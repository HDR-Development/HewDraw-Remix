use super::*;

const COLOR_CHARGED: [f32; 4] = [255.0 / 255.0, 216.0 / 255.0, 59.0 / 255.0, 1.0];
const COLOR_PROGRESS: [f32; 4] = [255.0 / 255.0, 255.0 / 255.0, 255.0 / 215.0, 1.0];

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
pub struct PichuMeter {
    // Panes
    pub meter_lightning: u64,
    pub meter_base: u64,
    pub meter_bar: u64,
    pub meter_bar_lucario: u64,
    pub meter_div: u64,

    // Initial state
    pub meter_bar_width_height: (f32, f32),
    pub meter_bar_lucario_width_height: (f32, f32),

    // Progress tracking
    pub actual_percentage: f32,
    pub visual_percentage: f32,

    // Number tracking
    pub is_charged: bool,

    is_enabled: bool,
}

impl PichuMeter {
    pub fn new(layout_data: u64) -> Self {
        let meter_lightning = get_pane_from_layout(layout_data, "poke_meter_lightning\0")
            .expect("Couldn't find poke_meter_lightning");
        let meter_base = get_pane_from_layout(layout_data, "poke_meter_base\0")
            .expect("Couldn't find poke_meter_base");
        let meter_bar = get_pane_from_layout(layout_data, "poke_meter_bar\0")
            .expect("Couldn't find poke_meter_bar");
        let meter_bar_lucario = get_pane_from_layout(layout_data, "poke_meter_bar_lucario\0")
            .expect("Couldn't find poke_meter_bar_lucario");
        let meter_div = get_pane_from_layout(layout_data, "poke_meter_div\0")
            .expect("Couldn't find poke_meter_div");

        return Self {
            meter_lightning,
            meter_base,
            meter_bar,
            meter_bar_lucario,
            meter_div,

            meter_bar_width_height: (-1.0, -1.0),
            meter_bar_lucario_width_height: (-1.0, -1.0),

            actual_percentage: -1.0,
            visual_percentage: -1.0,

            is_charged: false,

            is_enabled: false
        };
    }

    pub fn reset(&mut self) {
        set_pane_visible(self.meter_lightning, true);
        set_pane_visible(self.meter_base, true);
        set_pane_visible(self.meter_bar, false);
        set_pane_visible(self.meter_bar_lucario, false);
        set_pane_visible(self.meter_div, false);

        self.meter_bar_width_height = get_width_height(self.meter_bar);
        self.meter_bar_lucario_width_height = get_width_height(self.meter_bar_lucario);

        self.actual_percentage = 0.0;
        self.visual_percentage = 0.0;

        self.is_charged = false;
    }

    pub fn set_meter_info(&mut self, current: f32, _max: f32, per_level: f32, charged: bool) {
        let percent = current / _max;
        self.actual_percentage = percent;
        self.is_charged = charged;
    }

    pub fn set_tex_coords(&mut self) {
        // actual percentage
        set_tex_coords(
            self.meter_bar,
            [
                0.0, 0.0,
                self.actual_percentage, 0.0,
                0.0, 1.0,
                self.actual_percentage, 1.0
            ]
        );
        set_width_height(self.meter_bar, self.meter_bar_width_height.0 * self.actual_percentage, self.meter_bar_width_height.1);
        set_pane_visible(self.meter_bar, true);
        
        // visual percentage
        set_tex_coords(
            self.meter_bar_lucario,
            [
                0.0, 0.0,
                self.visual_percentage, 0.0,
                0.0, 1.0,
                self.visual_percentage, 1.0
            ]
        );
        set_width_height(self.meter_bar_lucario, self.meter_bar_lucario_width_height.0 * self.visual_percentage, self.meter_bar_lucario_width_height.1);
        set_pane_visible(self.meter_bar_lucario, true);
    }

    pub fn update_percentages(&mut self) {
        self.visual_percentage = f32::min(self.visual_percentage + 0.01, self.actual_percentage);
    }
}

impl UiObject for PichuMeter {
    fn update(&mut self) {
        self.set_tex_coords();
        self.update_percentages();
        if self.is_charged {
            set_pane_visible(self.meter_bar_lucario, false);
            set_pane_colors(self.meter_bar, COLOR_CHARGED, COLOR_CHARGED);
            set_tex_coords(
                self.meter_lightning,
                [
                    0.5, 0.0,
                    1.0, 0.0,
                    0.5, 1.0,
                    1.0, 1.0
                ]
            );
        } else {
            set_pane_colors(self.meter_bar, COLOR_PROGRESS, COLOR_PROGRESS);
            set_tex_coords(
                self.meter_lightning,
                [
                    0.0, 0.0,
                    0.5, 0.0,
                    0.0, 1.0,
                    0.5, 1.0
                ]
            );
        }
    }

    fn is_valid(&self) -> bool {
        return is_pane_valid(self.meter_base)
            && is_pane_valid(self.meter_bar)
            && is_pane_valid(self.meter_bar_lucario)
            && is_pane_valid(self.meter_div);
    }

    fn set_enable(&mut self, enable: bool) {
        if !enable {
            set_pane_visible(self.meter_lightning, false);
            set_pane_visible(self.meter_base, false);
            set_pane_visible(self.meter_bar, false);
            set_pane_visible(self.meter_bar_lucario, false);
            set_pane_visible(self.meter_div, false);
        } else if !self.is_enabled {
            self.reset();
        }
        self.is_enabled = enable;
    }

    fn is_enabled(&self) -> bool {
        return self.is_enabled;
    }
}