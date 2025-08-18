use super::*;
use super::super::consts::*;

const COLOR_NONE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const COLOR_WATER: [f32; 4] = [0.0 / 255.0, 68.0 / 255.0, 204.0 / 255.0, 1.0];
const COLOR_GRASS: [f32; 4] = [34.0 / 255.0, 195.0 / 255.0, 48.0 / 255.0, 1.0];
const COLOR_FIRE: [f32; 4] = [255.0 / 255.0, 20.0 / 255.0, 20.0 / 255.0, 1.0];
const COLOR_DISABLED: [f32; 4] = [75.0 / 255.0, 75.0 / 255.0, 75.0 / 255.0, 1.0];

const EMPTY_TEXCOORDS: [f32; 8] = [
    0.0, 0.0,
    0.0, 0.0,
    0.0, 1.0,
    0.0, 1.0
];

const FULL_TEXCOORDS: [f32; 8] = [
    0.0, 0.0,
    1.0, 0.0,
    0.0, 1.0,
    1.0, 1.0
];

const WATER_TEXCOORDS: [f32; 8] = [
    2.0 / 3.0, 0.0,
    1.0, 0.0,
    2.0 / 3.0, 1.0,
    1.0, 1.0
];

const GRASS_TEXCOORDS: [f32; 8] = [
    0.0, 0.0,
    1.0 / 3.0, 0.0,
    0.0, 1.0,
    1.0 / 3.0, 1.0
];

const FIRE_TEXCOORDS: [f32; 8] = [
    1.0 / 3.0, 0.0,
    2.0 / 3.0, 0.0,
    1.0 / 3.0, 1.0,
    2.0 / 3.0, 1.0
];

#[derive(Default, Copy, Clone)]
pub struct PledgeMeter {
    // Panes
    pub meter_lightning: u64,
    pub meter_pledge: u64,
    pub meter_base: u64,
    pub meter_bar_bg: u64,
    pub meter_bar_lucario: u64,
    pub meter_bar_pichu: u64,
    pub meter_div: u64,

    // Initial state
    pub meter_bar_bg_width_height: (f32, f32),
    pub meter_bar_lucario_width_height: (f32, f32),
    pub meter_bar_pichu_width_height: (f32, f32),

    // Progress tracking
    pub actual_percentage: f32,
    pub visual_percentage: f32,
    pub disabled: bool,

    // Number tracking
    pub pledge_state: i32,

    is_enabled: bool,
}

impl PledgeMeter {
    pub fn new(layout_data: u64) -> Self {
        let meter_lightning = get_pane_from_layout(layout_data, "poke_meter_lightning\0")
            .expect("Couldn't find poke_meter_lightning");
        let meter_pledge = get_pane_from_layout(layout_data, "poke_meter_pledge\0")
            .expect("Couldn't find poke_meter_pledge");
        let meter_base = get_pane_from_layout(layout_data, "poke_meter_base\0")
            .expect("Couldn't find poke_meter_base");
        let meter_bar_bg = get_pane_from_layout(layout_data, "poke_meter_bar_bg\0")
            .expect("Couldn't find poke_meter_bar");
        let meter_bar_lucario = get_pane_from_layout(layout_data, "poke_meter_bar_lucario\0")
            .expect("Couldn't find poke_meter_bar_lucario");
        let meter_bar_pichu = get_pane_from_layout(layout_data, "poke_meter_bar_pichu\0")
            .expect("Couldn't find poke_meter_bar_pichu");
        let meter_div = get_pane_from_layout(layout_data, "poke_meter_div\0")
            .expect("Couldn't find poke_meter_div");

        return Self {
            meter_lightning,
            meter_pledge,
            meter_base,
            meter_bar_bg,
            meter_bar_lucario,
            meter_bar_pichu,
            meter_div,

            meter_bar_bg_width_height: (-1.0, -1.0),
            meter_bar_lucario_width_height: (-1.0, -1.0),
            meter_bar_pichu_width_height: (-1.0, -1.0),

            actual_percentage: -1.0,
            visual_percentage: -1.0,
            disabled: false,

            pledge_state: *PLEDGE_STATE_NONE,

            is_enabled: false
        };
    }

