use adw::{prelude::*, Application};

mod main_window;
use main_window::MainWindow;
use gio::resources_register_include;

fn main() {
    resources_register_include!("password-manager.gresource")
        .expect("Failed to register resources.");

    let app = Application::builder()
        .application_id("com.example.pass")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let window = MainWindow::new(app);

    window.show();
}
