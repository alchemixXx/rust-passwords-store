pub type CustomResult<T> = core::result::Result<T, CustomError>;

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum CustomError {
    CommandExecution(String),
    UnknownBranch,
    ConfigParsingError(String),
    NpmConfigError(String),
    WrongSecretParams(String),
    CipherError(String),
    DerivingKeyError(String),
    MissingFile,
}

impl std::error::Error for CustomError {}
impl core::fmt::Display for CustomError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(f, "{self:?}")
    }
}
