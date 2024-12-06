#[derive(Debug)]
pub enum Error {
    ErrAreaUsecaseNotSet,
    ErrInvalidBaseUrl,
    ErrListAreaFailed,
    ErrFetchPageFailed,
    ErrSavePageFailed,
    ErrCreateDirFailed,
}
