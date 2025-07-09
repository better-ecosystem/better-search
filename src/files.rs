use std::path::Path;

use gdk_pixbuf::Pixbuf;
use gio::{traits::FileExt, FileIcon, Icon, ThemedIcon};
use gtk::Image;
use walkdir::WalkDir;

pub fn get_files(query: &str) -> Vec<(String, String, Image)> {
    let mut paths: Vec<(String, String, Image)> = Vec::new();

    /* Since the program will most likely be ran from a keybind or something similar,
       we just use HOME as the base directory.
    */
    let home: String = std::env::var("HOME").unwrap_or(String::from("~/"));
    let entries = WalkDir::new(home).into_iter().filter_map(|e| e.ok());
    let query_lower = query.to_lowercase();

    for entry in entries {
        let dir: &Path = entry.path();

        if !dir.is_file() {
            continue;
        }

        let name = match dir.file_name().and_then(|s| s.to_str()) {
            Some(name) => name.to_string(),
            None => continue,
        };

        if !name.to_lowercase().contains(&query_lower) {
            continue;
        }

        let path = match dir.to_str() {
            Some(p) => p.to_string(),
            None => continue,
        };

        let icon = get_file_icon(dir);

        paths.push((name, path, icon));
    }

    paths.sort_by(|a, b| a.1.cmp(&b.1));
    paths.dedup_by(|a, b| a.1 == b.1);

    paths
}

fn get_file_icon(filepath: &Path) -> Image {
    let guessed = gio::content_type_guess(Some(filepath), &[]);
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

    Image::from_icon_name(
        Some("application-x-executable"),
        gtk::IconSize::SmallToolbar,
    )
}
