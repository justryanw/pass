use adw::{prelude::*, Application, ApplicationWindow, HeaderBar, WindowTitle};
use adw::gtk::{Box, Orientation};

fn main() {
    let app = Application::new(Some("com.example.pass"), Default::default());

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let content = Box::new(Orientation::Vertical, 0);
    content.append(&HeaderBar::builder()
        .title_widget(&WindowTitle::new("Password Manager", ""))
        .build());

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Password Manager")
        .default_width(400)
        .default_height(250)
        .content(&content)
        .build();

    window.show();
}
