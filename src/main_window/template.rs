use std::rc::Rc;

use super::MainWindow;
use crate::{field_list::FieldList, password_item::PasswordItem, password_list::PasswordList};

use adw::{subclass::prelude::AdwApplicationWindowImpl, ApplicationWindow, Leaflet};
use glib::{
    clone, object_subclass,
    subclass::{
        object::{ObjectImpl, ObjectImplExt},
        types::ObjectSubclass,
        InitializingObject,
    },
    ObjectExt,
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

        self.password_list.set_model(password_model.clone());

        let password_model_rt = Rc::new(password_model.clone());

        self.password_list.connect_local(
            "changed",
            false,
            clone!(
            @strong password_model_rt => move |values| {
                let value: String = values[1].get().unwrap();
                let selection = password_model_rt.iter().find(|x| x.property::<String>("name") == value).unwrap();

                let name: String = selection.property::<String>("name");

                println!("{}", value);
                println!("{}", name);

                None
            }),
        );
    }
}

impl WidgetImpl for MainWindowTemplate {}
impl WindowImpl for MainWindowTemplate {}
impl ApplicationWindowImpl for MainWindowTemplate {}
impl AdwApplicationWindowImpl for MainWindowTemplate {}
