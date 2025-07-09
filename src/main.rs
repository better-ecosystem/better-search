use std::fs;
use std::path::Path;
use std::process::Command;
use std::env;
use std::cell::Cell;
use std::rc::Rc;
mod openers;
use openers::{get_openers};
use gtk::{
    gdk_pixbuf::Pixbuf, prelude::*, Application, ApplicationWindow, Box as GtkBox,
    Entry, Orientation, Label, Image,
};
use gio::{Icon, ThemedIcon, FileIcon, prelude::FileExt};
use glib::clone;
use glib::Propagation::{Proceed, Stop};

mod ui;
use ui::{apply_css, build_main_ui};

mod files;


fn main() {
    let app = Application::new(Some("com.better.search"), Default::default());

    app.connect_activate(|app| {
        apply_css();
        let (window, entry, mode_label, result_box) = build_main_ui(app);
        setup_search_ui(&entry, &result_box, &mode_label, &window);
        window.show_all();
        entry.grab_focus();
    });

    app.run();
}



pub fn search_apps(query: &str) -> Vec<(String, String, Image)> {
    let mut results = Vec::new();
    let openers_config = get_openers();
    let app_dirs = openers_config.app_dirs.clone();

    for dir in app_dirs {
        let path = Path::new(&dir);
        if !path.exists() {
            continue;
        }

        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                if let Some(fname) = entry.file_name().to_str().map(|s| s.to_string()) {
                    if !fname.ends_with(".desktop") {
                        continue;
                    }

                    let name = fname.strip_suffix(".desktop").unwrap_or("");
                    if name.to_lowercase().starts_with(query) {
                        let full_path = entry.path().to_string_lossy().to_string();
                        let icon = Image::from_icon_name(Some("application-x-executable"), gtk::IconSize::SmallToolbar);
                        results.push((name.to_string(), full_path, icon));
                    }
                }
            }
        }
    }

    results
}

pub fn web_search(query: &str) {
    let url = format!("https://www.duckduckgo.com/search?q={}", query);
    Command::new("xdg-open").arg(&url).spawn().ok();
}

use shell_escape::escape;

use crate::files::get_files;

pub fn open_with_configured_app(filepath: &str) {
    let ext = Path::new(filepath)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    let escaped = escape(filepath.into()).to_string();

    let openers = get_openers(); 
    let template = openers.openers.get(ext.as_str());

    let command = match template {
        Some(cmd) => cmd.replace("{file}", &escaped),
        None => format!("xdg-open {}", escaped),
    };


    println!("Launching: {}", command);

    let _ = Command::new("sh")
        .arg("-c")
        .arg(&command)
        .spawn()
        .unwrap_or_else(|e| {
            eprintln!("Failed to launch: {}", e);
            std::process::exit(1);
        });
}


fn launch_desktop_entry(path: &str) {
    if let Ok(content) = std::fs::read_to_string(path) {
        if let Some(exec_line) = content.lines().find(|line| line.starts_with("Exec=")) {
            let command_line = exec_line.trim_start_matches("Exec=")
                .split_whitespace()
                .map(|s| s.replace("%U", "").replace("%u", "").replace("%F", "").replace("%f", ""))
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>();

            if let Some((program, args)) = command_line.split_first() {
                let _ = std::process::Command::new(program)
                    .args(args)
                    .spawn();
            }
        }
    }
}

