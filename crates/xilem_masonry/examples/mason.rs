// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]

use xilem_masonry::view::{button, flex};
use xilem_masonry::{MasonryView, Xilem};

fn app_logic(data: &mut AppData) -> impl MasonryView<AppData> {
    // here's some logic, deriving state for the view from our state
    let count = data.count;
    let label = if count == 1 {
        "clicked 1 time".to_string()
    } else {
        format!("clicked {count} times")
    };

    // The actual UI Code starts here

    let sequence = (0..count)
        .map(|x| button(format!("+{x}"), move |data: &mut AppData| data.count += x))
        .collect::<Vec<_>>();
    flex((
        button(label, |data: &mut AppData| data.count += 1),
        button("Decrement", |data: &mut AppData| data.count -= 1),
        button("Reset", |data: &mut AppData| data.count = 0),
        sequence,
    ))
}

struct AppData {
    count: i32,
}

fn main() {
    let data = AppData { count: 0 };

    let app = Xilem::new(data, app_logic);
    app.run_windowed("First Example".into()).unwrap();
}