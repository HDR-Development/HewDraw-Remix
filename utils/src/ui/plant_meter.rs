use super::*;
use interpolation::Lerp;

const FULL_TEXCOORDS: [f32; 8] = [
    0.0, 0.0,
    1.0, 0.0,
    0.0, 1.0,
    1.0, 1.0
];

#[derive(Default, Copy, Clone)]
pub struct PlantMeter {
    pub icon: u64,
    pub element: i32,

    is_enabled: bool,
}

impl PlantMeter {
    pub fn new(layout_data: u64) -> Self {
        let element_pane = get_pane_from_layout(layout_data, "plant_element\0")
            .expect("Couldn't find plant_element");

        return Self {
            icon: element_pane,
            element: 0,

            is_enabled: false
        };
    }

    pub fn reset(&mut self) {
        set_pane_visible(self.icon, true);
        self.element = 0;
    }

    pub fn set_meter_info(&mut self, element: i32) {
        self.element = element;
    }

    pub fn update_icon(&mut self) {
        let offset = match (self.element) {
            0 => 0.0,
            1 => 1.0,
            2 => 2.0,
            _ => -1.0
        };
        if offset < 0.0 {
            set_pane_visible(self.icon, false);
            return;
        }
        let offset = offset / 3.0;
        let len = 1.0 / 3.0;
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

impl UiObject for PlantMeter {
    fn update(&mut self) {
        self.update_icon();
    }

    fn is_valid(&self) -> bool {
        return is_pane_valid(self.icon);
    }

    fn set_enable(&mut self, enable: bool) {
        if !enable {
            set_pane_visible(self.icon, false);
        } else if !self.is_enabled {
            self.reset();
        }
        self.is_enabled = enable;
    }

    fn is_enabled(&self) -> bool {
        return self.is_enabled;
    }
}