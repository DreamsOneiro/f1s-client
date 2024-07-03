use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, TextView, TextBuffer};

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("org.dreams.f1s")
        .build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let text_buff = TextBuffer::builder()
        .text("Hello World")
        .build();

    let text = TextView::builder()
        .buffer(&text_buff)
        .editable(false)
        .top_margin(50)
        .left_margin(50)
        .build();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Formula 1 Schedule")
        .height_request(800)
        .width_request(800)
        .child(&text)
        .build();

    window.present();
}
