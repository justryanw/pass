use super::PasswordList;
use glib::{
  object_subclass,
  subclass::{
    object::{ObjectImpl, ObjectImplExt},
    types::ObjectSubclass, InitializingObject,
  },
};
use gtk::{
  prelude::InitializingWidgetExt,
  subclass::{
    prelude::{BoxImpl, TemplateChild, WidgetImpl},
    widget::{CompositeTemplate, WidgetClassSubclassExt},
  },
  Box, CompositeTemplate,
};
use adw::HeaderBar;

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
  fn constructed(&self) {
    self.parent_constructed();
  }
}

impl WidgetImpl for PasswordListTemplate {}
impl BoxImpl for PasswordListTemplate {}