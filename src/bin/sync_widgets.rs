//! # Synchronizing Widgets
//!
//! You can use signals in order to synchronize the values of widgets. In this example a spin button and a horizontal scale will get interlocked.

extern crate gtk;

use gtk::Builder;
use gtk::prelude::*;

fn main() {
    gtk::init().expect("Failed to initialize GTK.");
    let glade_src = include_str!("sync_widgets.glade");
    let builder = Builder::new();
    builder.add_from_string(glade_src).unwrap();

    let slider: gtk::Scale = builder.get_object("slider").unwrap();
    let spin_button: gtk::SpinButton = builder.get_object("spin_button").unwrap();
    let slider_adj = slider.get_adjustment();
    spin_button.get_adjustment().connect_value_changed(move |adj| {
        slider_adj.set_value(adj.get_value());
    });
    let spin_button_adj = spin_button.get_adjustment();
    slider.get_adjustment().connect_value_changed(move |adj| {
        spin_button_adj.set_value(adj.get_value());
    });

    let window: gtk::Window = builder.get_object("window").unwrap();
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.show_all();

    gtk::main();
}
