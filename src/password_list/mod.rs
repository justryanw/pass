pub mod template;

use self::template::PasswordListTemplate;
use glib::{wrapper, Object};
use gtk::{Accessible, Box, Buildable, ConstraintTarget, Orientable, Widget};

wrapper! {
  pub struct PasswordList(ObjectSubclass<PasswordListTemplate>)
    @extends Widget, Box,
    @implements Accessible, Buildable, ConstraintTarget, Orientable;
}

impl Default for PasswordList {
    fn default() -> Self {
        Self::new()
    }
}

impl PasswordList {
    pub fn new() -> Self {
        Object::builder().build()
    }
}
