use rustube::*;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Label, Button, Grid};

fn main() {
    let app = Application::builder()
        .application_id("com.github.xk-rl.YTD")
        .build();

    build_ui(&app);
}

fn build_ui(app: &Application) {
    app.connect_activate(|app| {
        let ui = ApplicationWindow::builder()
            .application(app)
            .title("Youtube Downloader")
            .build();

        ui.show_all();
    });

    app.run();
}
