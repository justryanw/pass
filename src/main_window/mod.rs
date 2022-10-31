mod template;

use template::MainWindowTemplate;

use adw::Application;
use gio::{ActionGroup, ActionMap};
use glib::{wrapper, Object};
use gtk::{
    Accessible, ApplicationWindow, Buildable, ConstraintTarget, Native, Root, ShortcutManager,
    Widget, Window,
};

wrapper! {
    pub struct MainWindow(ObjectSubclass<MainWindowTemplate>)
        @extends ApplicationWindow, Window, Widget,
        @implements ActionGroup, ActionMap, Accessible, Buildable, ConstraintTarget, Native, Root, ShortcutManager;
}

impl MainWindow {
    pub fn new(app: &Application) -> Self {
        Object::builder().property("application", app).build()
    }
}
