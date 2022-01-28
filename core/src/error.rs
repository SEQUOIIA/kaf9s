use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("context `{0}` could not be found")]
    ContextNotFound(String),
    #[error("unknown config error")]
    Unknown,
    #[error("unsupported kafka.security.protocol option")]
    InvalidKafkaSecurityProtocol,
    #[error("config error: {0:?}")]
    CustomError(Box<dyn std::error::Error>),
    #[error("config error: {0}")]
    Message(String)
}