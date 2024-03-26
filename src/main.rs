use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Label, Entry, Button, Grid};
use tokio::task;

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

        let grid = Grid::new();
        grid.set_column_spacing(10);
        grid.set_row_spacing(10);
        
        let url = Entry::builder()
            .placeholder_text("https://www.youtube.com/watch?v=...")
            .build();

        let confirm = Button::builder()
            .label("Download")
            .build();

        grid.attach(&Label::new(Some("Enter a URL:")), 0, 0, 1, 1);
        grid.attach(&url, 1, 0, 1, 1);
        grid.attach(&confirm, 0, 1, 2, 1);

        confirm.connect_clicked(move |_| task::spawn(async move {
            let url = url.buffer().text().to_string().as_str();
            println!("Downloaded video to {:?}", rustube::download_best_quality(&url).await.unwrap());
        }));


        ui.add(&grid);

        ui.show_all();
    });

    app.run();
}
