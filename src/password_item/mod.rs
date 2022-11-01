mod template;

use glib::{wrapper, Object};
use template::PasswordItemTemplate;

wrapper! {
  pub struct PasswordItem(ObjectSubclass<PasswordItemTemplate>);
}

impl PasswordItem {
    pub fn new(name: &str, url: &str) -> Self {
        Object::new::<Self>(&[("name", &name), ("url", &url)])
    }
}
