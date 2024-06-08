use chrono::{NaiveDate, Local};
use serde_json;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;

pub mod setting {
    use super::*;
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct Settings {
        pub windowsettings: WindowSettings,
        // pub uiconfig: UIConfig,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct WindowSettings {
        pub resizable: bool,
        pub margin_bottom: i32,
        pub margin_end: i32,
        pub margin_start: i32,
    }

    pub struct TargetDate {
        pub year: i32,
        pub month: u32,
        pub day: u32,
    }

    pub struct DayDifference {
        pub target_date: TargetDate,
        pub days_left: i64,
    }

    impl DayDifference {
        pub fn get_date() -> i64 {
            let target_date = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
            let today = Local::now().date_naive();
            let days_left = target_date.signed_duration_since(today).num_days();
            days_left
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct GridSettings {
        pub row_spacing: i32,
        pub column_spacing: i32,
        pub margin_top: i32,
        pub margin_bottom: i32,
        pub margin_start: i32,
        pub margin_end: i32,
    }

    #[derive(Debug, Deserialize)]
    pub struct UIConfig {
        pub gridsettings: GridSettings,
    }

    impl Settings {
        pub fn new() -> Settings {
            let mut file = File::open("src/config.json").unwrap();
            let mut data = String::new();
            file.read_to_string(&mut data).unwrap();
            let settings: Settings = serde_json::from_str(&data).expect("JSON was not well-formatted");
            settings
        }
    }
}
