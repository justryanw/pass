use super::MainWindow;
use crate::{
    password_list::PasswordList,
    field_list::FieldList, password_item::PasswordItem,
};


use adw::{subclass::prelude::AdwApplicationWindowImpl, ApplicationWindow, Leaflet};
use glib::{
    object_subclass,
    subclass::{
        object::{ObjectImpl, ObjectImplExt},
        types::{ObjectSubclass},
        InitializingObject,
    },
};
use gtk::{
    prelude::InitializingWidgetExt,
    subclass::{
        application_window::ApplicationWindowImpl,
        prelude::{TemplateChild, WidgetImpl, WindowImpl},
        widget::{CompositeTemplate, WidgetClassSubclassExt},
    },
    CompositeTemplate,
};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/main-window.ui")]
pub struct MainWindowTemplate {
    #[template_child]
    pub leaflet: TemplateChild<Leaflet>,
  
    #[template_child]
    pub password_list: TemplateChild<PasswordList>,
  
    #[template_child]
    pub field_list: TemplateChild<FieldList>,
}

#[object_subclass]
impl ObjectSubclass for MainWindowTemplate {
    const NAME: &'static str = "MainWindow";

    type Type = MainWindow;
    type ParentType = ApplicationWindow;

    fn class_init(my_class: &mut Self::Class) {
        Self::bind_template(my_class);
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for MainWindowTemplate {
    fn constructed(&self) {
        self.parent_constructed();

        let password_model = vec![
            PasswordItem::new("Google", "example"),
            PasswordItem::new("Facebook", "example"),
            PasswordItem::new("Amazon", "example"),
        ];

        self.password_list.set_model(password_model);
    }
}

impl WidgetImpl for MainWindowTemplate {}
impl WindowImpl for MainWindowTemplate {}
impl ApplicationWindowImpl for MainWindowTemplate {}
impl AdwApplicationWindowImpl for MainWindowTemplate {}
