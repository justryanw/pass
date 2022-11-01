use super::PasswordList;
use adw::HeaderBar;
use glib::{
    object_subclass,
    once_cell::sync::Lazy,
    subclass::{
        object::{ObjectImpl, ObjectImplExt},
        types::ObjectSubclass,
        InitializingObject,
    },
    ParamFlags, ParamSpec, ParamSpecBoolean, ToValue,
};
use gtk::{
    prelude::InitializingWidgetExt,
    subclass::{
        prelude::{BoxImpl, TemplateChild, WidgetImpl},
        widget::{CompositeTemplate, WidgetClassSubclassExt},
    },
    Box, CompositeTemplate,
};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/password-list.ui")]
pub struct PasswordListTemplate {
    #[template_child]
    pub header_bar: TemplateChild<HeaderBar>,
}

#[object_subclass]
impl ObjectSubclass for PasswordListTemplate {
    const NAME: &'static str = "PasswordList";
    type Type = PasswordList;
    type ParentType = Box;

    fn class_init(my_class: &mut Self::Class) {
        Self::bind_template(my_class);
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for PasswordListTemplate {
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![ParamSpecBoolean::new(
                "show-end-title-buttons",
                "show-end-title-buttons",
                "Shows the title buttons in the header bar",
                true,
                ParamFlags::READWRITE,
            )]
        });

        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &glib::Value, pspec: &ParamSpec) {
        match pspec.name() {
            "show-end-title-buttons" => {
                let bool_value = value.get().expect("The value needs to be of type `bool`.");
                self.header_bar.set_show_end_title_buttons(bool_value);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &ParamSpec) -> glib::Value {
        match pspec.name() {
            "show-end-title-buttons" => self.header_bar.shows_end_title_buttons().to_value(),
            _ => unimplemented!(),
        }
    }

    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl WidgetImpl for PasswordListTemplate {}
impl BoxImpl for PasswordListTemplate {}
