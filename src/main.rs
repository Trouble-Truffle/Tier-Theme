use gtk4::prelude::*;
//use gtk4::glib::*;
use glib::clone;
use gtk4::{Application, ApplicationWindow, Button, Box, Orientation};
use std::rc::*;
use std::cell::*;

const APP_ID: &str = "Tiertheme";
fn build_ui(app: &Application) {
    let button = Button::builder()
        .label("press me")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(35)
        .margin_end(35)
        .build();

    let button_increase = Button::builder().label("+").build();
    let button_decrease = Button::builder().label("-").build();

    let number = Rc::new(Cell::new(0));

    // Connect callbacks
    // When a button is clicked, `number` and label of the other button will be changed
    button_increase.connect_clicked(clone!(@weak number, @weak button_decrease =>
        move |_| {
            number.set(number.get() + 1);
            button_decrease.set_label(&number.get().to_string());
    }));
    button_decrease.connect_clicked(clone!(@weak button_increase =>
        move |_| {
            number.set(number.get() - 1);
            button_increase.set_label(&number.get().to_string());
    }));

    let gtk_box = Box::builder()
                .orientation(Orientation::Vertical)
                .build();

    gtk_box.append(&button_increase);
    gtk_box.append(&button_decrease);
    gtk_box.append(&button);

    button.connect_clicked(|button| { 
        button.set_label("why"); });

    let window = ApplicationWindow::builder()
        .application(app)
        .title(APP_ID)
        .child(&gtk_box)
        .build();
    window.present();
}

fn main() {
    let app =  Application::builder()
                           .application_id(APP_ID)
                           .build();

    app.connect_activate(build_ui);
    app.run();
}
