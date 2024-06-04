use gtk::{prelude::*, Button};
use gtk::{glib, Application, PolicyType,Label,ListBox,ScrolledWindow ,gio, Switch, Align, FlowBox};
use std::thread;
use std::time::Duration;
use glib::clone;
use gio::Settings;


use chrono::TimeZone;
use chrono::Local;

const APP_ID: &str = "org.gtk_rs.DaysTillCounter";

fn main() -> glib::ExitCode{
        let app = Application::builder().application_id(APP_ID).build();
        app.connect_activate(build_ui);
        app.run()

        
}


fn get_date() -> i64{
        let today = Local::now();
        let target_date = Local.with_ymd_and_hms(2024, 9, 11, 0, 0, 0).unwrap();
        let duration = target_date - today;

        let seconds_left = duration.num_seconds();
        let days = seconds_left / (24 * 60 * 60);
        let seconds_left = seconds_left % (24 * 60 * 60);
        let hours = seconds_left / (60 * 60);
        let seconds_left = seconds_left % (60 * 60);
        let minutes = seconds_left / (60);

        days
}

fn build_ui(app: &Application) {
        // let settings = Settings::new(APP_ID);
        //         let button = Button::builder()
        //             .label("Press me!")
        //             .margin_top(12)
        //             .margin_bottom(12)
        //             .margin_start(12)
        //             .margin_end(12)
        //             .build();
            
                
                // button.connect_clicked(move |_|{
                //         gio::spawn_blocking( move || {
                //         let five_seconds = Duration::from_secs(5);
                //         thread::sleep(five_seconds);
                //         });
                // });

                // // Create channel that can hold at most 1 message at a time
                // let(sender, receiver) = async_channel::bounded(1);
                //  // Connect to "clicked" signal of `button`
                //  button.connect_clicked(move |_| {
                //         let sender = sender.clone();
                //         // The long running operation runs now in a separate thread
                //         gio::spawn_blocking(move || {
                //             // Deactivate the button until the operation is done
                //             sender
                //                 .send_blocking(false)
                //                 .expect("The channel needs to be open.");
                //             let five_seconds = Duration::from_secs(5);
                //             thread::sleep(five_seconds);
                //             // Activate the button again
                //             sender
                //                 .send_blocking(true)
                //                 .expect("The channel needs to be open.");
                //         });
                //     });

                //     glib::spawn_future_local(clone!(@weak button => async move {
                //         while let Ok(enable_button) = receiver.recv().await {
                //             button.set_sensitive(enable_button);
                //         }
                //     }));

                //     let is_switch_enabled = settings.boolean("is-switch-enabled");
                
                //     // Create a switch
                //     let switch = Switch::builder()
                //         .margin_top(48)
                //         .margin_bottom(48)
                //         .margin_start(48)
                //         .margin_end(48)
                //         .valign(Align::Center)
                //         .halign(Align::Center)
                //         .state(is_switch_enabled)
                //         .build();
                // switch.connect_state_set(move |_, is_enabled| {
                //         // Save changed switch state in the settings
                //         settings
                //             .set_boolean("is-switch-enabled", is_enabled)
                //             .expect("Could not set setting.");
                //         // Allow to invoke other event handlers
                //         glib::Propagation::Proceed
                //     });

                //     let list_box = ListBox::new();
                    let flow_box = FlowBox::new();
                //     for number in 0..=100 {
                //         let label = Label::new(Some(&number.to_string()));
                //         list_box.append(&label);
                //      }   
                let days:i64 = get_date();
                for number in 0..=days {
                        let label = Label::new(Some(&number.to_string()));
                        flow_box.append(&label);
                }


//     let scrolled_window = ScrolledWindow::builder()
//         .hscrollbar_policy(PolicyType::Never) // Disable horizontal scrolling
//         .min_content_width(360)
//         .child(&flow_box)
//         .build();
                
        let window = gtk::ApplicationWindow::builder()
               .application(app)
               .default_width(300)
               .default_height(100)
               .title("Days Till Counter")
               .child(&flow_box)
               .build();

        window.present();
}