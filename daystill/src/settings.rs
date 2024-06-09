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
        pub datetime: DateTime,
        pub widgets: Widgets
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


    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct DateTime{
        pub startdate: StartDate,
        pub enddate: EndDate   
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct StartDate{
        pub year: i32,
        pub month: u32,
        pub day: u32,
    }


    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct EndDate {
        pub year: i32,
        pub month: u32,
        pub day: u32,
    }
    // pub struct DayDifference {
    //     pub end_date: EndDate,
    //     pub days_left: i64,
    // }
    impl DateTime {
        pub fn get_days_left(&self) -> i64 {
            let target_date:NaiveDate = NaiveDate::from_ymd_opt(self.enddate.year, self.enddate.month, self.enddate.day).unwrap();
            let today = Local::now().date_naive();
            let days_left = target_date.signed_duration_since(today).num_days();
            days_left
        }

        pub fn total_days(&self) -> i64{
            let start_date:NaiveDate = NaiveDate::from_ymd_opt(self.startdate.year, self.startdate.month, self.startdate.day).unwrap();
            let end_date:NaiveDate = NaiveDate::from_ymd_opt(self.enddate.year, self.enddate.month, self.enddate.day).unwrap();
            let days_left = end_date.signed_duration_since(start_date).num_days();
            days_left
        }
    }


    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct Color{
        pub r:f64,
        pub g:f64,
        pub b:f64
    }


    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct GridSettings {
        pub row_spacing: u32,
        pub column_spacing: u32,
        pub row_width: i64,
        pub width: i32,
        pub height: i32,
        pub days_passed_color:Color,
        pub days_left_color:Color
    }


    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct Label {
        pub text: String,
        pub alignment: String,
     }


    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct Widgets {
       pub widget_type: String,
       pub label: Label,
       pub grid: GridSettings,
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
