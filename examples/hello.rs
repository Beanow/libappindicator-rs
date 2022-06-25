use std::path::{Path, PathBuf};

use gtk::prelude::*;
use libappindicator::{AppIndicator, AppIndicatorStatus};

fn main() {
  gtk::init().unwrap();

  let mut indicator = AppIndicator::new("libappindicator test application", "");
  indicator.set_status(AppIndicatorStatus::Active);

  let icon_path: Option<PathBuf> = if let Some(dir) = option_env!("TRAY_ICON_DIR") {
    match dir {
      "" => None,
      dir => Some(Path::new(dir).to_path_buf()),
    }
  } else if let Some(dir) = option_env!("CARGO_MANIFEST_DIR") {
    Some(Path::new(dir).join("examples"))
  } else {
    None
  };

  if let Some(icon_path) = icon_path {
    indicator.set_icon_theme_path(icon_path.to_str().unwrap());
  }

  let icon_name = option_env!("TRAY_ICON_NAME").unwrap_or("rust-logo");
  let icon_desc = option_env!("TRAY_ICON_DESC").unwrap_or("icon");
  indicator.set_icon_full(icon_name, icon_desc);

  let mut m = gtk::Menu::new();
  let mi = gtk::CheckMenuItem::with_label("Hello Rust!");
  mi.connect_activate(|_| {
    gtk::main_quit();
  });
  m.append(&mi);
  indicator.set_menu(&mut m);
  m.show_all();

  gtk::main();
}
