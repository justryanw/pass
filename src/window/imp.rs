use std::cell::RefCell;

use adw::subclass::prelude::*;
use adw::Leaflet;
use adw::{prelude::*, EntryRow, PasswordEntryRow};
use glib::once_cell::sync::OnceCell;
use glib::signal::Inhibit;
use glib::subclass::InitializingObject;
use glib::Binding;
use gtk::{gio, glib, Button, CompositeTemplate, ListBox, Stack};

use crate::login_object::{LoginData, LoginObject};
use crate::utils::{self};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/ryanjwalker/pass/window.ui")]
pub struct Window {
    pub master_password: RefCell<String>,
    #[template_child]
    pub title_field: TemplateChild<EntryRow>,
    #[template_child]
    pub username_field: TemplateChild<EntryRow>,
    #[template_child]
    pub password_field: TemplateChild<PasswordEntryRow>,
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
        obj.setup_callbacks();
        obj.setup_actions();
    }
}

impl WidgetImpl for Window {}

impl WindowImpl for Window {
    fn close_request(&self) -> Inhibit {
        let master_password = self.master_password.borrow();

        // Don't save data if no master password is set
        if master_password.is_empty() {
            println!("master password empty {}", master_password);
            return self.parent_close_request();
        }

        println!("master password not empty {}", master_password);

        // Store login data in vector
        let data: Vec<LoginData> = self
            .obj()
            .logins()
            .snapshot()
            .iter()
            .filter_map(Cast::downcast_ref::<LoginObject>)
            .map(LoginObject::to_login_data)
            .collect();

        utils::write_database(data, &master_password);

        // Pass close request on to the parent
        self.parent_close_request()
    }
}

impl ApplicationWindowImpl for Window {}
impl AdwApplicationWindowImpl for Window {}
