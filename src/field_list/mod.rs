pub mod template;

use self::template::FieldListTemplate;

use glib::{wrapper, Object};
use gtk::{Accessible, Box, Buildable, ConstraintTarget, Orientable, Widget};

wrapper! {
    pub struct FieldList(ObjectSubclass<FieldListTemplate>)
        @extends Widget, Box,
        @implements Accessible, Buildable, ConstraintTarget, Orientable;
}

impl Default for FieldList {
    fn default() -> Self {
        Self::new()
    }
}

impl FieldList {
    pub fn new() -> Self {
        Object::builder().build()
    }
}