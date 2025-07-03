use std::fs;
use std::path::Path;
use gtk::{gdk_pixbuf::Pixbuf, prelude::*, Image};
use gio::{prelude::*, Icon, ThemedIcon, FileIcon};
use gio::prelude::FileExt;
use std::env;

pub fn search_apps(query: &str) -> Vec<(String, String, Image)> {
    let mut results = Vec::new();

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

pub fn search_files(query: &str) -> Vec<(String, String, Image)> {
    let mut results = Vec::new();
    let home = env::var("HOME").unwrap_or("~".to_string());

    for entry in walkdir::WalkDir::new(&home).into_iter().filter_map(Result::ok) {
        if !entry.file_type().is_file() {
            continue;
        }

        let fname = entry.file_name().to_string_lossy();
        if fname.to_lowercase().starts_with(query) {
            let full_path = entry.path().to_string_lossy().to_string();
            let icon = get_file_icon(&full_path);
            results.push((fname.to_string(), full_path, icon));
        }

        if results.len() > 100 {
            break;
        }
    }

    results
}

fn get_file_icon(filepath: &str) -> Image {
    let guessed = gio::content_type_guess(Some(Path::new(filepath)), &[]);
    let content_type = guessed.0;
    let icon = gio::content_type_get_icon(&content_type);
    icon_to_image(&icon)
}

fn icon_to_image(icon: &Icon) -> Image {
    use gio::prelude::Cast;

    if let Some(themed) = icon.downcast_ref::<ThemedIcon>() {
        if let Some(name) = themed.names().get(0) {
            return Image::from_icon_name(Some(name.as_str()), gtk::IconSize::SmallToolbar);
        }
    } else if let Some(file_icon) = icon.downcast_ref::<FileIcon>() {
        if let Some(path) = file_icon.file().path() {
            if let Ok(pixbuf) = Pixbuf::from_file_at_size(path, 24, 24) {
                return Image::from_pixbuf(Some(&pixbuf));
            }
        }
    }

    Image::from_icon_name(Some("application-x-executable"), gtk::IconSize::SmallToolbar)
}

