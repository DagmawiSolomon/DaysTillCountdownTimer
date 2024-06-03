// use std::cell::Cell;
// use std::rc::Rc;

// use gtk::glib::clone;
// use gtk::{prelude::*, Button};
// use gtk::{glib, Application, ApplicationWindow,Orientation};

// const APP_ID: &str = "org.gtk_rs.HelloWorld";
// fn main() -> glib::ExitCode {
//     let app = Application::builder().application_id(APP_ID).build();
//     app.connect_activate(build_ui);
//     app.run()
// }
// fn build_ui(app:&Application){
//     let button_increase = Button::builder()
//         .label("Increase")
//         .margin_top(12)
//         .margin_bottom(12)
//         .margin_start(12)
//         .margin_end(12)
//         .build();

//     let button_decrease: Button = Button::builder()
//         .label("Increase")
//         .margin_top(12)
//         .margin_bottom(12)
//         .margin_start(12)
//         .margin_end(12)
//         .build();


//     let number = Rc::new(Cell::new(0)) ;
//     button_increase.connect_clicked(clone!(@weak number, @weak button_decrease =>
//         move |_| {
//             number.set(number.get() + 1);
//             button_decrease.set_label(&number.get().to_string());
//     }));
//     button_decrease.connect_clicked(clone!(@weak button_increase =>
//         move |_| {
//             number.set(number.get() - 1);
//             button_increase.set_label(&number.get().to_string());
//     }));

//     let gtk_box = gtk::Box::builder()
//     .orientation(Orientation::Vertical)
//     .build();
// gtk_box.append(&button_increase);
// gtk_box.append(&button_decrease);

//     let window = ApplicationWindow::builder()
//     .application(app)
//     .title("My GTK App")
//     .child(&gtk_box)
//     .build();
//     // remove the titlebar
//     window.set_decorated(true);
//     window.present();
// }

use chrono::TimeZone;
use chrono::{Utc, Local};

fn main() {
    while(true){
        let today = Local::now();
        let target_date = Local.ymd(2024, 9, 11).and_hms(0, 0, 0);
        let duration = target_date - today;

        let seconds_left = duration.num_seconds();
        let days = seconds_left / (24 * 60 * 60);
        let seconds_left = seconds_left % (24 * 60 * 60);
        let hours = seconds_left / (60 * 60);
        let seconds_left = seconds_left % (60 * 60);
        let minutes = seconds_left / (60);
        println!("{} {} {}", days, hours, minutes);
    }
    
}