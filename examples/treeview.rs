//! # TreeView Sample
//!
//! This sample demonstrates how to create a TreeView with either a ListStore or TreeStore.

#![feature(globs)]
#![crate_type = "bin"]

extern crate rgtk;

use rgtk::*;
use rgtk::gtk::signals;
use std::ptr;

fn append_text_column(tree: &mut gtk::TreeView) {
    let column = gtk::TreeViewColumn::new().unwrap();
    let cell = gtk::CellRendererText::new().unwrap();
    column.pack_start(&cell, true);
    column.add_attribute(&cell, "text", 0);
    tree.append_column(&column);
}

fn main() {
    gtk::init();

    let mut window = gtk::Window::new(gtk::window_type::TopLevel).unwrap();
    window.set_title("TreeView Sample");
    window.set_window_position(gtk::window_position::Center);

    window.connect(signals::DeleteEvent::new(|_| {
        gtk::main_quit();
        true
    }));

    // left pane

    let mut left_tree = gtk::TreeView::new().unwrap();
    let column_types = vec![glib::ffi::g_type_string];
    let left_store = gtk::ListStore::new(column_types).unwrap();
    let left_model = left_store.get_model().unwrap();
    left_tree.set_model(&left_model);
    left_tree.set_headers_visible(false);
    append_text_column(&mut left_tree);

    let mut iter = gtk::ffi::C_GtkTreeIter;
    left_model.get_iter_first(&mut iter);
    for _ in range(0i, 10i) {
        left_store.append(&mut iter);
        left_store.set_column_text(&mut iter, 0, "I'm in a list");
    }

    // right pane

    let mut right_tree = gtk::TreeView::new().unwrap();
    let column_types = vec![glib::ffi::g_type_string];
    let right_store = gtk::TreeStore::new(column_types).unwrap();
    let right_model = right_store.get_model().unwrap();
    right_tree.set_model(&right_model);
    right_tree.set_headers_visible(false);
    append_text_column(&mut right_tree);

    let mut iter = gtk::ffi::C_GtkTreeIter;
    right_model.get_iter_first(&mut iter);
    for _ in range(0i, 10i) {
        right_store.append(&mut iter, ptr::null_mut());
        right_store.set_column_text(&mut iter, 0, "I'm in a tree");

        let mut child = gtk::ffi::C_GtkTreeIter;
        right_store.append(&mut child, &mut iter);
        right_store.set_column_text(&mut child, 0, "I'm a child node");
    }

    // display the panes

    let mut split_pane = gtk::Box::new(gtk::orientation::Horizontal, 10).unwrap();
    split_pane.set_size_request(-1, -1);
    split_pane.add(&left_tree);
    split_pane.add(&right_tree);

	window.add(&split_pane);
    window.show_all();
    gtk::main();
}
