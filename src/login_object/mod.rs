mod imp;

use adw::subclass::prelude::*;
use glib::Object;
use gtk::glib;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct LoginData {
    pub title: String,
}

glib::wrapper! {
    pub struct LoginObject(ObjectSubclass<imp::LoginObject>);
}

impl LoginObject {
    pub fn new(title: &str) -> Self {
        Object::builder().property("title", title).build()
    }

    pub fn to_login_data(&self) -> LoginData {
        let title = self.imp().title.borrow().clone();

        LoginData { title }
    }

    pub fn from_login_data(login_data: LoginData) -> Self {
        let title = login_data.title;

        Self::new(&title)
    }
}
