use crate::{repository::shortcut::ShortcutRepository, service::shortcut::ShortcutService};

#[derive(Clone)]
pub struct AppState {
  pub shortcut_service: ShortcutService<ShortcutRepository>,
}

impl AppState {
  pub fn new(shortcut_service: ShortcutService<ShortcutRepository>) -> Self {
    Self {
      shortcut_service,
    }
  }
}