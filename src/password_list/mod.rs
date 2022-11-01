pub mod template;

use self::template::PasswordListTemplate;
use crate::password_item::PasswordItem;

use adw::ActionRow;
use gio::{subclass::prelude::ObjectSubclassExt, ListStore};
use glib::{wrapper, Object, StaticType, ObjectExt};
use gtk::{Accessible, Box, Buildable, ConstraintTarget, Orientable, Widget, SingleSelection, traits::WidgetExt};

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

    pub fn set_model(&self, model: Vec<PasswordItem>) {
        let template = PasswordListTemplate::from_instance(self);
        let list_store_model = ListStore::new(PasswordItem::static_type());

        for element in model {
            list_store_model.append(&element);
        }

        let selection_model = SingleSelection::new(Some(&list_store_model));

        template.list_box.bind_model(Some(&selection_model), |x| {
            let name: String = x.property("name");
            let action_row = ActionRow::builder().title(&name).build();
            let result = action_row.ancestor(Widget::static_type());

            result.unwrap()
        })
    }
}
