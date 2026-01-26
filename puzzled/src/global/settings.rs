use adw::gio::Settings;
use adw::glib;
use adw::prelude::{IsA, SettingsExt, SettingsExtManual};

#[derive(Debug, Clone)]
pub struct Preferences {
    settings: Settings,
}

impl Default for Preferences {
    fn default() -> Self {
        Preferences {
            settings: Settings::new("de.til7701.Puzzled"),
        }
    }
}

impl Preferences {
    pub fn get<S: SettingKey>(&self, setting: S) -> S::Value {
        setting.get(&self.settings)
    }

    pub fn bind<S: SettingKey>(&self, setting: S, obj: &impl IsA<glib::Object>, property: &str) {
        setting.bind(&self.settings, obj, property);
    }
}

pub trait SettingKey {
    type Value;

    fn key(&self) -> &'static str;

    fn get(&self, settings: &Settings) -> Self::Value;

    fn bind(&self, settings: &Settings, obj: &impl IsA<glib::Object>, property: &str) {
        settings.bind(self.key(), obj, property).build();
    }
}

pub struct SolverEnabled;

impl SettingKey for SolverEnabled {
    type Value = bool;

    fn key(&self) -> &'static str {
        "solver-enabled"
    }

    fn get(&self, settings: &Settings) -> Self::Value {
        settings.boolean(self.key())
    }
}
