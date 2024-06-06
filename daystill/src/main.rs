use gtk::{prelude::*, DrawingArea, ProgressBar};
use gtk::{glib, Application,Grid, CssProvider};

use gtk::gdk::Display;
use gtk::cairo::{Context};
use gtk::prelude::WidgetExt;
use chrono::{DateTime, TimeZone};
use chrono::Local;


const APP_ID: &str = "org.gtk_rs.DaysTillCounter";

fn main() -> glib::ExitCode{
        let app = Application::builder().application_id(APP_ID).build();

        app.connect_startup(|_| load_css());
        app.connect_activate(build_ui);
        app.run()

        
}


fn load_css() {
        
    let provider = CssProvider::new();
    
    provider.load_from_path("src/style.css");
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

struct DayDifference{
     day: i64,
}



impl DayDifference{

    fn get_date(target_date:DateTime<Local>) -> i64{
        let today = Local::now();
        let duration = target_date - today;
        let seconds_left = duration.num_seconds();
        let days = seconds_left / (24 * 60 * 60);

       days
    }

}


fn build_ui(app: &Application) {

    let container = gtk::Box::builder().orientation(gtk::Orientation::Vertical).build();
    let target_date = Local.with_ymd_and_hms(2024, 9, 11, 0, 0, 0).unwrap();
    let days:i64 = DayDifference::get_date(target_date);
        
    let grid_box = Grid::new();
    grid_box.set_margin_top(15);  
    grid_box.set_margin_bottom(20);  
    grid_box.set_margin_start(20); 
    grid_box.set_margin_end(20); 
           
                let mut count_row = 0;
                let mut count_col = 0;
                for number in 0 ..=110{
                    let drawing_area = DrawingArea::new();
                    drawing_area.set_size_request(15, 15);
                
                    drawing_area.set_draw_func(move|_, cr, _, _| {
                        draw(cr, 96, number);
                       
                        
                    });
                    
                        count_col = count_col + 1;
                        if number % 21 == 0{
                                count_row = count_row + 5;
                                count_col = 0;
                        }
                        grid_box.set_row_spacing(2);
                        grid_box.set_column_spacing(2);
                        grid_box.attach(&drawing_area, count_col, count_row, 1, 1);
    
                       
                }

                let daysleft = format!("In {} days",days);
                let label: gtk::Label = gtk::Label::new(Some("Holidays"));
                label.set_xalign(0.0);
                let daysleftlabel = gtk::Label::new(Some(&daysleft));
                daysleftlabel.set_xalign(0.0);
                label.add_css_class("h1");
                daysleftlabel.add_css_class("h2");


         
                let pgbar = ProgressBar::new();
                let total_days:f64 = 110 as f64;
                let days_elapsed:f64 = (total_days - days as f64)/total_days;
                println!("{} {} ", 110-days, days_elapsed);
                pgbar.set_fraction(days_elapsed);   

                container.append(&label);
                container.append(&daysleftlabel);
                container.append(&grid_box);
    

    
          
    let window = gtk::ApplicationWindow::builder()
               .application(app)
               .default_width(300)
               .margin_bottom(50)
               .margin_end(50)
               .margin_start(50)
               .margin_end(50)
               .default_height(100)
               .title("Days Till Counter")
               .child(&container)
               .build();

        window.set_decorated(false);
        window.present();
        window.add_css_class("window");
}
fn draw(cr: &Context, days:i32, number:i32) {
    let width = 15.0;
    let height = 15.0;
    let radius = f64::min(width, height) / 2.0; // Radius is half of the minimum dimension
    let center_x = width / 2.0;
    let center_y = height / 2.0;

    cr.arc(center_x, center_y, radius, 0.0, 2.0 * std::f64::consts::PI);
    
    if (110-number) > days{
        cr.set_source_rgb(0.2235, 0.8275, 0.3255);
        
    }
    else{
        cr.set_source_rgb(0.0863, 0.1059, 0.1333);
    }

    cr.fill();

}