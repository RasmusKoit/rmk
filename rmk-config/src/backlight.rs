use crate::BacklightConfig;

impl crate::KeyboardTomlConfig {
    pub fn get_backlight_config(&self) -> BacklightConfig {
        let default = BacklightConfig::default();
        match self.backlight.clone() {
            Some(mut backlight_config) => {
                backlight_config.backlight = backlight_config.backlight.or(default.backlight);
                backlight_config
            }
            None => default,
        }
    }
}
