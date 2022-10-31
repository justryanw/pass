use adw::{prelude::*, Application};

mod main_window;
use main_window::MainWindow;

fn main() {
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
