use crate::sketch::*;

#[derive(Clone, Debug)]
pub(crate) struct Settings {
    pub render_mode: u8,
    pub settings_per_render_mode: [Option<Vec<SettingsItem>>; 256],
    pub show_points: SettingsItem,
    pub mouse_push: SettingsItem,
    pub perlin_push: SettingsItem,
    pub centroid_push: SettingsItem,
    pub simulation_speed: SettingsItem,
    pub timer_pull: SettingsItem,
    pub perlin_seed: SettingsItem,
}

impl Settings {
    pub(crate) fn get_setting_for_render_mode(mode: u8) -> Option<Vec<SettingsItem>> {
        match mode {
            0 => {
                Some(vec![SettingsItem {
                    slider_label: "Radial Distance Scaling Factor".to_string(),
                    slider: SettingsSlider::SettingSliderF32(SettingSliderF32{
                        value: 0.5,
                        range_min: 0.0,
                        range_max: 1.0,
                    }),
                    show_slider: true,
                    bool_label: "".to_string(),
                    bool: false,
                    show_bool: false,
                }])
            }
            1 => {
                Some(vec![SettingsItem{
                    slider_label: "Cell border weight".to_string(),
                    slider: SettingsSlider::SettingSliderF32(SettingSliderF32{
                        value: 1.25,
                        range_min: 0.0,
                        range_max: 4.0,
                    }),
                    show_slider: true,
                    bool_label: "Show cell border?".to_string(),
                    bool: true,
                    show_bool: true,
                }])
            }
            // 7 => {
            //     Some(vec![SettingsItem {
            //         slider_label: "Transition".to_string(),
            //         slider: SettingsSlider::SettingSliderF32(SettingSliderF32{
            //             value: 1.0,
            //             range_min: 0.0,
            //             range_max: 1.0
            //         }),
            //         show_slider: true,
            //         bool_label: "".to_string(),
            //         bool: false,
            //         show_bool: false,
            //     }])
            // }
            _ => {None}
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct SettingsItem {
    pub slider_label: String,
    pub slider: SettingsSlider,
    pub show_slider: bool,
    pub bool_label: String,
    pub bool: bool,
    pub show_bool: bool,
}

impl SettingsItem {
    pub(crate) fn show(&mut self, ui: &mut Ui) {
        if self.show_bool {
            let _ = ui.checkbox(&mut self.bool, &self.bool_label);
        }

        if self.show_slider {
            ui.add(egui::Label::new(&self.slider_label));
            match &mut self.slider {
                SettingsSlider::SettingSliderF32(slider) => {
                    ui.add(egui::Slider::new(&mut slider.value, slider.range_min..=slider.range_max));
                }
                SettingsSlider::SettingsSliderU32(slider) => {
                    ui.add(egui::Slider::new(&mut slider.value, slider.range_min..=slider.range_max));
                }
            }
        }

    }

    pub(crate) fn value_f32(&self) -> f32 {
        return match &self.slider {
            SettingsSlider::SettingSliderF32(x) => {x.value}
            SettingsSlider::SettingsSliderU32(_) => {panic!("Wrong slider")}
        }
    }

    pub(crate) fn value_u32(&self) -> u32 {
        return match &self.slider {
            SettingsSlider::SettingSliderF32(_) => {panic!("Wrong slider")}
            SettingsSlider::SettingsSliderU32(x) => {x.value}
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) enum SettingsSlider {
    SettingSliderF32(SettingSliderF32),
    SettingsSliderU32(SettingSliderU32)
}

#[derive(Clone, Debug)]
pub(crate) struct SettingSliderF32 {
    pub value: f32,
    pub range_min: f32,
    pub range_max: f32,
}

#[derive(Clone, Debug)]
pub(crate) struct SettingSliderU32 {
    pub value: u32,
    pub range_min: u32,
    pub range_max: u32,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            render_mode: 1,
            settings_per_render_mode: get_render_modes(),

            show_points: SettingsItem {
                slider_label: "Points size".to_string(),
                slider: SettingsSlider::SettingSliderF32(SettingSliderF32{
                    value: 3.0,
                    range_min: 0.0,
                    range_max: 12.0,
                }),
                show_slider: true,
                bool_label: "Show points?".to_string(),
                bool: false,
                show_bool: true,
            },
            mouse_push: SettingsItem {
                slider_label: "Mouse push strength".to_string(),
                slider: SettingsSlider::SettingSliderF32(SettingSliderF32{
                    value: 1.0,
                    range_min: -5.0,
                    range_max: 5.0,
                }),
                show_slider: true,
                bool_label: "Mouse push?".to_string(),
                bool: false,
                show_bool: true,
            },
            perlin_push: SettingsItem {
                slider_label: "Perlin push strength".to_string(),
                slider: SettingsSlider::SettingSliderF32(SettingSliderF32{
                    value: 1.0,
                    range_min: 0.0,
                    range_max: 10.0,
                }),
                show_slider: true,
                bool_label: "Perlin push?".to_string(),
                bool: true,
                show_bool: true,
            },
            centroid_push: SettingsItem {
                slider_label: "Centroid push strength".to_string(),
                slider: SettingsSlider::SettingSliderF32(SettingSliderF32{
                    value: 1.0,
                    range_min: 0.0,
                    range_max: 10.0,
                }),
                show_slider: true,
                bool_label: "Centroid push?".to_string(),
                bool: true,
                show_bool: true,
            },
            simulation_speed: SettingsItem {
                slider_label: "Simulation speed".to_string(),
                slider: SettingsSlider::SettingSliderF32(SettingSliderF32{
                    value: 3.6,
                    range_min: 0.0,
                    range_max: 15.0,
                }),
                show_slider: true,
                bool_label: "Movement?".to_string(),
                bool: true,
                show_bool: true,
            },
            timer_pull: SettingsItem {
                slider_label: "Restore timer duration".to_string(),
                slider: SettingsSlider::SettingSliderF32(SettingSliderF32{
                    value: 100.0,
                    range_min: 10.0,
                    range_max: 500.0,
                }),
                show_slider: true,
                bool_label: "Restore timer?".to_string(),
                bool: true,
                show_bool: true,
            },
            perlin_seed: SettingsItem {
                slider_label: "Perlin seed".to_string(),
                slider: SettingsSlider::SettingsSliderU32(SettingSliderU32{
                    value: random_range(0,65535),
                    range_min: 0,
                    range_max: 65535,
                }),
                show_slider: true,
                bool_label: "".to_string(),
                bool: false,
                show_bool: false,
            },
        }
    }
}

fn get_render_modes() -> [Option<Vec<SettingsItem>>; 256] {
    (0_u8..=255).into_iter().map(|i| Settings::get_setting_for_render_mode(i)).collect::<Vec<Option<Vec<SettingsItem>>>>().try_into().unwrap()
}
