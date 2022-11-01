use super::FieldList;

use glib::{
    object_subclass,
    subclass::{
        object::{ObjectImpl, ObjectImplExt},
        types::ObjectSubclass,
        InitializingObject,
    },
};
use gtk::{
    prelude::InitializingWidgetExt,
    subclass::{
        prelude::{BoxImpl, TemplateChild, WidgetImpl},
        widget::{CompositeTemplate, WidgetClassSubclassExt},
    },
    Box, Button, CompositeTemplate,
};
use adw::HeaderBar;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/field-list.ui")]
pub struct FieldListTemplate {
    #[template_child]
    pub header_bar: TemplateChild<HeaderBar>,

    #[template_child]
    pub back_button: TemplateChild<Button>,
}

#[object_subclass]
impl ObjectSubclass for FieldListTemplate {
    const NAME: &'static str = "FieldList";

    type Type = FieldList;
    type ParentType = Box;

    fn class_init(my_class: &mut Self::Class) {
        Self::bind_template(my_class);
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for FieldListTemplate {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl WidgetImpl for FieldListTemplate {}
impl BoxImpl for FieldListTemplate {}