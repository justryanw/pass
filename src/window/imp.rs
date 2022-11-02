use std::cell::RefCell;
use std::fs::File;

use adw::{prelude::*, EntryRow};
use adw::subclass::prelude::*;
use adw::Leaflet;
use glib::Binding;
use glib::signal::Inhibit;
use glib::subclass::InitializingObject;
use gtk::{
    gio, glib, Button, CompositeTemplate, ListBox, Stack,
};
use glib::once_cell::sync::OnceCell;

use crate::login_object::{LoginData, LoginObject};
use crate::utils::data_path;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/ryanjwalker/pass/window.ui")]
pub struct Window {
    #[template_child]
    pub title_field: TemplateChild<EntryRow>,
    #[template_child]
    pub username_field: TemplateChild<EntryRow>,
    #[template_child]
    pub logins_list: TemplateChild<ListBox>,
    #[template_child]
    pub leaflet: TemplateChild<Leaflet>,
    #[template_child]
    pub stack: TemplateChild<Stack>,
    #[template_child]
    pub back_button: TemplateChild<Button>,
    pub logins: OnceCell<gio::ListStore>,
    pub current_login: RefCell<Option<LoginObject>>,
    pub bindings: RefCell<Vec<Binding>>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "Window";
    type Type = super::Window;
    type ParentType = adw::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();
        obj.setup_logins();
        obj.restore_data();
        obj.setup_callbacks();
        obj.setup_actions();
    }
}

impl WidgetImpl for Window {}

impl WindowImpl for Window {
    fn close_request(&self) -> Inhibit {
        // Store login data in vector
        let backup_data: Vec<LoginData> = self
            .obj()
            .logins()
            .snapshot()
            .iter()
            .filter_map(Cast::downcast_ref::<LoginObject>)
            .map(LoginObject::to_login_data)
            .collect();

        // Save state to file
        let file = File::create(data_path()).expect("Could not create json file.");
        serde_json::to_writer_pretty(file, &backup_data)
            .expect("Could not write data to json file");

        // Pass close request on to the parent
        self.parent_close_request()
    }
}

impl ApplicationWindowImpl for Window {}
impl AdwApplicationWindowImpl for Window {}
