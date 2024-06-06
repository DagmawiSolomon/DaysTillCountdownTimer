use gtk::{prelude::*, Button, DrawingArea, ProgressBar};
use gtk::{glib, Application, PolicyType,Label,CssProvider,Grid,GridLayout,ScrolledWindow ,gio, Switch, Align, FlowBox};
use std::thread;
use std::time::Duration;
use glib::clone;
use gio::Settings;
use std::fs::File;
use std::path::Path;
use gtk::cairo::{Context};
use gtk::prelude::WidgetExt;


const APP_ID: &str = "org.gtk_rs.DaysTillCounter";

fn main() -> glib::ExitCode{
        let app = Application::builder().application_id(APP_ID).build();


        app.connect_activate(build_ui);
        app.run()

        
}

fn build_ui(app: &Application) {

        
        
    let grid_box = Grid::new();
    grid_box.set_margin_top(20);  
    grid_box.set_margin_bottom(20);  
    grid_box.set_margin_start(20); 
    grid_box.set_margin_end(20); 
           
                let mut count_row = 0;
                let mut count_col = 0;
                for number in 0 ..=110{
                    let drawing_area = DrawingArea::new();
                    drawing_area.set_size_request(20, 20);
                
                    drawing_area.set_draw_func(move|_, cr, _, _| {
                        draw(cr, 96, number);
                       
                        
                    });
                    
                        count_col = count_col + 1;
                        if number % 10 == 0{
                                count_row = count_row + 5;
                                count_col = 0;
                        }
                        grid_box.set_row_spacing(2);
                        grid_box.set_column_spacing(2);
                        grid_box.attach(&drawing_area, count_col, count_row, 1, 1);
    
                       
                }
         
        
    

    
          
    let window = gtk::ApplicationWindow::builder()
               .application(app)
               .default_width(300)
               .margin_bottom(50)
               .margin_end(50)
               .margin_start(50)
               .margin_end(50)
               .default_height(100)
               .title("Days Till Counter")
               .child(&grid_box)
               .build();

        window.present();
}
fn draw(cr: &Context, days:i32, number:i32) {
    let width = 12.0;
    let height = 12.0;
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