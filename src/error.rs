use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShortcutError {
    #[error("shortcut not found.")]
    NotFound,
    #[error("Failed to get shortcuts.")]
    FailedToGet,
    #[error("Failed to create shortcut. Please try again.")]
    FailedToCreate,
    #[error("Failed to update shortcut. Please try again.")]
    FailedToUpdate,
    #[error("Failed to create or update shortcut. Please try again.")]
    FailedToCreateOrUpdate,
    #[error("Failed to delete shortcut. Please try again.")]
    FailedToDelete,
    #[error("Failed to create shortcut. Empty shortcut.")]
    EmptyShortcut,
}