mod imp;

use adw::prelude::*;
use adw::subclass::prelude::*;
use adw::NavigationDirection;
use glib::{clone, Object};
use gtk::glib::BindingFlags;
use gtk::{
    gio, glib, pango, Dialog, DialogFlags, Entry, Label, ListBoxRow, PasswordEntry, ResponseType,
    SelectionMode,
};

use crate::login_object::LoginObject;
use crate::utils::{database_exists, read_database};

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

    fn master_password(&self) -> String {
        self.imp().master_password.borrow().clone()
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

    fn unlock_and_restore(&self, password: &String) -> Result<(), ()> {
        if database_exists() {
            // Read database
            if let Ok(data) = read_database(password) {
                // Convert `Vec<LoginData>` to `Vec<LoginObject>`
                let logins: Vec<LoginObject> =
                    data.into_iter().map(LoginObject::from_login_data).collect();

                // Insert restored objects into model
                self.logins().extend_from_slice(&logins);

                // Set first login as current
                if let Some(first_login) = logins.first() {
                    self.set_current_login(first_login.clone());
                }

                return Ok(());
            }
            return Err(());
        }
        Ok(())
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
        let username_field = self.imp().username_field.get();
        let password_field = self.imp().password_field.get();

        // Unbind all fields
        self.unbind();
        let mut bindings = self.imp().bindings.borrow_mut();

        // Bind fields to selected login data
        bindings.append(&mut vec![
            login
                .bind_property("title", &title_field, "text")
                .flags(BindingFlags::BIDIRECTIONAL | BindingFlags::SYNC_CREATE)
                .build(),
            login
                .bind_property("username", &username_field, "text")
                .flags(BindingFlags::BIDIRECTIONAL | BindingFlags::SYNC_CREATE)
                .build(),
            login
                .bind_property("password", &password_field, "text")
                .flags(BindingFlags::BIDIRECTIONAL | BindingFlags::SYNC_CREATE)
                .build(),
        ]);

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
        self.logins()
            .connect_items_changed(clone!(@weak self as window => move |_, _, _, _| {
                window.set_stack();
            }));

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
        if self.master_password().is_empty() {
            if database_exists() {
                self.imp()
                    .stack
                    .set_visible_child_name("password-placeholder");
            } else {
                self.imp().stack.set_visible_child_name("vault-placeholder");
            }
        } else if self.logins().n_items() > 0 {
            self.imp().stack.set_visible_child_name("main");
        } else {
            self.imp().stack.set_visible_child_name("placeholder");
        }
    }

    fn setup_actions(&self) {
        // Create action to create new login and add to action group "win"
        let action_new_login = gio::SimpleAction::new("new-login", None);
        action_new_login.connect_activate(clone!(@weak self as window => move |_, _| {
            window.new_login();
        }));
        self.add_action(&action_new_login);

        let action_remove_login = gio::SimpleAction::new("remove-login", None);
        action_remove_login.connect_activate(clone!(@weak self as window => move |_, _| {
            window.remove_login();
        }));
        self.add_action(&action_remove_login);

        let action_unlock_vault = gio::SimpleAction::new("unlock-vault", None);
        action_unlock_vault.connect_activate(clone!(@weak self as window => move |_, _| {
            window.unlock_vault();
        }));
        self.add_action(&action_unlock_vault);
    }

    fn remove_login(&self) {
        if let Some(index) = self.logins().find(&self.current_login()) {
            self.logins().remove(index);

            if let Some(next_login) = self.logins().item(index) {
                let next_login = next_login.downcast::<LoginObject>().unwrap();
                self.set_current_login(next_login);
            } else if let Some(previous_login) = self.logins().item(index.wrapping_sub(1)) {
                let previous_login = previous_login.downcast::<LoginObject>().unwrap();
                self.set_current_login(previous_login);
            }
        }
    }

    fn unlock_vault(&self) {
        // Create new Dialog
        let dialog = Dialog::with_buttons(
            Some("Unlock Vault"),
            Some(self),
            DialogFlags::MODAL | DialogFlags::DESTROY_WITH_PARENT | DialogFlags::USE_HEADER_BAR,
            &[
                ("Cancel", ResponseType::Cancel),
                ("Unlock", ResponseType::Accept),
            ],
        );
        dialog.set_default_response(ResponseType::Accept);

        // Make the dialog button insensitive initially
        let dialog_button = dialog
            .widget_for_response(ResponseType::Accept)
            .expect("The dialog needs to have a widget for response type `Accept`.");
        dialog_button.set_sensitive(false);

        let entry = PasswordEntry::builder()
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .placeholder_text("Master Password")
            .activates_default(true)
            .show_peek_icon(true)
            .build();
        dialog.content_area().append(&entry);

        // Set entry's css class to "error", when there is no text in it
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
                // Return if the user chose a response different than `Accept`
                if response != ResponseType::Accept {
                    dialog.destroy();
                    return;
                }

                let password = entry.text().to_string();

                // Set entry's css class to "error" if the vault fails to unlock
                if window.unlock_and_restore(&password).is_err() {
                    entry.add_css_class("error");
                    return;
                }

                dialog.destroy();
                window.imp().master_password.replace(password);
                window.set_stack();

                // Let the leaflet navigate to the next child
                window.imp().leaflet.navigate(NavigationDirection::Forward);
            }),
        );
        dialog.present();
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

        // create entry and add it to the dialog
        let entry = Entry::builder()
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .placeholder_text("name")
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
