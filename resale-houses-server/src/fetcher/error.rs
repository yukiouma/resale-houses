#[derive(Debug)]
pub enum Error {
    ErrRepoNotSet,
    ErrInvalidBaseUrl,
    ErrInvalidBaseDir,
    ErrListAreaFailed,
    ErrFetchPageFailed,
    ErrSavePageFailed,
    ErrCreateDirFailed,
}
