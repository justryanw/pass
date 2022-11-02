mod login_object;
mod utils;
mod window;

use adw::prelude::*;
use gtk::gio;
use window::Window;

static APP_ID: &str = "com.ryanjwalker.pass";

fn main() {
    gio::resources_register_include!("pass.gresource")
        .expect("Failed to register resources.");

    // Create a new application
    let app = adw::Application::builder().application_id(APP_ID).build();

    // Connect to signals
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &adw::Application) {
    // Create a new custom window and show it
    let window = Window::new(app);
    window.show();
}
