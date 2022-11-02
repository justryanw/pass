use std::cell::RefCell;

use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::once_cell::sync::Lazy;
use glib::{ParamSpec, ParamSpecString, Value};
use gtk::glib;

#[derive(Default)]
pub struct LoginObject {
    pub title: RefCell<String>,
}

#[glib::object_subclass]
impl ObjectSubclass for LoginObject {
    const NAME: &'static str = "LoginObject";
    type Type = super::LoginObject;
}

impl ObjectImpl for LoginObject {
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> =
            Lazy::new(|| vec![ParamSpecString::builder("title").build()]);
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "title" => {
                let input_value = value
                    .get()
                    .expect("The value needs to be of type `String`.");
                self.title.replace(input_value);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "title" => self.title.borrow().to_value(),
            _ => unimplemented!(),
        }
    }
}