pub fn setup_search_ui(entry: &Entry, result_box: &GtkBox, mode_label: &Label, window: &ApplicationWindow) {
    let mode_app = Rc::new(Cell::new(true));
    let selected_index = Rc::new(Cell::new(0));

    refresh_results("".to_string(), result_box, &mode_app, &selected_index);
    highlight_selected_row(result_box, selected_index.get());


    // Toggle mode on Tab key
    let mode_clone = mode_app.clone();
    let label_clone = mode_label.clone();
    let result_box_clone = result_box.clone();
    let selected_index_clone = selected_index.clone();

    entry.connect_key_press_event(move |_, event| {
        if event.keyval() == gdk::keys::constants::Tab {
            let new_mode = !mode_clone.get();
            mode_clone.set(new_mode);
    
            label_clone.set_text(if new_mode {
                "Apps"
            } else {
                "Files"
            });
    
            refresh_results("".to_string(), &result_box_clone, &mode_clone, &selected_index_clone);
            highlight_selected_row(&result_box_clone, selected_index_clone.get()); // ← 👈 move it here
            Stop
        } else {
            Proceed
        }
    });



    entry.connect_key_press_event(clone!(@weak result_box, @strong selected_index => @default-return Proceed, move |_, event| {
        let children: Vec<_> = result_box.children();
        if children.is_empty() {
            return Proceed;
        }

        match event.keyval() {
            gdk::keys::constants::Up => {
                let mut idx = selected_index.get();
                if idx > 0 { idx -= 1; }
                selected_index.set(idx);
            },
            gdk::keys::constants::Down => {
                let mut idx = selected_index.get();
                if idx + 1 < children.len() { idx += 1; }
                selected_index.set(idx);
            },
            _ => return Proceed,
        }

        for (_i, _child) in children.iter().enumerate() {
           highlight_selected_row(&result_box, selected_index.get()); 
        }

        Stop
    }));

    let window_clone = window.clone();
    let entry_clone = entry.clone();
    entry.connect_activate(clone!(@strong entry_clone, @strong window_clone, @strong mode_app, @strong selected_index => move |_| {
        let query = entry_clone.text();
        if !query.is_empty() {
            let results = if mode_app.get() {
                search_apps(&query)
            } else {
                get_files(&query)
            };

            match results.get(selected_index.get()) {
                Some((_, path, _)) => {
                    println!("Selected file: {}", path);
                    if mode_app.get() {
                        launch_desktop_entry(path);
                    } else {
                        open_with_configured_app(path);
                    }
                },
                None => {
                    println!("No result selected, doing web search...");
                    web_search(&query);
                }
            }

            window_clone.close();
        }
    }));

entry.connect_changed(clone!(@weak result_box, @strong mode_app, @strong selected_index => move |entry| {
    let query = entry.text().to_string().to_lowercase();
    result_box.foreach(|child| result_box.remove(child));
    highlight_selected_row(&result_box, selected_index.get());
    selected_index.set(0);

    if query.is_empty() {
        return;
    }

    let results = if mode_app.get() {
    search_apps(&query)
} else {
    get_files(&query)
};

if results.is_empty() {
    let row = GtkBox::new(Orientation::Horizontal, 6);
    row.style_context().add_class("result-row");

    let icon = Image::from_icon_name(Some("system-search-symbolic"), gtk::IconSize::SmallToolbar);
    row.pack_start(&icon, false, false, 0);

    let label = Label::new(None);
    label.style_context().add_class("result-label");
    label.set_use_markup(true);
    label.set_xalign(0.0);
    label.set_line_wrap(true);
    label.set_max_width_chars(80);
    label.set_ellipsize(pango::EllipsizeMode::End);

    label.set_markup(&format!(
        "<span foreground='#aaa'>Search for: \n</span><span foreground='#fff' weight='bold'>{}</span>",
        glib::markup_escape_text(&query)
    ));

    row.pack_start(&label, true, true, 0);
    result_box.pack_start(&row, false, false, 0);
    row.show_all();
} else {
    for (name, path, icon) in results.into_iter().take(50) {
        let row = GtkBox::new(Orientation::Horizontal, 6);
        row.style_context().add_class("result-row");

        let label = Label::new(None);
        label.style_context().add_class("result-label");
        label.set_use_markup(true);
        label.set_xalign(0.0);
        label.set_line_wrap(true);
        label.set_max_width_chars(80);
        label.set_ellipsize(pango::EllipsizeMode::End);

        label.set_markup(&format!(
            "<span foreground='#fff' weight='bold'>{}</span>\n<span size='small' foreground='#888'>{}</span>",
            glib::markup_escape_text(&name),
            glib::markup_escape_text(&path)
        ));

        row.pack_start(&icon, false, false, 0);
        row.pack_start(&label, true, true, 0);
        result_box.pack_start(&row, false, false, 0);
        row.show_all();
    }
}

    highlight_selected_row(&result_box, selected_index.get());

}));





fn refresh_results(query: String, result_box: &GtkBox, mode_app: &Cell<bool>, selected_index: &Cell<usize>) {
    result_box.foreach(|child| result_box.remove(child));
    selected_index.set(0);

    let results = if mode_app.get() {
        search_apps(&query)
    } else {
        get_files(&query)
    };

    for (name, path, icon) in results.into_iter().take(50) {
        let row = GtkBox::new(Orientation::Horizontal, 6);
        row.style_context().add_class("result-row");

        row.pack_start(&icon, false, false, 0);

        let label = Label::new(None);
        label.style_context().add_class("result-label");
        label.set_use_markup(true);
        label.set_xalign(0.0);
        label.set_line_wrap(true);
        label.set_max_width_chars(80);
        label.set_ellipsize(pango::EllipsizeMode::End);

        label.set_markup(&format!(
            "<span foreground='#fff' weight='bold'>{}</span>\n<span size='small' foreground='#888'>{}</span>",
            glib::markup_escape_text(&name),
            glib::markup_escape_text(&path)
        ));

        row.pack_start(&label, true, true, 0);
        result_box.pack_start(&row, false, false, 0);
        row.show_all();
    }
}



}

fn highlight_selected_row(result_box: &GtkBox, selected_index: usize) {
    let children: Vec<_> = result_box.children();

    for (i, child) in children.iter().enumerate() {
        if let Some(box_) = child.downcast_ref::<GtkBox>() {
            if let Some(label_widget) = box_.children().get(1) {
                if let Some(label) = label_widget.downcast_ref::<Label>() {
                    let markup = format!(
                        "{}{}{}",
                        if i == selected_index { "<b>" } else { "" },
                        glib::markup_escape_text(&label.text()),
                        if i == selected_index { "</b>" } else { "" }
                    );
                    label.set_markup(&markup);
                }
            }

            if i == selected_index {
                box_.style_context().add_class("selected");
            } else {
                box_.style_context().remove_class("selected");
            }
        }
    }
}

