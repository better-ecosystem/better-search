use gtk::{
    ApplicationWindow, Box as GtkBox, Entry, Label, Orientation,
    CssProvider, StyleContext,
    gdk, prelude::*,
};


pub fn apply_css() {
    let provider = CssProvider::new();
    provider
        .load_from_data(b"
            window {
                border-radius: 20px;
                padding: 12px;
            }

            entry {
                border-radius: 6px;
                padding: 8px;
                background-color: transparent;
                border: none;
                font-size:15px;
            }

            box.result-row {
                background-color: rgba(255, 255, 255, 0.03); 
                border-radius: 8px;
                padding: 10px;
                margin-bottom: 6px;
                transition: background-color 150ms ease;
            }

            box.result-row:hover {
                background-color: rgba(255, 255, 255, 0.07); 
            }

            box.result-row:selected,
            box.result-row.selected {
                background-color: rgba(255, 255, 255, 0.12); 
            }

            label.result-label {
                font-size: 14px;
            }
        ").unwrap();

    StyleContext::add_provider_for_screen(
        &gdk::Screen::default().unwrap(),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

pub fn build_main_ui(app: &gtk::Application) -> (ApplicationWindow, Entry, Label, GtkBox) {
    let window = ApplicationWindow::new(app);

    window.set_border_width(0);
    window.set_default_size(800, 450);   
    window.set_resizable(false);


    window.set_keep_above(true);
    window.set_skip_taskbar_hint(true);
    window.set_position(gtk::WindowPosition::Center);
    window.set_decorated(false);
    window.set_accept_focus(true);

    window.set_skip_pager_hint(true);    
    window.set_type_hint(gdk::WindowTypeHint::Dialog);




    let outer_vbox = GtkBox::new(Orientation::Vertical, 0);
    outer_vbox.set_margin_top(12);
    outer_vbox.set_margin_bottom(12);
    outer_vbox.set_margin_start(12);
    outer_vbox.set_margin_end(12);

    let titlebar = GtkBox::new(Orientation::Horizontal, 4);
    let mode_label = Label::new(Some("Apps"));
    mode_label.set_xalign(0.0);
    let diamond1 = Label::new(Some("")); //[coming soon]

    titlebar.pack_start(&mode_label, true, true, 0);
    titlebar.pack_end(&diamond1, false, false, 4);

    let entry = Entry::new();
    entry.set_placeholder_text(Some("name of apps/files"));
    
    use gtk::{ScrolledWindow, PolicyType};

    let result_box = GtkBox::new(Orientation::Vertical, 6);
    result_box.set_margin_top(8);
    result_box.set_margin_bottom(8);
    
    let scroll = ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
    scroll.set_policy(PolicyType::Never, PolicyType::Automatic);
    scroll.set_vexpand(true); 
    scroll.add(&result_box);
    
    outer_vbox.pack_start(&titlebar, false, false, 0);
    outer_vbox.pack_start(&entry, false, false, 8);
    outer_vbox.pack_end(&scroll, true, true, 0);


    window.add(&outer_vbox);

    (window, entry, mode_label, result_box)
}

