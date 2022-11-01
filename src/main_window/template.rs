use super::MainWindow;
use crate::{
    password_list::{template::PasswordListTemplate, PasswordList},
    field_list::{template::FieldListTemplate, FieldList},
};


use adw::{subclass::prelude::AdwApplicationWindowImpl, ApplicationWindow, Leaflet};
use gio::subclass::prelude::ObjectSubclassExt;
use glib::{
    object_subclass,
    subclass::{
        object::{ObjectImpl, ObjectImplExt},
        types::{ObjectSubclass},
        InitializingObject,
    },
};
use gtk::{
    prelude::{InitializingWidgetExt, GObjectPropertyExpressionExt},
    subclass::{
        application_window::ApplicationWindowImpl,
        prelude::{TemplateChild, WidgetImpl, WindowImpl},
        widget::{CompositeTemplate, WidgetClassSubclassExt},
    },
    CompositeTemplate, Widget,
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

        let password_list = self.password_list.get();
        let password_list_tempalte = PasswordListTemplate::from_instance(&password_list);

        let field_list = self.field_list.get();
        let field_list_template = FieldListTemplate::from_instance(&field_list);

        self.leaflet.property_expression("folded")
            .bind(&password_list_tempalte.header_bar.get(), "show-end-title-buttons", Widget::NONE);
        
        self.leaflet.property_expression("folded")
            .bind(&field_list_template.header_bar.get(), "show-start-title-buttons", Widget::NONE);

        self.leaflet.property_expression("folded")
            .bind(&field_list_template.back_button.get(), "visible", Widget::NONE);
    }
}

impl WidgetImpl for MainWindowTemplate {}
impl WindowImpl for MainWindowTemplate {}
impl ApplicationWindowImpl for MainWindowTemplate {}
impl AdwApplicationWindowImpl for MainWindowTemplate {}
