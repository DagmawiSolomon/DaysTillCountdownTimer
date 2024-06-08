pub mod Setting{

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct Settings{
        windowsettings: WindowSettings,
        uiconfig: UIConfig,
    }

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct WindowSettings{
    resizable:bool,
    margin_bottom: i32,
    margin_end: i32,
    margin_start: i32,
}

struct TargetDate{
    year: i32,
    month: u32,
    day:u32,
}
struct DayDifference{
     target_date: TargetDate,
     days_left: i64,
}

impl DayDifference{


    fn get_date() -> i64{
        // let target_date: NaiveDate = NaiveDate::from_ymd_opt(self.target_date.year,
        //                                                     self.target_date.month, 
        //                                                     self.target_date.day).unwrap();
        let target_date = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(); 
        let today = Local::now().date_naive();
        let days_left = target_date.signed_duration_since(today).num_days();
        days_left
    }

}

#[derive(Debug, Deserialize)]
struct GridSettings{
    row_spacing: i32,
    column_spacing: i32,
    margin_top: i32,
    margin_bottom: i32,
    margin_start: i32,
    margin_end: i32
}

#[derive(Debug, Deserialize)]
struct UIConfig{
    gridsettings:GridSettings,
}



impl Settings{
    fn new() -> Settings{
        let mut file = File::open("src/config.json").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        let settings:Settings = serde_json::from_str(&data).expect("JSON was not well-formatted");
        settings
    }
}

}