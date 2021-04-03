use fltk::{app, window::*};

fn main() {
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 400, 300, "Rustcord");
    wind.end();
    wind.show();
    app.run().unwrap();
}