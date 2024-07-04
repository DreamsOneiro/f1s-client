use gtk::gdk::Display;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, TextView, TextBuffer, CssProvider};
use f1s_lib::{schedule, Races, time, RaceInfo};

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("org.dreams.f1s")
        .build();

    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);

    app.run()
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_string(include_str!("style.css"));

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to display"), 
        &provider, 
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(app: &Application) {
    let races: Vec<Races> = Races::from_ergast_json();
    let index = schedule::search_current(&races).expect("Problem finding race");
    let race_c = &races[index];

    let mut race_schedule = format!("Season: {}, Round: {}\n", race_c.season, race_c.round);
    race_schedule.push_str(&format!("Race: {}\n", race_c.race_name));
    race_schedule.push_str(&format!("Circuit: {}\n", race_c.circuit.circuit_name));
    race_schedule.push_str(&format!("Location: {}, {}\n"
            , race_c.circuit.location.locality
            , race_c.circuit.location.country));
    race_schedule.push_str(&sub_info(&race_c.fp1, "FP1"));
    race_schedule.push_str(&sub_info_verify(&race_c.fp2, &race_c.sprint, "FP1", "SQ"));
    race_schedule.push_str(&sub_info(&race_c.fp3, "FP3"));
    race_schedule.push_str(&sub_info(&race_c.quali, "Quali"));
    race_schedule.push_str(&format!("Main Race:\n\t{}", 
        time::to_str_localtz(&time::to_utc(&race_c.date, &race_c.time))));

    let text_buff = TextBuffer::builder()
        .text(race_schedule)
        .build();

    let text = TextView::builder()
        .buffer(&text_buff)
        .editable(false)
        .can_focus(false)
        .top_margin(5)
        .left_margin(10)
        .build();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Formula 1 Schedule")
        .height_request(350)
        .width_request(250)
        .child(&text)
        .build();

    window.present();
}

// Handle Option<RaceInfo>
fn sub_info(info: &Option<RaceInfo>, name: &str) -> String {
    match info {
        Some(ri) => {
            let mut s = format!("{}:\n", name);
            s.push_str(&sub(&ri));
            return s
        },
        None => {return "".to_string()},
    }

    fn sub(ri: &RaceInfo) -> String {
        let dt = time::to_utc(&ri.date, &ri.time);
        format!("\tDate: {}\n", time::to_str_localtz(&dt))
    }
}

// Check if Option contain Some or None before printing
fn sub_info_verify(
    info: &Option<RaceInfo>, 
    switch: &Option<RaceInfo>, 
    name: &str, 
    alt_name: &str) -> String {
    match switch {
        Some(_) => {return sub_info(info, alt_name)},
        None => {return sub_info(info, name)},
    }
}
