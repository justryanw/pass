mod imp;

use adw::subclass::prelude::*;
use glib::{Object, ObjectExt};
use gtk::glib;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct LoginData {
    pub title: String,
    pub username: String,
    pub password: String,
}

glib::wrapper! {
    pub struct LoginObject(ObjectSubclass<imp::LoginObject>);
}

impl LoginObject {
    pub fn new(title: &str) -> Self {
        Object::builder()
            .property("title", title)
            .property("username", "")
            .property("password", "")
            .build()
    }

    pub fn to_login_data(&self) -> LoginData {
        let title = self.imp().title.borrow().clone();
        let username = self.imp().username.borrow().clone();
        let password = self.imp().password.borrow().clone();

        LoginData {
            title,
            username,
            password,
        }
    }

    pub fn from_login_data(login_data: LoginData) -> Self {
        let title = login_data.title;

        let login_object = Self::new(&title);
        login_object.set_property("username", login_data.username);
        login_object.set_property("password", login_data.password);
        login_object
    }
}