    pub fn reset(&mut self) {
        set_pane_visible(self.meter_lightning, false);
        set_pane_visible(self.meter_pledge, true);
        set_pane_visible(self.meter_base, true);
        set_pane_visible(self.meter_bar_bg, true);
        set_pane_visible(self.meter_bar_lucario, false);
        set_pane_visible(self.meter_bar_pichu, true);
        set_pane_visible(self.meter_div, false);

        if self.meter_bar_bg_width_height == (-1.0, -1.0) {
            self.meter_bar_bg_width_height = get_width_height(self.meter_bar_bg);
        }
        if self.meter_bar_lucario_width_height == (-1.0, -1.0) {
            self.meter_bar_lucario_width_height = get_width_height(self.meter_bar_lucario);
        }
        if self.meter_bar_pichu_width_height == (-1.0, -1.0) {
            self.meter_bar_pichu_width_height = get_width_height(self.meter_bar_pichu);
        }

        self.actual_percentage = 0.0;
        self.visual_percentage = 0.0;

        self.pledge_state = *PLEDGE_STATE_NONE;
    }

    pub fn set_meter_info(&mut self, current_pledge: f32, max_pledge: f32, current_swap: f32, max_swap: f32, pledge_state: i32, disabled: bool) {
        let percent = current_pledge.clamp(0.0, max_pledge) / max_pledge;
        self.actual_percentage = percent;

        if pledge_state != self.pledge_state {
            self.visual_percentage = self.actual_percentage;
        }
        if pledge_state == *PLEDGE_STATE_NONE {
            self.actual_percentage = 0.0;
            self.visual_percentage = 0.0;
        }
        self.pledge_state = pledge_state;
        self.disabled = disabled;
    }

    pub fn update_meter_progress(&mut self) {
        set_tex_coords(
            self.meter_bar_pichu,
            [
                1.0 - self.visual_percentage, 0.0,
                1.0, 0.0,
                1.0 - self.visual_percentage, 1.0,
                1.0, 1.0
            ]
        );
        set_width_height(self.meter_bar_pichu, self.meter_bar_pichu_width_height.0 * self.visual_percentage, self.meter_bar_pichu_width_height.1);

        // meter_bar_bg
        set_tex_coords(
            self.meter_bar_bg,
            FULL_TEXCOORDS
        );
        set_width_height(self.meter_bar_bg, self.meter_bar_bg_width_height.0, self.meter_bar_bg_width_height.1);
    }

    pub fn update_percentages(&mut self) {
        let speed = 0.04;
        if self.visual_percentage < self.actual_percentage {
            self.visual_percentage = f32::min(self.visual_percentage + speed, self.actual_percentage);
        } else {
            self.visual_percentage = f32::max(self.visual_percentage - speed, self.actual_percentage);
        }
    }

    pub fn update_charged_visuals(&mut self) {
        let bar_color =  if self.disabled { COLOR_DISABLED } 
        else {
            match self.pledge_state {
                _ if self.pledge_state == *PLEDGE_STATE_WATER => COLOR_WATER,
                _ if self.pledge_state == *PLEDGE_STATE_GRASS => COLOR_GRASS,
                _ if self.pledge_state == *PLEDGE_STATE_FIRE => COLOR_FIRE,
                _ => COLOR_NONE,
            }
        };
        let symbol_coords = match self.pledge_state {
            _ if self.pledge_state == *PLEDGE_STATE_WATER => WATER_TEXCOORDS,
            _ if self.pledge_state == *PLEDGE_STATE_GRASS => GRASS_TEXCOORDS,
            _ if self.pledge_state == *PLEDGE_STATE_FIRE => FIRE_TEXCOORDS,
            _ => EMPTY_TEXCOORDS,
        };
        set_pane_colors(self.meter_bar_pichu, bar_color, bar_color);
        set_tex_coords(
            self.meter_pledge,
            symbol_coords
        );
    }
}

impl UiObject for PledgeMeter {
    fn update(&mut self) {
        self.update_meter_progress();
        self.update_percentages();
        self.update_charged_visuals();
    }

    fn is_valid(&self) -> bool {
        return is_pane_valid(self.meter_lightning)
            && is_pane_valid(self.meter_pledge)
            && is_pane_valid(self.meter_base)
            && is_pane_valid(self.meter_bar_bg)
            && is_pane_valid(self.meter_bar_lucario)
            && is_pane_valid(self.meter_bar_pichu)
            && is_pane_valid(self.meter_div);
    }

    fn set_enable(&mut self, enable: bool) {
        if !enable {
            set_pane_visible(self.meter_lightning, false);
            set_pane_visible(self.meter_pledge, false);
            set_pane_visible(self.meter_base, false);
            set_pane_visible(self.meter_bar_bg, false);
            set_pane_visible(self.meter_bar_lucario, false);
            set_pane_visible(self.meter_bar_pichu, false);
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