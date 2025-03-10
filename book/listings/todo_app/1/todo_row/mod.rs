mod imp;

use crate::todo_object::TodoObject;
use glib::{BindingFlags, Object};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, pango};
use pango::{AttrList, Attribute};

glib::wrapper! {
    pub struct TodoRow(ObjectSubclass<imp::TodoRow>)
    @extends gtk::Box, gtk::Widget;
}

impl Default for TodoRow {
    fn default() -> Self {
        Self::new()
    }
}

impl TodoRow {
    pub fn new() -> Self {
        Object::new(&[]).expect("Failed to create `TodoRow`.")
    }

    pub fn bind(&self, todo_object: &TodoObject) {
        // Get state
        let imp = imp::TodoRow::from_instance(self);
        let completed_button = imp.completed_button.get();
        let content_label = imp.content_label.get();

        // Bind `todo_object.completed` to `todo_row.completed_button.active` and save binding
        // The binding is bidirectional so changes on both sides will be mirrored by the other side
        let completed_button_binding = todo_object
            .bind_property("completed", &completed_button, "active")
            .flags(BindingFlags::SYNC_CREATE | BindingFlags::BIDIRECTIONAL)
            .build()
            .expect("Could not bind properties");
        imp.bindings.borrow_mut().push(completed_button_binding);

        // Bind `todo_object.content` to `todo_row.content_label.label` and save binding
        // The binding is bidirectional so changes on both sides will be mirrored by the other side
        let content_label_binding = todo_object
            .bind_property("content", &content_label, "label")
            .flags(BindingFlags::SYNC_CREATE | BindingFlags::BIDIRECTIONAL)
            .build()
            .expect("Could not bind properties");
        imp.bindings.borrow_mut().push(content_label_binding);

        // Bind `todo_row.completed_button.active` to `todo_row.content_label.attributed` and save binding
        // We transform the boolean "active" so that whenever "active" is true
        // the content of the label will be strikethrough
        completed_button
            .bind_property("active", &content_label, "attributes")
            .flags(BindingFlags::SYNC_CREATE | BindingFlags::DEFAULT)
            .transform_to(|_, completed_value| {
                let attribute_list = AttrList::new();
                let completed = completed_value
                    .get::<bool>()
                    .expect("The value needs to be of type `bool`.");
                if completed {
                    let attribute = Attribute::new_strikethrough(true);
                    attribute_list.insert(attribute);
                }
                Some(attribute_list.to_value())
            })
            .build()
            .expect("Could not bind properties");
    }

    pub fn unbind(&self) {
        // Get state
        let imp = imp::TodoRow::from_instance(self);

        // Unbind all stored bindings
        for binding in imp.bindings.borrow().iter() {
            binding.unbind();
        }
        // Clear the vector
        imp.bindings.borrow_mut().clear();
    }
}
