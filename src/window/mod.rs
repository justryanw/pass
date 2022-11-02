mod imp;

use std::fs::File;

use adw::prelude::*;
use adw::subclass::prelude::*;
use adw::NavigationDirection;
use glib::{clone, Object};
use gtk::glib::BindingFlags;
use gtk::{
    gio, glib, pango, Dialog, DialogFlags, Entry, Label, ListBoxRow, ResponseType, SelectionMode,
};

use crate::login_object::{LoginData, LoginObject};
use crate::utils::data_path;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &adw::Application) -> Self {
        // Create new window
        Object::builder().property("application", app).build()
    }

    fn current_login(&self) -> LoginObject {
        self.imp()
            .current_login
            .borrow()
            .clone()
            .expect("`current_login` should be set in `set_current_logins`.")
    }

    fn logins(&self) -> gio::ListStore {
        self.imp()
            .logins
            .get()
            .expect("`logins` should be set in `setup_logins`.")
            .clone()
    }

    fn setup_logins(&self) {
        let logins = gio::ListStore::new(LoginObject::static_type());
        self.imp()
            .logins
            .set(logins.clone())
            .expect("Could not set logins");

        self.imp().logins_list.bind_model(
            Some(&logins),
            clone!(@weak self as window => @default-panic, move |obj| {
                let login_object = obj
                    .downcast_ref()
                    .expect("The object should be of type `LoginObject`.");
                let row = window.create_login_row(login_object);
                row.upcast()
            }),
        )
    }

    fn restore_data(&self) {
        if let Ok(file) = File::open(data_path()) {
            // Deserialize data from file to vector
            let backup_data: Vec<LoginData> = serde_json::from_reader(file)
                .expect("It should be possible to read `backup_data` from the json file.");

            // Convert `Vec<LoginData>` to `Vec<LoginObject>`
            let logins: Vec<LoginObject> = backup_data
                .into_iter()
                .map(LoginObject::from_login_data)
                .collect();

            // Insert restored objects into model
            self.logins().extend_from_slice(&logins);

            // Set first login as current
            if let Some(first_login) = logins.first() {
                self.set_current_login(first_login.clone());
            }
        }
    }

    fn create_login_row(&self, login_object: &LoginObject) -> ListBoxRow {
        let label = Label::builder()
            .ellipsize(pango::EllipsizeMode::End)
            .xalign(0.0)
            .build();

        login_object
            .bind_property("title", &label, "label")
            .flags(glib::BindingFlags::SYNC_CREATE)
            .build();

        ListBoxRow::builder().child(&label).build()
    }

    fn set_current_login(&self, login: LoginObject) {
        let title_field = self.imp().title_field.get();
        // let username_field = self.imp().username_field.get();

        // Unbind all fields
        self.unbind();
        let mut bindings = self.imp().bindings.borrow_mut();

        // Bind fields to selected login data
        bindings.push(
            login
                .bind_property("title", &title_field, "text")
                .flags(BindingFlags::BIDIRECTIONAL | BindingFlags::SYNC_CREATE)
                .build(),
        );

        self.imp().current_login.replace(Some(login));
        self.select_login_row();
    }

    pub fn unbind(&self) {
        // Unbind all stored bindings
        for binding in self.imp().bindings.borrow_mut().drain(..) {
            binding.unbind();
        }
    }

    fn select_login_row(&self) {
        if let Some(index) = self.logins().find(&self.current_login()) {
            let row = self.imp().logins_list.row_at_index(index as i32);
            self.imp().logins_list.select_row(row.as_ref());
        }
    }

    fn setup_callbacks(&self) {
        // Setup callback when items of logins change
        self.set_stack();
        self.logins().connect_items_changed(
            clone!(@weak self as window => move |_, _, _, _| {
                window.set_stack();
            }),
        );

        // Setup callback for activating a row of logins list
        self.imp().logins_list.connect_row_activated(
            clone!(@weak self as window => move |_, row| {
                let index = row.index();
                let selected_login = window.logins()
                    .item(index as u32)
                    .expect("There needs to be an object at this position.")
                    .downcast::<LoginObject>()
                    .expect("The object needs to be a `LoginObject`.");
                window.set_current_login(selected_login);
                window.imp().leaflet.navigate(NavigationDirection::Forward);
            }),
        );

        // Setup callback for folding the leaflet
        self.imp()
            .leaflet
            .connect_folded_notify(clone!(@weak self as window => move |leaflet| {
                if leaflet.is_folded() {
                    window
                        .imp()
                        .logins_list
                        .set_selection_mode(SelectionMode::None)
                } else {
                    window
                        .imp()
                        .logins_list
                        .set_selection_mode(SelectionMode::Single);
                    window.select_login_row();
                }
            }));

        self.imp()
            .back_button
            .connect_clicked(clone!(@weak self as window => move |_| {
                window.imp().leaflet.navigate(NavigationDirection::Back);
            }));
    }

    fn set_stack(&self) {
        if self.logins().n_items() > 0 {
            self.imp().stack.set_visible_child_name("main");
        } else {
            self.imp().stack.set_visible_child_name("placeholder");
        }
    }

    fn setup_actions(&self) {
        // Create action to create new login and add to action group "win"
        let action_new_list = gio::SimpleAction::new("new-login", None);
        action_new_list.connect_activate(clone!(@weak self as window => move |_, _| {
            window.new_login();
        }));
        self.add_action(&action_new_list);
    }

    fn new_login(&self) {
        // Create new Dialog
        let dialog = Dialog::with_buttons(
            Some("New Item"),
            Some(self),
            DialogFlags::MODAL | DialogFlags::DESTROY_WITH_PARENT | DialogFlags::USE_HEADER_BAR,
            &[
                ("Cancel", ResponseType::Cancel),
                ("Create", ResponseType::Accept),
            ],
        );
        dialog.set_default_response(ResponseType::Accept);

        // Make the dialog button insensitive initially
        let dialog_button = dialog
            .widget_for_response(ResponseType::Accept)
            .expect("The dialog needs to have a widget for response type `Accept`.");
        dialog_button.set_sensitive(false);

        // Create entry and add it to the dialog
        let entry = Entry::builder()
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .placeholder_text("Name")
            .activates_default(true)
            .build();
        dialog.content_area().append(&entry);

        // Set entry's css class to "error", when there is not text in it
        entry.connect_changed(clone!(@weak dialog => move |entry| {
            let text = entry.text();
            let dialog_button = dialog.
                widget_for_response(ResponseType::Accept).
                expect("The dialog needs to have a widget for response type `Accept`.");
            let empty = text.is_empty();

            dialog_button.set_sensitive(!empty);

            if empty {
                entry.add_css_class("error");
            } else {
                entry.remove_css_class("error");
            }
        }));

        // Connect response to dialog
        dialog.connect_response(
            clone!(@weak self as window, @weak entry => move |dialog, response| {
                // Destroy dialog
                dialog.destroy();

                // Return if the user chose a response different than `Accept`
                if response != ResponseType::Accept {
                    return;
                }

                // Create a new login object from the title the user provided
                let title = entry.text().to_string();
                let login = LoginObject::new(&title);

                // Add new login object and set current tasks
                window.logins().append(&login);
                window.set_current_login(login);

                // Let the leaflet navigate to the next child
                window.imp().leaflet.navigate(NavigationDirection::Forward);
            }),
        );
        dialog.present();
    }
}
