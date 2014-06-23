
#![feature(globs)]

extern crate rgtk;
extern crate log;
extern crate debug;
extern crate collections;

use log::macros::*;

use rgtk::*;
use rgtk::gtk::signals;

use collections::String;

#[doc(hidden)]
#[cfg(target_os="macos")]
mod platform {
    #[link(name = "glib-2.0")]
    #[link(name = "gtk-3.0")]
    #[link(name = "gobject-2.0")]
    #[link(name = "gdk-3.0")]
    extern{}
}

#[doc(hidden)]
#[cfg(target_os="linux")]
mod platform {
    #[link(name = "glib-2.0")]
    #[link(name = "gtk-3")]
    #[link(name = "gobject-2.0")]
    #[link(name = "gdk-3")]
    extern{}
}

fn main() {
    gtk::init();
    println!("Major: {}, Minor: {}", gtk::version::get_major_version(), gtk::version::get_minor_version());
    let mut window = gtk::Window::new(gtk::WindowTopLevel).unwrap();
    let mut frame = gtk::Frame::new(Some("Yep a frame")).unwrap();
    let mut _box = gtk::_Box::new(gtk::OrientationHorizontal, 10).unwrap();
    let mut v_box = gtk::_Box::new(gtk::OrientationHorizontal, 10).unwrap();
    let mut button_box = gtk::ButtonBox::new(gtk::OrientationHorizontal).unwrap();
    let mut label = gtk::Label::new("Yeah a wonderful label too !").unwrap();
    let mut button = gtk::Button::new_with_label("Whattttt a button !").unwrap();
    let font_button = gtk::FontButton::new().unwrap();
    let toggle_button = gtk::ToggleButton::new_with_label("Toggle Me !").unwrap();
    let check_button = gtk::CheckButton::new_with_label("Labeled check button").unwrap();
    let color_button = gtk::ColorButton::new().unwrap();
    let menu_button = gtk::MenuButton::new().unwrap();
    let link_button = gtk::LinkButton::new("www.rust-lang.org").unwrap();
    let mut volume_button = gtk::VolumeButton::new().unwrap();
    let mut entry = gtk::Entry::new().unwrap();
    let search_entry = gtk::SearchEntry::new().unwrap();
    let separator = gtk::Separator::new(gtk::OrientationHorizontal).unwrap();
    let separator2 = gtk::Separator::new(gtk::OrientationHorizontal).unwrap();
    let switch = gtk::Switch::new().unwrap();
    let mut switch2 = gtk::Switch::new().unwrap();
    let scale = gtk::Scale::new_with_range(gtk::OrientationHorizontal, 0., 100., 1.).unwrap();
    let mut level_bar = gtk::LevelBar::new_for_interval(0., 100.).unwrap();
    let spin_button = gtk::SpinButton::new_with_range(0., 100., 1.).unwrap();
    let mut spinner = gtk::Spinner::new().unwrap();
    let image = gtk::Image::new_from_file("./resources/gtk.jpg").unwrap();
    let mut progress_bar = gtk::ProgressBar::new().unwrap();
    let arrow = gtk::Arrow::new(gtk::ArrowRight, gtk::ShadowEtchedOut).unwrap();
    let calendar = gtk::Calendar::new().unwrap();
    let mut info_bar = gtk::InfoBar::new().unwrap();

    println!("test");

    let mut test = 0;
    info_bar.show_close_button(true);

    /*info_bar.connect(signals::Response::new(|response_id| {
        info_bar.hide()
    }));*/ //TODO: Why does this not work?

    progress_bar.set_fraction(0.7);
    spinner.start();
    level_bar.set_value(37.);
    switch2.set_active(true);
    frame.set_border_width(10);
    _box.set_border_width(5);
    entry.set_placeholder("An Entry with a placeholder !");
    volume_button.set_orientation(gtk::OrientationHorizontal);
    label.set_justify(gtk::JustifyLeft);
    window.set_title("Yeah a beautiful window with rgtk !");
    window.add(&frame);

    button.connect(signals::Clicked::new(||{
        entry.set_text("Clicked!".to_string());
    }));

    window.connect(signals::DeleteEvent::new(|event_type|{
        gtk::main_quit();
        true
    }));

    frame.add(&_box);
    button_box.add(&button);
    button_box.add(&font_button);
    button_box.add(&toggle_button);
    button_box.add(&color_button);
    button_box.add(&volume_button);
    v_box.add(&switch);
    v_box.add(&menu_button);
    v_box.add(&switch2);
    v_box.add(&check_button);
    v_box.add(&link_button);
    v_box.add(&spin_button);
    _box.add(&info_bar);
    _box.add(&v_box);
    _box.add(&scale);
    _box.add(&level_bar);
    _box.add(&button_box);
    _box.add(&progress_bar);
    _box.add(&separator);
    _box.add(&label);
    _box.add(&entry);
    _box.add(&separator2);
    _box.add(&search_entry);
    _box.add(&spinner);
    _box.add(&image);
    _box.add(&arrow);
    _box.add(&calendar);
    _box.set_orientation(gtk::OrientationVertical);

    window.show_all();
    gtk::main();
}

