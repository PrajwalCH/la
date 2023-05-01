use std::cell::RefCell;

use gtk::glib;
use gtk::glib::PropertySet;
use gtk::prelude::*;

pub trait Results {
    fn rows(&self) -> Vec<gtk::ListBoxRow>;
    fn on_row_selected(&self, index: usize);
}

pub struct SearchResults {
    scrollable_container: gtk::ScrolledWindow,
    container: gtk::ListBox,
    results_row: Vec<gtk::ListBoxRow>,
    selected_handler_id: RefCell<Option<glib::SignalHandlerId>>,
}

impl SearchResults {
    pub fn new() -> Self {
        let scrollable_container = gtk::ScrolledWindow::new();
        scrollable_container.set_min_content_height(200);
        // Only show it when we get the results later
        scrollable_container.set_visible(false);

        let container = gtk::ListBox::new();
        scrollable_container.set_child(Some(&container));

        Self {
            scrollable_container,
            container,
            results_row: Vec::new(),
            selected_handler_id: RefCell::new(None),
        }
    }

    pub fn container(&self) -> &gtk::ScrolledWindow {
        &self.scrollable_container
    }

    pub fn show<R: Results + 'static>(&mut self, results: R) {
        self.results_row.extend(results.rows());

        for row in &self.results_row {
            self.container.append(row);
        }
        let signal_handler_id = self.container.connect_row_selected(move |container, row| {
            if let Some(row) = row {
                results.on_row_selected(row.index() as usize);
                container
                    .activate_action("window.close", None)
                    .expect("`window.close` action should exist");
            };
        });
        self.selected_handler_id.set(Some(signal_handler_id));
        self.scrollable_container.show();
    }

    pub fn clear(&mut self) {
        for result in self.results_row.iter() {
            self.container.remove(result);
        }
        if let Some(selected_handler_id) = self.selected_handler_id.take() {
            self.container.disconnect(selected_handler_id);
        }
        self.results_row.clear();
        self.scrollable_container.hide();
    }
}
