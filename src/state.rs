use crate::service::shortcut::ShortcutService;

#[derive(Clone)]
pub struct AppState {
  pub shortcut_service: ShortcutService,
}

impl AppState {
  pub fn new(shortcut_service: ShortcutService) -> Self {
    Self {
      shortcut_service,
    }
  }
}