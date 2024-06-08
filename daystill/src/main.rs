use gtk::{prelude::*, DrawingArea, ProgressBar};
use gtk::{glib, Application,Grid, CssProvider};

use gtk::gdk::Display;
use gtk::cairo::{Context};
use gtk::prelude::WidgetExt;

mod settings;


use settings::setting::{EndDate, Settings, StartDate};
use settings::setting::DateTime;
use settings::setting::WindowSettings;


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
fn build_ui(app: &Application) {
    let container = gtk::Box::builder().orientation(gtk::Orientation::Vertical).build();  
    let grid_box = Grid::new();
    grid_box.add_css_class("grid");

    let datetime = Settings::new().datetime;
    let days_left:i64 = DateTime::get_days_left(&datetime);  
    let total_days:i64 = DateTime::total_days(&datetime);


    let (label, daysleftlabel)=  add_countdown_labels(days_left);
    github_grid(days_left, total_days, &grid_box, &container);
    progress_bar(days_left, total_days);

    container.append(&daysleftlabel);
    container.append(&label);
    container.append(&grid_box);

    draw_window(app,&container);


}

fn draw_window(app: &Application, container: &gtk::Box) {
    let window_settings:WindowSettings = Settings::new().windowsettings;
    let window = gtk::ApplicationWindow::builder()
               .application(app)
               .margin_bottom(window_settings.margin_bottom)
               .margin_end(window_settings.margin_end)
               .margin_start(window_settings.margin_start)
               .child(container)
               .build();

    window.set_resizable(window_settings.resizable);
    window.set_size_request(-1, -1);
    window.set_decorated(false);
    window.present();
    window.add_css_class("window");
    
}


fn progress_bar(days_left:i64, total_days: i64){
    let pgbar = ProgressBar::new();
    let days_elapsed:f64 = (total_days as f64 - days_left as f64)/ total_days as f64;
    pgbar.set_fraction(days_elapsed);   
    pgbar.add_css_class("grid");
}
fn github_grid(days_left:i64, total_days: i64, grid_box: &Grid, container:&gtk::Box){
    let mut count_row = 0;
    let mut count_col = 0;
    for number in 0 ..=total_days{
        let drawing_area = DrawingArea::new();
        drawing_area.set_size_request(15, 15);
                
        drawing_area.set_draw_func(move|_, cr, _, _| {
            draw_github_grid(cr, days_left, number, total_days);
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
   
}
fn draw_github_grid(cr: &Context, days:i64, number:i64, total_days: i64) {
    let width = 15.0;
    let height = 15.0;
    let radius = f64::min(width, height) / 2.0; 
    let center_x = width / 2.0;
    let center_y = height / 2.0;

    cr.arc(center_x, center_y, radius, 0.0, 2.0 * std::f64::consts::PI);

    if (total_days-number) > days{
        cr.set_source_rgb(0.2235, 0.8275, 0.3255);   
    }
    else{
        cr.set_source_rgb(0.0863, 0.1059, 0.1333);
    }
    cr.fill();

}


fn add_countdown_labels(days_left:i64) -> (gtk::Label, gtk::Label){
    let daysleft = format!("In {} days",days_left);
    let label: gtk::Label = gtk::Label::new(Some("Holidays"));
    label.set_xalign(0.0);
    let daysleftlabel = gtk::Label::new(Some(&daysleft));
    daysleftlabel.set_xalign(0.0);
    label.add_css_class("h1");
    daysleftlabel.add_css_class("h2");

    (daysleftlabel, label)

}