#[derive(Debug)]
pub enum Error {
    ErrAreaUsecaseNotSet,
    ErrInvalidBaseUrl,
    ErrInvalidBaseDir,
    ErrListAreaFailed,
    ErrFetchPageFailed,
    ErrSavePageFailed,
    ErrCreateDirFailed,
}
