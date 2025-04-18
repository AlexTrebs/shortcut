use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShortcutError {
    #[error("shortcut not found.")]
    NotFound,
    #[error("Failed to get shortcuts.")]
    FailedToGet,
    #[error("Failed to create shortcut. Please try again.")]
    FailedToCreate,
    #[error("Keyword is not unique.")]
    UniqueConstraintError,
    #[error("Failed to update shortcut. Please try again.")]
    FailedToUpdate,
    #[error("Failed to find match for keyword, would you like to create one?.")]
    NoMatches,
    #[error("Failed to search shortcuts.")]
    FailedToSearch,
    #[error("Failed to delete shortcuts.")]
    FailedToDelete
}